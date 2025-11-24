pub type Piece = Vec<(usize, usize)>;

pub fn named_pentominoes() -> Vec<(char, Piece)> {
    vec![
        ('F', vec![(0, 0), (1, 0), (1, 1), (1, 2), (2, 1)]),
        ('I', vec![(0, 0), (0, 1), (0, 2), (0, 3), (0, 4)]),
        ('L', vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 0)]),
        ('N', vec![(0, 1), (0, 2), (0, 3), (1, 0), (1, 1)]),
        ('P', vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 1)]),
        ('T', vec![(0, 0), (0, 1), (0, 2), (1, 1), (2, 1)]),
        ('U', vec![(0, 0), (0, 1), (0, 2), (1, 0), (1, 2)]),
        ('V', vec![(0, 0), (0, 1), (0, 2), (1, 0), (2, 0)]),
        ('W', vec![(0, 0), (1, 0), (1, 1), (2, 1), (2, 2)]),
        ('X', vec![(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)]),
        ('Y', vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 1)]),
        ('Z', vec![(0, 0), (0, 1), (1, 1), (2, 1), (2, 2)]),
    ]
}

pub fn named_tetrominoes() -> Vec<(char, Piece)> {
    vec![
        ('I', vec![(0, 0), (0, 1), (0, 2), (0, 3)]),
        ('L', vec![(0, 0), (1, 0), (2, 0), (0, 1)]),
        ('O', vec![(0, 0), (0, 1), (1, 0), (1, 1)]),
        ('S', vec![(0, 0), (0, 1), (1, 1), (1, 2)]),
        ('T', vec![(0, 0), (0, 1), (0, 2), (1, 1)]),
    ]
}

pub fn bbox(piece: &[(usize, usize)]) -> (usize, usize) {
    let mut h = 0;
    let mut w = 0;
    for &(y, x) in piece {
        h = h.max(y + 1);
        w = w.max(x + 1);
    }
    (h, w)
}

pub fn rotate(piece: &[(usize, usize)]) -> Piece {
    let (h, _w) = bbox(piece);
    piece.iter().map(|&(y, x)| (x, h - y - 1)).collect()
}

pub fn flip(piece: &[(usize, usize)]) -> Piece {
    let (h, _w) = bbox(piece);
    piece.iter().map(|&(y, x)| (h - y - 1, x)).collect()
}

pub fn enumerate_variants(piece: &[(usize, usize)]) -> Vec<Piece> {
    let mut cands = vec![];
    cands.push(piece.to_owned());
    for i in 0..3 {
        cands.push(rotate(&cands[i]));
    }
    for i in 0..4 {
        cands.push(flip(&cands[i]));
    }
    for cand in &mut cands {
        cand.sort();
    }
    cands.sort();
    cands.dedup();

    cands
}

/// Returns the coordinates of cell edges adjacent to 2 cells in the piece.
///
/// The first element in the returned tuple is for horizontal edges between (y, x) and (y+1, x).
/// The second element is for vertical edges between (y, x) and (y, x+1).
pub fn adjacent_edges(piece: &[(usize, usize)]) -> (Vec<(usize, usize)>, Vec<(usize, usize)>) {
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

pub fn adjacent_outside_cells(piece: &[(usize, usize)]) -> Vec<(isize, isize)> {
    let mut ret = vec![];

    for &(y, x) in piece {
        for &(dy, dx) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let ny = y as isize + dy;
            let nx = x as isize + dx;
            if !piece.iter().any(|&p| p == (ny as usize, nx as usize)) {
                ret.push((ny, nx));
            }
        }
    }

    ret.sort();
    ret.dedup();

    ret
}
