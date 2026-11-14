use super::ClauseSet;
use crate::sat::Lit;
use std::collections::BTreeSet;

fn canonical_clause_set(clauses: ClauseSet) -> BTreeSet<Vec<Lit>> {
    let mut ret = BTreeSet::new();
    for i in 0..clauses.len() {
        let mut clause = clauses[i].to_vec();
        clause.sort_unstable();
        clause.dedup();
        ret.insert(clause);
    }
    ret
}

fn subsumes(lhs: &[Lit], rhs: &[Lit]) -> bool {
    lhs.iter().all(|lit| rhs.binary_search(lit).is_ok())
}

pub(super) fn assert_erase_subsumed_clauses(erased: ClauseSet, all: ClauseSet, instance: &str) {
    let erased = canonical_clause_set(erased);
    let all = canonical_clause_set(all);

    assert!(
        erased.len() < all.len() && erased.is_subset(&all),
        "the erased clause set must be a proper subset: {instance}\nerased={erased:?}\nall={all:?}"
    );

    for removed in all.difference(&erased) {
        assert!(
            erased.iter().any(|clause| subsumes(clause, removed)),
            "removed clause is not subsumed by a retained clause: {instance}\nremoved={removed:?}\nerased={erased:?}"
        );
    }

    // TODO: Enable this minimality check after the optimizations also remove
    // subsumed clauses from the retained clause set.
    /*
    for clause in &erased {
        assert!(
            !erased
                .iter()
                .any(|other| other != clause && subsumes(other, clause)),
            "retained clause is subsumed by another retained clause: {instance}\nclause={clause:?}\nerased={erased:?}"
        );
    }
    */
}
