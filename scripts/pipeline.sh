#!/usr/bin/env bash
# Pipeline de la GUI: format -> lint -> compilación -> prueba offscreen con motor falso.
set -euo pipefail
cd "$(dirname "$0")/.."
cargo fmt -p dictado
cargo clippy -p dictado -- -D warnings
cargo build -p dictado
bash tests/gui-offscreen.sh
