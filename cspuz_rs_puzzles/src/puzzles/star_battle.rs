use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Combinator, Context, DecInt, PrefixAndSuffix,
    Rooms, Size, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_star_battle(
    star_amount: i32,
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
) -> Option<Vec<Vec<Option<bool>>>> {
    let height = borders.vertical.len();

    if height != borders.vertical[0].len() + 1 {
        // Non-square grid, throw no solutions
        return None;
    }

    let mut solver = Solver::new();

    let has_star = solver.bool_var_2d((height, height));
    solver.add_answer_key_bool(&has_star);

    let rooms = graph::borders_to_rooms(borders);

    for i in 0..height {
        solver.add_expr(has_star.slice_fixed_y((i, ..)).count_true().eq(star_amount));
        solver.add_expr(has_star.slice_fixed_x((.., i)).count_true().eq(star_amount));
    }
    solver.add_expr(!(has_star.slice((..(height - 1), ..)) & has_star.slice((1.., ..))));
    solver.add_expr(!(has_star.slice((.., ..(height - 1))) & has_star.slice((.., 1..))));
    solver
        .add_expr(!(has_star.slice((..(height - 1), ..(height - 1))) & has_star.slice((1.., 1..))));
    solver
        .add_expr(!(has_star.slice((..(height - 1), 1..)) & has_star.slice((1.., ..(height - 1)))));

    for room in &rooms {
        solver.add_expr(has_star.select(room).count_true().eq(star_amount));
    }

    solver.irrefutable_facts().map(|f| f.get(&has_star))
}

pub type Problem = (i32, graph::InnerGridEdges<Vec<Vec<bool>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(PrefixAndSuffix::new("", DecInt, "/"), Rooms))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url_with_context(
        combinator(),
        "starbattle",
        problem.clone(),
        &Context::sized(problem.1.vertical.len(), problem.1.vertical[0].len() + 1),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["starbattle"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let star_amount = 1 as i32;
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 1, 0, 0, 0],
                [1, 0, 0, 1, 1, 0],
                [0, 1, 1, 1, 1, 0],
                [0, 1, 1, 0, 1, 1],
                [0, 1, 0, 1, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 0, 0, 1, 0],
                [1, 1, 1, 1, 0],
                [0, 1, 0, 0, 1],
                [1, 0, 0, 0, 0],
                [1, 0, 1, 1, 1],
                [0, 1, 0, 0, 1],
            ]),
        };
        (star_amount, borders)
    }

    #[test]
    fn test_star_battle_problem() {
        let (star_amount, borders) = problem_for_tests();
        let ans = solve_star_battle(star_amount, &borders);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0, 0],
            [1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1],
            [0, 0, 0, 1, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_star_battle_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?starbattle/6/6/1/2u9gn9c9jpmk";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
