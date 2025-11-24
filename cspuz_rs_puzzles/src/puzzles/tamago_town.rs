use crate::polyomino::{adjacent_edges, bbox, enumerate_variants, named_tetrominoes};
use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    get_kudamono_url_info, kudamono_url_info_to_problem, problem_to_kudamono_url_grid, Choice,
    Combinator, Dict, KudamonoGrid,
};
use cspuz_rs::solver::{any, Solver};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TamagoTownCell {
    Empty,
    Unused,
    Egg,
    Chicken,
    Pan,
    Question,
}

pub fn solve_tamago_town(
    clues: &[Vec<TamagoTownCell>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);

    let mut ranges = vec![];
    for y in 0..h {
        let mut row = vec![];
        for x in 0..w {
            if clues[y][x] == TamagoTownCell::Unused {
                row.push((1, 1));
            } else {
                row.push((4, 4));
            }
        }
        ranges.push(row);
    }
    let ranges = &solver.int_var_2d_from_ranges((h, w), &ranges);
    graph::graph_division_2d(&mut solver, ranges, &is_border);

    let mut variants = vec![];
    for (_, piece) in named_tetrominoes() {
        variants.extend(enumerate_variants(&piece));
    }

    let mut cands = vec![vec![vec![]; w]; h];

    for piece in &variants {
        let (ph, pw) = bbox(piece);
        if h < ph || w < pw {
            continue;
        }

        for y in 0..=(h - ph) {
            for x in 0..=(w - pw) {
                let mut num_intact_eggs = 0;
                let mut num_broken_eggs = 0;
                let mut num_chickens = 0;
                let mut num_pans = 0;
                let mut num_questions_on_floor = 0;
                let mut num_questions_not_on_floor = 0;
                let mut has_unused = false;

                for &(dy, dx) in piece {
                    match clues[y + dy][x + dx] {
                        TamagoTownCell::Egg => {
                            if piece.contains(&(dy + 1, dx)) {
                                num_broken_eggs += 1;
                            } else {
                                num_intact_eggs += 1;
                            }
                        }
                        TamagoTownCell::Chicken => num_chickens += 1,
                        TamagoTownCell::Pan => num_pans += 1,
                        TamagoTownCell::Question => {
                            if piece.contains(&(dy + 1, dx)) {
                                num_questions_not_on_floor += 1;
                            } else {
                                num_questions_on_floor += 1;
                            }
                        }
                        TamagoTownCell::Empty => (),
                        TamagoTownCell::Unused => {
                            has_unused = true;
                            break;
                        }
                    }
                }

                if has_unused {
                    continue;
                }

                let num_symbols = num_intact_eggs
                    + num_broken_eggs
                    + num_chickens
                    + num_pans
                    + num_questions_on_floor
                    + num_questions_not_on_floor;
                if num_symbols != 2 {
                    continue;
                }
                if num_questions_on_floor + num_questions_not_on_floor == 1 {
                    if num_chickens == 1 && num_questions_not_on_floor == 1 {
                        continue;
                    }
                    if num_pans == 1 && num_questions_on_floor == 1 {
                        continue;
                    }
                }
                if num_questions_on_floor + num_questions_not_on_floor == 0 {
                    if !((num_intact_eggs == 1 && num_chickens == 1)
                        || (num_broken_eggs == 1 && num_pans == 1))
                    {
                        continue;
                    }
                }

                let (conns_horizontal, conns_vertical) = adjacent_edges(piece);
                let v = solver.bool_var();
                for &(dy, dx) in &conns_horizontal {
                    solver.add_expr(v.imp(!is_border.horizontal.at((y + dy, x + dx))));
                }
                for &(dy, dx) in &conns_vertical {
                    solver.add_expr(v.imp(!is_border.vertical.at((y + dy, x + dx))));
                }

                for &(dy, dx) in piece {
                    cands[y + dy][x + dx].push(v.clone());
                }
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if clues[y][x] != TamagoTownCell::Unused {
                solver.add_expr(any(&cands[y][x]));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

type Problem = Vec<Vec<TamagoTownCell>>;

fn combinator() -> impl Combinator<Problem> {
    KudamonoGrid::new(
        Choice::new(vec![
            Box::new(Dict::new(TamagoTownCell::Chicken, "c")),
            Box::new(Dict::new(TamagoTownCell::Egg, "e")),
            Box::new(Dict::new(TamagoTownCell::Pan, "p")),
            Box::new(Dict::new(TamagoTownCell::Unused, "x")),
            Box::new(Dict::new(TamagoTownCell::Question, "y")),
        ]),
        TamagoTownCell::Empty,
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_kudamono_url_grid(combinator(), "tamago-town", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let info = get_kudamono_url_info(url)?;
    kudamono_url_info_to_problem(combinator(), info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[rustfmt::skip]
    fn problem_for_tests() -> Problem {
        vec![
            vec![TamagoTownCell::Egg, TamagoTownCell::Unused, TamagoTownCell::Egg, TamagoTownCell::Empty, TamagoTownCell::Empty, TamagoTownCell::Question],
            vec![TamagoTownCell::Empty, TamagoTownCell::Chicken, TamagoTownCell::Empty, TamagoTownCell::Egg, TamagoTownCell::Unused, TamagoTownCell::Question],
            vec![TamagoTownCell::Empty, TamagoTownCell::Pan, TamagoTownCell::Empty, TamagoTownCell::Egg, TamagoTownCell::Chicken, TamagoTownCell::Question],
            vec![TamagoTownCell::Empty, TamagoTownCell::Pan, TamagoTownCell::Empty, TamagoTownCell::Empty, TamagoTownCell::Empty, TamagoTownCell::Empty],
            vec![TamagoTownCell::Question, TamagoTownCell::Egg, TamagoTownCell::Empty, TamagoTownCell::Question, TamagoTownCell::Empty, TamagoTownCell::Empty],
        ]
    }

    #[test]
    fn test_tamago_town_problem() {
        let problem = problem_for_tests();
        let ans = solve_tamago_town(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 1, 1, 1],
                [0, 1, 0, 1, 1, 0],
                [1, 1, 1, 0, 0, 0],
                [0, 1, 1, 1, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 1, 0, 0, 0],
                [1, 0, 0, 1, 1],
                [0, 1, 1, 1, 1],
                [1, 0, 0, 1, 1],
                [0, 0, 1, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_tamago_town_serializer() {
        let problem = problem_for_tests();
        let url = "https://pedros.works/paper-puzzle-player?W=6x5&L=y0e4e1p1p1c1x1e5y1e2e1c4x1y4y1y1&G=tamago-town";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
