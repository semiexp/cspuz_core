use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::check_uniqueness;
use cspuz_rs_puzzles::puzzles::nikakoku;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let problem = nikakoku::deserialize_problem(url).ok_or("invalid url")?;
    let ans = nikakoku::solve_nikakoku(&problem);

    let height = problem.len();
    let width = problem[0].len();
    let mut board = Board::new(BoardKind::Grid, height, width, check_uniqueness(&ans));

    for y in 0..height {
        for x in 0..width {
            match problem[y][x] {
                Some(0) => board.push(Item::cell(y, x, "black", ItemKind::Circle)),
                Some(1) => board.push(Item::cell(y, x, "black", ItemKind::FilledCircle)),
                _ => (),
            }
        }
    }

    if let Some(is_line) = &ans {
        // Solver vertices have a one-cell margin; boundary segments end at the frame.
        for y in 0..=height {
            for x in 0..width {
                if let Some(b) = is_line.vertical[y][x + 1] {
                    let kind = if !b {
                        ItemKind::Cross
                    } else if y == 0 {
                        ItemKind::LineTo(1, (x * 2 + 1) as i32)
                    } else if y == height {
                        ItemKind::LineTo((height * 2 - 1) as i32, (x * 2 + 1) as i32)
                    } else {
                        ItemKind::Line
                    };
                    board.push(Item {
                        y: y * 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind,
                    });
                }
            }
        }
        for y in 0..height {
            for x in 0..=width {
                if let Some(b) = is_line.horizontal[y + 1][x] {
                    let kind = if !b {
                        ItemKind::Cross
                    } else if x == 0 {
                        ItemKind::LineTo((y * 2 + 1) as i32, 1)
                    } else if x == width {
                        ItemKind::LineTo((y * 2 + 1) as i32, (width * 2 - 1) as i32)
                    } else {
                        ItemKind::Line
                    };
                    board.push(Item {
                        y: y * 2 + 1,
                        x: x * 2,
                        color: "green",
                        kind,
                    });
                }
            }
        }
    }

    Ok(board)
}

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
            solve("https://opt-pan.github.io/penpa-edit/#m=edit&p=7ZRdb5swFIbv+RWTr30B5iMJd1nX7KbtPpoqihCKnIQ2qBB3BtbJUf57jw9MicGTpmnVejEhjl4en/gcO35dfWu4zOiIRtQfU5d68PhuQFkUwTvB1+2eeV4XWfyOTpt6JyQISj/NZvSeF1XmJF1W6hzUJFZTqj7GCfEIJQxej6RUfYkP6jpWS6puYYhQD9hVm+SDvGwlA7nAca0uWui5oG86DXIJcpPLTZGtrlvyOU7UnBJd5z3+WktSiu8Z6frQ3xtRrnMN1ryGxVS7/KkbqZqteGy6XC89UjX9rXa1bNvVytKuXsUrtztJj0fY9q/Q8CpOdO93Jzk+ydv4APEGo4dxGR8IC2AaBsXOGyQsAuoN6NiW67tWGtpoMLHR0LNVC3W1AR0xK7WuYmStNtbVehQ2Y4ZbwjDOYceo8jF+wOhiDDFeYc4lxgXGC4wBxghzRnrP//hfeaV2EhagxX8+4d/4Sp0EXE8qUayqRt7zDZxhvBTgmALbN+U6kwYqhHgq8r2Zlz/shcysQxpm2wdb/lrIbW/2Z14UBmgvOQO1f72BaglWO/vmUopng5S83hngzJbGTNm+Nhuoudkif+S9auVpzUeH/CD4wiFlcA/DDv+/VP/Jpar/BPetmfittYPnV0ir+QFb/A/U6vOOD6wOfGBqXXDoa6AWawPtuxvQ0OAABx4H9gub61n7Ttdd9c2uSw38rkudWz5JnRc="),
            Board {
                kind: BoardKind::Grid,
                height: 6,
                width: 7,
                data: vec![
                    Item { y: 1, x: 1, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 5, color: "black", kind: ItemKind::Circle },
                    Item { y: 1, x: 9, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 1, x: 13, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 3, x: 1, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 7, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 5, x: 11, color: "black", kind: ItemKind::Circle },
                    Item { y: 7, x: 3, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 9, color: "black", kind: ItemKind::Circle },
                    Item { y: 9, x: 13, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 1, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 11, x: 5, color: "black", kind: ItemKind::FilledCircle },
                    Item { y: 0, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 3, color: "green", kind: ItemKind::LineTo(1, 3) },
                    Item { y: 0, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 7, color: "green", kind: ItemKind::LineTo(1, 7) },
                    Item { y: 0, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 0, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 2, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 2, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 3, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 4, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 5, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 9, color: "green", kind: ItemKind::Line },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 13, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 1, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 8, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 7, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 11, color: "green", kind: ItemKind::Line },
                    Item { y: 10, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 1, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 3, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 12, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 3, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 5, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 12, color: "green", kind: ItemKind::Line },
                    Item { y: 7, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 2, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 6, color: "green", kind: ItemKind::Line },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 10, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 14, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 0, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 2, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 4, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 6, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 8, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 10, color: "green", kind: ItemKind::Line },
                    Item { y: 11, x: 12, color: "green", kind: ItemKind::Cross },
                    Item { y: 11, x: 14, color: "green", kind: ItemKind::Cross },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
