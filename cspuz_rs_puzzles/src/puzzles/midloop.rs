use crate::puzzles::loop_common::add_full_loop_constraints;
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, NumSpaces, Size, SizeDoubler, Spaces, Tuple2,
};
use cspuz_rs::solver::{Solver, TRUE};

pub type Problem = (bool, Vec<Vec<Option<i32>>>);

pub fn solve_midloop(
    full: bool,
    dots: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (dh, dw) = util::infer_shape(dots);
    let (h, w) = ((dh + 1) / 2, (dw + 1) / 2);
    let mut solver = Solver::new();
    let lines = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&lines.horizontal);
    solver.add_answer_key_bool(&lines.vertical);
    graph::single_cycle_grid_edges(&mut solver, lines);
    solver.add_expr(lines.horizontal.any() | lines.vertical.any());
    if full {
        add_full_loop_constraints(&mut solver, lines, h - 1, w - 1);
    }

    for y in 0..dh {
        for x in 0..dw {
            if dots[y][x] != Some(1) || (y % 2 == 1 && x % 2 == 1) {
                continue;
            }
            let (cy, cx) = (y / 2, x / 2);
            if y % 2 == 0 && x % 2 == 1 {
                solver.add_expr(lines.horizontal.at((cy, cx)));
            } else if y % 2 == 1 {
                solver.add_expr(lines.vertical.at((cy, cx)));
            } else {
                solver.add_expr(lines.vertex_neighbors((cy, cx)).any());
            }
            // Compare consecutive line prefixes; later disconnected segments do not count.
            if y % 2 == 0 {
                let (mut left, mut right) = (TRUE.expr(), TRUE.expr());
                let shift = (x % 2) as i32;
                for d in 0..w {
                    left = left
                        & lines
                            .horizontal
                            .at_offset((cy, cx), (0, -1 - d as i32), false);
                    right = right
                        & lines
                            .horizontal
                            .at_offset((cy, cx), (0, shift + d as i32), false);
                    solver.add_expr(left.clone().iff(right.clone()));
                }
            }
            if x % 2 == 0 {
                let (mut up, mut down) = (TRUE.expr(), TRUE.expr());
                let shift = (y % 2) as i32;
                for d in 0..h {
                    up = up
                        & lines
                            .vertical
                            .at_offset((cy, cx), (-1 - d as i32, 0), false);
                    down = down
                        & lines
                            .vertical
                            .at_offset((cy, cx), (shift + d as i32, 0), false);
                    solver.add_expr(up.clone().iff(down.clone()));
                }
            }
        }
    }
    solver.irrefutable_facts().map(|f| f.get(lines))
}

fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "f/")),
            Box::new(Dict::new(false, "")),
        ]),
        Size::new(SizeDoubler::new(ContextBasedGrid::new(Choice::new(vec![
            Box::new(NumSpaces::new(1, 7)),
            Box::new(Spaces::new(None, 'g')),
        ])))),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem.1);
    problem_to_url_with_context(
        combinator(),
        "midloop",
        problem.clone(),
        &Context::sized((h + 1) / 2, (w + 1) / 2),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["midloop"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midloop() {
        let url = "https://puzz.link/p?midloop/3/3/ifqf";
        let problem = deserialize_problem(url).unwrap();
        let ans = solve_midloop(problem.0, &problem.1).unwrap();
        assert_eq!(
            ans,
            graph::BoolGridEdgesIrrefutableFacts {
                horizontal: util::tests::to_option_bool_2d([[0, 1], [1, 0], [1, 1]]),
                vertical: util::tests::to_option_bool_2d([[0, 1, 1], [1, 0, 1]]),
            }
        );
        assert_eq!(
            deserialize_problem(&serialize_problem(&problem).unwrap()),
            Some(problem)
        );
    }

    #[test]
    fn test_full_loop_variant() {
        let problem = deserialize_problem("https://puzz.link/p?midloop/f/2/2/o").unwrap();
        assert!(problem.0);
        let ans = solve_midloop(problem.0, &problem.1).unwrap();
        assert!(ans
            .horizontal
            .iter()
            .flatten()
            .chain(ans.vertical.iter().flatten())
            .all(|v| *v == Some(true)));
        assert_eq!(
            deserialize_problem(&serialize_problem(&problem).unwrap()),
            Some(problem)
        );
    }

    #[test]
    fn test_loop_must_exist() {
        let problem = deserialize_problem("https://puzz.link/p?midloop/2/2/o").unwrap();
        let ans = solve_midloop(problem.0, &problem.1).unwrap();
        assert!(ans
            .horizontal
            .iter()
            .flatten()
            .chain(ans.vertical.iter().flatten())
            .all(|v| *v == Some(true)));
    }
}
