#!/bin/bash

set -eu

cargo fmt "$@"

PUZZLE_IMPL_FILES=`find cspuz_solver_backend/src/puzzle -name '*.rs'`
rustfmt $PUZZLE_IMPL_FILES "$@"
