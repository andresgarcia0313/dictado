#!/usr/bin/env bash
# Prueba A/B de la mejora de audio con WAV (sin altavoces ni microfono): mejora ON/OFF x ruido ON/OFF.
# Genera 3 frases con piper y 5 variantes con ffmpeg/python en tests/audio/ab-*.wav (si faltan),
# transcribe cada una con el motor y cuenta palabras de la referencia acertadas.
# Falla si la mejora ON es peor que OFF en la voz baja (-35 dB).
set -uo pipefail
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
AUD="$RAIZ/tests/audio"; SAL="$(mktemp -d)"; trap 'rm -rf "$SAL"' EXIT
VOCES="${PIPER_VOCES:-$HOME/.local/share/piper/voices}"
VOZ=(es_MX-ald-medium es_MX-ald-medium es_ES-davefx-medium) # una por frase
BIN="${DICTADO_MOTOR:-$RAIZ/target/release/dictado-motor}"
[ -x "$BIN" ] || { (cd "$RAIZ" && CARGO_BUILD_JOBS=4 timeout 900 nice -n 10 cargo build -q --release -p dictado-motor) && BIN="$RAIZ/target/release/dictado-motor"; }
FRASES=("Hola, buenos días, quiero que me envíes el informe, por favor."
  "Necesito agendar una reunión con el equipo para el próximo martes por la tarde."
  "Gracias por la ayuda, mañana te confirmo el valor de la factura.")
VARIANTES=(original baja35 baja25 ruido5 zumbido50)

generar() {
  mkdir -p "$AUD"
  for i in 0 1 2; do
    o="$AUD/ab-f$i-original.wav"; [ -f "$o" ] && continue
    echo "${FRASES[$i]}" | timeout 300 piper -m "$VOCES/${VOZ[$i]}.onnx" -f "$SAL/p.wav" >/dev/null 2>&1 || { echo "FALLO: piper"; exit 1; }
    ffmpeg -v error -y -i "$SAL/p.wav" -ar 16000 -ac 1 -sample_fmt s16 "$o"
    ffmpeg -v error -y -i "$o" -af volume=-35dB "$AUD/ab-f$i-baja35.wav"
    ffmpeg -v error -y -i "$o" -af volume=-25dB "$AUD/ab-f$i-baja25.wav"
    ffmpeg -v error -y -i "$o" -f lavfi -i "sine=f=50:sample_rate=16000" -filter_complex \
      "[1]volume=0.08[h];[0][h]amix=inputs=2:duration=first:normalize=0" -ar 16000 -ac 1 -sample_fmt s16 "$AUD/ab-f$i-zumbido50.wav"
    python3 - "$o" "$AUD/ab-f$i-ruido5.wav" <<'PY'
import sys, wave, numpy as np
w = wave.open(sys.argv[1]); x = np.frombuffer(w.readframes(w.getnframes()), np.int16).astype(float)
f = np.fft.rfft(np.random.default_rng(7).standard_normal(len(x))); f /= np.sqrt(np.maximum(np.arange(len(f)), 1))
n = np.fft.irfft(f, len(x)); n *= np.sqrt((x**2).mean() / 10**0.5) / np.sqrt((n**2).mean())  # SNR 5 dB, ruido rosa
o = wave.open(sys.argv[2], "wb"); o.setparams((1, 2, 16000, 0, "NONE", "")); o.writeframes(np.clip(x + n, -32768, 32767).astype(np.int16).tobytes())
PY
  done
}

source "$RAIZ/tests/lote-lib.sh"
VAR=(original baja35 baja25 ruido5 zumbido50)
# un arranque del modelo por configuracion (4) en vez de uno por archivo (60)
generar
for m in on off; do for r in no si; do
  a=(); [ "$m" = off ] && a+=(--sin-mejora); [ "$r" = si ] && a+=(--ruido)
  ARCH=(); NOM=(); for v in "${VAR[@]}"; do for i in 0 1 2; do ARCH+=("$AUD/ab-f$i-$v.wav"); NOM+=("$v.$i"); done; done
  lote_correr "$BIN" "$SAL/lote.$m.$r" "${a[*]}" "${ARCH[@]}"
  lote_dividir "$SAL/lote.$m.$r" "$SAL/" ".$m.$r.jsonl" "${NOM[@]}"
done; done

python3 - "$SAL" "${FRASES[@]}" <<'PY'
import sys, json, re, unicodedata, collections
sal, frases = sys.argv[1], sys.argv[2:]
norm = lambda t: re.findall(r"[a-z0-9]+", unicodedata.normalize("NFD", t.lower()).encode("ascii", "ignore").decode())
def puntaje(v, m, r):
    ok = tot = 0
    for i, ref in enumerate(frases):
        ev = [json.loads(l) for l in open(f"{sal}/{v}.{i}.{m}.{r}.jsonl") if l.strip()]
        hip = collections.Counter(norm(" ".join(e["texto"] for e in ev if e["t"] == "final")))
        ok += sum(min(c, hip[p]) for p, c in collections.Counter(norm(ref)).items()); tot += len(norm(ref))
    return ok, tot
cols = [("ON", "on", "no"), ("OFF", "off", "no"), ("ON+ruido", "on", "si"), ("OFF+ruido", "off", "si")]
print(f"{'variante':<12}" + "".join(f"{c[0]:>12}" for c in cols))
res = {}
for v in "original baja35 baja25 ruido5 zumbido50".split():
    res[v] = [puntaje(v, m, r) for _, m, r in cols]
    print(f"{v:<12}" + "".join(f"{o:>9}/{t:<3}" for o, t in res[v]))
(on, _), (off, _) = res["baja35"][0], res["baja35"][1]
print(f"\nvoz baja -35 dB: mejora ON {on} vs OFF {off} palabras")
sys.exit(0 if on >= off else (print("FALLO: la mejora empeora la voz baja") or 1))
PY
