use cspuz_rs::graph;
use cspuz_rs::serializer::{
    url_to_problem, Choice2, ContextBasedGrid, Dict, Map, MultiDigit, Optionalize, Rooms, Size,
    Tuple3,
};
use cspuz_rs::serializer::{Combinator, Context};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_railpool(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Vec<i32>>],
    holes: &Option<Vec<Vec<bool>>>,
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let h = borders.vertical.len();
    assert!(h > 0);
    let w = borders.vertical[0].len() + 1;

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let visited = &graph::single_cycle_grid_edges(&mut solver, is_line);
    let mut borders = borders.clone();
    if let Some(holes) = holes {
        for y in 0..h {
            for x in 0..w {
                if holes[y][x] {
                    solver.add_expr(!visited.at((y, x)));
                    if y > 0 {
                        borders.horizontal[y - 1][x] = true;
                    }
                    if y < h - 1 {
                        borders.horizontal[y][x] = true;
                    }
                    if x > 0 {
                        borders.vertical[y][x - 1] = true;
                    }
                    if x < w - 1 {
                        borders.vertical[y][x] = true;
                    }
                } else {
                    solver.add_expr(visited.at((y, x)));
                }
            }
        }
    } else {
        solver.add_expr(visited);
    }

    let horizontal_len = &solver.int_var_2d((h, w), 0, (w - 1) as i32);
    let vertical_len = &solver.int_var_2d((h, w), 0, (h - 1) as i32);
    for y in 0..h {
        for x in 0..w {
            solver.add_expr(
                horizontal_len.at((y, x)).eq(is_line
                    .horizontal
                    .slice_fixed_y((y, x..))
                    .consecutive_prefix_true()
                    + is_line
                        .horizontal
                        .slice_fixed_y((y, ..x))
                        .reverse()
                        .consecutive_prefix_true()),
            );
            solver.add_expr(
                vertical_len.at((y, x)).eq(is_line
                    .vertical
                    .slice_fixed_x((y.., x))
                    .consecutive_prefix_true()
                    + is_line
                        .vertical
                        .slice_fixed_x((..y, x))
                        .reverse()
                        .consecutive_prefix_true()),
            );
        }
    }

    let max_length = (h.max(w) as i32 - 1) as usize;
    let rooms = graph::borders_to_rooms(&borders);
    for room in &rooms {
        for &(y, x) in room {
            let clue = &clues[y][x];
            if clue.is_empty() {
                continue;
            }
            let lengths = &solver.bool_var_1d(max_length + 1);
            for i in 1..=max_length {
                solver.add_expr(lengths.at(i).iff(
                    horizontal_len.select(room).eq(i as i32).any()
                        | vertical_len.select(room).eq(i as i32).any(),
                ));
            }

            let mut n_question = 0;
            for &c in clue {
                if c == 0 {
                    n_question += 1;
                }
            }

            if n_question == 0 {
                for i in 1..=max_length {
                    solver.add_expr(lengths.at(i).iff(clue.contains(&(i as i32))));
                }
            } else {
                let mut other = vec![];
                for i in 1..=max_length {
                    if clue.contains(&(i as i32)) {
                        solver.add_expr(lengths.at(i));
                    } else {
                        other.push(lengths.at(i));
                    }
                }
                solver.add_expr(count_true(other).eq(n_question));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

pub struct RailpoolClueCombinator;

impl Combinator<Vec<Vec<Vec<i32>>>> for RailpoolClueCombinator {
    fn serialize(&self, _ctx: &Context, _input: &[Vec<Vec<Vec<i32>>>]) -> Option<(usize, Vec<u8>)> {
        unimplemented!()
    }

    fn deserialize(&self, ctx: &Context, input: &[u8]) -> Option<(usize, Vec<Vec<Vec<Vec<i32>>>>)> {
        let height = ctx.height.unwrap();
        let width = ctx.width.unwrap();

        let mut result = vec![vec![vec![]; width]; height];
        let mut pos: usize = 0;
        let mut idx: usize = 0;

        while idx < input.len() {
            if input[idx] == b'/' || pos >= height * width {
                break;
            }

            let c = input[idx];
            if b'0' <= c && c <= b'9' {
                let y = pos / width;
                let x = pos % width;
                result[y][x].push((c - b'0') as i32);
                idx += 1;
            } else if c >= b'k' && c <= b'z' {
                let advance = (c - b'k' + 1) as usize;
                pos += advance;
                idx += 1;
            } else {
                break;
            }
        }

        Some((idx, vec![result]))
    }
}

type Problem = (
    Vec<Vec<Vec<i32>>>,
    graph::InnerGridEdges<Vec<Vec<bool>>>,
    Option<Vec<Vec<bool>>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        RailpoolClueCombinator,
        Rooms,
        Choice2::new(
            Optionalize::new(ContextBasedGrid::new(Map::new(
                MultiDigit::new(2, 5),
                |x: bool| Some(if x { 1 } else { 0 }),
                |n: i32| Some(n == 1),
            ))),
            Dict::new(None, ""),
        ),
    ))
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["railpool"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let mut clues = vec![vec![vec![]; 7]; 5];
        clues[0][0] = vec![0];
        clues[3][1] = vec![0, 0];
        clues[3][6] = vec![2, 0];

        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0],
                [0, 1, 0, 0, 0, 0, 1],
                [0, 0, 0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 1],
                [0, 1, 0, 0, 0, 1],
            ]),
        };

        let holes = Some(crate::util::tests::to_bool_2d([
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0],
        ]));

        (clues, borders, holes)
    }

    #[test]
    fn test_railpool_problem() {
        let (clues, borders, holes) = problem_for_tests();
        let ans = solve_railpool(&borders, &clues, &holes);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 0, 1, 1],
                [0, 1, 1, 0, 0, 1],
                [0, 1, 0, 1, 0, 1],
                [1, 0, 1, 0, 1, 0],
                [0, 1, 1, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 0, 1, 1, 0, 1],
                [1, 1, 0, 0, 1, 1, 0],
                [1, 0, 1, 1, 0, 0, 1],
                [0, 1, 0, 0, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_railpool_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?railpool/7/5/0zp00o20rg8032h040gg00000020";
        assert_eq!(deserialize_problem(url).unwrap(), problem);
    }
}
