use std::collections::VecDeque;

use crate::sat::{CustomPropagator, Lit, SolverManipulator};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum EdgeState {
    Undecided,
    Disconnected,
    Connected,
}

enum LiteralInfo {
    Edge(usize, EdgeState),
    LowerBound(usize, i32),
    UpperBound(usize, i32),
}

enum UndoInfo {
    LevelBoundary,
    Edge(usize),
    LowerBound(usize, i32), // (index, old_value)
    UpperBound(usize, i32), // (index, old_value)
}

#[derive(Clone)]
enum Reason {
    NotPropagated,

    /// Both ends of the edge `edge_idx` are in the same group and the edge should be connected.
    EdgeInSameGroup {
        edge_idx: usize,
    },
    EdgeBetweenDifferentGroups {
        disconnected_edge_idx: usize,
    },
}

pub struct GraphDivision {
    num_vertices: usize,
    num_edges: usize,

    domains: Vec<Vec<i32>>,
    edges: Vec<(usize, usize)>,
    edge_lits: Vec<Lit>,
    literals: Vec<(Lit, LiteralInfo)>,
    adj: Vec<Vec<(usize, usize)>>, // (vertex, edge_idx)

    edge_state: Vec<EdgeState>,
    lower_bound: Vec<i32>,
    upper_bound: Vec<i32>,

    unique_lits: Vec<Lit>,

    propagations: Vec<Lit>,
    propagation_reasons: Vec<Reason>, // the reason why unique_lits[i] is propagated
    inconsistency_reason: Vec<Lit>,

    undo_stack: Vec<UndoInfo>,
}

impl GraphDivision {
    pub fn new(
        domains: &[Vec<i32>],
        dom_lits: &[Vec<Lit>],
        vertex_weights: &[i32],
        edges: &[(usize, usize)],
        edge_lits: &[Lit],
    ) -> GraphDivision {
        assert_eq!(domains.len(), dom_lits.len());
        assert_eq!(domains.len(), vertex_weights.len());

        for i in 0..vertex_weights.len() {
            assert!(
                vertex_weights[i] >= 0,
                "TODO: vertex_weights[{}] must be non-negative",
                i
            );
        }

        assert_eq!(edges.len(), edge_lits.len());

        let num_vertices = domains.len();
        let num_edges = edges.len();

        let mut literals = vec![];
        for i in 0..num_vertices {
            if domains[i].is_empty() {
                // no constraint is given for this vertex
                continue;
            }

            assert_eq!(domains[i].len(), dom_lits[i].len() + 1);

            for j in 0..dom_lits[i].len() {
                literals.push((
                    dom_lits[i][j],
                    LiteralInfo::LowerBound(i, domains[i][j + 1]),
                ));
                literals.push((!dom_lits[i][j], LiteralInfo::UpperBound(i, domains[i][j])));
            }
        }
        for i in 0..num_edges {
            literals.push((edge_lits[i], LiteralInfo::Edge(i, EdgeState::Disconnected)));
            literals.push((!edge_lits[i], LiteralInfo::Edge(i, EdgeState::Connected)));
        }
        literals.sort_by_key(|x| x.0);

        let mut unique_lits = literals.iter().map(|x| x.0).collect::<Vec<_>>();
        unique_lits.dedup();

        let mut adj = vec![];
        for _ in 0..num_vertices {
            adj.push(vec![]);
        }
        for i in 0..num_edges {
            let (u, v) = edges[i];
            adj[u].push((v, i));
            adj[v].push((u, i));
        }

        let num_unique_lits = unique_lits.len();
        let lower_bound = domains
            .iter()
            .map(|d| if d.is_empty() { 0 } else { d[0] })
            .collect::<Vec<_>>();
        let upper_bound = domains
            .iter()
            .map(|d| {
                if d.is_empty() {
                    i32::MAX
                } else {
                    d[d.len() - 1]
                }
            })
            .collect::<Vec<_>>();

        GraphDivision {
            num_vertices,
            num_edges,
            domains: domains.iter().cloned().collect(),
            edges: edges.iter().cloned().collect(),
            edge_lits: edge_lits.iter().cloned().collect(),
            literals,
            adj,
            edge_state: vec![EdgeState::Undecided; num_edges],
            lower_bound,
            upper_bound,
            unique_lits,
            propagations: vec![],
            propagation_reasons: vec![Reason::NotPropagated; num_unique_lits],
            inconsistency_reason: vec![],
            undo_stack: vec![],
        }
    }

