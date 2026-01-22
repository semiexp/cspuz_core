use crate::board::{Board, ItemKind};
use crate::uniqueness::Uniqueness;
use std::fs;
use std::io::Write;

#[macro_export]
macro_rules! compare_board {
    ($actual:expr, $expected:expr $(,)?) => {
        let actual = $actual;
        if actual.is_err() {
            panic!("Expected Ok(_), but got Err({})", actual.err().unwrap());
        }
        let actual = actual.unwrap();
        if actual != $expected {
            $crate::testing::expectation_mismatch(
                file!().to_string(),
                line!(),
                column!(),
                actual,
                $expected,
            );
        }
    };
}

#[macro_export]
macro_rules! compare_board_and_check_no_solution_case {
    ($actual:expr, $expected:expr $(,)?) => {
        let actual = $actual;
        if actual.is_err() {
            panic!("Expected Ok(_), but got Err({})", actual.err().unwrap());
        }
        let actual = actual.unwrap();
        if actual != $expected {
            $crate::testing::expectation_mismatch(
                file!().to_string(),
                line!(),
                column!(),
                actual,
                $expected,
            );
        }

        cspuz_core::solver::force_solver_fail_for_tests(true);
        let actual = $actual;
        cspuz_core::solver::force_solver_fail_for_tests(false);
        if actual.is_err() {
            panic!("Expected Ok(_), but got Err({})", actual.err().unwrap());
        }
        let actual = actual.unwrap();
        let expected_no_solution = $crate::testing::expectation_no_solution(&$expected);
        assert_eq!(actual, expected_no_solution);
    };
}

pub fn expectation_mismatch(file: String, line: u32, column: u32, actual: Board, expected: Board) {
    if std::env::var("CSPUZ_SOLVER_BACKEND_UPDATE_EXPECTATIONS").unwrap_or_default() != "1" {
        eprintln!(
            "Board does not match expectation at {}:{}:{}.",
            file, line, column
        );
        eprintln!("To update the expectation, set the environment variable CSPUZ_SOLVER_BACKEND_UPDATE_EXPECTATIONS=1 and re-run the tests.");
        eprintln!("Actual Board: {:?}", actual);
        eprintln!("Expected Board: {:?}", expected);
        panic!("Expectation mismatch");
    }

    let full_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(&file)
        .to_str()
        .unwrap()
        .to_string();
    // Read the file
    let content =
        fs::read_to_string(&full_path).expect(&format!("Failed to read file: {}", full_path));

    let lines: Vec<&str> = content.lines().collect();
    let target_line = (line - 1) as usize; // line numbers are 1-indexed

    assert_eq!(
        &lines[target_line][(column - 1) as usize..],
        "compare_board!("
    );

    // Find the start of Board { ... } and parse until the matching closing brace
    let mut start_line = target_line;
    let mut found_start = false;
    let mut end_line = target_line;

    // Search for "Board {" starting from the target line
    for i in target_line..lines.len() {
        if lines[i].contains("Board") && lines[i].contains("{") {
            start_line = i;
            found_start = true;
            let mut brace_count =
                lines[i].matches('{').count() as i32 - lines[i].matches('}').count() as i32;

            if brace_count == 0 {
                end_line = i;
                break;
            }

            // Continue counting braces on subsequent lines
            for j in (i + 1)..lines.len() {
                brace_count +=
                    lines[j].matches('{').count() as i32 - lines[j].matches('}').count() as i32;
                if brace_count == 0 {
                    end_line = j;
                    break;
                }
            }
            break;
        }
    }

    if !found_start {
        panic!("Could not find Board {{ ... }} at line {}", line);
    }

    assert!(lines[end_line + 1].ends_with(");"));

    // Get the indentation from the start line
    let indent = lines[start_line]
        .chars()
        .take_while(|c| c.is_whitespace())
        .collect::<String>();

    // Generate the new Board representation
    let new_board = board_to_string(&actual);
    let new_board = add_indent(&new_board, &indent);

    // TODO: Format the Board properly instead of using Debug formatting
    // todo!("Format Board struct with proper indentation and field layout");

    // Reconstruct the file content
    let mut new_lines = Vec::new();
    new_lines.extend_from_slice(&lines[..start_line]);
    new_lines.push(&new_board);
    new_lines.extend_from_slice(&lines[(end_line + 1)..]);

    // Write back to file
    let new_content = new_lines.join("\n") + "\n";
    let mut f =
        fs::File::create(&full_path).expect(&format!("Failed to create file: {}", full_path));
    f.write_all(new_content.as_bytes())
        .expect(&format!("Failed to write to file: {}", full_path));

    eprintln!("Updated expectation at {}:{}:{}", file, line, column);
}

