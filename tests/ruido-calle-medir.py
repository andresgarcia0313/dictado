"""Ayudante de ruido-calle.sh: `mezclar` crea las mezclas voz+calle; `medir` imprime las tablas."""
import collections, json, re, sys, unicodedata, wave
import numpy as np

SNRS = (0, 5, 10)
CONFIGS = [("neuronal", "neu"), ("energia", "ene"), ("neuronal+compuerta", "neu_g"), ("energia+compuerta", "ene_g")]
norm = lambda t: re.findall(r"[a-z0-9]+", unicodedata.normalize("NFD", t.lower()).encode("ascii", "ignore").decode())


def leer(ruta):
    w = wave.open(ruta)
    return np.frombuffer(w.readframes(w.getnframes()), np.int16).astype(float)


def escribir(ruta, x):
    w = wave.open(ruta, "wb")
    w.setparams((1, 2, 16000, 0, "NONE", ""))
    w.writeframes(np.clip(x, -32768, 32767).astype(np.int16).tobytes())


def mezclar(aud, previo=2.0, cola=1.5):
    """Frase con `previo` s de calle antes y `cola` s despues (el motor aprende el ruido en las pausas)."""
    calle = leer(f"{aud}/calle-30s.wav")
    rms = lambda x: np.sqrt((x**2).mean())
    for i in range(3):
        voz = leer(f"{aud}/ab-f{i}-original.wav")
        a, n = int(previo * 16000), int((previo + cola) * 16000) + len(voz)
        base, ruido = np.zeros(n), calle[i * 8 * 16000:][:n]
        base[a : a + len(voz)] = voz
        escribir(f"{aud}/calle-f{i}-limpio.wav", base + np.random.default_rng(i).normal(0, 20, n))  # cuarto: -64 dBFS
        for s in SNRS:  # SNR = RMS de la frase frente al RMS de la calle bajo la frase
            k = rms(voz) / 10 ** (s / 20) / rms(ruido[a : a + len(voz)])
            escribir(f"{aud}/calle-f{i}-snr{s}.wav", base + ruido * k)


def eventos(sal, nombre, cfg):
    return [json.loads(l) for l in open(f"{sal}/{nombre}.{cfg}.jsonl") if l.strip()]


def palabras(sal, nombre, cfg, ref):
    hip = collections.Counter(norm(" ".join(e["texto"] for e in eventos(sal, nombre, cfg) if e["t"] == "final")))
    return sum(min(c, hip[p]) for p, c in collections.Counter(norm(ref)).items())


def voz(sal, nombre, cfg):
    return np.array([e["estado"] == "voz" for e in eventos(sal, nombre, cfg) if e["t"] == "entorno"])


def medir(sal, *frases):
    total = sum(len(norm(f)) for f in frases)
    print(f"Palabras acertadas de {total} (3 frases):")
    print(f"{'caso':<12}" + "".join(f"{c[0]:>20}" for c in CONFIGS))
    for caso, pref in [("limpio", "calle-f{}-limpio")] + [(f"snr {s} dB", f"calle-f{{}}-snr{s}") for s in SNRS]:
        fila = [sum(palabras(sal, pref.format(i), c, frases[i]) for i in range(3)) for _, c in CONFIGS]
        print(f"{caso:<12}" + "".join(f"{n:>20}" for n in fila))
    print("\nRecall de voz (ventanas de 250 ms con voz en limpio que siguen con voz en la mezcla):")
    recall = {}
    for s in SNRS:
        for _, c in CONFIGS[:2]:
            par = [(voz(sal, f"calle-f{i}-limpio", c), voz(sal, f"calle-f{i}-snr{s}", c)) for i in range(3)]
            recall[s, c] = 100 * sum((r & m).sum() for r, m in par) / max(sum(r.sum() for r, _ in par), 1)
        print(f"  snr {s:>2} dB  " + "  ".join(f"{c}={recall[s, c]:.0f}%" for _, c in CONFIGS[:2]))
    sola = {}
    for archivo in ("calle-normal-30s", "calle-30s", "calle-dura-30s"):
        print(f"\n{archivo} (sola):")
        for nombre, c in CONFIGS:
            ev = eventos(sal, archivo, c)
            est = [e["estado"] for e in ev if e["t"] == "entorno"]
            f = lambda k: 100 * est.count(k) / max(len(est), 1)
            sola[archivo, c] = (sum(e["t"] == "final" for e in ev), f("voz"))
            print(f"  {nombre:<20} finales={sola[archivo, c][0]}  voz={f('voz'):.1f}%  ruido={f('ruido'):.0f}%  silencio={f('silencio'):.0f}%")
    print()
    malos = [f"finales en {a} con {c}" for (a, c), (n, _) in sola.items() if n > 0]
    malos += [f"recall {c} a {s} dB = {r:.0f}% (< 90%)" for (s, c), r in recall.items() if s >= 5 and c == "neu" and r < 90]
    print("FALLO: " + "; ".join(malos) if malos else "OK: sin finales en calle sola y recall de voz >= 90 % a SNR >= 5 dB")
    v = sola["calle-normal-30s", "neu"][1]
    print(f"Objetivo voz < 5 % en calle normal sola: {'CUMPLE' if v < 5 else 'NO CUMPLE'} ({v:.1f}% con el VAD neuronal)")
    return 1 if malos else 0


if __name__ == "__main__":
    {"mezclar": lambda: mezclar(sys.argv[2]), "medir": lambda: sys.exit(medir(sys.argv[2], *sys.argv[3:]))}[sys.argv[1]]()
