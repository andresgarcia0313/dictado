#!/usr/bin/env bash
# Vocabulario propio con voz sintetica (piper) por --wav, sin microfono ni altavoces:
# "el ambiente de cu a" debe salir como "QA"; el registro lleva reemplazos=N y jamas el texto.
set -uo pipefail
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
TMP="$(mktemp -d)"; trap 'rm -rf "$TMP"' EXIT
export XDG_CONFIG_HOME="$TMP/config" XDG_STATE_HOME="$TMP/estado"
VOZ="${PIPER_VOCES:-$HOME/.local/share/piper/voices}/es_ES-davefx-medium.onnx"
BIN="${DICTADO_MOTOR:-$RAIZ/target/debug/dictado-motor}"
[ -x "$BIN" ] || { (cd "$RAIZ" && CARGO_BUILD_JOBS=4 nice -n 10 cargo build -q -p dictado-motor) || { echo "FALLO: compilacion"; exit 1; }; }
fallo() { echo "FALLO: $1"; exit 1; }
CACHE="$RAIZ/tests/audio"   # la voz se genera una vez y se reutiliza (piper es lento con poca RAM)
hablar() { [ -s "$CACHE/voc-$2.wav" ] && return
  echo "$1" | timeout 300 piper -m "$VOZ" -f "$TMP/$2.raw.wav" >/dev/null 2>&1 &&
  ffmpeg -v error -y -i "$TMP/$2.raw.wav" -ar 16000 -ac 1 -sample_fmt s16 "$CACHE/voc-$2.wav" || fallo "piper"; }
hablar "abre redmine y revisa el ambiente de cu a" qa
final() { timeout 60 "$BIN" --wav "$1" 2>/dev/null | grep '"t":"final"' | tail -1; }
F1=$(final "$CACHE/voc-qa.wav"); echo "$F1"
echo "$F1" | grep -q 'QA' || fallo "'cu a' no salio como QA"
echo "$F1" | grep -q 'Redmine' || fallo "'redmine' no salio con su grafia"
[ -f "$XDG_CONFIG_HOME/dictado/vocabulario.txt" ] || fallo "no creo la semilla"
grep -q 'Kubernetes' "$XDG_CONFIG_HOME/dictado/vocabulario.txt" || fallo "semilla incompleta"
LOG=$(cat "$XDG_STATE_HOME"/dictado/registros/*.log 2>/dev/null)
echo "$LOG" | grep -q 'reemplazos=' || fallo "el registro no trae reemplazos=N"
echo "$LOG" | grep -qi 'ambiente\|redmine' && fallo "el registro filtra texto"
echo "OK: vocabulario por voz, semilla, registro sin texto"
