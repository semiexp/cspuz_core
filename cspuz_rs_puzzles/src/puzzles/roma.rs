use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    Dict, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{Solver, TRUE};

pub const CLUE_UP: i32 = 1;
pub const CLUE_DOWN: i32 = 2;
pub const CLUE_LEFT: i32 = 3;
pub const CLUE_RIGHT: i32 = 4;
pub const CLUE_GOAL: i32 = 5;

pub fn solve_roma(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let ans = &solver.int_var_2d((h, w), 1, 5);
    solver.add_answer_key_int(ans);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(ans.at((y, x)).eq(n));
            } else {
                solver.add_expr(ans.at((y, x)).ne(CLUE_GOAL));
            }
        }
    }

    let rooms = graph::borders_to_rooms(borders);
    for room in rooms {
        for i in 0..room.len() {
            for j in 0..i {
                solver.add_expr(
                    ans.at(room[i]).ne(ans.at(room[j]))
                        | ans.at(room[i]).eq(CLUE_GOAL)
                        | ans.at(room[j]).eq(CLUE_GOAL),
                );
            }
        }
    }

    let mut g = graph::Graph::new(h * w + 1);
    let vertices = vec![TRUE; h * w + 1];
    let mut edges = vec![];

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                g.add_edge(y * w + x, (y + 1) * w + x);
                edges.push(ans.at((y, x)).eq(CLUE_DOWN) | ans.at((y + 1, x)).eq(CLUE_UP));
            }
            if x < w - 1 {
                g.add_edge(y * w + x, y * w + (x + 1));
                edges.push(ans.at((y, x)).eq(CLUE_RIGHT) | ans.at((y, x + 1)).eq(CLUE_LEFT));
            }
            if clues[y][x] == Some(CLUE_GOAL) {
                g.add_edge(y * w + x, h * w);
                edges.push(TRUE);
            }
        }
    }
    graph::active_vertices_connected_via_active_edges(&mut solver, &vertices, &edges, &g);

    solver.irrefutable_facts().map(|f| f.get(ans))
}

pub(super) type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

pub(super) fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Spaces::new(None, 'a')),
            Box::new(Dict::new(Some(CLUE_UP), "1")),
            Box::new(Dict::new(Some(CLUE_DOWN), "2")),
            Box::new(Dict::new(Some(CLUE_LEFT), "3")),
            Box::new(Dict::new(Some(CLUE_RIGHT), "4")),
            Box::new(Dict::new(Some(CLUE_GOAL), "5")),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "roma",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["roma"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    // https://puzsq.logicpuzzle.app/puzzle/101366
    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 1, 0, 1, 1],
                [1, 0, 0, 1, 0],
                [0, 0, 0, 0, 1],
                [1, 1, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 1, 1, 0],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 0],
                [0, 0, 1, 0],
            ]),
        };
        let mut clues = vec![vec![None; 5]; 5];
        clues[0][3] = Some(CLUE_RIGHT);
        clues[1][1] = Some(CLUE_LEFT);
        clues[1][3] = Some(CLUE_GOAL);
        clues[1][4] = Some(CLUE_DOWN);
        clues[2][0] = Some(CLUE_DOWN);
        clues[2][3] = Some(CLUE_DOWN);
        clues[3][2] = Some(CLUE_UP);
        clues[3][3] = Some(CLUE_LEFT);
        clues[4][0] = Some(CLUE_RIGHT);
        clues[4][3] = Some(CLUE_RIGHT);
        (borders, clues)
    }

    #[test]
    fn test_roma_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_roma(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [4, 4, 2, 4, 2],
            [1, 3, 4, 5, 2],
            [2, 1, 3, 2, 3],
            [4, 4, 1, 3, 1],
            [4, 1, 3, 4, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_roma_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?roma/5/5/tvv2bi1vc4b3a522b2c13a4b4a";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
