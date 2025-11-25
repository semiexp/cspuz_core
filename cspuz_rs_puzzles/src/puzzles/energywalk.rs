use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, Dict, HexInt, Map, MultiDigit, Optionalize, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, BoolExpr, Solver};

pub fn solve_energywalk(
    colored: &[Vec<bool>],
    num: &[Vec<Option<i32>>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(colored);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    // The network is conneceted
    {
        let (vertices, g) = is_line.representation();
        let line_graph = g.line_graph();

        graph::active_vertices_connected(&mut solver, &vertices, &line_graph);
    }

    let direction = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    let up = &(&is_line.vertical & &direction.vertical);
    let down = &(&is_line.vertical & !&direction.vertical);
    let left = &(&is_line.horizontal & &direction.horizontal);
    let right = &(&is_line.horizontal & !&direction.horizontal);

    let line_size = solver.int_var_2d((h, w), 0, (h * w) as i32);
    let line_rank = solver.int_var_2d((h, w), 0, (h * w) as i32);
    let line_origin = solver.int_var_2d((h, w), 0, (h * w) as i32 - 1);

    let mut add_constraint = |src: (usize, usize), dest: (usize, usize), edge: BoolExpr| match (
        colored[src.0][src.1],
        colored[dest.0][dest.1],
    ) {
        (false, false) => {
            solver.add_expr(edge.imp(
                line_size.at(src).eq(line_size.at(dest))
                    & line_rank.at(src).eq(line_rank.at(dest) + 1),
            ));
            solver.add_expr(edge.imp(line_origin.at(src).eq(line_origin.at(dest))));
        }
        (false, true) => {
            solver.add_expr(edge.imp(line_rank.at(src).eq(1)));
            solver.add_expr(edge.imp(line_origin.at(src).ne((dest.0 * w + dest.1) as i32)));
        }
        (true, false) => {
            solver.add_expr(edge.imp(line_rank.at(dest).eq(line_size.at(dest))));
            solver.add_expr(edge.imp(line_origin.at(dest).eq((src.0 * w + src.1) as i32)));
        }
        (true, true) => (),
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
            if colored[y][x] {
                continue;
            }

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
            solver.add_expr(count_true(&inbound).eq(count_true(&outbound)));
        }
    }

    for y in 0..h {
        for x in 0..w {
            let is_passed = &solver.bool_var();
            if colored[y][x] {
                solver.add_expr(is_line.vertex_neighbors((y, x)).iff(is_passed));
            } else {
                solver.add_expr(
                    is_line
                        .vertex_neighbors((y, x))
                        .count_true()
                        .eq(is_passed.ite(2, 0)),
                );
                if let Some(n) = num[y][x] {
                    if n >= 0 {
                        solver.add_expr(line_size.at((y, x)).eq(n));
                    }
                    solver.add_expr(is_line.vertex_neighbors((y, x)).any());
                }
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && (colored[y][x] ^ colored[y + 1][x]) {
                solver.add_expr(
                    is_line
                        .vertical
                        .at((y, x))
                        .imp(line_size.at((y, x)).ne(line_size.at((y + 1, x)))),
                );
            }
            if x < w - 1 && (colored[y][x] ^ colored[y][x + 1]) {
                solver.add_expr(
                    is_line
                        .horizontal
                        .at((y, x))
                        .imp(line_size.at((y, x)).ne(line_size.at((y, x + 1)))),
                );
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = (Vec<Vec<bool>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(2, 5),
            |x| Some(if x { 1 } else { 0 }),
            |x| Some(x == 1),
        )),
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = util::infer_shape(&problem.0);
    problem_to_url_with_context_and_site(
        combinator(),
        "energywalk",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["energywalk"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        (
            crate::util::tests::to_bool_2d([
                [0, 0, 1, 0, 1, 0],
                [0, 1, 0, 0, 0, 0],
                [1, 0, 1, 1, 0, 0],
                [0, 0, 0, 0, 0, 1],
                [0, 1, 1, 0, 0, 1],
            ]),
            vec![
                vec![Some(3), None, None, Some(1), None, None],
                vec![None, None, None, None, None, None],
                vec![None, Some(2), None, None, None, None],
                vec![None, None, None, None, None, None],
                vec![None, None, None, None, None, None],
            ],
        )
    }

    #[test]
    fn test_energywalk_problem() {
        let (water, num) = problem_for_tests();
        let ans = solve_energywalk(&water, &num);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::GridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 1, 1, 1, 1],
                [0, 0, 1, 1, 0],
                [1, 0, 0, 0, 0],
                [0, 0, 1, 1, 1],
                [1, 1, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 0, 1, 0, 1, 1],
                [1, 0, 0, 0, 0, 1],
                [1, 1, 0, 0, 0, 1],
                [1, 1, 1, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_energywalk_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?energywalk/6/5/545g2p3h1o2v";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
