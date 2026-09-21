use cspuz_core::custom_constraints::SimpleCustomConstraint;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EdgeState {
    Unknown,
    Line,
    NoLine,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum VertexState {
    Unknown,
    Traversed,
    NotTraversed,
}

pub(super) struct PathPropagator {
    height: usize,
    width: usize,
    clue: Vec<Option<i32>>,

    edges: Vec<(usize, usize)>,
    adjacent: Vec<Vec<(usize, usize)>>,
    edge_state: Vec<EdgeState>,
    vertex_state: Vec<VertexState>,

    // Union-Find structure
    parent: Vec<usize>,
    size: Vec<usize>,
    terminal: Vec<Option<usize>>,

    updates: Vec<Update>,
    conflicts: Vec<Conflict>,

    workspace: std::cell::RefCell<PathWorkspace>,
}

struct Merge {
    child: usize,
    parent: usize,
    old_size: usize,
    old_terminal: Option<usize>,
}

enum Update {
    Edge {
        index: usize,
        merge: Option<Merge>,
        conflict: bool,
    },
    Vertex {
        index: usize,
    },
}

/// The path connecting two vertices `a` and `b`, excluding the edge `cycle_edge` is the reason for the conflict.
struct Conflict {
    a: usize,
    b: usize,
    cycle_edge: Option<usize>,
}

struct PathWorkspace {
    previous: Vec<(usize, usize)>,
    visited: Vec<bool>,
    queue: Vec<usize>,
}

impl PathPropagator {
    pub(super) fn new(height: usize, width: usize, clues: &[Vec<Option<i32>>]) -> PathPropagator {
        let mut edges = vec![];
        let mut adjacent = vec![vec![]; height * width];
        for y in 0..height {
            for x in 0..(width - 1) {
                edges.push((y * width + x, y * width + (x + 1)));
            }
        }
        for y in 0..(height - 1) {
            for x in 0..width {
                edges.push((y * width + x, (y + 1) * width + x));
            }
        }
        for (e, &(a, b)) in edges.iter().enumerate() {
            adjacent[a].push((b, e));
            adjacent[b].push((a, e));
        }
        let edge_state = vec![EdgeState::Unknown; edges.len()];
        let vertex_state = vec![VertexState::Unknown; height * width];
        let mut terminal = vec![None; height * width];
        for y in 0..height {
            for x in 0..width {
                if clues[y][x].is_some() {
                    terminal[y * width + x] = Some(y * width + x);
                }
            }
        }

        let parent = (0..(height * width)).collect();
        let size = vec![1; height * width];

        let workspace = PathWorkspace {
            previous: vec![(0, 0); height * width],
            visited: vec![false; height * width],
            queue: vec![],
        };

        let mut clue = vec![];
        for y in 0..height {
            for x in 0..width {
                clue.push(clues[y][x]);
            }
        }

        PathPropagator {
            height,
            width,
            clue,
            edges,
            adjacent,
            edge_state,
            vertex_state,
            parent,
            size,
            terminal,
            updates: vec![],
            conflicts: vec![],
            workspace: std::cell::RefCell::new(workspace),
        }
    }

    fn root(&self, mut v: usize) -> usize {
        // We don't do path compression here because we want to keep the parent structure intact for backtracking.
        while self.parent[v] != v {
            let p = self.parent[v];
            v = p;
        }
        v
    }
}

impl PathPropagator {
    fn find_path(&self, a: usize, b: usize, exclude: Option<usize>) -> Vec<usize> {
        if a == b {
            return vec![];
        }

        let workspace = &mut self.workspace.borrow_mut();
        for i in 0..workspace.queue.len() {
            let p = workspace.queue[i];
            workspace.visited[p] = false;
        }
        workspace.queue.clear();
        workspace.queue.push(a);
        workspace.visited[a] = true;

        let mut top = 0;
        while top < workspace.queue.len() && !workspace.visited[b] {
            let p = workspace.queue[top];
            top += 1;

            for &(u, e) in &self.adjacent[p] {
                if Some(e) == exclude {
                    continue;
                }
                if self.edge_state[e] == EdgeState::Line && !workspace.visited[u] {
                    workspace.visited[u] = true;
                    workspace.previous[u] = (p, e);
                    workspace.queue.push(u);
                }
            }
        }

        assert!(workspace.visited[b]);

        let mut path = vec![];
        let mut v = b;
        while v != a {
            let (p, e) = workspace.previous[v];
            path.push(e);
            v = p;
        }
        path
    }

    fn shortcut_path(&self, a: usize, b: usize) -> Option<Vec<usize>> {
        let ra = self.root(a);
        let rb = self.root(b);
        if ra == rb {
            return Some(self.find_path(a, b, None));
        }
        let ta = self.terminal[ra]?;
        let tb = self.terminal[rb]?;
        if self.clue[ta] != self.clue[tb] {
            return None;
        }
        let mut reason = self.find_path(a, ta, None);
        reason.extend(self.find_path(tb, b, None));
        Some(reason)
    }

    fn edge_between(&self, a: usize, b: usize) -> usize {
        for &(u, e) in &self.adjacent[a] {
            if u == b {
                return e;
            }
        }
        panic!("No edge between {} and {}", a, b);
    }

    fn find_shortcut(&self, start: usize, end: usize, stride: usize) -> Option<Vec<(usize, bool)>> {
        let mut v = start;
        let mut last = None;

        while v <= end {
            match self.vertex_state[v] {
                VertexState::Unknown => {
                    last = None;
                }
                VertexState::NotTraversed => {}
                VertexState::Traversed => {
                    if let Some(last) = last {
                        let blocker = if v - last == stride {
                            let e = self.edge_between(v, last);
                            if self.edge_state[e] != EdgeState::NoLine {
                                None
                            } else {
                                Some(e)
                            }
                        } else {
                            Some(!0)
                        };

                        if let Some(blocker) = blocker {
                            if let Some(reason) = self.shortcut_path(last, v) {
                                let mut reason =
                                    reason.iter().map(|&e| (e, true)).collect::<Vec<_>>();
                                let mut w = last + stride;
                                while w < v {
                                    reason.push((self.edges.len() + w, false));
                                    w += stride;
                                }
                                if blocker != !0 {
                                    reason.push((blocker, false));
                                }

                                // Needed because this reason is found only when vertex_state is set to Traversed
                                reason.push((self.edges.len() + last, true));
                                reason.push((self.edges.len() + v, true));
                                return Some(reason);
                            }
                        }
                    }
                    last = Some(v);
                }
            }
            v += stride;
        }

        None
    }
}

impl SimpleCustomConstraint for PathPropagator {
    fn initialize_sat(&mut self, num_inputs: usize) {
        assert_eq!(num_inputs, self.edge_state.len() + self.vertex_state.len());
    }

    fn lazy_propagation(&self) -> bool {
        true
    }

    fn notify(&mut self, index: usize, value: bool) {
        if index < self.edges.len() {
            assert_eq!(self.edge_state[index], EdgeState::Unknown);
            self.edge_state[index] = if value {
                EdgeState::Line
            } else {
                EdgeState::NoLine
            };

            let mut merge = None;
            let mut conflict = None;

            if value {
                let (a, b) = self.edges[index];
                let root_a = self.root(a);
                let root_b = self.root(b);

                if root_a == root_b {
                    conflict = Some(Conflict {
                        a,
                        b,
                        cycle_edge: Some(index),
                    });
                } else {
                    if let (Some(ta), Some(tb)) = (self.terminal[root_a], self.terminal[root_b]) {
                        if self.clue[ta] != self.clue[tb] {
                            conflict = Some(Conflict {
                                a: ta,
                                b: tb,
                                cycle_edge: None,
                            });
                        }
                    }

                    let mut root_a = root_a;
                    let mut root_b = root_b;
                    if self.size[root_a] < self.size[root_b] {
                        std::mem::swap(&mut root_a, &mut root_b);
                    }
                    merge = Some(Merge {
                        child: root_b,
                        parent: root_a,
                        old_size: self.size[root_a],
                        old_terminal: self.terminal[root_a],
                    });
                    self.parent[root_b] = root_a;
                    self.size[root_a] += self.size[root_b];
                    if self.terminal[root_a].is_none() {
                        self.terminal[root_a] = self.terminal[root_b];
                    }
                }
            }

            self.updates.push(Update::Edge {
                index,
                merge,
                conflict: conflict.is_some(),
            });
            if let Some(c) = conflict {
                self.conflicts.push(c);
            }
        } else {
            let index = index - self.edges.len();
            assert_eq!(self.vertex_state[index], VertexState::Unknown);
            self.vertex_state[index] = if value {
                VertexState::Traversed
            } else {
                VertexState::NotTraversed
            };
            self.updates.push(Update::Vertex { index });
        }
    }

    fn find_inconsistency(&mut self) -> Option<Vec<(usize, bool)>> {
        if let Some(c) = self.conflicts.first() {
            let path = self.find_path(c.a, c.b, c.cycle_edge);
            let mut reason = vec![];
            for e in path {
                reason.push((e, true));
            }
            if let Some(e) = c.cycle_edge {
                reason.push((e, true));
            }
            return Some(reason);
        }

        for y in 0..self.height {
            if let Some(reason) = self.find_shortcut(y * self.width, (y + 1) * self.width - 1, 1) {
                return Some(reason);
            }
        }
        for x in 0..self.width {
            if let Some(reason) =
                self.find_shortcut(x, x + (self.height - 1) * self.width, self.width)
            {
                return Some(reason);
            }
        }
        None
    }

    fn undo(&mut self) {
        let update = self.updates.pop().unwrap();
        match update {
            Update::Edge {
                index,
                merge,
                conflict,
            } => {
                if conflict {
                    self.conflicts.pop();
                }
                if let Some(m) = merge {
                    self.parent[m.child] = m.child;
                    self.size[m.parent] = m.old_size;
                    self.terminal[m.parent] = m.old_terminal;
                }
                self.edge_state[index] = EdgeState::Unknown;
            }
            Update::Vertex { index } => {
                self.vertex_state[index] = VertexState::Unknown;
            }
        }
    }
}