    fn notify(&mut self, lit: Lit) {
        self.undo_stack.push(UndoInfo::LevelBoundary);

        let mut idx = self.literals.binary_search_by_key(&lit, |x| x.0).unwrap();
        while idx < self.literals.len() && self.literals[idx].0 == lit {
            match self.literals[idx].1 {
                LiteralInfo::Edge(edge_idx, s) => {
                    self.undo_stack.push(UndoInfo::Edge(edge_idx));
                    self.edge_state[edge_idx] = s;
                }
                LiteralInfo::LowerBound(vertex_idx, value) => {
                    if self.lower_bound[vertex_idx] < value {
                        self.undo_stack.push(UndoInfo::LowerBound(
                            vertex_idx,
                            self.lower_bound[vertex_idx],
                        ));
                        self.lower_bound[vertex_idx] = value;

                        assert!(
                            self.lower_bound[vertex_idx] <= self.upper_bound[vertex_idx],
                            "lower_bound[{}] is larger than upper_bound[{}]; since GraphDivision is lazily propagated, this should not happen",
                            vertex_idx,
                            vertex_idx
                        );
                    }
                }
                LiteralInfo::UpperBound(vertex_idx, value) => {
                    if self.upper_bound[vertex_idx] > value {
                        self.undo_stack.push(UndoInfo::UpperBound(
                            vertex_idx,
                            self.upper_bound[vertex_idx],
                        ));
                        self.upper_bound[vertex_idx] = value;

                        assert!(
                            self.lower_bound[vertex_idx] <= self.upper_bound[vertex_idx],
                            "lower_bound[{}] is larger than upper_bound[{}]; since GraphDivision is lazily propagated, this should not happen",
                            vertex_idx,
                            vertex_idx
                        );
                    }
                }
            }
            idx += 1;
        }
    }

    fn undo_internal(&mut self) {
        while let Some(info) = self.undo_stack.pop() {
            match info {
                UndoInfo::LevelBoundary => {
                    return;
                }
                UndoInfo::Edge(edge_idx) => {
                    self.edge_state[edge_idx] = EdgeState::Undecided;
                }
                UndoInfo::LowerBound(vertex_idx, value) => {
                    self.lower_bound[vertex_idx] = value;
                }
                UndoInfo::UpperBound(vertex_idx, value) => {
                    self.upper_bound[vertex_idx] = value;
                }
            }
        }
    }

    fn register_propagation(&mut self, lit: Lit, reason: Reason) {
        self.propagations.push(lit);

        let lit_id = self.unique_lits.binary_search(&lit).unwrap();
        self.propagation_reasons[lit_id] = reason;
    }

    /// Analyze the current state and perform the following:
    /// - If the current state is inconsistent, set `inconsistency_reason` to the reason of inconsistency.
    /// - If some literals can be propagated, add them to `propagations` and set the reason to `propagation_reasons`.
    ///
    /// Returns `true` if the current state is consistent.
    fn analyze(&mut self) -> bool {
        let num_vertices = self.num_vertices;
        let num_edges = self.num_edges;

        let (decided_group_id, decided_groups) = self.compute_groups(false);
        let (potential_group_id, potential_groups) = self.compute_groups(true);

        // 1.  Both sides of a "disconnected" edge should be in different groups.
        for i in 0..num_edges {
            let (u, v) = self.edges[i];

            if self.edge_state[i] == EdgeState::Disconnected {
                if decided_group_id[u] == decided_group_id[v] {
                    let mut reason = self.connected_path(u, v);
                    reason.push(self.edge_lits[i]);
                    self.inconsistency_reason = reason;
                    return false;
                }
            }
        }

        true
    }

