#!/usr/bin/env bash
# Biblioteca para correr el motor en modo lote (un solo arranque del modelo, 8-13 s) y repartir la salida por archivo.
# Uso: lote_correr BIN SALIDA.jsonl "arg1 arg2" archivo1.wav archivo2.wav ...   (60 s por archivo, tope 600 s)
lote_correr() {
  local bin="$1" salida="$2" extra="$3"; shift 3
  local tope=$(( 60 * $# )); (( tope > 600 )) && tope=600
  # shellcheck disable=SC2086
  timeout "$tope" "$bin" --wav "$@" $extra >"$salida" 2>/dev/null
}
# Uso: lote_dividir SALIDA.jsonl PREFIJO SUFIJO nombre1 nombre2 ...  -> PREFIJO<nombre>SUFIJO por cada `archivo`
lote_dividir() {
  local jl="$1" pre="$2" suf="$3"; shift 3
  awk -v pre="$pre" -v suf="$suf" -v names="$*" 'BEGIN{split(names,N," ")}
    /"t":"archivo"/{i++; f=pre N[i] suf; printf "" > f; next} i>0{print >> f}' "$jl"
}
