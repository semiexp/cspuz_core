use crate::util;
use cspuz_rs::graph;
use cspuz_rs::items::{Arrow, NumberedArrow};
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Grid, NumberedArrowCombinator, Optionalize,
    Spaces,
};
use cspuz_rs::solver::{any, Solver};

pub fn solve_hebiichigo(clues: &[Vec<Option<NumberedArrow>>]) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();
    let num = &solver.int_var_2d((h, w), 0, 5);
    solver.add_answer_key_int(num);

    let pointing_cells = |y: usize, x: usize, dir: Arrow| -> Vec<(usize, usize)> {
        let (dy, dx) = match dir {
            Arrow::Up => (-1, 0),
            Arrow::Down => (1, 0),
            Arrow::Left => (0, -1),
            Arrow::Right => (0, 1),
            _ => panic!(),
        };

        let mut y = y as i32;
        let mut x = x as i32;
        let mut ret = vec![];

        loop {
            y += dy;
            x += dx;

            if !(0 <= y && y < h as i32 && 0 <= x && x < w as i32) {
                break;
            }
            if clues[y as usize][x as usize].is_some() {
                break;
            }

            ret.push((y as usize, x as usize));
        }

        ret
    };

    // constraints on snakes
    let is_connected = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_expr(&is_connected.horizontal ^ (num.gt(0).conv2d_and((2, 1))));
    solver.add_expr(&is_connected.vertical ^ (num.gt(0).conv2d_and((1, 2))));

    let size = &solver.int_var_2d((h, w), 1, 5);
    solver.add_expr(size.eq(num.gt(0).ite(5, 1)));
    graph::graph_division_2d(&mut solver, &size, &is_connected);

    for y in 0..h {
        for x in 0..w {
            solver.add_expr(
                num.at((y, x)).ge(2).imp(
                    num.four_neighbors((y, x))
                        .eq(num.at((y, x)) - 1)
                        .count_true()
                        .eq(1),
                ),
            );
            solver.add_expr(
                (num.at((y, x)).ge(1) & num.at((y, x)).le(4)).imp(
                    num.four_neighbors((y, x))
                        .eq(num.at((y, x)) + 1)
                        .count_true()
                        .eq(1),
                ),
            );
        }
    }

    for y in 0..h {
        for x in 0..w {
            if y > 0 {
                solver.add_expr(
                    (num.at((y, x)).eq(1) & num.at((y - 1, x)).eq(2))
                        .imp(num.select(pointing_cells(y, x, Arrow::Down)).eq(0).all()),
                );
            }
            if y < h - 1 {
                solver.add_expr(
                    (num.at((y, x)).eq(1) & num.at((y + 1, x)).eq(2))
                        .imp(num.select(pointing_cells(y, x, Arrow::Up)).eq(0).all()),
                );
            }
            if x > 0 {
                solver.add_expr(
                    (num.at((y, x)).eq(1) & num.at((y, x - 1)).eq(2))
                        .imp(num.select(pointing_cells(y, x, Arrow::Right)).eq(0).all()),
                );
            }
            if x < w - 1 {
                solver.add_expr(
                    (num.at((y, x)).eq(1) & num.at((y, x + 1)).eq(2))
                        .imp(num.select(pointing_cells(y, x, Arrow::Left)).eq(0).all()),
                );
            }
        }
    }

    // constraints on clues
    for y in 0..h {
        for x in 0..w {
            if let Some((dir, n)) = clues[y][x] {
                solver.add_expr(num.at((y, x)).eq(0));

                if dir == Arrow::Unspecified || n < 0 {
                    continue;
                }
                let cells = num.select(pointing_cells(y, x, dir));
                if n == 0 {
                    solver.add_expr(cells.eq(0).all());
                } else {
                    let mut cands = vec![];
                    for i in 0..cells.len() {
                        cands.push(cells.slice(..i).eq(0).all() & cells.at(i).eq(n));
                    }
                    solver.add_expr(any(cands));
                }
            }
        }
    }
    solver.irrefutable_facts().map(|f| f.get(num))
}

type Problem = Vec<Vec<Option<NumberedArrow>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(NumberedArrowCombinator)),
        Box::new(Spaces::new(None, 'a')),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "hebi", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["hebi"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let mut ret = vec![vec![None; 6]; 5];
        ret[0][2] = Some((Arrow::Left, 1));
        ret[1][2] = Some((Arrow::Unspecified, -1));
        ret[1][4] = Some((Arrow::Right, 5));
        ret[3][1] = Some((Arrow::Down, 3));
        ret[4][3] = Some((Arrow::Right, 5));

        ret
    }

    #[test]
    fn test_hebiichigo_problem() {
        let problem = problem_for_tests();
        let ans = solve_hebiichigo(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [2, 1, 0, 2, 3, 4],
            [3, 4, 0, 1, 0, 5],
            [0, 5, 0, 0, 1, 0],
            [5, 0, 1, 0, 2, 3],
            [4, 3, 2, 0, 5, 4],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_hebiichigo_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?hebi/6/5/b31e0.a45h23g45b";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
