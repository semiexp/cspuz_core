use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn solve_tasquare(clues: &[Vec<Option<i32>>]) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    graph::active_vertices_connected_2d(&mut solver, !is_black);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(!is_black.at((y, x))); // Clue is not shaded
                solver.add_expr(is_black.four_neighbors((y, x)).count_true().ge(1));
                if n >= 1 {
                    let connected = &solver.bool_var_2d((h, w));
                    for y2 in 0..h {
                        for x2 in 0..w {
                            if y == y2 && x == x2 {
                                solver.add_expr(connected.at((y2, x2)));
                            } else {
                                solver.add_expr(connected.at((y2, x2)).imp(is_black.at((y2, x2))));
                            }
                        }
                    }
                    solver.add_expr(connected.count_true().eq(n + 1));
                    graph::active_vertices_connected_2d(&mut solver, connected);

                    for nb in connected.four_neighbor_indices((y, x)) {
                        solver.add_expr(is_black.at(nb).imp(connected.at(nb)));
                    }
                    solver.add_expr(
                        (is_black.slice((1.., ..)) & is_black.slice((..(h - 1), ..))).imp(
                            connected
                                .slice((1.., ..))
                                .iff(connected.slice((..(h - 1), ..))),
                        ),
                    );
                    solver.add_expr(
                        (is_black.slice((.., 1..)) & is_black.slice((.., ..(w - 1)))).imp(
                            connected
                                .slice((.., 1..))
                                .iff(connected.slice((.., ..(w - 1)))),
                        ),
                    );
                }
            }
        }
    }

    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_expr(
        (is_black.slice((.., ..(w - 1))) ^ is_black.slice((.., 1..))).iff(&is_border.vertical),
    );
    solver.add_expr(
        (is_black.slice((..(h - 1), ..)) ^ is_black.slice((1.., ..))).iff(&is_border.horizontal),
    );

    let num_up = &solver.int_var_2d((h, w), 0, h as i32 - 1);
    solver.add_expr(num_up.slice_fixed_y((0, ..)).eq(0));
    solver.add_expr(
        num_up.slice((1.., ..)).eq(is_border
            .horizontal
            .ite(0, num_up.slice((..(h - 1), ..)) + 1)),
    );
    let num_down = &solver.int_var_2d((h, w), 0, h as i32 - 1);
    solver.add_expr(num_down.slice_fixed_y((h - 1, ..)).eq(0));
    solver.add_expr(
        num_down
            .slice((..(h - 1), ..))
            .eq(is_border.horizontal.ite(0, num_down.slice((1.., ..)) + 1)),
    );
    let num_left = &solver.int_var_2d((h, w), 0, w as i32 - 1);
    solver.add_expr(num_left.slice_fixed_x((.., 0)).eq(0));
    solver.add_expr(
        num_left.slice((.., 1..)).eq(is_border
            .vertical
            .ite(0, num_left.slice((.., ..(w - 1))) + 1)),
    );
    let num_right = &solver.int_var_2d((h, w), 0, w as i32 - 1);
    solver.add_expr(num_right.slice_fixed_x((.., w - 1)).eq(0));
    solver.add_expr(
        num_right
            .slice((.., ..(w - 1)))
            .eq(is_border.vertical.ite(0, num_right.slice((.., 1..)) + 1)),
    );

    for x in 0..h {
        for y in 0..w {
            solver.add_expr(
                is_black.at((y, x)).imp(
                    (num_up.at((y, x)) + num_down.at((y, x)))
                        .eq(num_left.at((y, x)) + num_right.at((y, x))),
                ),
            );
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "tasquare", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["tasquare"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(-1), None, None, Some(4), None, None],
            vec![None, None, None, None, None, None],
            vec![Some(-1), None, None, None, Some(5), None],
            vec![None, Some(3), None, None, None, Some(-1)],
            vec![None, None, None, None, None, None],
            vec![None, None, Some(2), None, None, Some(-1)],
        ]
    }

    #[test]
    fn test_tasquare_problem() {
        let problem = problem_for_tests();
        let ans = solve_tasquare(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0, 1, 1],
            [0, 0, 0, 0, 1, 1],
            [0, 1, 0, 0, 0, 0],
            [1, 0, 1, 0, 1, 0],
            [0, 0, 0, 0, 0, 1],
            [0, 1, 0, 1, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_tasquare_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?tasquare/6/6/.h4n.i5h3i.n2h.";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
