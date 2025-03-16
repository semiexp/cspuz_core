use crate::sat::{CustomPropagator, Lit, SolverManipulator};

pub struct LinearTerm {
    lits: Vec<Lit>,
    domain: Vec<i32>,
    coef: i32,
}

impl LinearTerm {
    pub fn new(lits: Vec<Lit>, domain: Vec<i32>, coef: i32) -> LinearTerm {
        LinearTerm { lits, domain, coef }
    }

    fn normalize(&mut self) {
        if self.coef < 0 {
            self.lits.reverse();
            for lit in &mut self.lits {
                *lit = !*lit;
            }
            self.domain.reverse();
        }
        for d in &mut self.domain {
            *d *= self.coef;
        }
        self.coef = 1;
    }
}

pub struct OrderEncodingLinear {
    terms: Vec<LinearTerm>,
    lits: Vec<(Lit, usize, usize)>,
    ub_index: Vec<usize>,
    undo_list: Vec<Option<(usize, usize)>>,
    active_lits: Vec<Lit>,
    total_ub: i32,
    use_optimize: bool,
}

impl OrderEncodingLinear {
    pub fn new(
        mut terms: Vec<LinearTerm>,
        constant: i32,
        use_optimize: bool,
    ) -> OrderEncodingLinear {
        for term in &mut terms {
            assert!(term.coef != 0);
            if term.coef != 1 {
                term.normalize();
            }
        }
        let ub_index = terms.iter().map(|x| x.lits.len()).collect();
        let total_ub = constant
            + terms
                .iter()
                .map(|x| x.domain[x.domain.len() - 1])
                .sum::<i32>();
        let mut lits = vec![];
        for i in 0..terms.len() {
            for j in 0..terms[i].lits.len() {
                lits.push((terms[i].lits[j], i, j));
            }
        }
        lits.sort();
        OrderEncodingLinear {
            terms,
            lits,
            ub_index,
            undo_list: vec![],
            active_lits: vec![],
            total_ub,
            use_optimize,
        }
    }
}

unsafe impl<T: SolverManipulator> CustomPropagator<T> for OrderEncodingLinear {
    fn initialize(&mut self, solver: &mut T) -> bool {
        let mut unique_watchers = vec![];
        for &(lit, _, _) in &self.lits {
            unique_watchers.push(!lit);
        }
        unique_watchers.sort();
        unique_watchers.dedup();

        for &lit in &unique_watchers {
            unsafe {
                solver.add_watch(lit);
            }
        }

        for lit in unique_watchers {
            let val = unsafe { solver.value(lit) };
            if val == Some(true) {
                if !self.propagate(solver, lit, 0) {
                    return false;
                }
            }
        }

        if self.total_ub < 0 {
            return false;
        }

        true
    }

    fn propagate(&mut self, solver: &mut T, p: Lit, _num_pending_propagations: i32) -> bool {
        self.active_lits.push(p);
        self.undo_list.push(None);

        let mut idx = self.lits.partition_point(|&(lit, _, _)| lit < !p);
        while idx < self.lits.len() && self.lits[idx].0 == !p {
            let (_, i, j) = self.lits[idx];
            idx += 1;
            if self.ub_index[i] <= j {
                continue;
            }
            self.undo_list.push(Some((i, self.ub_index[i])));
            self.total_ub -= self.terms[i].domain[self.ub_index[i]] - self.terms[i].domain[j];
            self.ub_index[i] = j;

            if self.total_ub < 0 {
                return false;
            }
        }

        for i in 0..self.terms.len() {
            let ubi = self.ub_index[i];
            if ubi == 0 {
                continue;
            }
            if self.total_ub - (self.terms[i].domain[ubi] - self.terms[i].domain[0]) >= 0 {
                continue;
            }

            let threshold = self.terms[i].domain[ubi] - self.total_ub;
            let left = self.terms[i].domain.partition_point(|&x| x < threshold) - 1;
            if !unsafe { solver.enqueue(self.terms[i].lits[left]) } {
                return false;
            }
        }

        true
    }

    fn calc_reason(&mut self, _: &mut T, p: Option<Lit>, extra: Option<Lit>) -> Vec<Lit> {
        let mut p_idx: Option<usize> = None;
        if self.use_optimize {
            if let Some(p) = p {
                let idx = self.lits.partition_point(|&(lit, _, _)| lit < p);
                assert!(self.lits[idx].0 == p);
                if idx + 1 == self.lits.len() || self.lits[idx + 1].0 != p {
                    p_idx = Some(self.lits[idx].1);
                }
            }
        }
        let mut ret = vec![];
        for i in 0..self.terms.len() {
            if Some(i) == p_idx {
                continue;
            }
            if self.ub_index[i] < self.terms[i].lits.len() {
                ret.push(!self.terms[i].lits[self.ub_index[i]]);
            }
        }
        ret.extend(extra);
        ret
    }

    fn undo(&mut self, _: &mut T, _: Lit) {
        while let Some((i, j)) = self.undo_list.pop().unwrap() {
            self.total_ub += self.terms[i].domain[j] - self.terms[i].domain[self.ub_index[i]];
            self.ub_index[i] = j;
        }
    }
}
