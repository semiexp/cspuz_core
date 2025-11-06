use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, Spaces,
};
use cspuz_rs::solver::{all, any, Solver};

fn triominoes() -> Vec<Vec<(usize, usize)>> {
    Vec::from([
        (vec![(0, 0), (1, 0), (2, 0)]),
        (vec![(0, 0), (1, 0), (0, 1)]),
        (vec![(0, 0), (1, 0), (1, 1)]),
        (vec![(0, 0), (0, 1), (1, 1)]),
        (vec![(0, 1), (1, 0), (1, 1)]),
        (vec![(0, 0), (0, 1), (0, 2)]),
    ])
}

fn bbox(piece: &[(usize, usize)]) -> (usize, usize) {
    let mut h = 0;
    let mut w = 0;
    for &(y, x) in piece {
        h = h.max(y + 1);
        w = w.max(x + 1);
    }
    (h, w)
}

fn adjacent_edges(piece: &[(usize, usize)]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
    let mut horizontal = vec![];
    let mut vertical = vec![];

    for &(y, x) in piece {
        if piece.iter().any(|&p| p == (y + 1, x)) {
            horizontal.push((y, x));
        }
        if piece.iter().any(|&p| p == (y, x + 1)) {
            vertical.push((y, x));
        }
    }

    (horizontal, vertical)
}

pub fn solve_heteromino(
    clues: &[Vec<Option<i32>>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(clues);
    let polyset = triominoes();

    let mut solver = Solver::new();
    let kind_ranges = clues
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| if x == Some(-1) { (-1, -1) } else { (0, 5) })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let kind = &solver.int_var_2d_from_ranges((h, w), &kind_ranges);

    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);

    solver.add_expr(
        &is_border.horizontal
            ^ (kind.slice((..(h - 1), ..)).ge(0)
                & (kind.slice((..(h - 1), ..)).eq(kind.slice((1.., ..))))),
    );
    solver.add_expr(
        &is_border.vertical
            ^ (kind.slice((.., ..(w - 1))).ge(0)
                & (kind.slice((.., ..(w - 1))).eq(kind.slice((.., 1..))))),
    );

    let sizes = clues
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| if x == Some(-1) { (1, 1) } else { (3, 3) })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let sizes = &solver.int_var_2d_from_ranges((h, w), &sizes);
    graph::graph_division_2d(&mut solver, sizes, &is_border);

    let poly_adjacent_edges = polyset
        .iter()
        .map(|pat| adjacent_edges(pat))
        .collect::<Vec<_>>();

    for y in 0..h {
        for x in 0..w {
            if clues[y][x] == Some(-1) {
                continue;
            }
            let mut conds = vec![];
            for i in 0..6 {
                let (ph, pw) = bbox(&polyset[i]);
                for j in 0..3 {
                    if y < polyset[i][j].0 || x < polyset[i][j].1 {
                        continue;
                    }
                    let ty = y - polyset[i][j].0;
                    let tx = x - polyset[i][j].1;
                    if ty + ph > h || tx + pw > w {
                        continue;
                    }

                    let mut c = vec![kind.at((y, x)).eq(i as i32)];
                    for &(dy, dx) in &poly_adjacent_edges[i].0 {
                        c.push(!is_border.horizontal.at((ty + dy, tx + dx)));
                    }
                    for &(dy, dx) in &poly_adjacent_edges[i].1 {
                        c.push(!is_border.vertical.at((ty + dy, tx + dx)));
                    }
                    conds.push(all(c));
                }
            }

            solver.add_expr(any(conds));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Spaces::new(None, 'a')),
        Box::new(Dict::new(Some(-1), "7")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "heteromino", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["heteromino"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        // Rule example puzzle for puzz.link
        vec![
            vec![None, None, None, None, Some(-1)],
            vec![None, None, Some(-1), None, Some(-1)],
            vec![None, None, Some(-1), None, None],
            vec![Some(-1), Some(-1), None, None, None],
            vec![Some(-1), None, None, None, None],
        ]
    }

    #[test]
    fn test_heteromino_problem() {
        let clues = problem_for_tests();
        let ans = solve_heteromino(&clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();
        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 0, 1],
                [1, 0, 1, 1, 1],
                [1, 1, 1, 0, 0],
                [1, 1, 1, 1, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 1],
                [1, 1, 1, 1],
                [0, 1, 1, 1],
                [1, 1, 0, 1],
                [1, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_heteromino_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?heteromino/5/5/d7b7a7b7b77c7d";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
