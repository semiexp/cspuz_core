#!/usr/bin/env python3
"""
Extract problem URLs from cspuz_rs_puzzles for corresponding backend puzzle files.

For each file in cspuz_solver_backend/src/puzzles/, finds the corresponding file
in cspuz_rs_puzzles/src/puzzles/ and extracts the URL from lines like 'let url = "..."'.
"""

import argparse
import os
import re
from pathlib import Path
from string import Template

# Template for test_solve function
TEST_SOLVE_TEMPLATE = Template('''
#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::*;
    use crate::compare_board;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board!(
            solve("${url}"),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 6,
                data: vec![
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
''')


def find_url_in_file(file_path):
    """
    Find URL in a Rust file from lines like 'let url = "..."'.
    Returns the URL if exactly one such line exists, otherwise None.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Match lines like: let url = "https://...";
        pattern = r'let\s+url\s*=\s*"([^"]+)"'
        matches = re.findall(pattern, content)

        if len(matches) == 1:
            return matches[0]
        return None
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return None


def has_test_solve(file_path):
    """
    Check if a Rust file contains a test_solve() function.
    Returns True if found, False otherwise.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Match function declarations like: fn test_solve() or pub fn test_solve()
        pattern = r'(pub\s+)?fn\s+test_solve\s*\('
        return bool(re.search(pattern, content))
    except Exception as e:
        print(f"Error reading {file_path}: {e}")
        return False


def add_test_solve_to_file(file_path, url):
    """
    Add test_solve function to a Rust file if it doesn't already exist.
    Returns True if successfully added, False otherwise.
    """
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()

        # Check if test_solve already exists
        if has_test_solve(file_path):
            return False

        # Add the test module at the end of the file
        test_code = TEST_SOLVE_TEMPLATE.substitute(url=url)
        updated_content = content.rstrip() + '\n' + test_code

        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(updated_content)

        return True
    except Exception as e:
        print(f"Error updating {file_path}: {e}")
        return False

IGNORED_FILES = ["mod.rs", "heyawake_internal.rs"]


def main():
    # Parse arguments
    parser = argparse.ArgumentParser(
        description="Extract problem URLs and manage test_solve functions for backend puzzles"
    )
    parser.add_argument(
        "--auto-update",
        action="store_true",
        help="Automatically add test_solve() functions to files that don't have them"
    )
    parser.add_argument(
        "--show-all",
        action="store_true",
        help="Show all files including those with both test_solve and URL"
    )
    args = parser.parse_args()

    # Project root
    project_root = Path(__file__).parent.parent

    backend_dir = project_root / "cspuz_solver_backend" / "src" / "puzzle"
    rs_puzzles_dir = project_root / "cspuz_rs_puzzles" / "src" / "puzzles"

    if not backend_dir.exists():
        print(f"Backend directory not found: {backend_dir}")
        return

    if not rs_puzzles_dir.exists():
        print(f"RS puzzles directory not found: {rs_puzzles_dir}")
        return

    # Get all .rs files in backend directory
    backend_files = sorted(backend_dir.glob("*.rs"))

    results = []
    updates = []

    for backend_file in backend_files:
        filename = backend_file.name

        # Skip ignored files
        if filename in IGNORED_FILES:
            continue

        # Check if test_solve exists in backend file
        has_test = has_test_solve(backend_file)

        # Find corresponding file in rs_puzzles
        rs_puzzle_file = rs_puzzles_dir / filename

        if not rs_puzzle_file.exists():
            results.append((filename, None, "File not found in cspuz_rs_puzzles", has_test, False))
            continue

        # Extract URL
        url = find_url_in_file(rs_puzzle_file)

        added = False
        if args.auto_update and url and not has_test:
            if add_test_solve_to_file(backend_file, url):
                added = True
                updates.append(filename)
                has_test = True

        if url:
            results.append((filename, url, "Found", has_test, added))
        else:
            results.append((filename, None, "No unique URL found", has_test, added))

    # Print results
    print("Backend Puzzle Files - URL Extraction Results")
    print("=" * 80)

    for filename, url, status, has_test, added in results:
        # Skip files where both test_solve and URL are OK, unless --show-all is specified
        if not args.show_all and has_test and not added:
            continue

        print(f"\n{filename}:")
        if added:
            print(f"  test_solve(): ✅ Added automatically")
        else:
            print(f"  test_solve(): {'✅ Implemented' if has_test else '❌ Not found'}")
        if url:
            print(f"  URL: ✅ Found")
            print(f"       {url}")
        else:
            print(f"  URL: ❌ {status}")

    if args.auto_update and updates:
        print(f"\n{'=' * 80}")
        print(f"✅ Added test_solve() to {len(updates)} file(s):")
        for filename in updates:
            print(f"  - {filename}")


if __name__ == "__main__":
    main()
