use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    url_to_problem, Choice, Combinator, Context, ContextBasedGrid, DecInt, Dict, FixedLengthHexInt,
    MaybeSkip, Seq, Sequencer, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::{count_true, Solver};

#[derive(Debug, PartialEq, Eq)]
pub enum GateDir {
    Horizontal,
    Vertical,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Gate {
    cells: Vec<(usize, usize)>,
    dir: GateDir,
    ord: Option<i32>,
}

pub fn solve_slalom(
    origin: (usize, usize),
    is_black: &[Vec<bool>],
    gates: &[Gate],
) -> Option<graph::BoolGridEdgesIrrefutableFacts> {
    let (h, w) = util::infer_shape(is_black);

    let mut solver = Solver::new();
    let line = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    let line_dir = &graph::BoolGridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&line.horizontal);
    solver.add_answer_key_bool(&line.vertical);

    let passed = &graph::single_cycle_grid_edges(&mut solver, line);
    let gate_ord = &solver.int_var_2d((h, w), 0, gates.len() as i32);

    let mut gate_id: Vec<Vec<Option<usize>>> = vec![vec![None; w]; h];
    for (i, gate) in gates.iter().enumerate() {
        let mut cells = vec![];
        for &(y, x) in &gate.cells {
            assert!(gate_id[y][x].is_none());
            gate_id[y][x] = Some(i);
            cells.push(passed.at((y, x)));

            if gate.dir == GateDir::Horizontal {
                if x > 0 {
                    solver.add_expr(!line.horizontal.at((y, x - 1)));
                }
                if x + 1 < w {
                    solver.add_expr(!line.horizontal.at((y, x)));
                }
            } else {
                if y > 0 {
                    solver.add_expr(!line.vertical.at((y - 1, x)));
                }
                if y + 1 < h {
                    solver.add_expr(!line.vertical.at((y, x)));
                }
            }
        }
        solver.add_expr(count_true(cells).eq(1));
    }
    solver.add_expr(passed.at(origin));

    for y in 0..h {
        for x in 0..w {
            let neighbors = passed.four_neighbor_indices((y, x));
            solver.add_expr(
                count_true(
                    neighbors
                        .iter()
                        .map(|&(y2, x2)| {
                            line.at((y + y2, x + x2))
                                & (line_dir.at((y + y2, x + x2)) ^ ((y2, x2) < (y, x)))
                        })
                        .collect::<Vec<_>>(),
                )
                .eq(passed.at((y, x)).ite(1, 0)),
            );
            solver.add_expr(
                count_true(
                    neighbors
                        .iter()
                        .map(|&(y2, x2)| {
                            line.at((y + y2, x + x2))
                                & (line_dir.at((y + y2, x + x2)).iff((y2, x2) < (y, x)))
                        })
                        .collect::<Vec<_>>(),
                )
                .eq(passed.at((y, x)).ite(1, 0)),
            );
            if is_black[y][x] {
                solver.add_expr(!passed.at((y, x)));
            }
            if (y, x) == origin {
                continue;
            }
            if let Some(g) = gate_id[y][x] {
                for (y2, x2) in neighbors {
                    solver.add_expr(
                        (line.at((y + y2, x + x2))
                            & (line_dir.at((y + y2, x + x2)) ^ ((y2, x2) < (y, x))))
                            .imp(gate_ord.at((y2, x2)).eq(gate_ord.at((y, x)) - 1)),
                    );
                }
                if let Some(n) = gates[g].ord {
                    solver.add_expr(passed.at((y, x)).imp(gate_ord.at((y, x)).eq(n)));
                }
            } else {
                for (y2, x2) in neighbors {
                    solver.add_expr(
                        (line.at((y + y2, x + x2))
                            & (line_dir.at((y + y2, x + x2)) ^ ((y2, x2) < (y, x))))
                            .imp(gate_ord.at((y2, x2)).eq(gate_ord.at((y, x)))),
                    );
                }
            }
        }
    }

