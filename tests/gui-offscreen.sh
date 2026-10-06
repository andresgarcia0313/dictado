#!/usr/bin/env bash
# Prueba sin pantalla ni micrófono: la GUI arranca con el motor falso, recibe eventos y sale limpia.
set -uo pipefail
RAIZ="$(cd "$(dirname "$0")/.." && pwd)"
BIN="${DICTADO_BIN:-${CARGO_TARGET_DIR:-$RAIZ/target}/debug/dictado}"
LOG="$(mktemp)"
# Los registros de la prueba van a una carpeta temporal: no ensucian los reales.
ESTADO="$(mktemp -d)"
GUI_LOG="$ESTADO/dictado/registros/gui.log"
trap 'rm -f "$LOG"; rm -rf "${ESTADO:?}"' EXIT
fallos=0
ok() { echo "ok   $1"; }
mal() { echo "FALLA $1"; fallos=$((fallos + 1)); }
cuenta() { grep -c -- "$1" "$LOG" || true; }
cuenta_reg() { grep -c -- "$1" "$GUI_LOG" 2>/dev/null || true; }

correr() { # $1=modo del motor falso, $2=autosalir, $3=ajustes de usuario simulados (opcional)
  : >"$LOG"; rm -rf "${ESTADO:?}/dictado"
  XDG_STATE_HOME="$ESTADO" DICTADO_LOG=depura QT_QPA_PLATFORM=offscreen DICTADO_DEPURA=1 DICTADO_AUTOSALIR="$2" DICTADO_AJUSTES="${3:-}" \
    DICTADO_MOTOR="$RAIZ/tests/motor-falso.sh" DICTADO_FALSO_MODO="$1" \
    timeout 40 "$BIN" 2>"$LOG"
}

[[ -x "$BIN" ]] || { echo "falta el binario: cargo build -p dictado"; exit 2; }

correr normal fin; rc=$?
((rc == 0)) && ok "normal: salida 0" || mal "normal: salida $rc"
(($(cuenta '\[gui\] listo') == 1)) && ok "normal: listo" || mal "normal: listo"
(($(cuenta '\[gui\] parcial') >= 3)) && ok "normal: parciales" || mal "normal: parciales"
(($(cuenta '\[gui\] final') == 3)) && ok "normal: 3 finales" || mal "normal: finales"
(($(cuenta 'fin limpio=true') == 1)) && ok "normal: fin limpio" || mal "normal: fin limpio"
(($(cuenta -E 'ReferenceError|TypeError|is not a type|Error:') == 0)) && ok "normal: sin errores QML" || mal "normal: errores QML"
(($(cuenta 'ram app=') >= 1)) && ok "normal: mide RAM" || mal "normal: sin RAM"
(($(cuenta '\[gui\] espectro') >= 10)) && ok "normal: espectro" || mal "normal: espectro"
(($(cuenta '\[gui\] ganancia') >= 1)) && ok "normal: ganancia" || mal "normal: ganancia"
(($(cuenta '\[gui\] mics Micrófono A|Micrófono B actual=Micrófono A') >= 1)) && ok "normal: lista de mics" || mal "normal: mics"
(($(cuenta '\[gui\] entorno') >= 2)) && ok "normal: entorno" || mal "normal: entorno"
[[ -s "$GUI_LOG" ]] && ok "registro: existe gui.log" || mal "registro: falta gui.log"
(($(cuenta_reg 'arranque') >= 1)) && ok "registro: arranque" || mal "registro: arranque"
(($(cuenta_reg 'motor lanzado pid=') >= 1)) && ok "registro: motor lanzado" || mal "registro: motor lanzado"
(($(cuenta_reg 'comando enviado=') >= 1)) && ok "registro: comandos" || mal "registro: comandos"
(($(cuenta_reg 'ram app_mb=') >= 1)) && ok "registro: RAM" || mal "registro: RAM"
(($(cuenta_reg 'cierre completo') == 1)) && ok "registro: cierre" || mal "registro: cierre"
# Privacidad: el texto dictado (frases del motor falso) jamás llega al registro.
grep -qi -E 'hola como estas|prueba del dictado|corrige el acento' "$GUI_LOG" && mal "registro: contiene texto dictado" || ok "registro: sin texto dictado"

