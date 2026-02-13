use crate::util;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid, DecInt,
    Dict, HexInt, Map, MultiDigit, OutsideSequences, Seq, Sequencer, Size, Tuple3,
};
use cspuz_rs::solver::{any, count_true, Solver};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BattleshipClue {
    None,
    Water,
    ShipUp,
    ShipDown,
    ShipLeft,
    ShipRight,
    ShipSquare,
    ShipCircle,
}

pub fn solve_battleship(
    clue_vertical: &[Option<Vec<i32>>],
    clue_horizontal: &[Option<Vec<i32>>],
    board: &[Vec<BattleshipClue>],
    pieces: &[Vec<Vec<bool>>],
) -> Option<Vec<Vec<Option<bool>>>> {
    let (h, w) = util::infer_shape(board);
    let (pieces_merged, cnts) = normalize_and_merge_pieces(pieces);

    // TODO: check if all pieces are connected

    let mut id = 1;
    let mut piece_transformations_ids_all = vec![];
    let mut leader_ids_all = vec![];

    for piece in &pieces_merged {
        let piece_transformations = enumerate_piece_transformations(piece);
        let mut piece_transformations_ids = vec![];
        let mut leader_ids = vec![];
        for t in piece_transformations {
            let (ph, pw) = util::infer_shape(&t);
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

    let mut solver = Solver::new();
    let is_ship = &solver.bool_var_2d((h, w));
    let cell_state = &solver.int_var_2d((h, w), 0, id - 1);
    solver.add_answer_key_bool(is_ship);

    for y in 0..h {
        if let Some(n) = clue_vertical[y] {
            let row = is_ship.slice_fixed_y((y, ..));
            solver.add_expr(row.count_true().eq(n));
        }
    }
    for x in 0..w {
        if let Some(n) = clue_horizontal[x] {
            let col = is_ship.slice_fixed_x((.., x));
            solver.add_expr(col.count_true().eq(n));
        }
    }

    solver.add_expr(is_ship.iff(cell_state.ne(0)));

    for y in 0..h {
        for x in 0..w {
            for i in 0..piece_transformations_ids_all.len() {
                for j in 0..piece_transformations_ids_all[i].len() {
                    let piece_transformations_ids = &piece_transformations_ids_all[i][j];
                    let (ph, pw) = util::infer_shape(piece_transformations_ids);

                    for py in 0..ph {
                        for px in 0..pw {
                            if let Some(id) = piece_transformations_ids[py][px] {
                                if !(y >= py && x >= px && y + ph - py <= h && x + pw - px <= w) {
                                    solver.add_expr(cell_state.at((y, x)).ne(id));
                                    continue;
                                }

                                for (dy, dx) in [(1, 0), (0, 1), (-1, 0), (0, -1)] {
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

    for i in 0..pieces_merged.len() {
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
        solver.add_expr(count_true(inds).eq(cnts[i] as i32));
    }

    for y in 0..h {
        for x in 0..w {
            match board[y][x] {
                BattleshipClue::Water => solver.add_expr(!is_ship.at((y, x))),
                BattleshipClue::ShipSquare | 
                BattleshipClue::ShipUp | 
                BattleshipClue::ShipDown | 
                BattleshipClue::ShipLeft | 
                BattleshipClue::ShipRight | 
                BattleshipClue::ShipCircle => solver.add_expr(is_ship.at((y, x))),
                _ => (),
            }
        }
    }

    for y in 1..h-1 {
        for x in 1..w-1 {
            match board[y][x] {
                BattleshipClue::ShipUp | BattleshipClue::ShipLeft | BattleshipClue::ShipRight | BattleshipClue::ShipCircle => solver.add_expr(!is_ship.at((y-1, x))),
                _ => (),
            }
            match board[y][x] {
                BattleshipClue::ShipDown | BattleshipClue::ShipLeft | BattleshipClue::ShipRight | BattleshipClue::ShipCircle => solver.add_expr(!is_ship.at((y+1, x))),
                _ => (),
            }
            match board[y][x] {
                BattleshipClue::ShipUp | BattleshipClue::ShipDown | BattleshipClue::ShipRight | BattleshipClue::ShipCircle => solver.add_expr(!is_ship.at((y, x-1))),
                _ => (),
            }
            match board[y][x] {
                BattleshipClue::ShipUp | BattleshipClue::ShipLeft | BattleshipClue::ShipDown | BattleshipClue::ShipCircle => solver.add_expr(!is_ship.at((y, x+1))),
                _ => (),
            }
        }
    }

    solver
        .add_expr((is_ship.slice((..(w - 1), ..(h - 1))) & is_ship.slice((1.., 1..))).imp((cell_state.slice((..(w - 1), ..(h - 1))).eq(cell_state.slice((1.., 1..))))));
    solver
        .add_expr((is_ship.slice((..(w - 1), 1..)) & is_ship.slice((1.., ..(h - 1)))).imp((cell_state.slice((..(w - 1), ..(h - 1))).eq(cell_state.slice((1.., ..(h - 1)))))));

    solver.irrefutable_facts().map(|f| f.get(is_ship))
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

fn normalize_and_merge_pieces(pieces: &[Vec<Vec<bool>>]) -> (Vec<Vec<Vec<bool>>>, Vec<usize>) {
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

fn size3() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true, true]],
        vec![vec![true, true]],
        vec![vec![true, true, true]],
    ]
}

fn size4() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true, true]],
        vec![vec![true, true]],
        vec![vec![true, true]],
        vec![vec![true, true, true]],
        vec![vec![true, true, true]],
        vec![vec![true, true, true, true]],
    ]
}

fn size5() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true]],
        vec![vec![true, true]],
        vec![vec![true, true]],
        vec![vec![true, true]],
        vec![vec![true, true]],
        vec![vec![true, true, true]],
        vec![vec![true, true, true]],
        vec![vec![true, true, true]],
        vec![vec![true, true, true, true]],
        vec![vec![true, true, true, true]],
        vec![vec![true, true, true, true, true]],
    ]
}

