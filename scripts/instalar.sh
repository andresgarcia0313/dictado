#!/usr/bin/env bash
# Instala Dictado para el usuario actual en Linux: binarios, icono y entrada de menú (sin sudo).
# Uso: bash scripts/instalar.sh   (compila en release si hace falta)   |   bash scripts/instalar.sh --quitar
set -euo pipefail
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
DESTINO="${DICTADO_DESTINO:-$HOME/.local/opt/dictado}"
DATOS="${XDG_DATA_HOME:-$HOME/.local/share}"
ICONOS="$DATOS/icons/hicolor"
ENTRADA="$DATOS/applications/dictado.desktop"

if [ "${1:-}" = "--quitar" ]; then
    rm -f -- "${ENTRADA:?}" "${ICONOS:?}/scalable/apps/dictado.svg" "${ICONOS:?}/256x256/apps/dictado.png"
    rm -f -- "${DESTINO:?}/dictado" "${DESTINO:?}/dictado-motor"
    rmdir -- "${DESTINO:?}" 2>/dev/null || true
    echo "Dictado quitado"
    exit 0
fi

[ -x "$RAIZ/target/release/dictado" ] || (cd "$RAIZ" && cargo build --release)
mkdir -p "$DESTINO" "$ICONOS/scalable/apps" "$ICONOS/256x256/apps" "$(dirname "$ENTRADA")"
install -m 755 "$RAIZ/target/release/dictado" "$RAIZ/target/release/dictado-motor" "$DESTINO/"
install -m 644 "$RAIZ/recursos/dictado.svg" "$ICONOS/scalable/apps/dictado.svg"
install -m 644 "$RAIZ/recursos/dictado-256.png" "$ICONOS/256x256/apps/dictado.png"
sed "s|@RUTA@|$DESTINO|" "$RAIZ/recursos/dictado.desktop" > "$ENTRADA"
command -v desktop-file-validate >/dev/null && desktop-file-validate "$ENTRADA"
command -v kbuildsycoca6 >/dev/null && { kbuildsycoca6 >/dev/null 2>&1 || true; }
command -v gtk-update-icon-cache >/dev/null && { gtk-update-icon-cache -q -t "$ICONOS" 2>/dev/null || true; }
echo "Dictado instalado en $DESTINO (menú: Dictado)"
