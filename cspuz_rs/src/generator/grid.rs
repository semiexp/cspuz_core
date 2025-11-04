use rand::Rng;

use super::Pattern;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Symmetry {
    None,
    HorizontalLine,
    VerticalLine,
    Rotate180,
    Rotate90,
}

fn symmetry_group_of_cell(
    symmetry: Symmetry,
    height: usize,
    width: usize,
    y: usize,
    x: usize,
) -> Vec<(usize, usize)> {
    match symmetry {
        Symmetry::None => vec![(y, x)],
        Symmetry::HorizontalLine => {
            if y == height - 1 - y {
                vec![(y, x)]
            } else {
                vec![(y, x), (height - 1 - y, x)]
            }
        }
        Symmetry::VerticalLine => {
            if x == width - 1 - x {
                vec![(y, x)]
            } else {
                vec![(y, x), (y, width - 1 - x)]
            }
        }
        Symmetry::Rotate180 => {
            if y == height - 1 - y && x == width - 1 - x {
                vec![(y, x)]
            } else {
                vec![(y, x), (height - 1 - y, width - 1 - x)]
            }
        }
        Symmetry::Rotate90 => {
            assert_eq!(height, width);
            if y == height - 1 - y && x == width - 1 - x {
                vec![(y, x)]
            } else {
                vec![
                    (y, x),
                    (x, height - 1 - y),
                    (height - 1 - y, width - 1 - x),
                    (width - 1 - x, y),
                ]
            }
        }
    }
}

fn groups_from_symmetry(
    symmetry: Symmetry,
    height: usize,
    width: usize,
) -> Vec<Vec<(usize, usize)>> {
    let mut visited = vec![vec![false; width]; height];
    let mut groups = vec![];
    for y in 0..height {
        for x in 0..width {
            if !visited[y][x] {
                let group = symmetry_group_of_cell(symmetry, height, width, y, x);
                for &(gy, gx) in &group {
                    visited[gy][gx] = true;
                }
                groups.push(group);
            }
        }
    }
    groups
}

pub struct Grid<T: Clone + PartialEq> {
    height: usize,
    width: usize,
    default: T,
    non_default: Vec<T>,
    groups: Vec<Vec<(usize, usize)>>,
}

impl<T: Clone + PartialEq> Grid<T> {
    pub fn new(
        height: usize,
        width: usize,
        candidates: &[T],
        default: T,
        symmetry: Symmetry,
    ) -> Grid<T> {
        assert!(candidates.contains(&default));
        let non_default = candidates
            .iter()
            .filter(|&c| c != &default)
            .cloned()
            .collect();

        Grid {
            height,
            width,
            default,
            non_default,
            groups: groups_from_symmetry(symmetry, height, width),
        }
    }
}

impl<T: Clone + PartialEq> Pattern for Grid<T> {
    type Output = Vec<Vec<T>>;
    type Update = (usize, Option<Vec<usize>>); // (group_index, candidate_indices)

    fn initial(&self) -> Self::Output {
        vec![vec![self.default.clone(); self.width]; self.height]
    }

    fn enumerate_update_candidates(
        &self,
        current: &Self::Output,
        rng: &mut rand::prelude::StdRng,
    ) -> Vec<Self::Update> {
        let mut ret = vec![];

        for (group_index, group) in self.groups.iter().enumerate() {
            let (y0, x0) = group[0];
            if &current[y0][x0] != &self.default {
                ret.push((group_index, None));
            }

            if group.len() == 1 {
                for (cand_index, cand) in self.non_default.iter().enumerate() {
                    if cand != &current[y0][x0] {
                        ret.push((group_index, Some(vec![cand_index])));
                    }
                }
            } else {
                for _ in 0..self.non_default.len() {
                    let mut cand_indices = vec![];
                    for _ in 0..group.len() {
                        let cand_index = rng.gen_range(0..self.non_default.len());
                        cand_indices.push(cand_index);
                    }
                    ret.push((group_index, Some(cand_indices)));
                }
            }
        }

        ret
    }

    fn apply_update(&self, current: &Self::Output, update: &Self::Update) -> Self::Output {
        let mut new_grid = current.clone();
        let (group_index, candidate_indices_option) = update;
        let group = &self.groups[*group_index];

        match candidate_indices_option {
            None => {
                for &(y, x) in group {
                    new_grid[y][x] = self.default.clone();
                }
            }
            Some(candidate_indices) => {
                for (i, &(y, x)) in group.iter().enumerate() {
                    let cand_index = candidate_indices[i];
                    new_grid[y][x] = self.non_default[cand_index].clone();
                }
            }
        }

        new_grid
    }
}