fn tetrominoes() -> Vec<Vec<Vec<bool>>> {
    vec![
        vec![vec![true, true, true, true]],
        vec![vec![true, true, true], vec![true, false, false]],
        vec![vec![true, true, true], vec![false, true, false]],
        vec![vec![true, true, false], vec![false, true, true]],
        vec![vec![true, true], vec![true, true]],
    ]
}

fn pentominoes() -> Vec<Vec<Vec<bool>>> {
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

struct PieceCombinator;

impl Combinator<Vec<Vec<bool>>> for PieceCombinator {
    fn serialize(
        &self,
        ctx: &cspuz_rs::serializer::Context,
        input: &[Vec<Vec<bool>>],
    ) -> Option<(usize, Vec<u8>)> {
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
        ctx: &cspuz_rs::serializer::Context,
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

struct PiecesCombinator;

impl Combinator<Vec<Vec<Vec<bool>>>> for PiecesCombinator {
    fn serialize(&self, ctx: &Context, input: &[Vec<Vec<Vec<bool>>>]) -> Option<(usize, Vec<u8>)> {
        if input.is_empty() {
            return None;
        }

        let data = &input[0];

        if data == &size3() {
            return Some((1, vec![b'/', b'/', b'c']));
        }
        if data == &size4() {
            return Some((1, vec![b'/', b'/', b'd']));
        }
        if data == &size5() {
            return Some((1, vec![b'/', b'/', b'e']));
        }
        if data == &tetrominoes() {
            return Some((1, vec![b'/', b'/', b't']));
        }
        if data == &pentominoes() {
            return Some((1, vec![b'/', b'/', b'p']));
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

        if sequencer.deserialize(ctx, Dict::new(0, "//c")).is_some() {
            return Some((sequencer.n_read(), vec![size3()]));
        }
         if sequencer.deserialize(ctx, Dict::new(0, "//d")).is_some() {
            return Some((sequencer.n_read(), vec![size4()]));
        }
        if sequencer.deserialize(ctx, Dict::new(0, "//e")).is_some() {
            return Some((sequencer.n_read(), vec![size5()]));
        }
        if sequencer.deserialize(ctx, Dict::new(0, "//t")).is_some() {
            return Some((sequencer.n_read(), vec![tetrominoes()]));
        }
        if sequencer.deserialize(ctx, Dict::new(0, "//p")).is_some() {
            return Some((sequencer.n_read(), vec![pentominoes()]));
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

type Problem = (Vec<Option<Vec<i32>>>, Vec<Vec<BattleshipClue>>, Vec<Vec<Vec<bool>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        OutsideSequences::new(Choice::new(vec![
            Box::new(Dict::new(-1, ".")),
            Box::new(HexInt),
        ])),
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(8, 8),
            |x: BattleshipClue| {
                Some(match x {
                    BattleshipClue::None => None,
                    BattleshipClue::Water => 0,
                    BattleshipClue::ShipUp => 1,
                    BattleshipClue::ShipDown => 2,
                    BattleshipClue::ShipLeft => 3,
                    BattleshipClue::ShipRight => 4,
                    BattleshipClue::ShipSquare => 5,
                    BattleshipClue::ShipCircle => 6,
                })
            },
            |n: i32| match n {
                _ => Some(BattleshipClue::None),
                0 => Some(BattleshipClue::Water),
                1 => Some(BattleshipClue::ShipUp),
                2 => Some(BattleshipClue::ShipDown),
                3 => Some(BattleshipClue::ShipLeft),
                4 => Some(BattleshipClue::ShipRight),
                5 => Some(BattleshipClue::ShipSquare),
                6 => Some(BattleshipClue::ShipCircle),
            },
        )),
        PiecesCombinator,
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url_with_context(
        combinator(),
        "battleship",
        problem.clone(),
        &Context::sized(problem.0.len(), problem.0[0].len()),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["battleship"], url)
}