    fn compute_groups(&mut self, potential: bool) -> (Vec<usize>, Vec<Vec<usize>>) {
        let mut group_id = vec![!0; self.domains.len()];
        let mut groups = vec![];

        let mut queue = VecDeque::<usize>::new();
        for i in 0..self.num_vertices {
            if group_id[i] != !0 {
                continue;
            }

            let id = groups.len();
            let mut group = vec![];

            group_id[i] = id;
            queue.push_back(i);

            while let Some(p) = queue.pop_front() {
                group.push(p);

                for &(q, edge_idx) in &self.adj[p] {
                    if self.edge_state[edge_idx] == EdgeState::Disconnected
                        || (!potential && self.edge_state[edge_idx] == EdgeState::Undecided)
                    {
                        continue;
                    }

                    if group_id[q] == !0 {
                        group_id[q] = id;
                        queue.push_back(q);
                    }
                }
            }

            groups.push(group);
        }

        (group_id, groups)
    }

    fn connected_path(&self, u: usize, v: usize) -> Vec<Lit> {
        let mut prev: Vec<Option<(usize, Lit)>> = vec![None; self.num_vertices];
        let mut queue = VecDeque::<usize>::new();
        queue.push_back(u);

        while let Some(p) = queue.pop_front() {
            if p == v {
                break;
            }

            for &(q, edge_idx) in &self.adj[p] {
                if self.edge_state[edge_idx] == EdgeState::Connected && prev[q].is_none() {
                    prev[q] = Some((p, self.edge_lits[edge_idx]));
                    queue.push_back(q);
                }
            }
        }

        assert!(prev[v].is_some());
        let mut ret = vec![];
        let mut p = v;
        while p != u {
            let (q, lit) = prev[p].unwrap();
            ret.push(!lit); // negated because the edge is connected
            p = q;
        }

        ret
    }
}

unsafe impl<T: SolverManipulator> CustomPropagator<T> for GraphDivision {
    fn initialize(&mut self, solver: &mut T) -> bool {
        for &lit in &self.unique_lits {
            unsafe {
                solver.add_watch(lit);
            }
        }

        // TODO: handle already decided literals

        true
    }

    fn propagate(&mut self, solver: &mut T, p: Lit, num_pending_propagations: i32) -> bool {
        self.notify(p);

        if num_pending_propagations != 0 {
            // lazy propagation
            return true;
        }

        self.inconsistency_reason.clear();
        self.propagations.clear();

        if !self.analyze() {
            return false;
        }

        for p in &self.propagations {
            if unsafe { !solver.enqueue(*p) } {
                return false;
            }
        }

        true
    }

    fn calc_reason(&mut self, solver: &mut T, p: Option<Lit>, extra: Option<Lit>) -> Vec<Lit> {
        assert!(extra.is_none());

        if p.is_none() {
            // TODO: handle the case where the inconsistency is detected on `enqueue`
            return self.inconsistency_reason.clone();
        }

        todo!();
    }

    fn undo(&mut self, _solver: &mut T, p: Lit) {
        self.undo_internal();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::glucose::Solver;

    #[test]
    fn test_graph_division_extra_disconnection() {
        let mut solver = Solver::new();
        let mut vars = vec![];
        for _ in 0..4 {
            vars.push(solver.new_var());
        }

        let constr = GraphDivision::new(
            &vec![vec![]; 4],
            &vec![vec![]; 4],
            &[1, 1, 1, 1],
            &[(0, 1), (1, 2), (2, 3), (0, 3)],
            &vars.iter().map(|&x| x.as_lit(false)).collect::<Vec<_>>(),
        );
        assert!(solver.add_custom_constraint(Box::new(constr)));

        let mut n_assignments = 0;
        loop {
            match solver.solve() {
                Some(model) => {
                    n_assignments += 1;
                    let mut new_clause = vec![];
                    for &v in &vars {
                        new_clause.push(v.as_lit(model.assignment(v)));
                    }
                    solver.add_clause(&new_clause);
                }
                None => break,
            }
        }

        assert_eq!(n_assignments, 12);
    }
}
