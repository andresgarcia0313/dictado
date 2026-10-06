#!/usr/bin/env bash
# Ruido de calle sintetico frente a voz (sin altavoces ni microfono). Mezcla 3 frases de piper con trafico
# (marron <500 Hz, pasadas con modulacion, bocinas 400/500 Hz en rafagas, viento) a SNR 0, 5 y 10 dB y compara
# con --wav: VAD neuronal vs energia (--vad-energia), compuerta espectral ON (--ruido) vs OFF, calle sola 30 s y limpio.
# Falla si la calle sola da algun final o si el recall de voz con VAD neuronal baja de 90 % a SNR >= 5 dB;
# la fraccion de voz en calle sola (objetivo < 5 %) se informa aparte.
set -uo pipefail
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
AUD="$RAIZ/tests/audio"; SAL="$(mktemp -d)"; trap 'rm -rf "$SAL"' EXIT
BIN="${DICTADO_MOTOR:-$RAIZ/target/release/dictado-motor}"
[ -x "$BIN" ] || { (cd "$RAIZ" && CARGO_BUILD_JOBS=4 timeout 900 nice -n 10 cargo build -q --release -p dictado-motor) && BIN="$RAIZ/target/release/dictado-motor"; }
MED="$RAIZ/tests/ruido-calle-medir.py"
FRASES=("Hola, buenos días, quiero que me envíes el informe, por favor."
  "Necesito agendar una reunión con el equipo para el próximo martes por la tarde."
  "Gracias por la ayuda, mañana te confirmo el valor de la factura.")
for i in 0 1 2; do [ -f "$AUD/ab-f$i-original.wav" ] || { echo "falta $AUD/ab-f$i-original.wav: ejecuta tests/mejora-ab.sh"; exit 1; }; done

gen() { # salida filtro_pasadas filtro_viento (calle sintetica de 30 s: motor marron <500 Hz, pasadas, bocinas, viento)
  ffmpeg -v error -y -f lavfi -i "anoisesrc=color=brown:r=16000:d=30:seed=11" -f lavfi -i "anoisesrc=color=pink:r=16000:d=30:seed=12" \
    -f lavfi -i "sine=f=400:r=16000:d=30" -f lavfi -i "sine=f=500:r=16000:d=30" -f lavfi -i "anoisesrc=color=$4:r=16000:d=30:seed=13" \
    -filter_complex "[0]lowpass=f=500,volume=0.2[m];[1]$2[p];
      [2][3]amix=inputs=2:normalize=0,volume='0.25*(lt(mod(t,13),0.7)+between(mod(t,13),0.9,1.5))':eval=frame[b];[4]$3[v];
      [m][p][b][v]amix=inputs=4:normalize=0,aformat=sample_fmts=s16:sample_rates=16000:channel_layouts=mono" "$AUD/$1.wav"
}
SWELL="volume='0.05+0.95*pow(max(0,sin(2*PI*t/8)),2)':eval=frame"
# calle-30s = fuerte (-24 dBFS); calle-normal = -6 dB (-30 dBFS); dura = pasadas <1500 Hz y viento de 150-700 Hz
[ -f "$AUD/calle-30s.wav" ] || gen calle-30s "$SWELL,volume=0.5" "volume='0.1+0.1*sin(2*PI*t/5.3)':eval=frame" brown || exit 1
[ -f "$AUD/calle-dura-30s.wav" ] || gen calle-dura-30s "lowpass=f=1500,$SWELL,volume=1.2" \
  "lowpass=f=700,highpass=f=150,volume='0.2+0.2*sin(2*PI*t/5.3)':eval=frame" white || exit 1
[ -f "$AUD/calle-normal-30s.wav" ] || ffmpeg -v error -y -i "$AUD/calle-30s.wav" -af volume=-6dB "$AUD/calle-normal-30s.wav" || exit 1
python3 "$MED" mezclar "$AUD" || exit 1

source "$RAIZ/tests/lote-lib.sh"
ARCH=(calle-normal-30s calle-30s calle-dura-30s); for i in 0 1 2; do ARCH+=("calle-f$i-limpio"); for s in 0 5 10; do ARCH+=("calle-f$i-snr$s"); done; done
RUTAS=(); for a in "${ARCH[@]}"; do RUTAS+=("$AUD/$a.wav"); done
# un arranque del modelo por configuracion (4) en vez de uno por archivo (60)
declare -A OPC=([neu]="" [ene]="--vad-energia" [neu_g]="--ruido" [ene_g]="--vad-energia --ruido")
for c in neu ene neu_g ene_g; do
  lote_correr "$BIN" "$SAL/lote.$c" "${OPC[$c]}" "${RUTAS[@]}"
  lote_dividir "$SAL/lote.$c" "$SAL/" ".$c.jsonl" "${ARCH[@]}"
done

python3 "$MED" medir "$SAL" "${FRASES[@]}"
