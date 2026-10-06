#!/usr/bin/env bash
# Motor simulado: imprime eventos JSON del protocolo y atiende stdin.
# Comandos: pausa | reanuda | salir | mejora on|off | ruido on|off | mic <nombre>.
# Cada comando recibido se registra en stderr como "[falso] cmd <comando>" (la prueba los cuenta).
# DICTADO_FALSO_MODO: normal (sale 0 al terminar) | espera (vive hasta "salir") | falla (sale 3).
set -u
modo="${DICTADO_FALSO_MODO:-normal}"
pausado=0
mics='["Micrófono A","Micrófono B"]'
actual="Micrófono A"

emitir() { printf '%s\n' "$1"; }

# Espera $1 segundos atendiendo comandos; EOF en stdin = la GUI se fue.
espera() {
  local linea
  read -r -t "$1" linea || { (($? > 128)) || exit 0; }
  [[ -n "${linea:-}" ]] && echo "[falso] cmd $linea" >&2
  case "${linea:-}" in
    pausa) pausado=1 ;;
    reanuda) pausado=0 ;;
    salir) exit 0 ;;
    "mic "*) actual="${linea#mic }"; [[ "$actual" == auto ]] && actual="Micrófono A"
      emitir "{\"t\":\"listo\",\"modelo\":\"es-kroko\",\"dispositivo\":\"$actual\"}"
      emitir "{\"t\":\"mics\",\"lista\":$mics,\"actual\":\"$actual\"}" ;;
  esac
}

# Entorno simulado (~4/s): voz, ruido y silencio por turnos, con su SNR.
entorno() {
  local est=(voz ruido silencio) snr=(14.2 3.8 0.0)
  local i=$((ciclo / 40 % 3))
  emitir "{\"t\":\"entorno\",\"estado\":\"${est[i]}\",\"p\":0.83,\"snr\":${snr[i]}}"
}

# Espectro simulado (~25/s), ganancia y nivel; atiende comandos en el mismo hilo.
barra() {
  local i b="" n=$((RANDOM % 200 + 20))
  for i in {0..31}; do b+="${b:+,}$(((n * (32 - i) / 32 + RANDOM % 30) % 256))"; done
  emitir "{\"t\":\"espectro\",\"b\":[$b]}"
  ((++ciclo % 6 == 0)) && entorno
  ((ciclo % 12 == 0)) && emitir "{\"t\":\"ganancia\",\"db\":$((RANDOM % 20)).5}"
  espera 0.04
}
ciclo=0

emitir '{"t":"listo","modelo":"es-kroko","dispositivo":"Micrófono A"}'
emitir "{\"t\":\"mics\",\"lista\":$mics,\"actual\":\"$actual\"}"
for _ in 1 2 3 4; do espera 0.1; done # deja llegar los ajustes reenviados
[[ "$modo" == "falla" ]] && { emitir '{"t":"error","msg":"fallo simulado del motor"}'; exit 3; }

frases=("hola como estas" "esto es una prueba del dictado" "por favor corrige el acento")
for frase in "${frases[@]}"; do
  acumulado=""
  for palabra in $frase; do
    while ((pausado)); do espera 0.2; done
    acumulado="${acumulado:+$acumulado }$palabra"
    emitir "{\"t\":\"nivel\",\"rms\":0.$((RANDOM % 9 + 1))}"
    emitir "{\"t\":\"parcial\",\"texto\":\"$acumulado\"}"
    for _ in 1 2 3 4; do barra; done
  done
  emitir "{\"t\":\"final\",\"texto\":\"${acumulado^}.\"}"
  espera 0.2
done

[[ "$modo" == "espera" ]] && while true; do barra; done
exit 0
