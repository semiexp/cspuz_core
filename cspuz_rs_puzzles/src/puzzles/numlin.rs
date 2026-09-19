use crate::util;
use cspuz_core::custom_constraints::SimpleCustomConstraint;
use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url, url_to_problem, Choice, Combinator, Dict, Grid, HexInt, Optionalize, Spaces,
};
use cspuz_rs::solver::Solver;

pub fn enumerate_answers_numlin(
    clues: &[Vec<Option<i32>>],
    num_max_answers: usize,
) -> Vec<graph::BoolGridEdgesModel> {
    let (h, w) = util::infer_shape(clues);

    let mut solver = Solver::new();

    let is_line = graph::GridEdges::new(&mut solver, (h - 1, w - 1));
    solver.add_answer_key_bool(&is_line.horizontal);
    solver.add_answer_key_bool(&is_line.vertical);

    for y in 0..h {
        for x in 0..w {
            if clues[y][x].is_some() {
                solver.add_expr(is_line.vertex_neighbors((y, x)).count_true().eq(1));
            } else {
                solver.add_expr(
                    is_line.vertex_neighbors((y, x)).count_true().eq(0)
                        | is_line.vertex_neighbors((y, x)).count_true().eq(2),
                );
            }
        }
    }

    // forbid trivial detour
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            solver.add_expr(is_line.cell_neighbors((y, x)).count_true().le(2));
        }
    }

    // L-shape canonization
    for y in 0..(h - 1) {
        for x in 0..(w - 1) {
            if clues[y + 1][x + 1].is_none() {
                if y == h - 2 || x == w - 2 {
                    solver.add_expr(!(is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x))));
                } else {
                    solver.add_expr(
                        (is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x))).imp(
                            is_line.horizontal.at((y + 1, x + 1))
                                & is_line.vertical.at((y + 1, x + 1)),
                        ),
                    );
                }
            }

            if clues[y + 1][x].is_none() {
                if y == h - 2 || x == 0 {
                    solver.add_expr(
                        !(is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x + 1))),
                    );
                } else {
                    solver.add_expr(
                        (is_line.horizontal.at((y, x)) & is_line.vertical.at((y, x + 1))).imp(
                            is_line.horizontal.at((y + 1, x - 1)) & is_line.vertical.at((y + 1, x)),
                        ),
                    );
                }
            }
        }
    }

    let mut max_clue = 0;
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                max_clue = max_clue.max(n);
            }
        }
    }

    let cell_clue_id = &solver.int_var_2d((h, w), 0, max_clue);
    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(cell_clue_id.at((y, x)).eq(n));
            }
        }
    }

    solver.add_expr(
        is_line.horizontal.imp(
            cell_clue_id
                .slice((.., 1..))
                .eq(cell_clue_id.slice((.., ..(w - 1)))),
        ),
    );
    solver.add_expr(
        is_line.vertical.imp(
            cell_clue_id
                .slice((1.., ..))
                .eq(cell_clue_id.slice((..(h - 1), ..))),
        ),
    );

    let mut values = vec![];
    for y in 0..h {
        for x in 0..(w - 1) {
            values.push(is_line.horizontal.at((y, x)).expr());
        }
    }
    for y in 0..(h - 1) {
        for x in 0..w {
            values.push(is_line.vertical.at((y, x)).expr());
        }
    }
    for y in 0..h {
        for x in 0..w {
            values.push(is_line.vertex_neighbors((y, x)).any());
        }
    }
    solver.add_custom_constraint(Box::new(PathPropagator::new(h, w, clues)), values);

    solver
        .answer_iter()
        .take(num_max_answers)
        .map(|f| f.get_unwrap(&is_line))
        .collect()
}

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

struct PathPropagator {
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
    fn new(height: usize, width: usize, clues: &[Vec<Option<i32>>]) -> PathPropagator {
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

type Problem = Vec<Vec<Option<i32>>>;

fn combinator() -> impl Combinator<Problem> {
    Grid::new(Choice::new(vec![
        Box::new(Optionalize::new(HexInt)),
        Box::new(Spaces::new(None, 'g')),
        Box::new(Dict::new(Some(-1), ".")),
    ]))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    problem_to_url(combinator(), "numlin", problem.clone())
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["numlin"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        vec![
            vec![None, None, None, None, Some(4), None],
            vec![None, Some(1), Some(4), None, None, None],
            vec![None, None, None, None, Some(2), None],
            vec![None, None, None, None, None, None],
            vec![None, Some(2), None, None, Some(1), Some(3)],
            vec![Some(3), None, None, None, None, None],
        ]
    }

    #[test]
    fn test_numlin_problem() {
        let problem = problem_for_tests();
        let ans = enumerate_answers_numlin(&problem, 3);
        assert_eq!(ans.len(), 1);
        let ans = &ans[0];
        let expected = graph::BoolGridEdgesModel {
            horizontal: util::tests::to_bool_2d([
                [1, 1, 1, 0, 1],
                [0, 0, 0, 1, 0],
                [0, 0, 1, 0, 0],
                [0, 1, 0, 1, 1],
                [1, 0, 1, 1, 0],
                [1, 1, 1, 1, 1],
            ]),
            vertical: util::tests::to_bool_2d([
                [1, 0, 0, 1, 0, 1],
                [1, 1, 1, 0, 1, 1],
                [1, 1, 0, 1, 0, 1],
                [1, 0, 1, 0, 0, 0],
                [0, 0, 0, 0, 0, 1],
            ]),
        };
        assert_eq!(ans, &expected);
    }

    #[test]
    fn test_numlin_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?numlin/6/6/j4h14m2n2h133k";
        util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