pub fn expectation_no_solution(board: &Board) -> Board {
    let filtered_data = board
        .data
        .iter()
        .filter(|item| item.color != "green")
        .cloned()
        .collect::<Vec<_>>();

    Board {
        kind: board.kind,
        height: board.height,
        width: board.width,
        data: filtered_data,
        uniqueness: Uniqueness::NoAnswer,
    }
}

fn add_indent(s: &str, indent: &str) -> String {
    s.lines()
        .map(|line| format!("{}{}", indent, line))
        .collect::<Vec<_>>()
        .join("\n")
}

fn item_kind_to_string(kind: &ItemKind) -> String {
    match kind {
        ItemKind::Text(s) => format!("ItemKind::Text(\"{}\")", s),
        ItemKind::TextString(s) => format!("ItemKind::TextString(\"{}\".to_string())", s),
        ItemKind::Num(n) => format!("ItemKind::Num({})", n),
        ItemKind::NumUpperLeft(n) => format!("ItemKind::NumUpperLeft({})", n),
        ItemKind::NumUpperRight(n) => format!("ItemKind::NumUpperRight({})", n),
        ItemKind::NumLowerLeft(n) => format!("ItemKind::NumLowerLeft({})", n),
        ItemKind::NumLowerRight(n) => format!("ItemKind::NumLowerRight({})", n),
        ItemKind::TapaClue(c) => format!(
            "ItemKind::TapaClue([{}, {}, {}, {}])",
            c[0], c[1], c[2], c[3]
        ),
        ItemKind::Compass(c) => format!(
            "ItemKind::Compass(Compass {{ up: {:?}, down: {:?}, left: {:?}, right: {:?} }})",
            c.up, c.down, c.left, c.right
        ),
        ItemKind::SudokuCandidateSet(n, k) => {
            format!("ItemKind::SudokuCandidateSet({}, vec!{:?})", n, k)
        }
        ItemKind::Firefly(d, n) => format!("ItemKind::Firefly(FireflyDir::{:?}, {})", d, n),
        ItemKind::LineTo(a, b) => format!("ItemKind::LineTo({}, {})", a, b),
        other => format!("ItemKind::{:?}", other),
    }
}

fn board_to_string(board: &Board) -> String {
    let mut lines = vec![];
    lines.push("Board {".to_string());
    lines.push(format!("    kind: BoardKind::{:?},", board.kind));
    lines.push(format!("    height: {},", board.height));
    lines.push(format!("    width: {},", board.width));
    lines.push("    data: vec![".to_string());

    for item in &board.data {
        lines.push(format!(
            "        Item {{ y: {}, x: {}, color: \"{}\", kind: {} }},",
            item.y,
            item.x,
            item.color,
            item_kind_to_string(&item.kind)
        ));
    }

    lines.push("    ],".to_string());
    lines.push(format!(
        "    uniqueness: Uniqueness::{:?},",
        board.uniqueness
    ));
    lines.push("},".to_string());
    lines.join("\n")
}

mod tests {
    use super::*;
    use crate::board::*;

    #[test]
    #[rustfmt::skip]
    fn test_item_kind_to_string() {
        assert_eq!(
            item_kind_to_string(&ItemKind::Text("hello")),
            "ItemKind::Text(\"hello\")",
        );
        assert_eq!(
            item_kind_to_string(&ItemKind::TextString("hello".to_string())),
            "ItemKind::TextString(\"hello\".to_string())",
        );
        assert_eq!(
            item_kind_to_string(&ItemKind::Num(42)),
            "ItemKind::Num(42)",
        );
        assert_eq!(
            item_kind_to_string(&ItemKind::NumUpperLeft(42)),
            "ItemKind::NumUpperLeft(42)",
        );
        assert_eq!(
            item_kind_to_string(&ItemKind::Compass(Compass { up: Some(1), down: None, left: Some(3), right: Some(4) })),
            "ItemKind::Compass(Compass { up: Some(1), down: None, left: Some(3), right: Some(4) })",
        );
        assert_eq!(
            item_kind_to_string(&ItemKind::TapaClue([1, 2, 3, 4])),
            "ItemKind::TapaClue([1, 2, 3, 4])",
        );
        assert_eq!(
            item_kind_to_string(&ItemKind::SudokuCandidateSet(3, vec![2, 3, 4])),
            "ItemKind::SudokuCandidateSet(3, vec![2, 3, 4])",
        );
        assert_eq!(
            item_kind_to_string(&ItemKind::Firefly(FireflyDir::Down, 4)),
            "ItemKind::Firefly(FireflyDir::Down, 4)",
        );
    }
}
