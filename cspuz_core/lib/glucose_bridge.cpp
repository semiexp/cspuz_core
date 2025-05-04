#include "glucose_bridge.h"

#include "core/Solver.h"
#include "constraints/DirectEncodingExtension.h"
#include "constraints/Graph.h"
#include "constraints/GraphDivision.h"
#include "constraints/OrderEncodingLinear.h"

namespace Glucose {

bool RustExtraConstraint::initialize(Solver& solver) {
    return Glucose_CallCustomPropagatorInitialize(&solver, this, trait_object_) != 0;
}

bool RustExtraConstraint::propagate(Solver& solver, Lit p) {
    solver.registerUndo(var(p), this);
    return Glucose_CallCustomPropagatorPropagate(&solver, this, trait_object_, p.x, num_pending_propagation()) != 0;
}

void RustExtraConstraint::calcReason(Solver& solver, Lit p, Lit extra, vec<Lit>& out_reason) {
    Glucose_CallCustomPropagatorCalcReason(&solver, trait_object_, p.x, extra.x, &out_reason);
}

void RustExtraConstraint::undo(Solver& solver, Lit p) {
    Glucose_CallCustomPropagatorUndo(&solver, trait_object_, p.x);
}

}

extern "C" {

Glucose::Solver* Glucose_CreateSolver() {
    Glucose::Solver* solver = new Glucose::Solver();
    return solver;
}

void Glucose_DestroySolver(Glucose::Solver* solver) {
    delete solver;
}

int Glucose_NewVar(Glucose::Solver* solver) {
    return solver->newVar();
}

int32_t Glucose_NewNamedVar(Glucose::Solver* solver, const char* name) {
    std::string name_str(name);
    return solver->newNamedVar(name_str);
}

int32_t Glucose_AddClause(Glucose::Solver* solver, int32_t* lits, int32_t n_lits) {
    Glucose::vec<Glucose::Lit> lits_vec;
    for (int i = 0; i < n_lits; ++i) {
        lits_vec.push(Glucose::Lit{lits[i]});
    }
    return solver->addClause(lits_vec);
}

int32_t Glucose_Solve(Glucose::Solver* solver) {
    return solver->solve();
}

int32_t Glucose_NumVar(const Glucose::Solver* solver) {
    return solver->nVars();
}

int32_t Glucose_GetModelValueVar(const Glucose::Solver* solver, int32_t var) {
    return solver->modelValue(Glucose::Var(var)) == l_True ? 1 : 0;
}

void Glucose_SetPolarity(Glucose::Solver* solver, int32_t var, int32_t polarity) {
    solver->setPolarity(Glucose::Var(var), polarity != 0);
}

int32_t Glucose_AddOrderEncodingLinear(Glucose::Solver* solver, int32_t n_terms, const int32_t* domain_size, const int32_t* lits, const int32_t* domain, const int32_t* coefs, int32_t constant) {
    std::vector<Glucose::LinearTerm> terms;
    int lits_offset = 0, domain_offset = 0;
    for (int i = 0; i < n_terms; ++i) {
        std::vector<Glucose::Lit> term_lits;
        for (int j = 0; j < domain_size[i] - 1; ++j) {
            term_lits.push_back(Glucose::Lit{lits[lits_offset++]});
        }
        std::vector<int> term_domain;
        for (int j = 0; j < domain_size[i]; ++j) {
            term_domain.push_back(domain[domain_offset++]);
        }
        terms.push_back(Glucose::LinearTerm{ term_lits, term_domain, coefs[i] });
    }
    return solver->addConstraint(std::make_unique<Glucose::OrderEncodingLinear>(std::move(terms), constant)) ? 1 : 0;
}

int32_t Glucose_AddActiveVerticesConnected(Glucose::Solver* solver, int32_t n_vertices, const int32_t* lits, int32_t n_edges, const int32_t* edges) {
    std::vector<Glucose::Lit> g_lits;
    for (int i = 0; i < n_vertices; ++i) {
        g_lits.push_back(Glucose::Lit{lits[i]});
    }
    std::vector<std::pair<int, int>> g_edges;
    for (int i = 0; i < n_edges; ++i) {
        g_edges.push_back({edges[i * 2], edges[i * 2 + 1]});
    }
    return solver->addConstraint(std::make_unique<Glucose::ActiveVerticesConnected>(std::move(g_lits), std::move(g_edges))) ? 1 : 0;
}

#if PUZZLE_SOLVER_MINIMAL
int32_t Glucose_AddDirectEncodingExtensionSupports(Glucose::Solver* solver, int32_t n_vars, const int32_t* domain_size, const int32_t* lits, int32_t n_supports, const int32_t* supports) {
    abort();
}
#else
int32_t Glucose_AddDirectEncodingExtensionSupports(Glucose::Solver* solver, int32_t n_vars, const int32_t* domain_size, const int32_t* lits, int32_t n_supports, const int32_t* supports) {
    std::vector<std::vector<Glucose::Lit>> g_lits;
    int lits_offset = 0;
    for (int i = 0; i < n_vars; ++i) {
        std::vector<Glucose::Lit> var_lits;
        for (int j = 0; j < domain_size[i]; ++j) {
            var_lits.push_back(Glucose::Lit{lits[lits_offset++]});
        }
        g_lits.push_back(var_lits);
    }
    std::vector<std::vector<int>> g_supports;
    for (int i = 0; i < n_supports; ++i) {
        std::vector<int> s;
        for (int j = 0; j < n_vars; ++j) {
            s.push_back(supports[i * n_vars + j]);
        }
        g_supports.push_back(s);
    }
    return solver->addConstraint(std::make_unique<Glucose::DirectEncodingExtensionSupports>(std::move(g_lits), std::move(g_supports)));
}
#endif

int32_t Glucose_AddGraphDivision(Glucose::Solver* solver, int32_t n_vertices, const int32_t* dom_sizes, const int32_t* domains, const int32_t* dom_lits, int32_t n_edges, const int32_t* edges, const int32_t* edge_lits) {
    std::vector<Glucose::OptionalOrderEncoding> vertices;
    int domains_offset = 0;
    int dom_lits_offset = 0;
    for (int i = 0; i < n_vertices; ++i) {
        Glucose::OptionalOrderEncoding ooe;
        if (dom_sizes[i] > 0) {
            for (int j = 0; j < dom_sizes[i]; ++j) {
                ooe.values.push_back(domains[domains_offset++]);
            }
            for (int j = 1; j < dom_sizes[i]; ++j) {
                ooe.lits.push_back(Glucose::Lit{ dom_lits[dom_lits_offset++] });
            }
        }
        vertices.push_back(ooe);
    }

    std::vector<std::pair<int, int>> graph;
    for (int i = 0; i < n_edges; ++i) {
        graph.push_back({edges[i * 2], edges[i * 2 + 1]});
    }

    std::vector<Glucose::Lit> edge_lits_l;
    for (int i = 0; i < n_edges; ++i) {
        edge_lits_l.push_back(Glucose::Lit{ edge_lits[i] });
    }

    return solver->addConstraint(std::make_unique<Glucose::GraphDivision>(vertices, graph, edge_lits_l));
}

uint64_t Glucose_SolverStats_decisions(Glucose::Solver* solver) {
    return solver->decisions;
}

uint64_t Glucose_SolverStats_propagations(Glucose::Solver* solver) {
    return solver->propagations;
}

uint64_t Glucose_SolverStats_conflicts(Glucose::Solver* solver) {
    return solver->conflicts;
}

void Glucose_Set_random_seed(Glucose::Solver* solver, double random_seed) {
    solver->random_seed = random_seed;
}

void Glucose_Set_rnd_init_act(Glucose::Solver* solver, int32_t rnd_init_act) {
    solver->rnd_init_act = rnd_init_act != 0;
}

void Glucose_Set_dump_analysis_info(Glucose::Solver* solver, int32_t value) {
    solver->dump_analysis_info = value != 0;
}

int32_t Glucose_AddRustExtraConstraint(Glucose::Solver* solver, void* trait_object) {
    return solver->addConstraint(std::make_unique<Glucose::RustExtraConstraint>(trait_object)) ? 1 : 0;
}

void Glucose_CustomPropagatorCopyReason(void* reason_vec, int32_t n_lits, int32_t* lits) {
    Glucose::vec<Glucose::Lit>* v = static_cast<Glucose::vec<Glucose::Lit>*>(reason_vec);
    for (int i = 0; i < n_lits; ++i) {
        v->push(Glucose::Lit{lits[i]});
    }
}

int32_t Glucose_SolverValue(Glucose::Solver* solver, int32_t lit) {
    return Glucose::toInt(solver->value(Glucose::Lit{lit}));
}

void Glucose_SolverAddWatch(Glucose::Solver* solver, int32_t lit, void* wrapper_object) {
    solver->addWatch(Glucose::Lit{lit}, static_cast<Glucose::Constraint*>(wrapper_object));
}

int32_t Glucose_SolverEnqueue(Glucose::Solver* solver, int32_t lit, void* wrapper_object) {
    return solver->enqueue(Glucose::Lit{lit}, static_cast<Glucose::Constraint*>(wrapper_object)) ? 1 : 0;
}

int32_t Glucose_IsCurrentLevel(Glucose::Solver* solver, int32_t lit) {
    return solver->decisionLevel() == solver->level(Glucose::var(Glucose::Lit{lit})) ? 1 : 0;
}

}
