#include "cadical_bridge.h"

#include "cadical.hpp"

int to_cadical_lit(int l) {
    if (l & 1) {
        return -((l >> 1) + 1);
    } else {
        return (l >> 1) + 1;
    }
}

extern "C" {

CaDiCaL::Solver* CaDiCaL_CreateSolver() {
    CaDiCaL::Solver* solver = new CaDiCaL::Solver();
    solver->set("chrono", 0);  // TODO: do this explicitly
    return solver;
}

void CaDiCaL_DestroySolver(CaDiCaL::Solver* solver) {
    delete solver;
}

void CaDiCaL_AddClause(CaDiCaL::Solver* solver, int32_t* lits, int32_t n_lits) {
    for (int i = 0; i < n_lits; ++i) {
        solver->add(to_cadical_lit(lits[i]));
    }
    solver->add(0);
}

int32_t CaDiCaL_Solve(CaDiCaL::Solver* solver) {
    int res = solver->solve();
    if (res == 10) return 1;
    return 0;
}

int32_t CaDiCaL_GetModelValueVar(CaDiCaL::Solver* solver, int32_t var) {
    int res = solver->val(var + 1);
    return (res > 0) ? 1 : 0;
}

}
