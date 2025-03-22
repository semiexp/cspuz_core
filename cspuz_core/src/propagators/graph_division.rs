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
    LowerBound(usize, i32, Option<Lit>), // (index, old_value, old_lit)
    UpperBound(usize, i32, Option<Lit>), // (index, old_value, old_lit)
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
        newly_decided_edge_idx: usize,

        /// Let (u1, v1) = edges[disconnected_edge_idx] and (u2, v2) = edges[newly_decided_edge_idx].
        /// Then one of the following holds:
        /// - u1 and u2 are in the same group, and v1 and v2 are in the same group.
        /// - u1 and v2 are in the same group, and v1 and u2 are in the same group.
        /// In the first case, `flip` is false. In the second case, `flip` is true.
        flip: bool,
    },

    /// For edge `disconnected_edge_idx`, if regions which two ends of the edge belong to are merged,
    /// the resulting regions will be too large.
    TooLargeIfRegionsAreMerged {
        disconnected_edge_idx: usize,
        upper_bound_lit: Option<Lit>,
    },

    /// For vertex `vertex_idx`, the decided region it belongs to is so large that more strict
    /// lower bound can be propagated.
    RegionAlreadyLarge {
        vertex_idx: usize,
    },

    /// For vertex `vertex_idx`, the potential region it belongs to is so large that more strict
    /// upper bound can be propagated.
    RegionAlreadySmall {
        vertex_idx: usize,
    },

    /// For edge `disconnected_edge_idx`, if regions which two ends of the edge belong to are merged,
    /// the resulting regions will contain vertices with inconsistent lower / upper bounds.
    InconsistentBoundsIfRegionsAreMerged {
        disconnected_edge_idx: usize,
        upper_bound_vertex_idx: usize,
        lower_bound_vertex_idx: usize,

        /// Let (u1, v1) = edges[disconnected_edge_idx] and u2 = upper_bound_vertex_idx and v2 = lower_bound_vertex_idx.
        /// Then one of the following holds:
        /// - u1 and u2 are in the same group, and v1 and v2 are in the same group.
        /// - u1 and v2 are in the same group, and v1 and u2 are in the same group.
        /// In the first case, `flip` is false. In the second case, `flip` is true.
        flip: bool,
    },
}

pub struct GraphDivision {
    num_vertices: usize,
    num_edges: usize,

    vertex_weights: Vec<i32>,

    domains: Vec<Vec<i32>>,
    dom_lits: Vec<Vec<Lit>>,
    edges: Vec<(usize, usize)>,
    edge_lits: Vec<Lit>,
    literals: Vec<(Lit, LiteralInfo)>,
    adj: Vec<Vec<(usize, usize)>>, // (vertex, edge_idx)

    edge_state: Vec<EdgeState>,
    lower_bound: Vec<i32>,
    lower_bound_lit: Vec<Option<Lit>>,
    upper_bound: Vec<i32>,
    upper_bound_lit: Vec<Option<Lit>>,

    unique_lits: Vec<Lit>,

    propagations: Vec<Lit>,
    propagation_reasons: Vec<Reason>, // the reason why unique_lits[i] is propagated

    /// The reason why the current state is inconsistent.
    /// Since this reason will be immediately used to calculate the reason of the next propagation,
    /// we directly store it as a vector of literals.
    inconsistency_reason: Vec<Lit>,