    // auxiliary constraint
    for y0 in 0..h {
        for x0 in 0..w {
            for y1 in 0..h {
                for x1 in 0..w {
                    if (y0, x0) < (y1, x1) && gate_id[y0][x0].is_some() && gate_id[y1][x1].is_some()
                    {
                        solver.add_expr(
                            (passed.at((y0, x0)) & passed.at((y1, x1)))
                                .imp(gate_ord.at((y0, x0)).ne(gate_ord.at((y1, x1)))),
                        );
                    }
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(line))
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SlalomBlackCellDir {
    NoClue,
    NoDir,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum SlalomCell {
    White,
    Black(SlalomBlackCellDir, i32),
    Vertical,
    Horizontal,
}

impl SlalomCell {
    fn clue_num(&self) -> i32 {
        if let SlalomCell::Black(_, n) = self {
            return *n;
        }
        panic!();
    }
}

struct SlalomAuxCombinator;

impl Combinator<Vec<Vec<SlalomCell>>> for SlalomAuxCombinator {
    fn serialize(&self, _: &Context, _: &[Vec<Vec<SlalomCell>>]) -> Option<(usize, Vec<u8>)> {
        todo!();
    }

    fn deserialize(
        &self,
        ctx: &Context,
        input: &[u8],
    ) -> Option<(usize, Vec<Vec<Vec<SlalomCell>>>)> {
        let mut sequencer = Sequencer::new(input);
        let height = ctx.height.unwrap();
        let width = ctx.width.unwrap();

        let grid_combinator = ContextBasedGrid::new(Choice::new(vec![
            Box::new(Dict::new(
                SlalomCell::Black(SlalomBlackCellDir::NoClue, -1),
                "1",
            )),
            Box::new(Dict::new(SlalomCell::Vertical, "2")),
            Box::new(Dict::new(SlalomCell::Horizontal, "3")),
            Box::new(Spaces::new(SlalomCell::White, '4')),
        ]));
        let mut grid: Vec<Vec<SlalomCell>> =
            sequencer.deserialize_one_elem(ctx, grid_combinator)?;

        let mut n_black = 0usize;
        for y in 0..height {
            for x in 0..width {
                if let SlalomCell::Black(_, _) = grid[y][x] {
                    n_black += 1;
                }
            }
        }

        let seq_combinator = Seq::new(
            Choice::new(vec![
                Box::new(Spaces::new((SlalomBlackCellDir::NoClue, -1), 'g')),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::NoDir, "0"),
                    FixedLengthHexInt::new(1),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Up, "1"),
                    FixedLengthHexInt::new(1),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Down, "2"),
                    FixedLengthHexInt::new(1),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Left, "3"),
                    FixedLengthHexInt::new(1),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Right, "4"),
                    FixedLengthHexInt::new(1),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::NoDir, "5"),
                    FixedLengthHexInt::new(2),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Up, "6"),
                    FixedLengthHexInt::new(2),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Down, "7"),
                    FixedLengthHexInt::new(2),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Left, "8"),
                    FixedLengthHexInt::new(2),
                )),
                Box::new(Tuple2::new(
                    Dict::new(SlalomBlackCellDir::Right, "9"),
                    FixedLengthHexInt::new(2),
                )),
            ]),
            n_black,
        );
        let seq = sequencer.deserialize_one_elem(ctx, seq_combinator)?;
        let mut idx = 0;
        for y in 0..height {
            for x in 0..width {
                if let SlalomCell::Black(_, _) = grid[y][x] {
                    let (d, n) = seq[idx];
                    grid[y][x] = SlalomCell::Black(d, n);
                    idx += 1;
                }
            }
        }

        Some((sequencer.n_read(), vec![grid]))
    }
}

type PrimitiveProblem = (Vec<Vec<SlalomCell>>, (usize, usize));
type Problem = (Vec<Vec<bool>>, Vec<Gate>, (usize, usize));

pub fn deserialize_problem_as_primitive(url: &str) -> Option<PrimitiveProblem> {
    let combinator = MaybeSkip::new(
        "d/",
        Size::new(Tuple2::new(
            SlalomAuxCombinator,
            MaybeSkip::new("/", DecInt),
        )),
    );
    let (cell, origin) = url_to_problem(combinator, &["slalom"], url)?;
    let width = cell[0].len();

    Some((cell, (origin as usize / width, origin as usize % width)))
}

