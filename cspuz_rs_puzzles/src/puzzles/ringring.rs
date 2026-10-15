use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::strip_prefix;
use cspuz_rs::solver::Solver;

pub fn solve_ringring(is_black: &[Vec<bool>]) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(is_black);

    let mut solver = Solver::new();
    let is_line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    graph::add_rectangular_loops(&mut solver, is_line);
    for y in 0..h {
        for x in 0..w {
            solver.add_expr(is_line.vertex_neighbors((y, x)).any().iff(!is_black[y][x]));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(is_line))
}

type Problem = Vec<Vec<bool>>;

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let serialized = strip_prefix(url)?;
    let pos = serialized.find('/')?;
    let kind = &serialized[0..pos];
    if kind != "ringring" {
        return None;
    }
    let body = &serialized[(pos + 1)..];
    let toks = body.split("/").collect::<Vec<_>>();
    if toks.len() < 3 {
        return None;
    }
    let width = toks[0].parse::<usize>().ok()?;
    let height = toks[1].parse::<usize>().ok()?;
    let mut ret = vec![vec![false; width]; height];
    let body = toks[2].as_bytes();
    let mut pos = 0;
    for &b in body {
        if b == b'.' {
            pos += 36;
        } else if (b'0'..=b'9').contains(&b) {
            pos += (b - b'0') as usize;
            if pos >= height * width {
                return None;
            }
            ret[pos / width][pos % width] = true;
            pos += 1;
        } else if (b'a'..=b'z').contains(&b) {
            pos += (b - b'a') as usize + 10;
            if pos >= height * width {
                return None;
            }
            ret[pos / width][pos % width] = true;
            pos += 1;
        }
    }
    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        crate::util::tests::to_bool_2d([
            [1, 0, 0, 0, 0, 0, 0, 1],
            [0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 1, 0],
            [0, 0, 0, 1, 0, 0, 0, 0],
        ])
    }

    #[test]
    fn test_ringring_problem() {
        let is_black = problem_for_tests();
        let ans = solve_ringring(&is_black);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 1, 1, 1, 0],
                [1, 1, 0, 0, 1, 1, 1],
                [1, 1, 0, 1, 1, 0, 0],
                [0, 1, 1, 1, 1, 1, 0],
                [1, 1, 0, 1, 1, 0, 0],
                [1, 1, 0, 0, 1, 1, 1],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 0, 0, 0, 0, 1, 0],
                [1, 1, 1, 0, 1, 0, 1, 1],
                [0, 1, 0, 1, 1, 1, 1, 1],
                [0, 0, 0, 1, 1, 1, 0, 1],
                [1, 0, 1, 0, 1, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_ringring_deserializer() {
        let url = "https://puzz.link/p?ringring/8/6/063cd4";
        assert_eq!(deserialize_problem(url), Some(problem_for_tests()));
    }
}
