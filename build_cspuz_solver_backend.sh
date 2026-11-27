#!/bin/bash

set -eux

MODE=${1:-debug}

mkdir -p build/cspuz_solver_backend
export CSPUZ_WASM_OUTPUT_DIR=build/cspuz_solver_backend

if [ "$MODE" = "release" ]; then
    cargo build --target wasm32-unknown-emscripten --release --no-default-features
elif [ "$MODE" = "debug" ]; then
    cargo build --target wasm32-unknown-emscripten --no-default-features
else
    echo "Invalid mode: $MODE"
    exit 1
fi
