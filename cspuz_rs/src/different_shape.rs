use cspuz_core::custom_constraints::SimpleCustomConstraint;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum CellState {
    White,
    Black,
    Undecided,
}

fn flip_block(block: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut ymax = 0;
    for &(y, _) in block.iter() {
        ymax = ymax.max(y);
    }
    let mut ret = block
        .iter()
        .map(|&(y, x)| (ymax - y, x))
        .collect::<Vec<_>>();
    ret.sort();
    ret
}

fn rotate_block(block: &[(i32, i32)]) -> Vec<(i32, i32)> {
    let mut ymax = 0;
    for &(y, _) in block.iter() {
        ymax = ymax.max(y);
    }

    let mut ret = block
        .iter()
        .map(|&(y, x)| (x, ymax - y))
        .collect::<Vec<_>>();
    ret.sort();
    ret
}

fn normalize_block(mut block: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    block.sort();

    let mut ret = block.clone();
    for i in 0..4 {
        ret = ret.min(block.clone());
        ret = ret.min(flip_block(&block));
        if i < 3 {
            block = rotate_block(&block);
        }
    }

    ret
}

pub struct DifferentShape {
    height: usize,
    width: usize,
    boards: [Vec<Vec<CellState>>; 2],
    decision_stack: Vec<(usize, usize, usize)>,
}

impl DifferentShape {
    pub fn new(height: usize, width: usize) -> DifferentShape {
        DifferentShape {
            height,
            width,
            boards: [
                vec![vec![CellState::Undecided; width]; height],
                vec![vec![CellState::Undecided; width]; height],
            ],
            decision_stack: vec![],
        }
    }
}

impl SimpleCustomConstraint for DifferentShape {
    fn lazy_propagation(&self) -> bool {
        true
    }

    fn initialize_sat(&mut self, num_inputs: usize) {
        assert_eq!(num_inputs, self.height * self.width * 2);
    }

    fn notify(&mut self, index: usize, value: bool) {
        let (y, x, board) = (
            (index / self.width) % self.height,
            index % self.width,
            index / (self.height * self.width),
        );
        self.boards[board][y][x] = if value {
            CellState::Black
        } else {
            CellState::White
        };
        self.decision_stack.push((y, x, board));
    }

    fn find_inconsistency(&mut self) -> Option<Vec<(usize, bool)>> {
        let height = self.height;
        let width = self.width;

        for b in 0..2 {
            let mut has_black = false;

            for y in 0..height {
                for x in 0..width {
                    if self.boards[b][y][x] == CellState::Black {
                        has_black = true;

                        if y > 0 && self.boards[b][y - 1][x] == CellState::Undecided {
                            return None;
                        }
                        if y < height - 1 && self.boards[b][y + 1][x] == CellState::Undecided {
                            return None;
                        }
                        if x > 0 && self.boards[b][y][x - 1] == CellState::Undecided {
                            return None;
                        }
                        if x < width - 1 && self.boards[b][y][x + 1] == CellState::Undecided {
                            return None;
                        }
                    }
                }
            }

            if !has_black {
                return None;
            }
        }

        let mut shapes = [vec![], vec![]];
        for b in 0..2 {
            let mut ymin = height;
            let mut ymax = 0;
            let mut xmin = width;
            let mut xmax = 0;

            for y in 0..height {
                for x in 0..width {
                    if self.boards[b][y][x] == CellState::Black {
                        ymin = ymin.min(y);
                        ymax = ymax.max(y);
                        xmin = xmin.min(x);
                        xmax = xmax.max(x);
                    }
                }
            }

            assert!(ymin <= ymax);
            assert!(xmin <= xmax);

            let mut shape = vec![];
            for y in ymin..=ymax {
                for x in xmin..=xmax {
                    if self.boards[b][y][x] == CellState::Black {
                        shape.push(((y - ymin) as i32, (x - xmin) as i32));
                    }
                }
            }

            shapes[b] = normalize_block(shape);
        }

        if shapes[0] != shapes[1] {
            return None;
        }

        // inconsistent (same shape)
        let mut reason = vec![];
        for b in 0..2 {
            for y in 0..height {
                for x in 0..width {
                    match self.boards[b][y][x] {
                        CellState::Black => {
                            reason.push((y * width + x + b * height * width, true));
                        }
                        CellState::White => {
                            // add to the reason if it is adjacent to a black cell
                            let mut is_adjacent = false;

                            if y > 0 && self.boards[b][y - 1][x] == CellState::Black {
                                is_adjacent = true;
                            }
                            if y < height - 1 && self.boards[b][y + 1][x] == CellState::Black {
                                is_adjacent = true;
                            }
                            if x > 0 && self.boards[b][y][x - 1] == CellState::Black {
                                is_adjacent = true;
                            }
                            if x < width - 1 && self.boards[b][y][x + 1] == CellState::Black {
                                is_adjacent = true;
                            }

                            if is_adjacent {
                                reason.push((y * width + x + b * height * width, false));
                            }
                        }
                        CellState::Undecided => {}
                    }
                }
            }
        }

        Some(reason)
    }

    fn undo(&mut self) {
        let (y, x, board) = self.decision_stack.pop().unwrap();
        self.boards[board][y][x] = CellState::Undecided;
    }
}