    /// Since this constraint uses only the information of already propagated literals, it is possible that
    /// `enqueue` fails due to a conflict with already decided (but not yet propagated) literals.
    /// In such cases, we do not use `inconsistency_reason` and compute the reason from `propagation_reasons`.
    /// To do so, we need to store the literal that caused the propagation failure.
    propagation_failure_lit: Option<Lit>,

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
        for dom in domains {
            assert!(dom.is_sorted());
        }

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
            vertex_weights: vertex_weights.to_vec(),
            domains: domains.iter().cloned().collect(),
            dom_lits: dom_lits.iter().cloned().collect(),
            edges: edges.iter().cloned().collect(),
            edge_lits: edge_lits.iter().cloned().collect(),
            literals,
            adj,
            edge_state: vec![EdgeState::Undecided; num_edges],
            lower_bound,
            lower_bound_lit: vec![None; num_vertices],
            upper_bound,
            upper_bound_lit: vec![None; num_vertices],
            unique_lits,
            propagations: vec![],
            propagation_reasons: vec![Reason::NotPropagated; num_unique_lits],
            inconsistency_reason: vec![],
            propagation_failure_lit: None,
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
                // It is possible that lower_bound[v] > upper_bound[v] holds for some v.
                // However, since GraphDivision is lazily propagated, we should not run into such cases in `analyze`.
                LiteralInfo::LowerBound(vertex_idx, value) => {
                    if self.lower_bound[vertex_idx] < value {
                        self.undo_stack.push(UndoInfo::LowerBound(
                            vertex_idx,
                            self.lower_bound[vertex_idx],
                            self.lower_bound_lit[vertex_idx],
                        ));
                        self.lower_bound[vertex_idx] = value;
                        self.lower_bound_lit[vertex_idx] = Some(lit);
                    }
                }
                LiteralInfo::UpperBound(vertex_idx, value) => {
                    if self.upper_bound[vertex_idx] > value {
                        self.undo_stack.push(UndoInfo::UpperBound(
                            vertex_idx,
                            self.upper_bound[vertex_idx],
                            self.upper_bound_lit[vertex_idx],
                        ));
                        self.upper_bound[vertex_idx] = value;
                        self.upper_bound_lit[vertex_idx] = Some(lit);
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
                UndoInfo::LowerBound(vertex_idx, value, lit) => {
                    self.lower_bound[vertex_idx] = value;
                    self.lower_bound_lit[vertex_idx] = lit;
                }
                UndoInfo::UpperBound(vertex_idx, value, lit) => {
                    self.upper_bound[vertex_idx] = value;
                    self.upper_bound_lit[vertex_idx] = lit;
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

        let (decided_region_id, decided_regions) = self.compute_regions(false);
        let (potential_region_id, potential_regions) = self.compute_regions(true);

        // 1.  Both sides of a "disconnected" edge should be in different regions.
        let mut disconnected_groups = vec![];
        for i in 0..num_edges {
            let (u, v) = self.edges[i];

            if decided_region_id[u] == decided_region_id[v] {
                match self.edge_state[i] {
                    EdgeState::Disconnected => {
                        let mut reason = self.reason_connected_path(u, v);
                        reason.push(self.edge_lits[i]);
                        self.inconsistency_reason = reason;
                        return false;
                    }
                    EdgeState::Connected => (),
                    EdgeState::Undecided => {
                        self.register_propagation(
                            !self.edge_lits[i],
                            Reason::EdgeInSameGroup { edge_idx: i },
                        );
                    }
                }
            } else if self.edge_state[i] == EdgeState::Disconnected {
                let ui = decided_region_id[u];
                let vi = decided_region_id[v];

                if ui > vi {
                    disconnected_groups.push(((vi, ui), i));
                } else {
                    disconnected_groups.push(((ui, vi), i));
                }
            }
        }

        disconnected_groups.sort();
        for i in 0..num_edges {
            let (u, v) = self.edges[i];

            let ui = decided_region_id[u];
            let vi = decided_region_id[v];

            if ui == vi {
                continue;
            }
            if self.edge_state[i] != EdgeState::Undecided {
                continue;
            }

            let pair = if ui > vi { (vi, ui) } else { (ui, vi) };
            if let Ok(idx) = disconnected_groups.binary_search_by_key(&pair, |x| x.0) {
                let (_, edge_idx) = disconnected_groups[idx];

                let (eu, ev) = self.edges[edge_idx];
                assert!(ui == decided_region_id[eu] || ui == decided_region_id[ev]);
                assert!(vi == decided_region_id[eu] || vi == decided_region_id[ev]);

                self.register_propagation(
                    self.edge_lits[i],
                    Reason::EdgeBetweenDifferentGroups {
                        disconnected_edge_idx: edge_idx,
                        newly_decided_edge_idx: i,
                        flip: ui == decided_region_id[ev],
                    },
                );
            }
        }

        // 2. Within a decided region, weight variable must be at least the weight of the region
        let mut decided_region_weight = vec![];
        for region in &decided_regions {
            let weight = region.iter().map(|&i| self.vertex_weights[i]).sum::<i32>();
            decided_region_weight.push(weight);
        }

        for i in 0..num_vertices {
            assert!(self.lower_bound[i] <= self.upper_bound[i]);
        }

        for i in 0..num_vertices {
            // NOTE: Vertices with no constraint are ignored because their `upper_bound` is `i32::MAX`.
            if self.upper_bound[i] < decided_region_weight[decided_region_id[i]] {
                let mut reason = self.reason_decided_region(i);
                reason.extend(self.upper_bound_lit[i]);
                self.inconsistency_reason = reason;
                return false;
            }
        }

        let mut region_upper_bound = vec![i32::MAX; decided_regions.len()];
        let mut region_upper_bound_idx = vec![!0; decided_regions.len()];
        let mut region_lower_bound = vec![0; decided_regions.len()];
        let mut region_lower_bound_idx = vec![!0; decided_regions.len()];
        for i in 0..num_vertices {
            let region_id = decided_region_id[i];
            if self.upper_bound[i] < region_upper_bound[region_id] {
                region_upper_bound[region_id] = self.upper_bound[i];
                region_upper_bound_idx[region_id] = i;
            }
            if self.lower_bound[i] > region_lower_bound[region_id] {
                region_lower_bound[region_id] = self.lower_bound[i];
                region_lower_bound_idx[region_id] = i;
            }
        }

        for i in 0..num_vertices {
            if self.domains[i].is_empty() {
                continue;
            }

            let region_id = decided_region_id[i];
            if self.lower_bound[i] < decided_region_weight[region_id] {
                let new_lower_bound_idx =
                    self.domains[i].partition_point(|&x| x < decided_region_weight[region_id]);
                if new_lower_bound_idx == self.domains[i].len() {
                    panic!();
                }
                self.register_propagation(
                    self.dom_lits[i][new_lower_bound_idx - 1],
                    Reason::RegionAlreadyLarge { vertex_idx: i },
                );
            }
        }

        for i in 0..num_edges {
            let (u, v) = self.edges[i];

            if decided_region_id[u] == decided_region_id[v] {
                continue;
            }

            if self.edge_state[i] != EdgeState::Undecided {
                continue;
            }

            let ui = decided_region_id[u];
            let vi = decided_region_id[v];

            if region_upper_bound[ui].min(region_upper_bound[vi])
                < decided_region_weight[ui] + decided_region_weight[vi]
            {
                let idx = if region_upper_bound[ui] < region_upper_bound[vi] {
                    region_upper_bound_idx[ui]
                } else {
                    region_upper_bound_idx[vi]
                };
                self.register_propagation(
                    self.edge_lits[i],
                    Reason::TooLargeIfRegionsAreMerged {
                        disconnected_edge_idx: i,
                        upper_bound_lit: self.upper_bound_lit[idx],
                    },
                );
            }
        }

        // 3. Within a potential region, weight variable must be at most the weight of the region
        let mut potential_region_weight = vec![];
        for region in &potential_regions {
            let weight = region.iter().map(|&i| self.vertex_weights[i]).sum::<i32>();
            potential_region_weight.push(weight);
        }

        for i in 0..num_vertices {
            if self.domains[i].is_empty() {
                continue;
            }

            if self.lower_bound[i] > potential_region_weight[potential_region_id[i]] {
                // TODO: use DSU-based algorithm to minimize the reason
                let mut reason = self.reason_potential_region(i);
                reason.extend(self.lower_bound_lit[i]);
                self.inconsistency_reason = reason;
                return false;
            }

            if self.upper_bound[i] > potential_region_weight[potential_region_id[i]] {
                let new_upper_bound_idx = self.domains[i]
                    .partition_point(|&x| x <= potential_region_weight[potential_region_id[i]]);
                assert!(new_upper_bound_idx > 0);
                self.register_propagation(
                    !self.dom_lits[i][new_upper_bound_idx - 1],
                    Reason::RegionAlreadySmall { vertex_idx: i },
                );
            }
        }

        // 4. All the weight variables in a decided region must have the same value.
        // Therefore, in a decided region, it is not allowed that a vertex has the upper bound less than
        // the lower bound of another vertex.
        // NOTE: this is not necessary (but sufficient) for consistency
        for region in &decided_regions {
            let mut lower_bound = self.lower_bound[region[0]];
            let mut upper_bound = self.upper_bound[region[0]];
            let mut lower_bound_idx = region[0];
            let mut upper_bound_idx = region[0];

            for &c in region {
                if self.lower_bound[c] > lower_bound {
                    lower_bound = self.lower_bound[c];
                    lower_bound_idx = c;
                }
                if self.upper_bound[c] < upper_bound {
                    upper_bound = self.upper_bound[c];
                    upper_bound_idx = c;
                }
            }

            if lower_bound > upper_bound {
                let mut reason = self.reason_connected_path(lower_bound_idx, upper_bound_idx);
                reason.extend(self.lower_bound_lit[lower_bound_idx]);
                reason.extend(self.upper_bound_lit[upper_bound_idx]);
                self.inconsistency_reason = reason;
                return false;
            }
        }

        for i in 0..num_edges {
            let (u, v) = self.edges[i];

            if decided_region_id[u] == decided_region_id[v] {
                continue;
            }

            if self.edge_state[i] != EdgeState::Undecided {
                continue;
            }

            let ui = decided_region_id[u];
            let vi = decided_region_id[v];

            if region_lower_bound[ui] > region_upper_bound[vi] {
                self.register_propagation(
                    self.edge_lits[i],
                    Reason::InconsistentBoundsIfRegionsAreMerged {
                        disconnected_edge_idx: i,
                        upper_bound_vertex_idx: region_upper_bound_idx[vi],
                        lower_bound_vertex_idx: region_lower_bound_idx[ui],
                        flip: true,
                    },
                );
            }
            if region_lower_bound[vi] > region_upper_bound[ui] {
                self.register_propagation(
                    self.edge_lits[i],
                    Reason::InconsistentBoundsIfRegionsAreMerged {
                        disconnected_edge_idx: i,
                        upper_bound_vertex_idx: region_upper_bound_idx[ui],
                        lower_bound_vertex_idx: region_lower_bound_idx[vi],
                        flip: false,
                    },
                );
            }
        }

        true
    }

    fn compute_regions(&mut self, potential: bool) -> (Vec<usize>, Vec<Vec<usize>>) {
        let mut region_id = vec![!0; self.domains.len()];
        let mut regions = vec![];

        let mut queue = VecDeque::<usize>::new();
        for i in 0..self.num_vertices {
            if region_id[i] != !0 {
                continue;
            }

            let id = regions.len();
            let mut group = vec![];

            region_id[i] = id;
            queue.push_back(i);

            while let Some(p) = queue.pop_front() {
                group.push(p);

                for &(q, edge_idx) in &self.adj[p] {
                    if self.edge_state[edge_idx] == EdgeState::Disconnected
                        || (!potential && self.edge_state[edge_idx] == EdgeState::Undecided)
                    {
                        continue;
                    }

                    if region_id[q] == !0 {
                        region_id[q] = id;
                        queue.push_back(q);
                    }
                }
            }

            regions.push(group);
        }

        (region_id, regions)
    }

    fn reason_connected_path(&self, u: usize, v: usize) -> Vec<Lit> {
        if u == v {
            return vec![];
        }

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

    fn reason_decided_region(&self, u: usize) -> Vec<Lit> {
        let mut queue = VecDeque::<usize>::new();
        let mut visited = vec![false; self.num_vertices];
        visited[u] = true;
        queue.push_back(u);

        let mut ret = vec![];

        // TODO: Use Prim algorithm to find the minimum spanning tree
        while let Some(p) = queue.pop_front() {
            for &(q, edge_idx) in &self.adj[p] {
                if self.edge_state[edge_idx] != EdgeState::Connected {
                    continue;
                }

                if !visited[q] {
                    visited[q] = true;
                    ret.push(!self.edge_lits[edge_idx]);
                    queue.push_back(q);
                }
            }
        }

        ret
    }

    fn reason_potential_region(&self, u: usize) -> Vec<Lit> {
        let mut queue = VecDeque::<usize>::new();
        let mut visited = vec![false; self.num_vertices];
        visited[u] = true;
        queue.push_back(u);

        let mut ret = vec![];

        while let Some(p) = queue.pop_front() {
            for &(q, edge_idx) in &self.adj[p] {
                if self.edge_state[edge_idx] == EdgeState::Disconnected {
                    ret.push(self.edge_lits[edge_idx]);
                    continue;
                }

                if !visited[q] {
                    visited[q] = true;
                    queue.push_back(q);
                }
            }
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

        let unique_lits = self.unique_lits.clone();
        for p in unique_lits {
            if unsafe { solver.value(p) } == Some(true) {
                if !self.propagate(solver, p, 0) {
                    return false;
                }
            }
        }

        true
    }

    fn propagate(&mut self, solver: &mut T, p: Lit, num_pending_propagations: i32) -> bool {
        self.notify(p);

        if num_pending_propagations != 0 {
            // lazy propagation
            return true;
        }

        self.propagation_failure_lit = None;
        self.inconsistency_reason.clear();
        self.propagations.clear();

        if !self.analyze() {
            return false;
        }

        for p in &self.propagations {
            if unsafe { solver.value(*p) } == Some(false) {
                self.propagation_failure_lit = Some(*p);
                return false;
            }

            assert!(unsafe { solver.enqueue(*p) });
        }

        true
    }

    fn calc_reason(&mut self, _solver: &mut T, p: Option<Lit>, extra: Option<Lit>) -> Vec<Lit> {
        assert!(extra.is_none());

        if p.is_none() && self.propagation_failure_lit.is_none() {
            return self.inconsistency_reason.clone();
        }

        let p = p.unwrap();
        let idx = self.unique_lits.binary_search(&p).unwrap();
        let reason = &self.propagation_reasons[idx];

        let mut res = match reason {
            &Reason::NotPropagated => panic!(),
            &Reason::EdgeInSameGroup { edge_idx } => {
                let (u, v) = self.edges[edge_idx];
                let ret = self.reason_connected_path(u, v);
                ret
            }
            &Reason::EdgeBetweenDifferentGroups {
                disconnected_edge_idx,
                newly_decided_edge_idx,
                flip,
            } => {
                let (u1, v1) = self.edges[disconnected_edge_idx];
                let (u2, v2) = self.edges[newly_decided_edge_idx];
                let (u2, v2) = if flip { (v2, u2) } else { (u2, v2) };

                let mut ret = self.reason_connected_path(u1, u2);
                let path = self.reason_connected_path(v1, v2);
                ret.extend(path);
                ret.push(self.edge_lits[disconnected_edge_idx]);

                ret
            }
            &Reason::TooLargeIfRegionsAreMerged {
                disconnected_edge_idx,
                upper_bound_lit,
            } => {
                let (u, v) = self.edges[disconnected_edge_idx];
                let mut ret = self.reason_decided_region(u);
                ret.extend(self.reason_decided_region(v));
                ret.extend(upper_bound_lit);
                ret
            }
            &Reason::RegionAlreadyLarge { vertex_idx } => self.reason_decided_region(vertex_idx),
            &Reason::RegionAlreadySmall { vertex_idx } => self.reason_potential_region(vertex_idx),
            &Reason::InconsistentBoundsIfRegionsAreMerged {
                disconnected_edge_idx,
                upper_bound_vertex_idx,
                lower_bound_vertex_idx,
                flip,
            } => {
                let (u1, v1) = self.edges[disconnected_edge_idx];
                let (u1, v1) = if flip { (v1, u1) } else { (u1, v1) };

                let mut ret = self.reason_connected_path(u1, upper_bound_vertex_idx);
                ret.extend(self.reason_connected_path(v1, lower_bound_vertex_idx));
                ret.extend(self.upper_bound_lit[upper_bound_vertex_idx]);
                ret.extend(self.lower_bound_lit[lower_bound_vertex_idx]);
                ret
            }
        };

        if let Some(p) = self.propagation_failure_lit {
            res.push(p);
        }

        res
    }

    fn undo(&mut self, _solver: &mut T, _p: Lit) {
        self.undo_internal();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::glucose::Solver;

    fn compare_counts(
        num_vertices: usize,
        domains: &[Option<Vec<i32>>],
        vertex_weights: Option<Vec<i32>>,
        edges: &[(usize, usize)],
        predetermined_edges: &[Option<bool>],
    ) {
        let num_edges = edges.len();
        assert!(num_edges <= 20);

        let mut solver = Solver::new();

        let mut domains_vec = vec![];
        let mut dom_lits = vec![];
        let mut all_vars = vec![];

        for i in 0..num_vertices {
            if let Some(dom) = &domains[i] {
                domains_vec.push(dom.clone());
                let mut lits = vec![];
                for _ in 0..(dom.len() - 1) {
                    let v = solver.new_var();
                    lits.push(v.as_lit(false));
                    all_vars.push(v);
                }
                for j in 1..lits.len() {
                    solver.add_clause(&[lits[j - 1], !lits[j]]);
                }
                dom_lits.push(lits);
            } else {
                domains_vec.push(vec![]);
                dom_lits.push(vec![]);
            }
        }

        let vertex_weights = &vertex_weights.unwrap_or_else(|| vec![1; num_vertices]);
        let mut edge_lits = vec![];

        for _ in 0..num_edges {
            let v = solver.new_var();
            edge_lits.push(v.as_lit(false));
            all_vars.push(v);
        }

        for i in 0..num_edges {
            if let Some(v) = predetermined_edges[i] {
                solver.add_clause(&[if v { edge_lits[i] } else { !edge_lits[i] }]);
            }
        }

        solver.add_custom_constraint(Box::new(GraphDivision::new(
            &domains_vec,
            &dom_lits,
            vertex_weights,
            &edges,
            &edge_lits,
        )));

        let mut n_assignments_sat = 0;
        loop {
            match solver.solve() {
                Some(model) => {
                    n_assignments_sat += 1;
                    let mut new_clause = vec![];
                    for &v in &all_vars {
                        new_clause.push(v.as_lit(model.assignment(v)));
                    }
                    solver.add_clause(&new_clause);
                }
                None => break,
            }
        }

        let mut adj = vec![vec![]; num_vertices];
        for (i, &(u, v)) in edges.iter().enumerate() {
            adj[u].push((v, i));
            adj[v].push((u, i));
        }

        let mut n_assignments_naive = 0;
        for m in 0u32..(1 << num_edges) {
            let is_border = (0..num_edges)
                .map(|i| (m >> i) & 1 == 1)
                .collect::<Vec<_>>();

            let mut is_consistent = true;

            // consistent with predetermined_edges?
            for i in 0..num_edges {
                if let Some(v) = predetermined_edges[i] {
                    if is_border[i] != v {
                        is_consistent = false;
                        break;
                    }
                }
            }

            if !is_consistent {
                continue;
            }

            let mut group_id = vec![!0; num_vertices];
            let mut group_size = vec![];

            for i in 0..num_vertices {
                if group_id[i] != !0 {
                    continue;
                }

                let id = group_size.len();
                let mut size = 0;

                let mut queue = VecDeque::<usize>::new();
                queue.push_back(i);
                group_id[i] = id;

                while let Some(p) = queue.pop_front() {
                    size += vertex_weights[p];

                    for &(q, edge_idx) in &adj[p] {
                        if is_border[edge_idx] {
                            continue;
                        }

                        if group_id[q] == !0 {
                            group_id[q] = id;
                            queue.push_back(q);
                        }
                    }
                }

                group_size.push(size);
            }

            // no extra border?
            for i in 0..num_edges {
                if is_border[i] {
                    let (u, v) = edges[i];
                    if group_id[u] == group_id[v] {
                        is_consistent = false;
                        break;
                    }
                }
            }

            if !is_consistent {
                continue;
            }

            // consistent with domains?
            for i in 0..num_vertices {
                if let Some(dom) = &domains[i] {
                    if !dom.contains(&group_size[group_id[i]]) {
                        is_consistent = false;
                        break;
                    }
                }
            }

            if !is_consistent {
                continue;
            }

            n_assignments_naive += 1;
        }

        assert_eq!(n_assignments_sat, n_assignments_naive);
    }

    #[test]
    fn test_graph_division_extra_disconnection() {
        // 2x2 grid graph
        compare_counts(
            4,
            &vec![None; 4],
            None,
            &[(0, 1), (1, 2), (2, 3), (0, 3)],
            &[None; 4],
        );

        // 2x3 grid graph
        compare_counts(
            6,
            &vec![None; 6],
            None,
            &[(0, 1), (1, 2), (3, 4), (4, 5), (0, 3), (1, 4), (2, 5)],
            &[None; 7],
        );

        // 3x3 grid graph
        compare_counts(
            9,
            &vec![None; 9],
            None,
            &[
                (0, 1),
                (1, 2),
                (3, 4),
                (4, 5),
                (6, 7),
                (7, 8),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 6),
                (4, 7),
                (5, 8),
            ],
            &[None; 12],
        );
    }

    #[test]
    fn test_graph_division_extra_disconnection_predetermined() {
        // 2x3 grid graph
        compare_counts(
            6,
            &vec![None; 6],
            None,
            &[(0, 1), (1, 2), (3, 4), (4, 5), (0, 3), (1, 4), (2, 5)],
            &[
                None,
                Some(true),
                None,
                Some(false),
                None,
                Some(false),
                Some(false),
            ],
        );

        // 3x3 grid graph
        compare_counts(
            9,
            &vec![None; 9],
            None,
            &[
                (0, 1),
                (1, 2),
                (3, 4),
                (4, 5),
                (6, 7),
                (7, 8),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 6),
                (4, 7),
                (5, 8),
            ],
            &[
                None,
                Some(true),
                None,
                None,
                None,
                None,
                Some(false),
                None,
                None,
                None,
                None,
                None,
            ],
        );
    }

    #[test]
    fn test_graph_division_vertex_constraints() {
        // 2x3 grid graph
        compare_counts(
            6,
            &vec![Some(vec![2, 3]), None, None, None, None, None],
            None,
            &[(0, 1), (1, 2), (3, 4), (4, 5), (0, 3), (1, 4), (2, 5)],
            &[None; 7],
        );

        // 3x3 grid graph
        compare_counts(
            9,
            &vec![
                Some(vec![2, 3]),
                None,
                None,
                None,
                None,
                Some(vec![1, 3, 4]),
                None,
                None,
                None,
            ],
            None,
            &[
                (0, 1),
                (1, 2),
                (3, 4),
                (4, 5),
                (6, 7),
                (7, 8),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 6),
                (4, 7),
                (5, 8),
            ],
            &[None; 12],
        );

        // 3x3 grid graph
        compare_counts(
            9,
            &vec![
                Some(vec![4]),
                None,
                None,
                None,
                None,
                None,
                None,
                None,
                Some(vec![6]),
            ],
            None,
            &[
                (0, 1),
                (1, 2),
                (3, 4),
                (4, 5),
                (6, 7),
                (7, 8),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 6),
                (4, 7),
                (5, 8),
            ],
            &[None; 12],
        );

        // 3x3 grid graph, many domain constraints
        compare_counts(
            9,
            &vec![
                Some(vec![1, 2]),
                Some(vec![3]),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3]),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3]),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3, 4]),
            ],
            None,
            &[
                (0, 1),
                (1, 2),
                (3, 4),
                (4, 5),
                (6, 7),
                (7, 8),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 6),
                (4, 7),
                (5, 8),
            ],
            &[None; 12],
        );
    }

    #[test]
    fn test_graph_division_vertex_constraints_weighted() {
        // 3x3 grid graph, many domain constraints
        compare_counts(
            9,
            &vec![
                Some(vec![1, 2]),
                Some(vec![3]),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3]),
                Some(vec![1, 2, 3, 4]),
                None,
                Some(vec![1, 2, 3]),
                Some(vec![1, 2, 3, 4]),
                Some(vec![1, 2, 3, 4]),
            ],
            Some(vec![1, 2, 1, 2, 2, 3, 1, 1, 0]),
            &[
                (0, 1),
                (1, 2),
                (3, 4),
                (4, 5),
                (6, 7),
                (7, 8),
                (0, 3),
                (1, 4),
                (2, 5),
                (3, 6),
                (4, 7),
                (5, 8),
            ],
            &[None; 12],
        );
    }
}
