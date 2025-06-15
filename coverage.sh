#!/bin/bash

set -eux

# move to the script's directory
cd "$(dirname "$0")"

TIMESTAMP=$(date +%Y%m%d-%H%M%S)

BUILD_DIR="target/llvm-cov-target"
OUT_DIR="target/coverage/${TIMESTAMP}"
mkdir -p "$OUT_DIR"

cargo llvm-cov --html -p cspuz_core -p cspuz_rs
cp -r target/llvm-cov/html/ "${OUT_DIR}/all"

run_partial_coverage() {
    module="$1"
    filepath="$2"

    rm -f ${BUILD_DIR}/*.profraw ${BUILD_DIR}/*.profdata ${BUILD_DIR}/*-profraw-list
    LLVM_COV_FLAGS="-sources $(realpath ./cspuz_core/src/$2)" cargo llvm-cov test --html -p cspuz_core --no-clean "${module}::"
    cp -r target/llvm-cov/html/ "${OUT_DIR}/${module}"
}

run_partial_coverage csp csp/
run_partial_coverage norm_csp norm_csp/
run_partial_coverage normalizer normalizer.rs
run_partial_coverage encoder encoder/
run_partial_coverage integration integration.rs

echo "Done!"
echo "Output directory: $(realpath $OUT_DIR)"
