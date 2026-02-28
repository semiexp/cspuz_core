use crate::serializer::{Combinator, Context, DecInt, Dict, MultiDigit, Seq, Sequencer};
use crate::solver::{any, count_true, traits::Operand, Solver};
use cspuz_core::csp::BoolExpr as CSPBoolExpr;

pub type Polyomino = Vec<Vec<bool>>;

pub fn polyomino_placement<T>(
    solver: &mut Solver,
    board: T,
    pieces: &[Polyomino],
    piece_count_min: &[usize],
    piece_count_max: &[usize],
    disallow_corner_touch: bool,
) where
    T: Operand<Shape = (usize, usize), Value = CSPBoolExpr>,
{
    let board = board.as_ndarray();

    // TODO: check if all pieces are connected

    let mut id = 1;
    let mut piece_transformations_ids_all = vec![];
    let mut leader_ids_all = vec![];

    for piece in pieces {
        let piece_transformations = enumerate_piece_transformations(piece);
        let mut piece_transformations_ids = vec![];
        let mut leader_ids = vec![];
        for t in piece_transformations {
            let ph = t.len();
            let pw = t[0].len();

            let mut ids = vec![];
            let mut ld = None;
            for y in 0..ph {
                let mut row = vec![];
                for x in 0..pw {
                    if t[y][x] {
                        if ld.is_none() {
                            ld = Some(id);
                        }
                        row.push(Some(id));
                        id += 1;
                    } else {
                        row.push(None);
                    }
                }
                ids.push(row);
            }
            piece_transformations_ids.push(ids);

            assert!(ld.is_some());
            leader_ids.push(ld.unwrap());
        }
        piece_transformations_ids_all.push(piece_transformations_ids);
        leader_ids_all.push(leader_ids);
    }

    let (h, w) = board.shape();
    let cell_state = &solver.int_var_2d((h, w), 0, id - 1);
    solver.add_expr(board.iff(cell_state.ne(0)));

    let neighbors = if disallow_corner_touch {
        vec![
            (1, 0),
            (0, 1),
            (-1, 0),
            (0, -1),
            (1, 1),
            (1, -1),
            (-1, 1),
            (-1, -1),
        ]
    } else {
        vec![(1, 0), (0, 1), (-1, 0), (0, -1)]
    };

    for y in 0..h {
        for x in 0..w {
            for i in 0..piece_transformations_ids_all.len() {
                for j in 0..piece_transformations_ids_all[i].len() {
                    let piece_transformations_ids = &piece_transformations_ids_all[i][j];
                    let ph = piece_transformations_ids.len();
                    let pw = piece_transformations_ids[0].len();

                    for py in 0..ph {
                        for px in 0..pw {
                            if let Some(id) = piece_transformations_ids[py][px] {
                                if !(y >= py && x >= px && y + ph - py <= h && x + pw - px <= w) {
                                    solver.add_expr(cell_state.at((y, x)).ne(id));
                                    continue;
                                }

                                for (dy, dx) in &neighbors {
                                    let pyi = py as i32;
                                    let pxi = px as i32;

                                    let py2 = pyi + dy;
                                    let px2 = pxi + dx;
                                    let y2 = y as i32 + dy;
                                    let x2 = x as i32 + dx;

                                    let id2 = if 0 <= py2
                                        && py2 < ph as i32
                                        && 0 <= px2
                                        && px2 < pw as i32
                                    {
                                        piece_transformations_ids[py2 as usize][px2 as usize]
                                    } else {
                                        None
                                    };

                                    if let Some(id2) = id2 {
                                        solver.add_expr(cell_state.at((y, x)).eq(id).imp(
                                            cell_state.at((y2 as usize, x2 as usize)).eq(id2),
                                        ));
                                    } else {
                                        if 0 <= y2 && y2 < h as i32 && 0 <= x2 && x2 < w as i32 {
                                            solver.add_expr(cell_state.at((y, x)).eq(id).imp(
                                                cell_state.at((y2 as usize, x2 as usize)).eq(0),
                                            ));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    for i in 0..pieces.len() {
        let mut inds = vec![];
        for y in 0..h {
            for x in 0..w {
                let mut ind = vec![];
                for &j in leader_ids_all[i].iter() {
                    ind.push(cell_state.at((y, x)).eq(j));
                }
                inds.push(any(ind));
            }
        }
        solver.add_expr(count_true(&inds).ge(piece_count_min[i] as i32));
        solver.add_expr(count_true(&inds).le(piece_count_max[i] as i32));
    }
}

fn rotate_piece_90(piece: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let h = piece.len();
    let w = piece[0].len();
    let mut ret = vec![vec![false; h]; w];
    for i in 0..h {
        for j in 0..w {
            ret[j][h - i - 1] = piece[i][j];
        }
    }
    ret
}

fn flip_piece(piece: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let h = piece.len();
    let w = piece[0].len();
    let mut ret = vec![vec![false; w]; h];
    for i in 0..h {
        for j in 0..w {
            ret[i][w - j - 1] = piece[i][j];
        }
    }
    ret
}

fn enumerate_piece_transformations(piece: &[Vec<bool>]) -> Vec<Vec<Vec<bool>>> {
    let mut piece = piece.to_vec();
    let mut ret = vec![];
    for _ in 0..4 {
        ret.push(piece.clone());
        let cur = flip_piece(&piece);
        ret.push(cur);

        piece = rotate_piece_90(&piece);
    }
    ret
}

fn normalize_piece(piece: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let mut transformations = enumerate_piece_transformations(piece);
    transformations.sort();
    transformations.into_iter().next().unwrap()
}

pub fn normalize_and_merge_pieces(pieces: &[Polyomino]) -> (Vec<Polyomino>, Vec<usize>) {
    let mut pieces = pieces
        .iter()
        .map(|p| normalize_piece(p))
        .collect::<Vec<_>>();
    pieces.sort();
    let mut ret = vec![];
    let mut cnt = vec![];
    for p in pieces {
        if !ret.is_empty() && ret[ret.len() - 1] == p {
            *cnt.last_mut().unwrap() += 1;
        } else {
            ret.push(p);
            cnt.push(1);
        }
    }
    (ret, cnt)
}

pub fn tetrominoes() -> Vec<Polyomino> {
    vec![
        vec![vec![true, true, true, true]],
        vec![vec![true, true, true], vec![true, false, false]],
        vec![vec![true, true, true], vec![false, true, false]],
        vec![vec![true, true, false], vec![false, true, true]],
        vec![vec![true, true], vec![true, true]],
    ]
}

pub fn pentominoes() -> Vec<Polyomino> {
    vec![
        vec![
            vec![false, false, true],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![vec![true], vec![true], vec![true], vec![true], vec![true]],
        vec![
            vec![false, true],
            vec![false, true],
            vec![false, true],
            vec![true, true],
        ],
        vec![
            vec![false, true],
            vec![false, true],
            vec![true, true],
            vec![true, false],
        ],
        vec![vec![false, true], vec![true, true], vec![true, true]],
        vec![
            vec![false, false, true],
            vec![true, true, true],
            vec![false, false, true],
        ],
        vec![vec![true, true], vec![false, true], vec![true, true]],
        vec![
            vec![false, false, true],
            vec![false, false, true],
            vec![true, true, true],
        ],
        vec![
            vec![false, false, true],
            vec![false, true, true],
            vec![true, true, false],
        ],
        vec![
            vec![false, true, false],
            vec![true, true, true],
            vec![false, true, false],
        ],
        vec![
            vec![false, true],
            vec![false, true],
            vec![true, true],
            vec![false, true],
        ],
        vec![
            vec![false, false, true],
            vec![true, true, true],
            vec![true, false, false],
        ],
    ]
}

pub struct PieceCombinator;

impl Combinator<Vec<Vec<bool>>> for PieceCombinator {
    fn serialize(&self, ctx: &Context, input: &[Vec<Vec<bool>>]) -> Option<(usize, Vec<u8>)> {
        if input.is_empty() {
            return None;
        }

        let data = &input[0];
        let height = data.len();
        let width = data[0].len();

        if !((1..=35).contains(&height) && (1..=35).contains(&width)) {
            return None;
        }

        let mut ret = vec![];
        let (_, app) = MultiDigit::new(36, 1).serialize(ctx, &[width as i32])?;
        ret.extend(app);
        let (_, app) = MultiDigit::new(36, 1).serialize(ctx, &[height as i32])?;
        ret.extend(app);
        let mut seq = vec![];
        for y in 0..height {
            for x in 0..width {
                seq.push(if data[y][x] { 1 } else { 0 });
            }
        }
        while seq.last() == Some(&0) {
            seq.pop();
        }
        let (_, app) = Seq::new(MultiDigit::new(2, 5), seq.len())
            .serialize(&Context::sized(height, width), &[seq])?;
        ret.extend(app);

        Some((1, ret))
    }

    fn deserialize(
        &self,
        ctx: &crate::serializer::Context,
        input: &[u8],
    ) -> Option<(usize, Vec<Vec<Vec<bool>>>)> {
        let mut sequencer = Sequencer::new(input);

        let width = sequencer.deserialize(ctx, MultiDigit::new(36, 1))?;
        assert_eq!(width.len(), 1);
        let width = width[0] as usize;

        let height = sequencer.deserialize(ctx, MultiDigit::new(36, 1))?;
        assert_eq!(height.len(), 1);
        let height = height[0] as usize;

        let mut ret = vec![vec![false; width]; height];
        let mut pos = 0;
        while pos < height * width {
            if let Some(subseq) = sequencer.deserialize(ctx, MultiDigit::new(2, 5)) {
                for i in 0..subseq.len() {
                    if pos >= height * width {
                        break;
                    }
                    ret[pos / width][pos % width] = subseq[i] == 1;
                    pos += 1;
                }
            } else {
                break;
            }
        }

        Some((sequencer.n_read(), vec![ret]))
    }
}

pub struct PiecesCombinator<'a> {
    known_piece_sets: Vec<(Vec<Polyomino>, &'a [u8])>,
}

impl<'a> PiecesCombinator<'a> {
    pub fn new(known_piece_sets: Vec<(Vec<Polyomino>, &'a [u8])>) -> PiecesCombinator<'a> {
        PiecesCombinator { known_piece_sets }
    }
}

impl<'a> Combinator<Vec<Vec<Vec<bool>>>> for PiecesCombinator<'a> {
    fn serialize(&self, ctx: &Context, input: &[Vec<Vec<Vec<bool>>>]) -> Option<(usize, Vec<u8>)> {
        if input.is_empty() {
            return None;
        }

        let data = &input[0];

        for (known_pieces, code) in &self.known_piece_sets {
            if data == known_pieces {
                return Some((1, code.to_vec()));
            }
        }

        let mut ret = vec![];
        ret.push(b'/');

        let (_, app) = DecInt.serialize(ctx, &[data.len() as i32])?;
        ret.extend(app);

        for i in 0..data.len() {
            ret.push(b'/');

            let (_, app) = PieceCombinator.serialize(ctx, &data[i..=i])?;
            ret.extend(app);
        }

        Some((1, ret))
    }

    fn deserialize(
        &self,
        ctx: &Context,
        input: &[u8],
    ) -> Option<(usize, Vec<Vec<Vec<Vec<bool>>>>)> {
        let mut sequencer = Sequencer::new(input);

        for (known_pieces, code) in &self.known_piece_sets {
            if sequencer
                .deserialize(ctx, Dict::new(0, code.to_vec()))
                .is_some()
            {
                return Some((sequencer.n_read(), vec![known_pieces.clone()]));
            }
        }

        sequencer.deserialize(ctx, Dict::new(0, "/"))?;

        let n_pieces = sequencer.deserialize(ctx, DecInt)?;
        assert_eq!(n_pieces.len(), 1);
        let n_pieces = n_pieces[0] as usize;

        let mut ret = vec![];
        for _ in 0..n_pieces {
            sequencer.deserialize(ctx, Dict::new(0, "/"))?;
            let piece: Vec<Vec<Vec<bool>>> = sequencer.deserialize(ctx, PieceCombinator)?;
            assert_eq!(piece.len(), 1);
            ret.push(piece.into_iter().next().unwrap());
        }

        Some((sequencer.n_read(), vec![ret]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polyomino_placement_simple() {
        for disallow_corner_touch in [false, true] {
            let mut solver = Solver::new();
            let board = &solver.bool_var_2d((4, 3));
            let pieces = vec![
                vec![vec![true, true, true], vec![true, false, false]],
                vec![vec![true, true, true], vec![true, true, false]],
            ];
            solver.add_expr(!board.at((1, 0)));
            solver.add_expr(!board.at((1, 1)));

            polyomino_placement(
                &mut solver,
                board,
                &pieces,
                &[1, 1],
                &[1, 1],
                disallow_corner_touch,
            );

            let ans = solver.solve();

            if disallow_corner_touch {
                assert!(ans.is_none());
            } else {
                assert!(ans.is_some());

                let ans = ans.unwrap();
                let expected = vec![
                    vec![true, true, true],
                    vec![false, false, true],
                    vec![true, true, false],
                    vec![true, true, true],
                ];
                assert_eq!(ans.get(board), expected);
            }
        }
    }

    #[test]
    fn test_polyomino_placement_multiple() {
        let mut solver = Solver::new();
        let board = &solver.bool_var_2d((4, 3));
        let pieces = vec![vec![vec![true, true, true], vec![true, false, false]]];
        solver.add_expr(!board.at((1, 0)));

        polyomino_placement(&mut solver, board, &pieces, &[2], &[2], true);

        let ans = solver.solve();

        assert!(ans.is_some());

        let ans = ans.unwrap();
        let expected = vec![
            vec![true, true, true],
            vec![false, false, true],
            vec![true, false, false],
            vec![true, true, true],
        ];
        assert_eq!(ans.get(board), expected);
    }

    #[test]
    fn test_polyomino_placement_self_diagonal_adjacency() {
        let mut solver = Solver::new();
        let board = &solver.bool_var_2d((3, 3));
        let pieces = vec![vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, true, false],
        ]];
        solver.add_expr(!board.at((0, 0)));

        polyomino_placement(&mut solver, board, &pieces, &[1], &[1], true);

        let ans = solver.solve();

        assert!(ans.is_some());

        let ans = ans.unwrap();
        let expected = vec![
            vec![false, true, true],
            vec![true, false, true],
            vec![true, true, true],
        ];
        assert_eq!(ans.get(board), expected);
    }
}