correr normal fin "mejora=off,ruido=on,mic=Micrófono B"; rc=$?
((rc == 0)) && ok "ajustes: salida 0" || mal "ajustes: salida $rc"
(($(cuenta '\[falso\] cmd mejora off') >= 1)) && ok "ajustes: mejora off llega al motor" || mal "ajustes: mejora"
(($(cuenta '\[falso\] cmd ruido on') >= 1)) && ok "ajustes: ruido on llega al motor" || mal "ajustes: ruido"
(($(cuenta '\[falso\] cmd mic Micrófono B') >= 1)) && ok "ajustes: mic elegido llega al motor" || mal "ajustes: mic"
grep -q 'actual=Micrófono B' "$LOG" && ok "ajustes: mic confirmado por el motor" || mal "ajustes: mic sin confirmar"
# 1 arranque + 1 por mic elegido + 1 posible por el valor inicial de QML; un bucle daría decenas.
(($(cuenta '\[gui\] listo') <= 3)) && ok "ajustes: cambiar mic no entra en bucle" || mal "ajustes: listo $(cuenta '\[gui\] listo')"

correr falla fin "mejora=off,ruido=on"; rc=$?
((rc == 0)) && ok "falla: salida 0" || mal "falla: salida $rc"
(($(cuenta '\[gui\] listo') == 3)) && ok "falla: 3 intentos" || mal "falla: intentos $(cuenta '\[gui\] listo')"
(($(cuenta 'fin limpio=false') == 1)) && ok "falla: fin con error" || mal "falla: fin"
(($(cuenta '\[falso\] cmd mejora off') >= 3)) && ok "falla: mejora reenviada en cada relanzamiento" || mal "falla: mejora $(cuenta '\[falso\] cmd mejora off')"
(($(cuenta '\[falso\] cmd ruido on') >= 3)) && ok "falla: ruido reenviado en cada relanzamiento" || mal "falla: ruido"
(($(cuenta_reg 'relanzamiento del motor') == 2)) && ok "registro: 2 relanzamientos" || mal "registro: relanzamientos $(cuenta_reg 'relanzamiento del motor')"
(($(cuenta_reg 'motor termino codigo=3') == 3)) && ok "registro: 3 salidas con código 3" || mal "registro: salidas"
(($(cuenta_reg 'evento error del motor') >= 3)) && ok "registro: errores del motor" || mal "registro: errores"
(($(cuenta_reg 'no se relanza más') == 1)) && ok "registro: se rinde tras 3 fallos" || mal "registro: rendición"

correr espera 3; rc=$?
((rc == 0)) && ok "espera: salida 0 con motor vivo" || mal "espera: salida $rc"
sleep 1
pgrep -f "tests/motor-falso.sh" >/dev/null && mal "espera: motor huérfano" || ok "espera: motor cerrado"
(($(cuenta 'ram app=') >= 3)) && ok "espera: RAM cada segundo" || mal "espera: RAM"

# Interruptor "Saltar filtros del sistema" OFF: relanza el motor una vez con --con-filtros, sin contarlo como fallo.
correr espera 6 "saltar=off"; rc=$?
((rc == 0)) && ok "filtros: salida 0" || mal "filtros: salida $rc"
(($(cuenta_reg 'se relanza el motor saltar_filtros=false') == 1)) && ok "filtros: relanza al apagar el interruptor" || mal "filtros: sin relanzamiento"
(($(cuenta_reg 'motor lanzado pid=.*con_filtros=true') >= 1)) && ok "filtros: el motor nuevo lleva --con-filtros" || mal "filtros: sin --con-filtros"
(($(cuenta_reg 'no se relanza más') == 0)) && ok "filtros: relanzar no cuenta como fallo" || mal "filtros: contado como fallo"

grep 'ram app=' "$LOG" | tail -1
((fallos == 0)) && echo "TODO BIEN" || { echo "$fallos fallos"; exit 1; }
