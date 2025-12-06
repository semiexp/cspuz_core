use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, HexInt, Map, Optionalize, PrefixAndSuffix, Size, Spaces, Tuple2, Tuple3,
};
use cspuz_rs::solver::{count_true, BoolExpr, Solver};

pub const CLUE_UP: i32 = -1;
pub const CLUE_DOWN: i32 = -2;
pub const CLUE_UNKNOWN: i32 = -3;

pub fn solve_bdwalk(
    start: (usize, usize),
    goal: (usize, usize),
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    if start == goal {
        return None;
    }

    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let mut max_level = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if n >= 0 {
                    max_level = max_level.max(n);
                }
            }
        }
    }

    let is_passed = &solver.bool_var_2d((h, w));
    {
        // single path
        let (is_line_flat, g) = is_line.representation();
        let line_graph = g.line_graph();
        graph::active_vertices_connected(&mut solver, &is_line_flat, &line_graph);

        for y in 0..h {
            for x in 0..w {
                if (y, x) == start || (y, x) == goal {
                    solver.add_expr(is_line.vertex_neighbors((y, x)).count_true().eq(1));
                } else {
                    solver.add_expr(
                        is_line
                            .vertex_neighbors((y, x))
                            .count_true()
                            .eq(is_passed.at((y, x)).ite(2, 0)),
                    );
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if clues[y][x].is_some() {
                solver.add_expr(is_passed.at((y, x)));
            }
        }
    }

    let direction = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    let up = &(&is_line.vertical & &direction.vertical);
    let down = &(&is_line.vertical & !&direction.vertical);
    let left = &(&is_line.horizontal & &direction.horizontal);
    let right = &(&is_line.horizontal & !&direction.horizontal);

    let level = &solver.int_var_2d((h, w), 1, max_level);

    let mut add_constraint =
        |src: (usize, usize), dest: (usize, usize), edge: BoolExpr| match clues[dest.0][dest.1] {
            Some(CLUE_UP) => {
                solver.add_expr(edge.imp(level.at(dest).gt(level.at(src))));
            }
            Some(CLUE_DOWN) => {
                solver.add_expr(edge.imp(level.at(dest).lt(level.at(src))));
            }
            Some(CLUE_UNKNOWN) => {
                solver.add_expr(edge.imp(level.at(dest).ne(level.at(src))));
            }
            _ => {
                solver.add_expr(edge.imp(level.at(dest).eq(level.at(src))));
            }
        };

    for y in 0..h {
        for x in 0..w {
            if y > 0 {
                add_constraint((y, x), (y - 1, x), up.at((y - 1, x)));
            }
            if y < h - 1 {
                add_constraint((y, x), (y + 1, x), down.at((y, x)));
            }
            if x > 0 {
                add_constraint((y, x), (y, x - 1), left.at((y, x - 1)));
            }
            if x < w - 1 {
                add_constraint((y, x), (y, x + 1), right.at((y, x)));
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                if (y, x) == start {
                    if n == CLUE_UP {
                        solver.add_expr(level.at((y, x)).gt(1));
                    } else if n == CLUE_DOWN {
                        solver.add_expr(level.at((y, x)).lt(max_level));
                    } else if n == CLUE_UNKNOWN {
                        if max_level == 1 {
                            return None;
                        }
                    }
                }
                if n > 0 {
                    solver.add_expr(level.at((y, x)).eq(n));
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            let mut inbound = vec![];
            let mut outbound = vec![];
            if y > 0 {
                inbound.push(is_line.vertical.at((y - 1, x)) & !direction.vertical.at((y - 1, x)));
                outbound.push(up.at((y - 1, x)));
            }
            if y < h - 1 {
                inbound.push(is_line.vertical.at((y, x)) & direction.vertical.at((y, x)));
                outbound.push(down.at((y, x)));
            }
            if x > 0 {
                inbound
                    .push(is_line.horizontal.at((y, x - 1)) & !direction.horizontal.at((y, x - 1)));
                outbound.push(left.at((y, x - 1)));
            }
            if x < w - 1 {
                inbound.push(is_line.horizontal.at((y, x)) & direction.horizontal.at((y, x)));
                outbound.push(right.at((y, x)));
            }

            if (y, x) == start {
                solver.add_expr(count_true(&inbound).eq(0));
                solver.add_expr(count_true(&outbound).eq(1));
            } else if (y, x) == goal {
                solver.add_expr(count_true(&inbound).eq(1));
                solver.add_expr(count_true(&outbound).eq(0));
            } else {
                solver.add_expr(count_true(&inbound).eq(is_passed.at((y, x)).ite(1, 0)));
                solver.add_expr(count_true(&outbound).eq(is_passed.at((y, x)).ite(1, 0)));
            }
        }
    }

    solver.solve().map(|f| {
        eprintln!("is_line: {:?}", f.get(is_line));
        eprintln!("level: {:?}", f.get(level));
    });
    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = ((usize, usize), (usize, usize), Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<((i32, i32), (i32, i32), Vec<Vec<Option<i32>>>)> {
    PrefixAndSuffix::new(
        "m/",
        Size::new(Tuple3::new(
            Tuple2::new(HexInt, HexInt),
            Tuple2::new(HexInt, HexInt),
            ContextBasedGrid::new(Choice::new(vec![
                Box::new(Dict::new(Some(CLUE_UP), "0")),
                Box::new(Dict::new(Some(CLUE_DOWN), "1")),
                Box::new(Dict::new(Some(CLUE_UNKNOWN), ".")),
                Box::new(Optionalize::new(Map::new(
                    HexInt,
                    |x: i32| if x >= 1 { Some(x + 1) } else { None },
                    |x: i32| if x >= 2 { Some(x - 1) } else { None },
                ))),
                Box::new(Spaces::new(None, 'g')),
            ])),
        )),
        "",
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem.2);
    let problem = (
        (problem.0 .1 as i32 + 1, problem.0 .0 as i32 + 1),
        (problem.1 .1 as i32 + 1, problem.1 .0 as i32 + 1),
        problem.2.clone(),
    );
    problem_to_url_with_context(combinator(), "bdwalk", problem, &Context::sized(h, w))
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let res = url_to_problem(combinator(), &["bdwalk"], url)?;
    Some((
        (res.0 .1 as usize - 1, res.0 .0 as usize - 1),
        (res.1 .1 as usize - 1, res.1 .0 as usize - 1),
        res.2,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        let start = (1, 0);
        let goal = (3, 3);
        let clues = vec![
            vec![None, None, Some(CLUE_DOWN), None, None],
            vec![Some(3), Some(3), Some(1), Some(CLUE_UNKNOWN), None],
            vec![None, None, None, None, Some(2)],
            vec![Some(CLUE_UP), None, Some(CLUE_UP), None, None],
        ];
        (start, goal, clues)
    }

    #[test]
    fn test_bdwalk_problem() {
        let (start, goal, clues) = problem_for_tests();
        let ans = solve_bdwalk(start, goal, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 1],
                [1, 0, 1, 0],
                [1, 1, 0, 1],
                [1, 1, 1, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 0, 1],
                [0, 0, 1, 1, 1],
                [1, 0, 0, 0, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_bdwalk_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?bdwalk/m/5/4/1244h1h442.k30g0h";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
