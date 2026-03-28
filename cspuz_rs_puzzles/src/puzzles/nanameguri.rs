use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Combinator, Context, ContextBasedGrid, MultiDigit,
    Rooms, Size, Tuple2,
};
use cspuz_rs::solver::{any, count_true, Solver};

pub const NANAMEGURI_EMPTY: i32 = 0;
pub const NANAMEGURI_BACKSLASH: i32 = 1;
pub const NANAMEGURI_SLASH: i32 = 2;

pub fn solve_nanameguri(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    cells: &[Vec<i32>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(cells);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    graph::single_cycle_grid_edges(&mut solver, is_line);

    for y in 0..h {
        for x in 0..w {
            if cells[y][x] == NANAMEGURI_SLASH {
                let mut cands = vec![];
                if y > 0 && x > 0 {
                    cands.push(is_line.vertical.at((y - 1, x)) & is_line.horizontal.at((y, x - 1)));
                }
                if y < h - 1 && x < w - 1 {
                    cands.push(is_line.vertical.at((y, x)) & is_line.horizontal.at((y, x)));
                }
                solver.add_expr(any(cands));
            } else if cells[y][x] == NANAMEGURI_BACKSLASH {
                let mut cands = vec![];
                if y > 0 && x < w - 1 {
                    cands.push(is_line.vertical.at((y - 1, x)) & is_line.horizontal.at((y, x)));
                }
                if y < h - 1 && x > 0 {
                    cands.push(is_line.vertical.at((y, x)) & is_line.horizontal.at((y, x - 1)));
                }
                solver.add_expr(any(cands));
            }
        }
    }

    // each cell has 4 segments:
    // \ 0 /
    //  \ /
    // 1 X 2
    //  / \
    // / 3 \
    let mut room_id = vec![vec![[!0, !0, !0, !0]; w]; h];
    let mut idx = 0;

    fn visit(
        room_id: &mut Vec<Vec<[usize; 4]>>,
        borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
        cells: &[Vec<i32>],
        y: usize,
        x: usize,
        seg: usize,
        idx: usize,
    ) {
        if room_id[y][x][seg] != !0 {
            return;
        }
        room_id[y][x][seg] = idx;

        match seg {
            0 => {
                if y > 0 && !borders.horizontal[y - 1][x] {
                    visit(room_id, borders, cells, y - 1, x, 3, idx);
                }
                if cells[y][x] != NANAMEGURI_BACKSLASH {
                    visit(room_id, borders, cells, y, x, 1, idx);
                }
                if cells[y][x] != NANAMEGURI_SLASH {
                    visit(room_id, borders, cells, y, x, 2, idx);
                }
            }
            1 => {
                if x > 0 && !borders.vertical[y][x - 1] {
                    visit(room_id, borders, cells, y, x - 1, 2, idx);
                }
                if cells[y][x] != NANAMEGURI_BACKSLASH {
                    visit(room_id, borders, cells, y, x, 0, idx);
                }
                if cells[y][x] != NANAMEGURI_SLASH {
                    visit(room_id, borders, cells, y, x, 3, idx);
                }
            }
            2 => {
                if x < cells[0].len() - 1 && !borders.vertical[y][x] {
                    visit(room_id, borders, cells, y, x + 1, 1, idx);
                }
                if cells[y][x] != NANAMEGURI_BACKSLASH {
                    visit(room_id, borders, cells, y, x, 3, idx);
                }
                if cells[y][x] != NANAMEGURI_SLASH {
                    visit(room_id, borders, cells, y, x, 0, idx);
                }
            }
            3 => {
                if y < cells.len() - 1 && !borders.horizontal[y][x] {
                    visit(room_id, borders, cells, y + 1, x, 0, idx);
                }
                if cells[y][x] != NANAMEGURI_BACKSLASH {
                    visit(room_id, borders, cells, y, x, 2, idx);
                }
                if cells[y][x] != NANAMEGURI_SLASH {
                    visit(room_id, borders, cells, y, x, 1, idx);
                }
            }
            _ => unreachable!(),
        }
    }

    for y in 0..h {
        for x in 0..w {
            for seg in 0..4 {
                if room_id[y][x][seg] == !0 {
                    visit(&mut room_id, borders, cells, y, x, seg, idx);
                    idx += 1;
                }
            }
        }
    }

    let mut borders_by_room = vec![vec![]; idx];
    for y in 0..h {
        for x in 0..w {
            if y > 0 && borders.horizontal[y - 1][x] {
                borders_by_room[room_id[y][x][0]].push(is_line.vertical.at((y - 1, x)));
            }
            if x > 0 && borders.vertical[y][x - 1] {
                borders_by_room[room_id[y][x][1]].push(is_line.horizontal.at((y, x - 1)));
            }
            if x < w - 1 && borders.vertical[y][x] {
                borders_by_room[room_id[y][x][2]].push(is_line.horizontal.at((y, x)));
            }
            if y < h - 1 && borders.horizontal[y][x] {
                borders_by_room[room_id[y][x][3]].push(is_line.vertical.at((y, x)));
            }
        }
    }

    if idx == 1 {
        solver.add_expr(!any(&borders_by_room[0]));
    } else {
        for room in &borders_by_room {
            solver.add_expr(count_true(room).eq(2));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<i32>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(MultiDigit::new(3, 3)),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "nanameguri",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["nanameguri"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [1, 1, 0, 1, 0],
                [0, 0, 1, 1, 1],
                [0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 0, 0, 0],
                [1, 0, 1, 0],
                [0, 1, 0, 1],
                [0, 0, 0, 0],
            ]),
        };

        let cells = vec![
            vec![0, 1, 0, 1, 0],
            vec![0, 0, 0, 0, 0],
            vec![0, 1, 0, 0, 0],
            vec![0, 0, 0, 2, 0],
        ];

        (borders, cells)
    }

    #[test]
    #[rustfmt::skip]
    fn test_nanameguri_problem() {
        let (borders, cells) = problem_for_tests();
        let ans = solve_nanameguri(&borders, &cells);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: util::tests::to_option_bool_2d([
                [1, 0, 1, 0],
                [0, 1, 0, 1],
                [1, 0, 0, 1],
                [0, 1, 1, 0],
            ]),
            vertical: util::tests::to_option_bool_2d([
                [1, 1, 1, 1, 0],
                [1, 0, 0, 0, 1],
                [0, 1, 0, 1, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_nanameguri_serializer() {
        {
            let problem = problem_for_tests();
            let url = "https://puzz.link/p?nanameguri/5/4/1980q70390100i";
            util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }
}
