use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::Uniqueness;
use cspuz_rs::graph::BoolGridEdgesModel;
use cspuz_rs_puzzles::puzzles::numlin;

pub fn enumerate(url: &str, num_max_answers: usize) -> Result<(Board, Vec<Board>), &'static str> {
    let problem = numlin::deserialize_problem(url).ok_or("invalid url")?;
    // At least two representative answers are needed to determine uniqueness.
    let answers = numlin::enumerate_answers_numlin(&problem, num_max_answers);
    let uniqueness = if num_max_answers <= 1 {
        Uniqueness::NotApplicable
    } else {
        match answers.len() {
            0 => Uniqueness::NoAnswer,
            1 if find_another_answer(&answers[0]).is_none() => Uniqueness::Unique,
            _ => Uniqueness::NonUnique,
        }
    };

    let height = problem.len();
    let width = problem[0].len();
    let mut board_common = Board::new(BoardKind::Grid, height, width, uniqueness);

    for y in 0..height {
        for x in 0..width {
            if let Some(clue) = problem[y][x] {
                if clue > 0 {
                    board_common.push(Item::cell(y, x, "black", ItemKind::Num(clue)));
                } else {
                    board_common.push(Item::cell(y, x, "black", ItemKind::Text("?")));
                }
            }
        }
    }

    let mut boards = vec![];
    for ans in answers.into_iter().take(num_max_answers) {
        let mut board_answer =
            Board::new(BoardKind::Empty, height, width, Uniqueness::NotApplicable);
        for y in 0..height {
            for x in 0..width {
                if x < width - 1 && ans.horizontal[y][x] {
                    board_answer.push(Item {
                        y: y * 2 + 1,
                        x: x * 2 + 2,
                        color: "green",
                        kind: ItemKind::Line,
                    });
                }
                if y < height - 1 && ans.vertical[y][x] {
                    board_answer.push(Item {
                        y: y * 2 + 2,
                        x: x * 2 + 1,
                        color: "green",
                        kind: ItemKind::Line,
                    });
                }
            }
        }

        if let Some(another_answer) = find_another_answer(&ans) {
            for y in 0..height {
                for x in 0..width {
                    if x < width - 1 && another_answer.horizontal[y][x] {
                        board_answer.push(Item {
                            y: y * 2 + 1,
                            x: x * 2 + 2,
                            color: "red",
                            kind: ItemKind::DottedLine,
                        });
                    }
                    if y < height - 1 && another_answer.vertical[y][x] {
                        board_answer.push(Item {
                            y: y * 2 + 2,
                            x: x * 2 + 1,
                            color: "red",
                            kind: ItemKind::DottedLine,
                        });
                    }
                }
            }
        }

        boards.push(board_answer);
    }

    Ok((board_common, boards))
}

const FOUR_NEIGHBOURS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

fn get_edge(edges: &BoolGridEdgesModel, y: i32, x: i32, dy: i32, dx: i32) -> bool {
    let height = edges.horizontal.len();
    let width = edges.horizontal[0].len() + 1;

    // check range
    if dy == 0 {
        let y = y;
        let x = if dx == 1 { x } else { x - 1 };
        if y < 0 || y >= height as i32 || x < 0 || x >= (width - 1) as i32 {
            return false;
        }
        edges.horizontal[y as usize][x as usize]
    } else {
        let y = if dy == 1 { y } else { y - 1 };
        let x = x;
        if y < 0 || y >= (height - 1) as i32 || x < 0 || x >= width as i32 {
            return false;
        }
        edges.vertical[y as usize][x as usize]
    }
}

fn fill_path_id(edges: &BoolGridEdgesModel, path_id: &mut Vec<Vec<i32>>, y: i32, x: i32, id: i32) {
    let height = edges.horizontal.len();
    let width = edges.horizontal[0].len() + 1;

    if y < 0 || y >= height as i32 || x < 0 || x >= width as i32 {
        return;
    }
    if path_id[y as usize][x as usize] != -1 {
        return;
    }
    path_id[y as usize][x as usize] = id;

    for &(dy, dx) in &FOUR_NEIGHBOURS {
        if get_edge(edges, y, x, dy, dx) {
            fill_path_id(edges, path_id, y + dy, x + dx, id);
        }
    }
}

fn find_another_answer(answer: &BoolGridEdgesModel) -> Option<BoolGridEdgesModel> {
    let height = answer.horizontal.len();
    let width = answer.horizontal[0].len() + 1;

    let mut path_id = vec![vec![-1; width]; height];
    {
        let mut next_path_id = 0;
        for y in 0..height {
            for x in 0..width {
                if path_id[y][x] != -1 {
                    continue;
                }
                let mut adj = 0;
                for &(dy, dx) in &FOUR_NEIGHBOURS {
                    if get_edge(answer, y as i32, x as i32, dy, dx) {
                        adj += 1;
                    }
                }
                if adj == 0 {
                    // keep path_id[y][x] = -1 for isolated cell
                    continue;
                }

                let id = next_path_id;
                next_path_id += 1;
                fill_path_id(answer, &mut path_id, y as i32, x as i32, id);
            }
        }
    }

    for y in 0..height {
        for x in 0..width {
            if path_id[y][x] == -1 {
                continue;
            }

            // find a path from (y, x) to another cell with the same path_id
            // which does not use any edge in the answer and does not pass through any cell with a different path_id
            let mut visited = vec![vec![false; width]; height];
            let mut pre = vec![vec![None; width]; height];
            let mut queue = std::collections::VecDeque::new();
            visited[y][x] = true;
            queue.push_back((y, x));

            while let Some((cy, cx)) = queue.pop_front() {
                for &(dy, dx) in &FOUR_NEIGHBOURS {
                    let ny = cy as i32 + dy;
                    let nx = cx as i32 + dx;
                    if ny < 0 || ny >= height as i32 || nx < 0 || nx >= width as i32 {
                        continue;
                    }
                    let ny = ny as usize;
                    let nx = nx as usize;
                    if visited[ny][nx] {
                        continue;
                    }
                    if path_id[ny][nx] != -1 && path_id[ny][nx] != path_id[y][x] {
                        continue;
                    }
                    if get_edge(answer, cy as i32, cx as i32, dy, dx) {
                        continue;
                    }
                    visited[ny][nx] = true;
                    pre[ny][nx] = Some((cy, cx));
                    queue.push_back((ny, nx));

                    if (ny, nx) != (y, x) && path_id[ny][nx] == path_id[y][x] {
                        // found a path
                        let mut new_path = BoolGridEdgesModel {
                            horizontal: vec![vec![false; width - 1]; height],
                            vertical: vec![vec![false; width]; height - 1],
                        };

                        let mut py = ny;
                        let mut px = nx;
                        while (py, px) != (y, x) {
                            let (pcy, pcx) = pre[py][px].unwrap();

                            if py == pcy {
                                // horizontal edge
                                let min_x = px.min(pcx);
                                new_path.horizontal[py][min_x] = true;
                            } else {
                                // vertical edge
                                let min_y = py.min(pcy);
                                new_path.vertical[min_y][px] = true;
                            }

                            py = pcy;
                            px = pcx;
                        }
                        return Some(new_path);
                    }
                }
            }
        }
    }

    None
}
