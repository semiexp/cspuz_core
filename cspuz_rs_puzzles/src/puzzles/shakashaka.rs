use crate::util;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, NumSpaces, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, IntVarArray1D, Solver, FALSE, TRUE};

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum ShakashakaCell {
    Blank,
    UpperLeft,
    LowerLeft,
    LowerRight,
    UpperRight,
}

pub fn solve_shakashaka(
    problem: &[Vec<Option<i32>>],
    no_square: bool,
) -> Option<Vec<Vec<Option<ShakashakaCell>>>> {
    let (h, w) = util::infer_shape(problem);

    // 1   2   3   4
    // +-+ +     + +-+
    // |/  |\   /|  \|
    // +   +-+ +-+   +
    let mut solver = Solver::new();
    let ans = &solver.int_var_2d((h, w), 0, 4);
    solver.add_answer_key_int(ans);

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = problem[y][x] {
                solver.add_expr(ans.at((y, x)).eq(0));
                if n >= 0 {
                    solver.add_expr(ans.four_neighbors((y, x)).ne(0).count_true().eq(n));
                }
            }
        }
    }
    for y in 0..=h {
        for x in 0..=w {
            let mut diagonals = vec![];
            let mut is_empty = vec![];
            let mut is_white_angle = vec![];

            if y > 0 && x > 0 {
                let a = &ans.at((y - 1, x - 1));
                diagonals.push(a.eq(4));
                diagonals.push(a.eq(2));
                if problem[y - 1][x - 1].is_none() {
                    is_empty.push(a.eq(0));
                    is_white_angle.push(a.eq(0) | a.eq(1));
                } else {
                    is_empty.push(FALSE);
                }
            } else {
                diagonals.push(FALSE);
                diagonals.push(FALSE);
                is_empty.push(FALSE);
            }
            if y < h && x > 0 {
                let a = &ans.at((y, x - 1));
                diagonals.push(a.eq(1));
                diagonals.push(a.eq(3));
                if problem[y][x - 1].is_none() {
                    is_empty.push(a.eq(0));
                    is_white_angle.push(a.eq(0) | a.eq(2));
                } else {
                    is_empty.push(FALSE);
                }
            } else {
                diagonals.push(FALSE);
                diagonals.push(FALSE);
                is_empty.push(FALSE);
            }
            if y < h && x < w {
                let a = &ans.at((y, x));
                diagonals.push(a.eq(2));
                diagonals.push(a.eq(4));
                if problem[y][x].is_none() {
                    is_empty.push(a.eq(0));
                    is_white_angle.push(a.eq(0) | a.eq(3));
                } else {
                    is_empty.push(FALSE);
                }
            } else {
                diagonals.push(FALSE);
                diagonals.push(FALSE);
                is_empty.push(FALSE);
            }
            if y > 0 && x < w {
                let a = &ans.at((y - 1, x));
                diagonals.push(a.eq(3));
                diagonals.push(a.eq(1));
                if problem[y - 1][x].is_none() {
                    is_empty.push(a.eq(0));
                    is_white_angle.push(a.eq(0) | a.eq(4));
                } else {
                    is_empty.push(FALSE);
                }
            } else {
                diagonals.push(FALSE);
                diagonals.push(FALSE);
                is_empty.push(FALSE);
            }

            for i in 0..8 {
                if i % 2 == 0 {
                    solver.add_expr(diagonals[i].imp(
                        &diagonals[(i + 3) % 8]
                            | (&is_empty[(i + 3) % 8 / 2] & &diagonals[(i + 5) % 8]),
                    ));
                } else {
                    solver.add_expr(diagonals[i].imp(
                        &diagonals[(i + 5) % 8]
                            | (&is_empty[(i + 5) % 8 / 2] & &diagonals[(i + 3) % 8]),
                    ));
                }
            }
            solver.add_expr(count_true(is_white_angle).ne(3));
        }
    }

    if no_square {
        let get_cells = |y: usize, x: usize, dy: i32, dx: i32| -> IntVarArray1D {
            let mut y = y as i32;
            let mut x = x as i32;
            let mut ret = vec![];

            loop {
                if !(0 <= y && y < h as i32 && 0 <= x && x < w as i32) {
                    break;
                }
                if problem[y as usize][x as usize].is_some() {
                    break;
                }
                ret.push(ans.at((y as usize, x as usize)));
                y += dy;
                x += dx;
            }

            IntVarArray1D::new(ret)
        };

        for y in 0..h {
            for x in 0..w {
                if problem[y][x].is_some() {
                    continue;
                }
                let down = get_cells(y, x, 1, 0).eq(0).consecutive_prefix_true();
                let right = get_cells(y, x, 0, 1).eq(0).consecutive_prefix_true();

                let up_wall = if y == 0 || problem[y - 1][x].is_some() {
                    TRUE
                } else {
                    ans.at((y - 1, x)).eq(2) | ans.at((y - 1, x)).eq(3)
                };
                let left_wall = if x == 0 || problem[y][x - 1].is_some() {
                    TRUE
                } else {
                    ans.at((y, x - 1)).eq(3) | ans.at((y, x - 1)).eq(4)
                };
                solver.add_expr((ans.at((y, x)).eq(0) & up_wall & left_wall).imp(down.ne(right)));
            }
        }

        for y in 0..=h {
            for x in 0..=w {
                if y < h && 0 < x && x < w {
                    let dl = get_cells(y, x - 1, 1, -1).eq(1).consecutive_prefix_true();
                    let dr = get_cells(y, x, 1, 1).eq(4).consecutive_prefix_true();
                    solver
                        .add_expr((ans.at((y, x - 1)).eq(1) & ans.at((y, x)).eq(4)).imp(dl.ne(dr)));
                }
                // following constraints are redundant
                /*
                if 0 < y && 0 < x && x < w {
                    let ul = get_cells(y - 1, x - 1, -1, -1).eq(2).consecutive_prefix_true();
                    let ur = get_cells(y - 1, x, -1, 1).eq(3).consecutive_prefix_true();
                    solver.add_expr((ans.at((y - 1, x - 1)).eq(2) & ans.at((y - 1, x)).eq(3)).imp(ul.ne(ur)));
                }
                if 0 < y && y < h && x < w {
                    let ur = get_cells(y - 1, x, -1, 1).eq(1).consecutive_prefix_true();
                    let dr = get_cells(y, x, 1, 1).eq(2).consecutive_prefix_true();
                    solver.add_expr((ans.at((y - 1, x)).eq(1) & ans.at((y, x)).eq(2)).imp(ur.ne(dr)));
                }
                if 0 < y && y < h && 0 < x {
                    let ul = get_cells(y - 1, x - 1, -1, -1).eq(4).consecutive_prefix_true();
                    let dl = get_cells(y, x - 1, 1, -1).eq(3).consecutive_prefix_true();
                    solver.add_expr((ans.at((y - 1, x - 1)).eq(4) & ans.at((y, x - 1)).eq(3)).imp(ul.ne(dl)));
                }
                */
            }
        }
    }

    solver.irrefutable_facts().map(|f| {
        let model = f.get(ans);
        model
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|v| {
                        v.map(|n| match n {
                            0 => ShakashakaCell::Blank,
                            1 => ShakashakaCell::UpperLeft,
                            2 => ShakashakaCell::LowerLeft,
                            3 => ShakashakaCell::LowerRight,
                            4 => ShakashakaCell::UpperRight,
                            _ => panic!(),
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
    })
}

type Problem = (Vec<Vec<Option<i32>>>, bool);

fn combinator() -> impl Combinator<Problem> {
    Tuple2::new(
        Grid::new(Choice::new(vec![
            Box::new(Spaces::new(None, 'g')),
            Box::new(NumSpaces::new(4, 2)),
            Box::new(Dict::new(Some(-1), ".")),
        ])),
        Choice::new(vec![
            Box::new(Dict::new(true, "/nosquare")),
            Box::new(Dict::new(false, "")),
        ]),
    )
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "shakashaka", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["shakashaka"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Vec<Vec<Option<i32>>> {
        // https://twitter.com/semiexp/status/1223794016593956864
        let height = 10;
        let width = 10;
        let mut problem = vec![vec![None; width]; height];
        problem[1][2] = Some(3);
        problem[2][7] = Some(2);
        problem[2][9] = Some(0);
        problem[3][0] = Some(1);
        problem[3][3] = Some(3);
        problem[4][6] = Some(3);
        problem[5][0] = Some(2);
        problem[5][3] = Some(2);
        problem[6][8] = Some(2);
        problem[9][3] = Some(2);
        problem[9][7] = Some(0);
        problem
    }

    #[test]
    fn test_shakashaka_problem() {
        let problem = (problem_for_tests(), false);
        let ans = solve_shakashaka(&problem.0, problem.1);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        util::tests::check_all_some(&ans);
        assert_eq!(ans[0][5], Some(ShakashakaCell::UpperLeft));
        assert_eq!(ans[7][4], Some(ShakashakaCell::UpperRight));
        assert_eq!(ans[6][2], Some(ShakashakaCell::LowerRight));
    }

    #[test]
    fn test_shakashaka_serializer() {
        let problem = (problem_for_tests(), false);
        let url = "https://puzz.link/p?shakashaka/10/10/rdr70bdpdgccrczhcga";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
