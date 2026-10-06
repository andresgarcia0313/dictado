#!/usr/bin/env bash
# Resumen de diagnóstico: ruta de los registros, últimos errores y avisos, relanzamientos del motor.
# Uso: scripts/diagnostico.sh [líneas]   (por defecto 40). Respeta XDG_STATE_HOME.
set -uo pipefail
N="${1:-40}"
case "$(uname -s)" in
  Darwin) BASE="$HOME/Library/Application Support" ;;
  MINGW* | MSYS* | CYGWIN*) BASE="${LOCALAPPDATA:-$HOME/AppData/Local}" ;;
  *) BASE="${XDG_STATE_HOME:-$HOME/.local/state}" ;;
esac
DIR="$BASE/dictado/registros"
echo "Registros: $DIR"
[[ -d "$DIR" ]] || { echo "No existe: la app aún no ha corrido o DICTADO_LOG apunta a otro lado."; exit 0; }

# El archivo vigente y su copia anterior, en orden cronológico.
leer() { cat "$DIR/$1.log.1" "$DIR/$1.log" 2>/dev/null; }

for nombre in motor gui; do
  echo
  echo "== $nombre =="
  if [[ ! -f "$DIR/$nombre.log" ]]; then
    echo "(sin registro)"
    continue
  fi
  echo "Último arranque: $(leer "$nombre" | grep ' arranque ' | tail -1 | cut -c1-23)"
  echo "-- Últimos $N con nivel Error o Aviso --"
  leer "$nombre" | grep -E '^[^ ]+ (Error|Aviso) ' | tail -n "$N"
done

echo
echo "== Relanzamientos y salidas del motor (gui) =="
leer gui | grep -E 'relanzamiento|motor termino|no se relanza' | tail -n "$N"
echo
echo "== Salidas del motor (motor) =="
leer motor | grep -E ' fin codigo=' | tail -n 10
echo
echo "Más detalle: DICTADO_LOG=depura (resumen cada 5 s) y DICTADO_LOG_STDERR=1 para verlo en la terminal."
