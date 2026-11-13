#pragma once

#include <cstdint>

namespace CaDiCaL {

class Solver;

}

extern "C" {

CaDiCaL::Solver* CaDiCaL_CreateSolver();
void CaDiCaL_DestroySolver(CaDiCaL::Solver* solver);
void CaDiCaL_AddClause(CaDiCaL::Solver* solver, int32_t* lits, int32_t n_lits);
int32_t CaDiCaL_Solve(CaDiCaL::Solver* solver);
int32_t CaDiCaL_GetModelValueVar(CaDiCaL::Solver* solver, int32_t var);

}
