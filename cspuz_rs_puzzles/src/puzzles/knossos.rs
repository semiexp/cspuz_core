use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    get_kudamono_url_info, kudamono_url_info_to_problem, problem_to_kudamono_url_grid, Choice,
    Combinator, DecInt, Dict, KudamonoGrid, Optionalize, PrefixAndSuffix,
};
use cspuz_rs::solver::{count_true, Solver};

pub fn solve_knossos(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut clue_pos = vec![];
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                clue_pos.push((y, x, n));
            }
        }
    }

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), 0, clue_pos.len() as i32 - 1);
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);
    solver.add_expr(
        num.slice((.., ..(w - 1)))
            .ne(num.slice((.., 1..)))
            .iff(&is_border.vertical),
    );
    solver.add_expr(
        num.slice((..(h - 1), ..))
            .ne(num.slice((1.., ..)))
            .iff(&is_border.horizontal),
    );

    for i in 0..(clue_pos.len() as i32) {
        graph::active_vertices_connected_2d(&mut solver, num.eq(i));

        let mut borders = vec![];
        for y in 0..h {
            for x in 0..w {
                if y == 0 {
                    borders.push(num.at((y, x)).eq(i));
                } else {
                    borders.push(num.at((y, x)).eq(i) & num.at((y - 1, x)).ne(i));
                }
                if x == 0 {
                    borders.push(num.at((y, x)).eq(i));
                } else {
                    borders.push(num.at((y, x)).eq(i) & num.at((y, x - 1)).ne(i));
                }
                if y == h - 1 {
                    borders.push(num.at((y, x)).eq(i));
                } else {
                    borders.push(num.at((y, x)).eq(i) & num.at((y + 1, x)).ne(i));
                }
                if x == w - 1 {
                    borders.push(num.at((y, x)).eq(i));
                } else {
                    borders.push(num.at((y, x)).eq(i) & num.at((y, x + 1)).ne(i));
                }
            }
        }

        let (y, x, n) = clue_pos[i as usize];
        solver.add_expr(num.at((y, x)).eq(i));
        if n > 0 {
            solver.add_expr(count_true(&borders).eq(n));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    KudamonoGrid::new(
        Optionalize::new(Choice::new(vec![
            Box::new(Dict::new(-1, "y")),
            Box::new(PrefixAndSuffix::new("(", DecInt, ")")),
        ])),
        None,
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_kudamono_url_grid(combinator(), "knossos", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let info = get_kudamono_url_info(url)?;
    kudamono_url_info_to_problem(combinator(), info)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![Some(8), None, Some(4), None, None],
            vec![Some(-1), None, Some(8), None, None],
            vec![Some(8), None, None, Some(8), None],
            vec![None, None, None, None, Some(10)],
        ]
    }

    #[test]
    fn test_knossos_problem() {
        let problem = problem_for_tests();
        let ans = solve_knossos(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 0, 1, 0, 0],
                [1, 1, 1, 1, 0],
                [0, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 1],
                [1, 1, 0, 1],
                [0, 1, 0, 1],
                [0, 1, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_knossos_serializer() {
        let problem = problem_for_tests();
        let url = "https://pedros.works/paper-puzzle-player?W=5x4&L=(8)1y1(8)1(8)7(4)1(8)2(10)3&G=knossos";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
