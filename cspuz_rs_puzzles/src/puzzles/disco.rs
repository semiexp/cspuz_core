use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Combinator, Context, Rooms, Size,
};
use cspuz_rs::solver::Solver;

pub fn solve_disco(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
) -> Option<Vec<Vec<Option<bool>>>> {
    let h = borders.vertical.len();
    assert!(h > 0);
    let w = borders.vertical[0].len() + 1;

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);
    graph::active_vertices_connected_2d(&mut solver, is_black);

    let group_a = &solver.bool_var_2d((h, w));
    let group_b = &solver.bool_var_2d((h, w));
    solver.add_expr(!(group_a & group_b));
    solver.add_expr((group_a | group_b).iff(is_black));

    solver.add_expr(!is_black.conv2d_and((2, 2)));

    let rooms = graph::borders_to_rooms(borders);

    for i in 0..rooms.len() {
        if (rooms[i].len() as i32) < 3 {
            return None;
        }

        solver.add_expr(group_a.select(&rooms[i]).count_true().ge(1));
        solver.add_expr(group_b.select(&rooms[i]).count_true().ge(1));
        graph::active_vertices_connected_2d_region(&mut solver, group_a, &rooms[i]);
        graph::active_vertices_connected_2d_region(&mut solver, group_b, &rooms[i]);
    }

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && !borders.horizontal[y][x] {
                solver.add_expr(!(group_a.at((y, x)) & group_b.at((y + 1, x))));
                solver.add_expr(!(group_b.at((y, x)) & group_a.at((y + 1, x))));
            }
            if x < w - 1 && !borders.vertical[y][x] {
                solver.add_expr(!(group_a.at((y, x)) & group_b.at((y, x + 1))));
                solver.add_expr(!(group_b.at((y, x)) & group_a.at((y, x + 1))));
            }
        }
    }
    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = graph::InnerGridEdges<Vec<Vec<bool>>>;

fn combinator() -> impl Combinator<Problem> {
    Size::new(Rooms)
}
pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.vertical.len();
    let width = problem.vertical[0].len() + 1;
    problem_to_url_with_context_pzprxs(
        combinator(),
        "disco",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["disco"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let ret = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 0, 1, 0],
                [1, 1, 0, 0, 1],
                [0, 1, 1, 1, 1],
                [1, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 1, 0],
                [1, 0, 0, 1],
                [0, 1, 0, 0],
                [1, 0, 1, 0],
                [0, 0, 1, 0],
            ]),
        };
        ret
    }

    #[test]
    fn test_disco_problem() {
        let problem = problem_for_tests();
        let ans = solve_disco(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 1, 0],
            [1, 1, 1, 1, 1],
            [0, 1, 0, 0, 1],
            [1, 1, 1, 0, 1],
            [1, 0, 1, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_disco_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?disco/5/5/d552apfg";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
