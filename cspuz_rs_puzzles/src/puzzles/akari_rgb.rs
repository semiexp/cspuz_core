use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    get_kudamono_url_info, kudamono_url_info_to_problem, problem_to_kudamono_url_grid, Choice,
    Combinator, DecInt, Dict, KudamonoGrid, Map, PrefixAndSuffix,
};
use cspuz_rs::solver::Solver;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AkariRGBClue {
    Empty,
    Block,
    Num(i32),
    R,
    G,
    B,
    RG,
    GB,
    BR,
}

impl AkariRGBClue {
    fn is_block(&self) -> bool {
        matches!(self, AkariRGBClue::Block | AkariRGBClue::Num(_))
    }
}

pub fn solve_akari_rgb(clues: &[Vec<AkariRGBClue>]) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let light = &solver.int_var_2d((h, w), 0, 3);
    solver.add_answer_key_int(light);

    for y in 0..h {
        for x in 0..w {
            match clues[y][x] {
                AkariRGBClue::Block => {
                    solver.add_expr(light.at((y, x)).eq(0));
                }
                AkariRGBClue::Num(n) => {
                    solver.add_expr(light.at((y, x)).eq(0));
                    solver.add_expr(light.four_neighbors((y, x)).ne(0).count_true().eq(n));
                }
                _ => (),
            }
        }
    }

    let blocked: Vec<Vec<_>> = clues
        .iter()
        .map(|row| row.iter().map(AkariRGBClue::is_block).collect())
        .collect();
    let segments = graph::orthogonal_segments(&blocked);
    let horizontal_group = solver.int_var_1d(segments.horizontal.len(), 0, 3);
    let vertical_group = solver.int_var_1d(segments.vertical.len(), 0, 3);
    for (runs, colors) in [
        (&segments.horizontal, &horizontal_group),
        (&segments.vertical, &vertical_group),
    ] {
        for (id, cells) in runs.iter().enumerate() {
            for c in 1..=3 {
                solver.add_expr(
                    light
                        .select(cells)
                        .eq(c)
                        .count_true()
                        .eq(colors.at(id).eq(c).ite(1, 0)),
                );
            }
        }
    }

    for y in 0..h {
        for x in 0..w {
            if clues[y][x].is_block() {
                continue;
            }
            let a = &horizontal_group.at(segments.horizontal_id[y][x].unwrap());
            let b = &vertical_group.at(segments.vertical_id[y][x].unwrap());

            match clues[y][x] {
                AkariRGBClue::Empty => (),
                AkariRGBClue::R => {
                    solver
                        .add_expr((a.eq(1) | b.eq(1)) & (a.eq(0) | a.eq(1)) & (b.eq(0) | b.eq(1)));
                }
                AkariRGBClue::G => {
                    solver
                        .add_expr((a.eq(2) | b.eq(2)) & (a.eq(0) | a.eq(2)) & (b.eq(0) | b.eq(2)));
                }
                AkariRGBClue::B => {
                    solver
                        .add_expr((a.eq(3) | b.eq(3)) & (a.eq(0) | a.eq(3)) & (b.eq(0) | b.eq(3)));
                }
                AkariRGBClue::RG => {
                    solver.add_expr((a.eq(1) & b.eq(2)) | (a.eq(2) & b.eq(1)));
                }
                AkariRGBClue::GB => {
                    solver.add_expr((a.eq(2) & b.eq(3)) | (a.eq(3) & b.eq(2)));
                }
                AkariRGBClue::BR => {
                    solver.add_expr((a.eq(3) & b.eq(1)) | (a.eq(1) & b.eq(3)));
                }
                AkariRGBClue::Num(_) | AkariRGBClue::Block => unreachable!(),
            }
            solver.add_expr(a.ne(0) | b.ne(0));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(light))
}

pub type Problem = Vec<Vec<AkariRGBClue>>;

fn combinator() -> impl Combinator<Problem> {
    KudamonoGrid::new(
        Choice::new(vec![
            Box::new(Dict::new(AkariRGBClue::R, "R")),
            Box::new(Dict::new(AkariRGBClue::G, "G")),
            Box::new(Dict::new(AkariRGBClue::B, "B")),
            Box::new(Dict::new(AkariRGBClue::RG, "Y")),
            Box::new(Dict::new(AkariRGBClue::GB, "C")),
            Box::new(Dict::new(AkariRGBClue::BR, "M")),
            Box::new(Dict::new(AkariRGBClue::Block, "z")),
            Box::new(Map::new(
                PrefixAndSuffix::new("(", DecInt, ")"),
                |c| match c {
                    AkariRGBClue::Num(n) => Some(n),
                    _ => None,
                },
                |n| Some(AkariRGBClue::Num(n)),
            )),
        ]),
        AkariRGBClue::Empty,
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_kudamono_url_grid(combinator(), "akari-rgb", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let info = get_kudamono_url_info(url)?;
    kudamono_url_info_to_problem(combinator(), info)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let mut ret = vec![vec![AkariRGBClue::Empty; 6]; 5];

        ret[0][1] = AkariRGBClue::B;
        ret[1][0] = AkariRGBClue::BR;
        ret[1][2] = AkariRGBClue::Block;
        ret[1][4] = AkariRGBClue::G;
        ret[2][1] = AkariRGBClue::Num(3);
        ret[2][4] = AkariRGBClue::R;
        ret[2][5] = AkariRGBClue::Block;
        ret[3][1] = AkariRGBClue::GB;
        ret[3][5] = AkariRGBClue::RG;
        ret[4][3] = AkariRGBClue::Block;

        ret
    }

    #[test]
    fn test_akari_rgb_problem() {
        let problem = problem_for_tests();
        let ans = solve_akari_rgb(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = crate::util::tests::to_option_2d([
            [0, 0, 3, 0, 0, 0],
            [0, 3, 0, 0, 0, 2],
            [1, 0, 1, 0, 0, 0],
            [0, 0, 0, 2, 0, 0],
            [0, 3, 0, 0, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_akari_rgb_serializer() {
        let problem = problem_for_tests();
        let url =
            "https://pedros.works/paper-puzzle-player?W=6x5&L=M3C3(3)1B2z4z2R7G1Y3z1&G=akari-rgb";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
