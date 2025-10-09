#!/bin/bash

set -eux

MODE=${1:-debug}

if [ "$MODE" = "release" ]; then
    cargo build --target wasm32-unknown-emscripten --release --no-default-features
elif [ "$MODE" = "debug" ]; then
    cargo build --target wasm32-unknown-emscripten --no-default-features
else
    echo "Invalid mode: $MODE"
    exit 1
fi

mkdir -p build/cspuz_solver_backend
cp target/wasm32-unknown-emscripten/${MODE}/deps/cspuz_solver_backend.js build/cspuz_solver_backend/
cp target/wasm32-unknown-emscripten/${MODE}/deps/cspuz_solver_backend.wasm build/cspuz_solver_backend/