pub fn parse_primitive_problem(problem: &PrimitiveProblem) -> Result<Problem, String> {
    // Criteria for determining gate order:
    // - Clues with arrows represent the order of gates which it points to.
    // - If a gate is between two clues with the same number, the gate's order is the same as that number.
    // - If a clue without an arrow is adjacent to only one gate with unknown order,
    //   the gate's order is the same as that clue's number.
    // - After applying the above criteria, if there is a clue which has not been associated with any gate,
    //   the problem is ambiguous.
    let (cell, origin) = problem;
    let height = cell.len();
    let width = cell[0].len();

    // gate cells, dir, candidate clue cells
    let mut gate_cands: Vec<(Vec<(usize, usize)>, GateDir, Vec<(usize, usize)>)> = vec![];
    let mut unmatched_clues = vec![];

    let mut is_black = vec![vec![false; width]; height];
    for y in 0..height {
        for x in 0..width {
            if let SlalomCell::Black(_, n) = cell[y][x] {
                is_black[y][x] = true;

                if n >= 0 {
                    unmatched_clues.push((y, x));
                }
            }
            if cell[y][x] == SlalomCell::Vertical {
                if y > 0 && cell[y - 1][x] == SlalomCell::Vertical {
                    continue;
                }
                let mut y2 = y;
                while y2 < height && cell[y2][x] == SlalomCell::Vertical {
                    y2 += 1;
                }
                let mut adj_clues = vec![];
                if y > 0 {
                    match cell[y - 1][x] {
                        SlalomCell::Black(SlalomBlackCellDir::Down, n)
                        | SlalomCell::Black(SlalomBlackCellDir::NoDir, n) => {
                            if n >= 0 {
                                adj_clues.push((y - 1, x));
                            }
                        }
                        _ => {}
                    }
                }
                if y2 < height {
                    match cell[y2][x] {
                        SlalomCell::Black(SlalomBlackCellDir::Up, n)
                        | SlalomCell::Black(SlalomBlackCellDir::NoDir, n) => {
                            if n >= 0 {
                                adj_clues.push((y2, x));
                            }
                        }
                        _ => {}
                    }
                }
                gate_cands.push((
                    (y..y2).map(|y| (y, x)).collect(),
                    GateDir::Vertical,
                    adj_clues,
                ));
            } else if cell[y][x] == SlalomCell::Horizontal {
                // horizontal
                if x > 0 && cell[y][x - 1] == SlalomCell::Horizontal {
                    continue;
                }
                let mut x2 = x;
                while x2 < width && cell[y][x2] == SlalomCell::Horizontal {
                    x2 += 1;
                }
                let mut adj_clues = vec![];
                if x > 0 {
                    match cell[y][x - 1] {
                        SlalomCell::Black(SlalomBlackCellDir::Right, _n)
                        | SlalomCell::Black(SlalomBlackCellDir::NoDir, _n) => {
                            adj_clues.push((y, x - 1));
                        }
                        _ => {}
                    }
                }
                if x2 < width {
                    match cell[y][x2] {
                        SlalomCell::Black(SlalomBlackCellDir::Left, _n)
                        | SlalomCell::Black(SlalomBlackCellDir::NoDir, _n) => {
                            adj_clues.push((y, x2));
                        }
                        _ => {}
                    }
                }
                gate_cands.push((
                    (x..x2).map(|x| (y, x)).collect(),
                    GateDir::Horizontal,
                    adj_clues,
                ));
            }
        }
    }

    let mut gates = vec![];

    // If a gate is between two clues with the same number, the gate's order is the same as that number.
    let mut gate_cands = {
        let mut res = vec![];

        for (cells, dir, adj_clues) in gate_cands {
            if adj_clues.len() > 2 {
                return Err("A gate is adjacent to more than 2 clues.".to_string());
            }
            if adj_clues.len() == 2 {
                let (y1, x1) = adj_clues[0];
                let (y2, x2) = adj_clues[1];
                let n1 = cell[y1][x1].clue_num();
                let n2 = cell[y2][x2].clue_num();

                assert!(n1 >= 0 && n2 >= 0);
                if n1 == n2 {
                    gates.push(Gate {
                        cells,
                        dir,
                        ord: Some(n1),
                    });

                    for (ry, rx) in [(y1, x1), (y2, x2)] {
                        if let Some(idx) = unmatched_clues
                            .iter()
                            .position(|&(y, x)| (y, x) == (ry, rx))
                        {
                            unmatched_clues.remove(idx);
                        } else {
                            return Err(format!(
                                "Clue at ({}, {}) has been already consumed by another gate.",
                                ry, rx
                            ));
                        }
                    }
                }
                continue;
            }
            res.push((cells, dir, adj_clues));
        }
        res
    };

    while !unmatched_clues.is_empty() {
        let mut gate_num = vec![None; gate_cands.len()];

        let mut unmatched_clues_next = vec![];
        for (y, x) in unmatched_clues {
            let mut rel_gate_idx = vec![];
            for (i, (_, _, adj_clues)) in gate_cands.iter().enumerate() {
                if adj_clues.contains(&(y, x)) {
                    rel_gate_idx.push(i);
                }
            }

            if rel_gate_idx.is_empty() {
                return Err(format!(
                    "Clue at ({}, {}) cannot be matched to any gate.",
                    y, x
                ));
            }
            if rel_gate_idx.len() == 1 {
                let idx = rel_gate_idx[0];
                let n = cell[y][x].clue_num();

                if gate_num[idx].is_some() && gate_num[idx] != Some(n) {
                    return Err(format!(
                        "Gate order conflict: {} and {}",
                        gate_num[idx].unwrap(),
                        n,
                    ));
                }
                gate_num[idx] = Some(n);
            } else {
                unmatched_clues_next.push((y, x));
            }
        }

        unmatched_clues = unmatched_clues_next;

        let mut gate_cands_next = vec![];
        let mut updated = false;
        for (i, gate_cand) in gate_cands.into_iter().enumerate() {
            if let Some(n) = gate_num[i] {
                let (cells, dir, _) = gate_cand;
                gates.push(Gate {
                    cells,
                    dir,
                    ord: Some(n),
                });
                updated = true;
            } else {
                gate_cands_next.push(gate_cand);
            }
        }

        if !updated {
            return Err("The problem is ambigious.".to_string());
        }

        gate_cands = gate_cands_next;
    }

    for gate_cand in gate_cands {
        let (cells, dir, _) = gate_cand;
        gates.push(Gate {
            cells,
            dir,
            ord: None,
        });
    }

    Ok((is_black, gates, *origin))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let origin = (5, 1);
        let is_black = crate::util::tests::to_bool_2d([
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 1, 0, 0, 0, 1, 0],
            [0, 0, 1, 0, 1, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 1, 0, 1],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 1, 0, 0],
            [1, 0, 1, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
            [0, 0, 1, 0, 0, 0, 0, 0, 0, 0],
        ]);

        let gates = vec![
            Gate {
                cells: vec![(1, 5), (1, 6), (1, 7)],
                dir: GateDir::Horizontal,
                ord: None,
            },
            Gate {
                cells: vec![(2, 3)],
                dir: GateDir::Horizontal,
                ord: None,
            },
            Gate {
                cells: vec![(3, 8)],
                dir: GateDir::Horizontal,
                ord: Some(1),
            },
            Gate {
                cells: vec![(6, 3), (6, 4), (6, 5), (6, 6)],
                dir: GateDir::Horizontal,
                ord: Some(3),
            },
            Gate {
                cells: vec![(7, 1)],
                dir: GateDir::Horizontal,
                ord: None,
            },
            Gate {
                cells: vec![(8, 6), (8, 7), (8, 8), (8, 9)],
                dir: GateDir::Horizontal,
                ord: Some(2),
            },
        ];

        (is_black, gates, origin)
    }

    #[test]
    fn test_slalom_problem() {
        // https://puzsq.jp/main/puzzle_play.php?pid=9522
        let (is_black, gates, origin) = problem_for_tests();
        let ans = solve_slalom(origin, &is_black, &gates);
        assert!(ans.is_some());

        // TODO: add expected answer
    }

    #[test]
    fn test_slalom_serializer() {
        let problem = problem_for_tests();
        let deserialized = deserialize_problem_as_primitive(
            "https://puzz.link/p?slalom/d/10/10/h133316131f131p1333315131f1333351aj41314333h42g/51",
        );
        assert!(deserialized.is_some());
        let deserialized = parse_primitive_problem(&deserialized.unwrap());
        assert_eq!(Ok(problem), deserialized);
    }
}
