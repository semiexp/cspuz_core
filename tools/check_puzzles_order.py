import argparse
import os
import re

CSPUZ_SOLVER_BACKEND_MOD_PATH = os.path.join(os.path.dirname(__file__), "../cspuz_solver_backend/src/puzzle/mod.rs")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--src", type=str, default=CSPUZ_SOLVER_BACKEND_MOD_PATH)
    args = parser.parse_args()

    with open(args.src, "r") as fp:
        source = fp.read()

    is_puzzle_list = False
    last_puzzle: str | None = None
    for i, line in enumerate(source.split("\n")):
        if line.startswith("puzzle_list!("):
            is_puzzle_list = True
            last_puzzle = None
            continue

        if not is_puzzle_list:
            continue

        if line.startswith(");"):
            is_puzzle_list = False
            continue

        m = re.match(r"^\s*\(([a-z_]+),.*", line)
        assert m is not None
        if m is None:
            raise ValueError(f"invalid line in puzzle list (line {i + 1})")
        puzzle_mod = m[1]
        if last_puzzle is None:
            last_puzzle = puzzle_mod
        else:
            if last_puzzle >= puzzle_mod:
                raise ValueError(
                    f"puzzles are not in order: {last_puzzle} >= {puzzle_mod} (line {i + 1})"
                )
            last_puzzle = puzzle_mod


if __name__ == "__main__":
    main()
