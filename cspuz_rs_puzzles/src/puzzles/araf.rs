use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::{any, count_true, Solver};

pub fn solve_araf(clues: &[Vec<Option<i32>>]) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);

    let mut clue_pos = vec![];
    let mut clue_max = 2;
    let mut clue_min = (h * w) as i32 + 1;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                clue_pos.push((y, x, n));
                if n == -1 {
                    clue_max = (h * w) as i32 + 1;
                    clue_min = 0;
                }
                clue_max = clue_max.max(n);
                clue_min = clue_min.min(n);
            }
        }
    }

    let mut solver = Solver::new();
    let mut blocks = vec![];
    for &(y, x, _) in &clue_pos {
        let block = solver.bool_var_2d((h, w));
        graph::active_vertices_connected_2d(&mut solver, &block);
        solver.add_expr(block.at((y, x)));
        blocks.push(block);
    }

    if clue_min + 1 > clue_max - 1 {
        return None;
    }

    for i in 0..blocks.len() {
        let size = &solver.int_var(clue_min + 1, clue_max - 1);
        solver.add_expr(blocks[i].count_true().eq(size));
        for j in (i + 1)..blocks.len() {
            let (yi, xi, ni) = clue_pos[i];
            let (yj, xj, nj) = clue_pos[j];

            if (ni - nj).abs() <= 1 && ni != -1 && nj != -1 {
                solver.add_expr(!(blocks[i].at((yj, xj))));
                solver.add_expr(!(blocks[j].at((yi, xi))));
            } else {
                let lo = ni.min(nj);
                let hi = ni.max(nj);
                if lo == -1 {
                    solver.add_expr(
                        (blocks[i].at((yj, xj)) | blocks[j].at((yi, xi)))
                            .imp(blocks[i].iff(&blocks[j]) & size.ne(hi)),
                    );
                } else {
                    solver.add_expr(
                        (blocks[i].at((yj, xj)) | blocks[j].at((yi, xi)))
                            .imp(blocks[i].iff(&blocks[j]) & size.gt(lo) & size.lt(hi)),
                    );
                }
            }
        }
    }
    for y in 0..h {
        for x in 0..w {
            let mut indicators = vec![];
            for i in 0..blocks.len() {
                indicators.push(blocks[i].at((y, x)));
            }
            solver.add_expr(count_true(indicators).eq(2));
        }
    }

    let border = &graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&border.horizontal);
    solver.add_answer_key_bool(&border.vertical);

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 {
                let diff = blocks
                    .iter()
                    .map(|block| block.at((y, x)) ^ block.at((y + 1, x)))
                    .collect::<Vec<_>>();
                solver.add_expr(border.horizontal.at((y, x)).iff(any(diff)));
            }
            if x < w - 1 {
                let diff = blocks
                    .iter()
                    .map(|block| block.at((y, x)) ^ block.at((y, x + 1)))
                    .collect::<Vec<_>>();
                solver.add_expr(border.vertical.at((y, x)).iff(any(diff)));
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(border))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "araf", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["araf"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let height = 6;
        let width = 6;
        let mut ret = vec![vec![None; width]; height];
        ret[0][2] = Some(3);
        ret[1][1] = Some(3);
        ret[2][0] = Some(3);
        ret[3][5] = Some(28);
        ret[4][4] = Some(8);
        ret[5][3] = Some(8);
        ret
    }

    #[test]
    fn test_araf_problem() {
        let problem = problem_for_tests();
        let ans = solve_araf(&problem);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        util::tests::check_all_some(&ans.horizontal);
        util::tests::check_all_some(&ans.vertical);
    }

    #[test]
    fn test_araf_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?araf/6/6/h3j3j3p-1cj8j8h";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
