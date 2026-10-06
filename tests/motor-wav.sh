#!/usr/bin/env bash
# Verifica el motor sin microfono: parciales, final completo ("por favor") y protocolo JSON.
set -uo pipefail
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${DICTADO_MOTOR:-}"
WAV="${1:-$RAIZ/tests/audio/frase.wav}"
if [ -n "$BIN" ]; then SAL=$(timeout 60 "$BIN" --wav "$WAV" 2>/dev/null); else
  SAL=$(cd "$RAIZ" && timeout 300 cargo run -q -p dictado-motor -- --wav "$WAV" 2>/dev/null); fi
rc=$?
fallo() { echo "FALLO: $1"; echo "$SAL" | grep -v '"nivel"' | head -20; exit 1; }
[ $rc -eq 0 ] || fallo "codigo de salida $rc"
echo "$SAL" | python3 -c 'import sys,json; [json.loads(l) for l in sys.stdin]' || fallo "linea no JSON"
echo "$SAL" | grep -q '"t":"listo"' || fallo "falta listo"
echo "$SAL" | grep -q '"t":"parcial"' || fallo "faltan parciales"
FINAL=$(echo "$SAL" | grep '"t":"final"' | tail -1)
echo "$FINAL" | grep -qi 'por favor' || fallo "final sin la ultima palabra: $FINAL"
echo "OK: $(echo "$SAL" | grep -c '"t":"parcial"') parciales; $FINAL"
