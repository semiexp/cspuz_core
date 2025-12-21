use std::ops::Index;

use crate::util::Grid;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_and_site, url_to_problem, Choice, Combinator, Context, Dict,
    HexInt, Optionalize, RoomsWithValues, Size, Spaces,
};
use cspuz_rs::solver::Solver;

use cspuz_core::custom_constraints::SimpleCustomConstraint;

pub fn solve_sendai(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Option<i32>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let (h, w) = borders.base_shape();

    let mut solver = Solver::new();
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);

    for y in 0..h {
        for x in 0..w {
            if y < h - 1 && borders.horizontal[y][x] {
                solver.add_expr(is_border.horizontal.at((y, x)));
            }
            if x < w - 1 && borders.vertical[y][x] {
                solver.add_expr(is_border.vertical.at((y, x)));
            }
        }
    }

    let edges_flat = is_border
        .vertical
        .clone()
        .into_iter()
        .chain(is_border.horizontal.clone())
        .collect::<Vec<_>>();

    #[cfg(not(test))]
    {
        let constraint = SendaiShapeConstraint {
            board: BoardManager::new(h, w),
        };
        solver.add_custom_constraint(Box::new(constraint), edges_flat);
    }

    #[cfg(test)]
    {
        let constraint = SendaiShapeConstraint {
            board: BoardManager::new(h, w),
        };
        let cloned_constraint = SendaiShapeConstraint {
            board: BoardManager::new(h, w),
        };

        solver.add_custom_constraint(
            Box::new(crate::util::tests::ReasonVerifier::new(
                constraint,
                cloned_constraint,
            )),
            edges_flat,
        );
    }

    let rooms = graph::borders_to_rooms(borders);
    for (room_id, room) in rooms.iter().enumerate() {
        if let Some(clue) = clues[room_id] {
            let num_vertices = room.len();
            let mut edges = vec![];
            let mut edge_values = vec![];
            let mut vertex_id_map = vec![None; h * w];
            for (i, &(y, x)) in room.iter().enumerate() {
                vertex_id_map[y * w + x] = Some(i);
            }
            for &(y, x) in room {
                if y + 1 < h
                    && vertex_id_map[(y + 1) * w + x].is_some()
                    && !borders.horizontal[y][x]
                {
                    edges.push((
                        vertex_id_map[y * w + x].unwrap(),
                        vertex_id_map[(y + 1) * w + x].unwrap(),
                    ));
                    edge_values.push(is_border.horizontal.at((y, x)).clone());
                }
                if x + 1 < w && vertex_id_map[y * w + (x + 1)].is_some() && !borders.vertical[y][x]
                {
                    edges.push((
                        vertex_id_map[y * w + x].unwrap(),
                        vertex_id_map[y * w + (x + 1)].unwrap(),
                    ));
                    edge_values.push(is_border.vertical.at((y, x)).clone());
                }
            }

            #[cfg(not(test))]
            {
                let constraint = CityCountConstraint::new(num_vertices, edges, clue as usize);
                solver.add_custom_constraint(Box::new(constraint), edge_values);
            }
            #[cfg(test)]
            {
                let constraint =
                    CityCountConstraint::new(num_vertices, edges.clone(), clue as usize);
                let cloned_constraint =
                    CityCountConstraint::new(num_vertices, edges, clue as usize);

                solver.add_custom_constraint(
                    Box::new(crate::util::tests::ReasonVerifier::new(
                        constraint,
                        cloned_constraint,
                    )),
                    edge_values,
                );
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Border {
    Undecided,
    Wall,
    Connected,
}

const NO_GROUP: usize = !0;

struct GroupInfo {
    group_id: Grid<usize>,
    groups_flat: Vec<(usize, usize)>,
    groups_offset: Vec<usize>,
}

impl GroupInfo {
    fn new(group_id: Grid<usize>) -> GroupInfo {
        let height = group_id.height();
        let width = group_id.width();

        let mut group_size = vec![];
        for y in 0..height {
            for x in 0..width {
                if group_id[(y, x)] == NO_GROUP {
                    continue;
                }

                while group_size.len() <= group_id[(y, x)] {
                    group_size.push(0);
                }

                group_size[group_id[(y, x)]] += 1;
            }
        }

        let mut groups_offset = vec![0];
        for i in 0..group_size.len() {
            groups_offset.push(groups_offset[i] + group_size[i]);
        }

        let mut groups_flat = vec![(0, 0); groups_offset[group_size.len()]];
        let mut cur_pos = groups_offset.clone();

        for y in 0..height {
            for x in 0..width {
                let id = group_id[(y, x)];
                if id == NO_GROUP {
                    continue;
                }

                groups_flat[cur_pos[id]] = (y, x);
                cur_pos[id] += 1;
            }
        }

        GroupInfo {
            group_id,
            groups_flat,
            groups_offset,
        }
    }

    fn num_groups(&self) -> usize {
        self.groups_offset.len() - 1
    }
}

impl Index<usize> for GroupInfo {
    type Output = [(usize, usize)];

    fn index(&self, index: usize) -> &Self::Output {
        let start = self.groups_offset[index];
        let end = self.groups_offset[index + 1];
        &self.groups_flat[start..end]
    }
}

// Connectivity information of a puzzle board.
struct BoardInfo {
    units: GroupInfo,
    potential_units: GroupInfo,
}

struct BoardManager {
    height: usize,
    width: usize,
    decision_stack: Vec<usize>,

    // borders between horizontally adjacent cells
    horizontal_borders: Grid<Border>,

    // borders between vertically adjacent cells
    vertical_borders: Grid<Border>,
}

impl BoardManager {
    pub fn new(height: usize, width: usize) -> BoardManager {
        BoardManager {
            height,
            width,
            decision_stack: vec![],
            horizontal_borders: Grid::new(height, width - 1, Border::Undecided),
            vertical_borders: Grid::new(height - 1, width, Border::Undecided),
        }
    }

    fn horizontal_idx(&self, y: usize, x: usize) -> usize {
        y * (self.width - 1) + x
    }

    fn vertical_idx(&self, y: usize, x: usize) -> usize {
        self.height * (self.width - 1) + y * self.width + x
    }

    fn idx_to_border(&self, idx: usize) -> (bool, usize, usize) {
        if idx >= self.height * (self.width - 1) {
            let idx = idx - self.height * (self.width - 1);
            let y = idx / self.width;
            let x = idx % self.width;
            (true, y, x)
        } else {
            let y = idx / (self.width - 1);
            let x = idx % (self.width - 1);
            (false, y, x)
        }
    }

    pub fn decide(&mut self, idx: usize, value: bool) {
        let (is_vertical, y, x) = self.idx_to_border(idx);

        if is_vertical {
            assert_eq!(self.vertical_borders[(y, x)], Border::Undecided);
            self.vertical_borders[(y, x)] = if value {
                Border::Wall
            } else {
                Border::Connected
            };
        } else {
            assert_eq!(self.horizontal_borders[(y, x)], Border::Undecided);
            self.horizontal_borders[(y, x)] = if value {
                Border::Wall
            } else {
                Border::Connected
            };
        }

        self.decision_stack.push(idx);
    }

    pub fn undo(&mut self) {
        assert!(!self.decision_stack.is_empty());
        let top = self.decision_stack.pop().unwrap();
        let (is_vertical, y, x) = self.idx_to_border(top);

        if is_vertical {
            assert_ne!(self.vertical_borders[(y, x)], Border::Undecided);
            self.vertical_borders[(y, x)] = Border::Undecided;
        } else {
            assert_ne!(self.horizontal_borders[(y, x)], Border::Undecided);
            self.horizontal_borders[(y, x)] = Border::Undecided;
        }
    }

    pub fn reason_for_unit_and_its_boundary(
        &self,
        info: &BoardInfo,
        unit_id: usize,
    ) -> Vec<(usize, bool)> {
        let mut ret = vec![];

        for &(y, x) in &info.units[unit_id] {
            if y < self.height - 1
                && info.units.group_id[(y + 1, x)] == unit_id
                && self.vertical_borders[(y, x)] == Border::Connected
            {
                ret.push((self.vertical_idx(y, x), false));
            }
            if x < self.width - 1
                && info.units.group_id[(y, x + 1)] == unit_id
                && self.horizontal_borders[(y, x)] == Border::Connected
            {
                ret.push((self.horizontal_idx(y, x), false));
            }
            if y > 0 && self.vertical_borders[(y - 1, x)] == Border::Wall {
                ret.push((self.vertical_idx(y - 1, x), true));
            }
            if x > 0 && self.horizontal_borders[(y, x - 1)] == Border::Wall {
                ret.push((self.horizontal_idx(y, x - 1), true));
            }
            if y < self.height - 1 && self.vertical_borders[(y, x)] == Border::Wall {
                ret.push((self.vertical_idx(y, x), true));
            }
            if x < self.width - 1 && self.horizontal_borders[(y, x)] == Border::Wall {
                ret.push((self.horizontal_idx(y, x), true));
            }
        }

        ret
    }

    pub fn reason_for_potential_unit_boundary(
        &self,
        info: &BoardInfo,
        unit_id: usize,
    ) -> Vec<(usize, bool)> {
        let mut ret = vec![];

        for &(y, x) in &info.potential_units[unit_id] {
            if y > 0
                && info.potential_units.group_id[(y - 1, x)] != unit_id
                && self.vertical_borders[(y - 1, x)] == Border::Wall
            {
                ret.push((self.vertical_idx(y - 1, x), true));
            }
            if y < self.height - 1
                && info.potential_units.group_id[(y + 1, x)] != unit_id
                && self.vertical_borders[(y, x)] == Border::Wall
            {
                ret.push((self.vertical_idx(y, x), true));
            }
            if x > 0
                && info.potential_units.group_id[(y, x - 1)] != unit_id
                && self.horizontal_borders[(y, x - 1)] == Border::Wall
            {
                ret.push((self.horizontal_idx(y, x - 1), true));
            }
            if x < self.width - 1
                && info.potential_units.group_id[(y, x + 1)] != unit_id
                && self.horizontal_borders[(y, x)] == Border::Wall
            {
                ret.push((self.horizontal_idx(y, x), true));
            }
        }

        ret
    }

    pub fn reason_for_path(
        &self,
        y1: usize,
        x1: usize,
        y2: usize,
        x2: usize,
    ) -> Vec<(usize, bool)> {
        let mut bfs: Grid<Option<(usize, usize)>> = Grid::new(self.height, self.width, None);
        bfs[(y1, x1)] = Some((y1, x1));

        let mut qu = std::collections::VecDeque::<(usize, usize)>::new();
        qu.push_back((y1, x1));
        while let Some((y, x)) = qu.pop_front() {
            if y == y2 && x == x2 {
                break;
            }

            if y > 0
                && self.vertical_borders[(y - 1, x)] == Border::Connected
                && bfs[(y - 1, x)].is_none()
            {
                bfs[(y - 1, x)] = Some((y, x));
                qu.push_back((y - 1, x));
            }
            if y < self.height - 1
                && self.vertical_borders[(y, x)] == Border::Connected
                && bfs[(y + 1, x)].is_none()
            {
                bfs[(y + 1, x)] = Some((y, x));
                qu.push_back((y + 1, x));
            }
            if x > 0
                && self.horizontal_borders[(y, x - 1)] == Border::Connected
                && bfs[(y, x - 1)].is_none()
            {
                bfs[(y, x - 1)] = Some((y, x));
                qu.push_back((y, x - 1));
            }
            if x < self.width - 1
                && self.horizontal_borders[(y, x)] == Border::Connected
                && bfs[(y, x + 1)].is_none()
            {
                bfs[(y, x + 1)] = Some((y, x));
                qu.push_back((y, x + 1));
            }
        }

        assert!(bfs[(y2, x2)].is_some());

        let mut ret = vec![];
        let mut y = y2;
        let mut x = x2;

        while !(y == y1 && x == x1) {
            let (yf, xf) = bfs[(y, x)].unwrap();

            if y == yf {
                ret.push((self.horizontal_idx(y, x.min(xf)), false));
            } else {
                ret.push((self.vertical_idx(y.min(yf), x), false));
            }

            y = yf;
            x = xf;
        }

        ret
    }

    pub fn compute_board_info(&self) -> BoardInfo {
        BoardInfo {
            units: self.compute_connected_components(false),
            potential_units: self.compute_connected_components(true),
        }
    }

    pub fn compute_connected_components(&self, is_potential: bool) -> GroupInfo {
        let mut group_id = Grid::new(self.height, self.width, NO_GROUP);
        let mut stack = vec![];
        let mut last_id = 0;

        for y in 0..self.height {
            for x in 0..self.width {
                if group_id[(y, x)] != NO_GROUP {
                    continue;
                }

                assert!(stack.is_empty());

                group_id[(y, x)] = last_id;
                stack.push((y, x));

                while let Some((y, x)) = stack.pop() {
                    let mut traverse = |y2: usize, x2: usize, border: Border| {
                        if border == Border::Connected
                            || (is_potential && border == Border::Undecided)
                        {
                            if group_id[(y2, x2)] == NO_GROUP {
                                group_id[(y2, x2)] = last_id;
                                stack.push((y2, x2));
                            }
                        }
                    };

                    if y > 0 {
                        traverse(y - 1, x, self.vertical_borders[(y - 1, x)]);
                    }
                    if y < self.height - 1 {
                        traverse(y + 1, x, self.vertical_borders[(y, x)]);
                    }
                    if x > 0 {
                        traverse(y, x - 1, self.horizontal_borders[(y, x - 1)]);
                    }
                    if x < self.width - 1 {
                        traverse(y, x + 1, self.horizontal_borders[(y, x)]);
                    }
                }

                last_id += 1;
            }
        }

        GroupInfo::new(group_id)
    }
}

struct SendaiShapeConstraint {
    board: BoardManager,
}

impl SimpleCustomConstraint for SendaiShapeConstraint {
    fn lazy_propagation(&self) -> bool {
        true
    }

    fn initialize_sat(&mut self, num_inputs: usize) {
        assert_eq!(
            num_inputs,
            self.board.height * (self.board.width - 1) + (self.board.height - 1) * self.board.width
        );
    }

    fn notify(&mut self, index: usize, value: bool) {
        self.board.decide(index, value);
    }

    fn undo(&mut self) {
        self.board.undo();
    }

    fn find_inconsistency(&mut self) -> Option<Vec<(usize, bool)>> {
        let height = self.board.height;
        let width = self.board.width;
        let info = self.board.compute_board_info();

        // no extra wall
        for y in 0..height {
            for x in 0..width {
                if y < height - 1
                    && self.board.vertical_borders[(y, x)] == Border::Wall
                    && info.units.group_id[(y, x)] == info.units.group_id[(y + 1, x)]
                {
                    let mut ret = self.board.reason_for_path(y, x, y + 1, x);
                    ret.push((self.board.vertical_idx(y, x), true));
                    return Some(ret);
                }
                if x < width - 1
                    && self.board.horizontal_borders[(y, x)] == Border::Wall
                    && info.units.group_id[(y, x)] == info.units.group_id[(y, x + 1)]
                {
                    let mut ret = self.board.reason_for_path(y, x, y, x + 1);
                    ret.push((self.board.horizontal_idx(y, x), true));
                    return Some(ret);
                }
            }
        }

        // For each unit, we must find the same shape in adjacent potential units (or the same potential unit)
        let mut adjacent_potential_units_flat = vec![];
        for y in 0..height {
            for x in 0..width {
                if y < height - 1 && self.board.vertical_borders[(y, x)] == Border::Wall {
                    let i = info.potential_units.group_id[(y, x)];
                    let j = info.potential_units.group_id[(y + 1, x)];
                    adjacent_potential_units_flat.push((i, j));
                    adjacent_potential_units_flat.push((j, i));
                }
                if x < width - 1 && self.board.horizontal_borders[(y, x)] == Border::Wall {
                    let i = info.potential_units.group_id[(y, x)];
                    let j = info.potential_units.group_id[(y, x + 1)];
                    adjacent_potential_units_flat.push((i, j));
                    adjacent_potential_units_flat.push((j, i));
                }
            }
        }
        for i in 0..info.potential_units.num_groups() {
            adjacent_potential_units_flat.push((i, i));
        }

        adjacent_potential_units_flat.sort();
        adjacent_potential_units_flat.dedup();

        for i in 0..info.units.num_groups() {
            let mut cells = vec![];
            let mut connections = vec![];
            let mut disconnections = vec![];

            for &(y, x) in &info.units[i] {
                cells.push((y as i32, x as i32));

                if y < height - 1 && info.units.group_id[(y + 1, x)] == i {
                    connections.push((y as i32 * 2 + 1, x as i32 * 2));
                }
                if x < width - 1 && info.units.group_id[(y, x + 1)] == i {
                    connections.push((y as i32 * 2, x as i32 * 2 + 1));
                }

                if y == 0 || self.board.vertical_borders[(y - 1, x)] == Border::Wall {
                    disconnections.push((y as i32 * 2 - 1, x as i32 * 2));
                }
                if y == height - 1 || self.board.vertical_borders[(y, x)] == Border::Wall {
                    disconnections.push((y as i32 * 2 + 1, x as i32 * 2));
                }
                if x == 0 || self.board.horizontal_borders[(y, x - 1)] == Border::Wall {
                    disconnections.push((y as i32 * 2, x as i32 * 2 - 1));
                }
                if x == width - 1 || self.board.horizontal_borders[(y, x)] == Border::Wall {
                    disconnections.push((y as i32 * 2, x as i32 * 2 + 1));
                }
            }

            let mut reason = vec![];
            let mut isok = false;

            'outer: for y in 0..(height as i32) {
                'inner: for x in 0..(width as i32) {
                    let mut has_adj_potential_unit = false;
                    for &(dy, dx) in &cells {
                        let y2 = y - cells[0].0 + dy;
                        let x2 = x - cells[0].1 + dx;
                        if y2 < 0 || y2 >= height as i32 || x2 < 0 || x2 >= width as i32 {
                            continue 'inner;
                        }
                        if info.units.group_id[(y2 as usize, x2 as usize)] == i {
                            continue 'inner;
                        }

                        if adjacent_potential_units_flat
                            .binary_search(&(
                                info.potential_units.group_id
                                    [(cells[0].0 as usize, cells[0].1 as usize)],
                                info.potential_units.group_id[(y2 as usize, x2 as usize)],
                            ))
                            .is_ok()
                        {
                            has_adj_potential_unit = true;
                        }
                    }
                    if !has_adj_potential_unit {
                        continue;
                    }

                    for &(dy, dx) in &connections {
                        let y2 = y * 2 - cells[0].0 * 2 + dy;
                        let x2 = x * 2 - cells[0].1 * 2 + dx;

                        if y2 < 0
                            || y2 >= (height as i32) * 2 - 1
                            || x2 < 0
                            || x2 >= (width as i32) * 2 - 1
                        {
                            continue 'inner;
                        }

                        if y2 % 2 == 1 {
                            if self.board.vertical_borders[((y2 / 2) as usize, (x2 / 2) as usize)]
                                == Border::Wall
                            {
                                reason.push((
                                    self.board
                                        .vertical_idx((y2 / 2) as usize, (x2 / 2) as usize),
                                    true,
                                ));
                                continue 'inner;
                            }
                        } else {
                            if self.board.horizontal_borders[((y2 / 2) as usize, (x2 / 2) as usize)]
                                == Border::Wall
                            {
                                reason.push((
                                    self.board
                                        .horizontal_idx((y2 / 2) as usize, (x2 / 2) as usize),
                                    true,
                                ));
                                continue 'inner;
                            }
                        }
                    }
                    for &(dy, dx) in &disconnections {
                        let y2 = y * 2 - cells[0].0 * 2 + dy;
                        let x2 = x * 2 - cells[0].1 * 2 + dx;

                        if y2 < 0
                            || y2 >= (height as i32) * 2 - 1
                            || x2 < 0
                            || x2 >= (width as i32) * 2 - 1
                        {
                            continue;
                        }

                        if y2 % 2 == 1 {
                            if self.board.vertical_borders[((y2 / 2) as usize, (x2 / 2) as usize)]
                                == Border::Connected
                            {
                                reason.push((
                                    self.board
                                        .vertical_idx((y2 / 2) as usize, (x2 / 2) as usize),
                                    false,
                                ));
                                continue 'inner;
                            }
                        } else {
                            if self.board.horizontal_borders[((y2 / 2) as usize, (x2 / 2) as usize)]
                                == Border::Connected
                            {
                                reason.push((
                                    self.board
                                        .horizontal_idx((y2 / 2) as usize, (x2 / 2) as usize),
                                    false,
                                ));
                                continue 'inner;
                            }
                        }
                    }

                    isok = true;
                    break 'outer;
                }
            }

            if !isok {
                reason.extend(self.board.reason_for_unit_and_its_boundary(&info, i));

                let p_unit =
                    info.potential_units.group_id[(cells[0].0 as usize, cells[0].1 as usize)];
                for &(i2, j2) in &adjacent_potential_units_flat {
                    if i2 == p_unit {
                        reason.extend(self.board.reason_for_potential_unit_boundary(&info, j2));
                    }
                }

                // visualize reason
                let mut vis = vec![vec![' '; width * 2 - 1]; height * 2 - 1];
                for &(idx, val) in &reason {
                    let (is_vertical, y, x) = self.board.idx_to_border(idx);
                    if is_vertical {
                        vis[y * 2 + 1][x * 2] = if val { '-' } else { 'x' };
                    } else {
                        vis[y * 2][x * 2 + 1] = if val { '|' } else { 'x' };
                    }
                }
                for y in 0..(height - 1) {
                    for x in 0..(width - 1) {
                        vis[y * 2 + 1][x * 2 + 1] = '+';
                    }
                }

                return Some(reason);
            }
        }

        None
    }
}

struct CityCountConstraint {
    num_vertices: usize,
    edges: Vec<(usize, usize)>,
    city_count: usize,
    decision_stack: Vec<usize>,
    adj: Vec<Vec<(usize, usize)>>, // adjacency list: (neighbor_vertex, edge_index)
    borders: Vec<Border>,
}

impl CityCountConstraint {
    pub fn new(
        num_vertices: usize,
        edges: Vec<(usize, usize)>,
        city_count: usize,
    ) -> CityCountConstraint {
        let mut adj = vec![vec![]; num_vertices];
        for (i, &(u, v)) in edges.iter().enumerate() {
            adj[u].push((v, i));
            adj[v].push((u, i));
        }
        let num_edges = edges.len();

        CityCountConstraint {
            num_vertices,
            edges,
            city_count,
            decision_stack: vec![],
            adj,
            borders: vec![Border::Undecided; num_edges],
        }
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    pub fn new(n: usize) -> UnionFind {
        UnionFind {
            parent: (0..n).collect(),
        }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    pub fn unite(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        if x != y {
            self.parent[x] = y;
            true
        } else {
            false
        }
    }
}

impl SimpleCustomConstraint for CityCountConstraint {
    fn lazy_propagation(&self) -> bool {
        true
    }

    fn initialize_sat(&mut self, num_inputs: usize) {
        assert_eq!(num_inputs, self.edges.len());
    }

    fn notify(&mut self, index: usize, value: bool) {
        assert_eq!(self.borders[index], Border::Undecided);
        self.borders[index] = if value {
            Border::Wall
        } else {
            Border::Connected
        };
        self.decision_stack.push(index);
    }

    fn undo(&mut self) {
        assert!(!self.decision_stack.is_empty());
        let top = self.decision_stack.pop().unwrap();
        self.borders[top] = Border::Undecided;
    }

    fn find_inconsistency(&mut self) -> Option<Vec<(usize, bool)>> {
        let mut union_find = UnionFind::new(self.num_vertices);

        let mut num_max_components = self.num_vertices;
        for (i, &(u, v)) in self.edges.iter().enumerate() {
            if self.borders[i] == Border::Connected {
                if union_find.unite(u, v) {
                    num_max_components -= 1;
                    if num_max_components < self.city_count {
                        let mut reason = vec![];
                        for j in 0..self.edges.len() {
                            if self.borders[j] == Border::Connected {
                                reason.push((j, false));
                            }
                        }
                        return Some(reason);
                    }
                }
            }
        }

        let mut num_min_components = 0;
        let mut visited = vec![false; self.num_vertices];

        fn dfs(u: usize, visited: &mut [bool], adj: &[Vec<(usize, usize)>], borders: &[Border]) {
            visited[u] = true;
            for &(v, edge_idx) in &adj[u] {
                if borders[edge_idx] != Border::Wall && !visited[v] {
                    dfs(v, visited, adj, borders);
                }
            }
        }

        for i in 0..self.num_vertices {
            if visited[i] {
                continue;
            }
            num_min_components += 1;
            dfs(i, &mut visited, &self.adj, &self.borders);
        }

        if num_min_components > self.city_count {
            let mut reason = vec![];
            for i in 0..self.edges.len() {
                if self.borders[i] == Border::Wall {
                    reason.push((i, true));
                }
            }
            Some(reason)
        } else {
            None
        }
    }
}

type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Option<i32>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(RoomsWithValues::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ])))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context_and_site(
        combinator(),
        "sendai",
        "https://pzprxs.vercel.app/p?",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["sendai"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        // https://puzsq.logicpuzzle.app/puzzle/166720
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [1, 0, 0, 1, 1],
                [0, 1, 0, 0, 0],
                [0, 0, 0, 1, 1],
                [1, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [0, 1, 1, 0],
                [1, 1, 0, 0],
                [1, 1, 0, 0],
                [1, 1, 1, 0],
                [1, 1, 1, 0],
            ]),
        };
        let clues = vec![Some(1), Some(4), None, None, None, Some(2), None];
        (borders, clues)
    }

    #[test]
    fn test_sendai_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_sendai(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::BoolInnerGridEdgesIrrefutableFacts {
            horizontal: crate::util::tests::to_option_bool_2d([
                [1, 0, 1, 1, 1],
                [1, 1, 1, 0, 0],
                [1, 0, 0, 1, 1],
                [1, 0, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [0, 1, 1, 1],
                [1, 1, 0, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
                [1, 1, 1, 1],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_sendai_serializer() {
        let problem = problem_for_tests();
        let url = "https://pzprxs.vercel.app/p?sendai/5/5/dj7ej83g14i2g";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
