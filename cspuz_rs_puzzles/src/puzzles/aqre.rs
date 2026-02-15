use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, Dict, HexInt,
    Optionalize, RoomsWithValues, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_aqre(
    border_exactly_once: bool,
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let h = borders.vertical.len();
    assert!(h > 0);
    let w = borders.vertical[0].len() + 1;

    let mut solver = Solver::new();
    let is_black = &solver.bool_var_2d((h, w));
    solver.add_answer_key_bool(is_black);

    graph::active_vertices_connected_2d(&mut solver, is_black);

    if w > 3 {
        solver.add_expr(!is_black.conv2d_and((1, 4)));
        solver.add_expr(is_black.conv2d_or((1, 4)));
    }
    if h > 3 {
        solver.add_expr(!is_black.conv2d_and((4, 1)));
        solver.add_expr(is_black.conv2d_or((4, 1)));
    }

    let rooms = graph::borders_to_rooms(borders);
    assert_eq!(rooms.len(), clues.len());

    if border_exactly_once {
        for y in 0..h {
        for x in 0..w {
            if y < h - 1 && borders.horizontal[y][x] {
                solver.add_expr(is_black.at((y, x)) ^ is_black.at((y + 1, x)));
            }
            if x < w - 1 && borders.vertical[y][x] {
                solver.add_expr(is_black.at((y, x)) ^ is_black.at((y, x + 1)));
            }
        }
    }
    }

    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            let mut cells = vec![];
            for &pt in &rooms[i] {
                cells.push(is_black.at(pt));
            }
            if n >= 0 {
                solver.add_expr(count_true(cells).eq(n));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_black))
}

type Problem = (bool, (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Option<i32>>));

fn combinator() -> impl Combinator<Problem> {
     Tuple2::new(
        Choice::new(vec![
            Box::new(Dict::new(true, "b/")),
            Box::new(Dict::new(false, "")),
        ]),
        Size::new(RoomsWithValues::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ])))
    )   
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.1.0.vertical.len();
    let width = problem.1.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "aqre",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["aqre"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests1() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 0, 1, 1],
                [1, 1, 0, 1, 0, 0],
                [0, 0, 0, 1, 1, 0],
                [1, 1, 0, 1, 1, 1],
                [0, 0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 0, 0, 0],
                [0, 1, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 1],
                [0, 1, 1, 0, 1],
                [0, 1, 1, 0, 1],
            ]),
        };
        let clues = vec![Some(0), None, Some(3), Some(0), Some(0), None];
        (false, (borders, clues))
    }

        fn problem_for_tests2() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0],
                [0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 1],
                [1, 1],
                [1, 1],
            ]),
        };
        let clues = vec![None, None, None];
        (true, (borders, clues))
    }

    #[test]
    fn test_aqre_problem1() {
        let (border_exactly_once, (borders, clues)) = problem_for_tests1();
        let ans = solve_aqre(border_exactly_once, &borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 0, 1, 0, 0, 0],
            [0, 0, 1, 1, 0, 0],
            [1, 1, 0, 1, 1, 1],
            [0, 1, 1, 1, 0, 0],
            [0, 0, 1, 0, 0, 0],
            [0, 0, 1, 0, 0, 0],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_aqre_problem2() {
        let (border_exactly_once, (borders, clues)) = problem_for_tests2();
        let ans = solve_aqre(border_exactly_once, &borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_bool_2d([
            [0, 1, 0],
            [0, 1, 0],
            [0, 1, 0],
        ]);
        assert_eq!(ans, expected);
    }



    #[test]
    fn test_aqre_serializer1() { 
        {
            let problem = problem_for_tests1();
            let url = "https://puzz.link/p?aqre/6/6/8a41dd1t0re00g300g";
            crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }

        {
            let problem = problem_for_tests2();
            let url = "https://puzz.link/p?aqre/b/3/3/vg00i";
            crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
        }
    }

}
