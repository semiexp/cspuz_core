use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, HexInt, Optionalize,
    RoomsWithValues, Size, Spaces,
};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_country_road(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let rooms = graph::borders_to_rooms(borders);
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    let is_passed = &graph::single_cycle_grid_edges(&mut solver, is_line);
    let mut room_id = vec![vec![0; w]; h];

    for i in 0..rooms.len() {
        for &(y, x) in &rooms[i] {
            room_id[y][x] = i;
        }
    }

    let mut room_entrance = vec![vec![]; rooms.len()];
    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && room_id[y][x] != room_id[y + 1][x] {
                room_entrance[room_id[y][x]].push(is_line.vertical.at((y, x)));
                room_entrance[room_id[y + 1][x]].push(is_line.vertical.at((y, x)));
            }
            if x < w - 1 && room_id[y][x] != room_id[y][x + 1] {
                room_entrance[room_id[y][x]].push(is_line.horizontal.at((y, x)));
                room_entrance[room_id[y][x + 1]].push(is_line.horizontal.at((y, x)));
            }
        }
    }

    for i in 0..rooms.len() {
        // Check every room is entered only once
        solver.add_expr(count_true(&room_entrance[i]).eq(2));
    }

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && borders.horizontal[y][x] {
                // Check that there aren't unused cells on both sides of a border
                solver.add_expr(is_passed.at((y, x)) | is_passed.at((y + 1, x)));
            }
            if x < w - 1 && borders.vertical[y][x] {
                solver.add_expr(is_passed.at((y, x)) | is_passed.at((y, x + 1)));
            }
        }
    }

    for i in 0..rooms.len() {
        if let Some(n) = clues[i] {
            let mut cells = vec![];
            for &pt in &rooms[i] {
                cells.push(is_passed.at(pt));
            }
            solver.add_expr(count_true(cells).eq(n));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Option<i32>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(RoomsWithValues::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
    ])))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "country",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["country"], url)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::util;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 1, 1],
                [1, 1, 1, 1, 1, 1],
                [1, 1, 0, 0, 1, 1],
                [0, 0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 1, 1, 0],
                [0, 1, 1, 1, 0],
                [1, 0, 1, 1, 0],
                [0, 1, 1, 0, 1],
                [0, 1, 1, 1, 0],
                [0, 1, 1, 1, 0],
            ]),
        };
        let clues = vec![
            Some(3),
            None,
            None,
            Some(2),
            None,
            None,
            None,
            None,
            None,
            None,
            Some(1),
            Some(4),
        ];
        (borders, clues)
    }

    #[test]
    fn test_country_road_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_country_road(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        #[rustfmt::skip]
        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 1, 0],
                [1, 0, 0, 0, 0],
                [0, 0, 1, 0, 1],
                [1, 0, 0, 1, 0],
                [0, 1, 0, 0, 0],
                [0, 0, 1, 0, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 1, 1, 0],
                [1, 0, 1, 1, 1, 0],
                [1, 0, 0, 0, 0, 1],
                [0, 1, 0, 1, 1, 1],
                [0, 0, 1, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }
    #[test]
    fn test_country_road_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?country/6/6/eemdee0cvv603h2l14";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
