use super::IntegrationTester;
use crate::arithmetic::CmpOp;
use crate::csp::Stmt;
use crate::integration::*;
use crate::sat::GraphDivisionMode;
use std::collections::VecDeque;
use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum FuzzerLogEncodingMode {
    Never,
    Allow,
    Force,
}

#[derive(Debug, Clone, Copy)]
enum FuzzerGraphDivisionMode {
    None,
    CppImpl,
    RustImpl,
}

struct Fuzzer {
    random_state: u64,
}

const NON_ZERO_FALLBACK_SEED: u64 = 0x9e3779b97f4a7c15;
const XORSHIFT_SHIFT_A: u32 = 13;
const XORSHIFT_SHIFT_B: u32 = 7;
const XORSHIFT_SHIFT_C: u32 = 17;

impl Fuzzer {
    fn new(seed: u64) -> Self {
        Fuzzer {
            random_state: if seed == 0 {
                NON_ZERO_FALLBACK_SEED
            } else {
                seed
            },
        }
    }

    fn next_random(&mut self) -> u64 {
        self.random_state ^= self.random_state << XORSHIFT_SHIFT_A;
        self.random_state ^= self.random_state >> XORSHIFT_SHIFT_B;
        self.random_state ^= self.random_state << XORSHIFT_SHIFT_C;
        self.random_state
    }

    fn next_u32(&mut self, max: u32) -> u32 {
        assert!(0 < max);
        (((self.next_random() as u128) * (max as u128)) >> 64) as u32
    }

    fn next_i32(&mut self, low: i32, high: i32) -> i32 {
        assert!(low < high);
        let range = (high - low) as u32;
        self.next_u32(range) as i32 + low
    }

    fn run_single_trial(
        &mut self,
        num_bool_vars: usize,
        num_int_vars: usize,
        num_exprs: usize,
        max_complexity: u32,
        log_encoding_mode: FuzzerLogEncodingMode,
        graph_division_mode: FuzzerGraphDivisionMode,
        encode_only: bool,
    ) {
        let mut tester = IntegrationTester::with_config(Config {
            use_log_encoding: !matches!(log_encoding_mode, FuzzerLogEncodingMode::Never),
            force_use_log_encoding: matches!(log_encoding_mode, FuzzerLogEncodingMode::Force),
            graph_division_mode: match graph_division_mode {
                FuzzerGraphDivisionMode::RustImpl => GraphDivisionMode::Rust,
                _ => GraphDivisionMode::Cpp,
            },
            ..Config::default()
        });

        let mut bool_vars = vec![];
        for _ in 0..num_bool_vars {
            bool_vars.push(tester.new_bool_var());
        }

        let mut int_vars = vec![];
        let mut int_var_descs = vec![];
        let (domain_low, domain_high) = match graph_division_mode {
            FuzzerGraphDivisionMode::None => (-3, 4),
            FuzzerGraphDivisionMode::CppImpl | FuzzerGraphDivisionMode::RustImpl => (0, 5),
        };
        for _ in 0..num_int_vars {
            if self.next_u32(2) == 0 {
                let a = self.next_i32(domain_low, domain_high);
                let b = self.next_i32(domain_low, domain_high);
                int_vars.push(tester.new_int_var(Domain::range(a.min(b), a.max(b))));
                int_var_descs.push(format!("{}..{}", a.min(b), a.max(b)));
            } else {
                let mut domain = vec![];
                for n in domain_low..domain_high {
                    if self.next_u32(2) == 0 {
                        domain.push(n);
                    }
                }
                if domain.is_empty() {
                    domain.push(self.next_i32(domain_low, domain_high));
                }
                int_var_descs.push(format!("{:?}", domain));
                int_vars.push(tester.new_int_var_from_list(domain));
            }
        }

        let mut stmt_descs = vec![];
        if matches!(
            graph_division_mode,
            FuzzerGraphDivisionMode::CppImpl | FuzzerGraphDivisionMode::RustImpl
        ) {
            let n_division_stmts = self.next_i32(1, 2);
            for _ in 0..n_division_stmts {
                // TODO: test with non-simple cases
                let stmt =
                    self.random_graph_division_stmt(&bool_vars, &int_vars, max_complexity, true);
                let mut buf = vec![];
                let _ = stmt.pretty_print(&mut buf);
                stmt_descs.push(String::from_utf8(buf).unwrap_or_default());
                tester.add_constraint(stmt);
            }
        }

        for _ in 0..num_exprs {
            let stmt = self.random_stmt(&bool_vars, &int_vars, max_complexity);
            let mut buf = vec![];
            let _ = stmt.pretty_print(&mut buf);
            stmt_descs.push(String::from_utf8(buf).unwrap_or_default());
            tester.add_constraint(stmt);
        }

        if encode_only {
            let _ = tester.solver.encode();
            return;
        }

        if !tester.check_internal(true) {
            eprintln!("Fuzzer failed!");
            eprintln!("Num bool vars: {}", num_bool_vars);
            eprintln!("Int vars:");
            for desc in &int_var_descs {
                eprintln!("- {}", desc);
            }
            eprintln!("Statements:");
            for desc in &stmt_descs {
                eprintln!("- {}", desc);
            }
            panic!();
        }
    }

    fn random_stmt(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        max_complexity: u32,
    ) -> Stmt {
        // Weights: 0,1 = AllDifferent, 2 = ActiveVerticesConnected,
        // 3 (feature-gated) = ExtensionSupports, rest = BoolExpr
        #[cfg(feature = "csp-extra-constraints")]
        let mode = self.next_u32(11);
        #[cfg(not(feature = "csp-extra-constraints"))]
        let mode = self.next_u32(10);

        match mode {
            0 | 1 => self.random_alldifferent_stmt(bool_vars, int_vars, max_complexity),
            2 => self.random_active_vertices_connected_stmt(bool_vars, int_vars, max_complexity),
            #[cfg(feature = "csp-extra-constraints")]
            3 => self.random_extension_supports_stmt(bool_vars, int_vars),
            _ => {
                let complexity = self.next_u32(max_complexity);
                Stmt::Expr(self.random_bool_expr(bool_vars, int_vars, complexity))
            }
        }
    }

    fn random_alldifferent_stmt(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        max_complexity: u32,
    ) -> Stmt {
        if int_vars.is_empty() {
            let complexity = self.next_u32(max_complexity);
            return Stmt::Expr(self.random_bool_expr(bool_vars, int_vars, complexity));
        }
        // 2 to 5 expressions
        let n = (self.next_u32(4) as usize + 2).min(5);
        let exprs: Vec<IntExpr> = (0..n)
            .map(|_| {
                let c = self.next_u32(max_complexity / 2 + 1);
                self.random_int_expr(bool_vars, int_vars, c)
            })
            .collect();
        Stmt::AllDifferent(exprs)
    }

    fn random_active_vertices_connected_stmt(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        max_complexity: u32,
    ) -> Stmt {
        if bool_vars.is_empty() {
            let complexity = self.next_u32(max_complexity);
            return Stmt::Expr(self.random_bool_expr(bool_vars, int_vars, complexity));
        }
        // 2 to 6 vertices
        let n = (self.next_u32(5) as usize + 2).min(bool_vars.len()).max(2);
        let vertex_exprs: Vec<BoolExpr> = (0..n)
            .map(|_| {
                let c = self.next_u32(max_complexity / 2 + 1);
                self.random_bool_expr(bool_vars, int_vars, c)
            })
            .collect();
        let mut edges = vec![];
        for i in 0..n {
            for j in (i + 1)..n {
                if self.next_u32(3) != 0 {
                    edges.push((i, j));
                }
            }
        }
        Stmt::ActiveVerticesConnected(vertex_exprs, edges)
    }

    fn random_graph_division_stmt(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        max_complexity: u32,
        simple_only: bool,
    ) -> Stmt {
        let num_vertices = self.next_i32(4, 8) as usize;
        let num_edges_max = if simple_only { bool_vars.len() } else { 15 };
        let num_edges = self.next_i32(num_vertices as i32, num_edges_max as i32) as usize;

        let mut vertex_exprs: Vec<Option<IntExpr>> = vec![];
        if simple_only {
            let mut used_vars = vec![false; int_vars.len()];
            for _ in 0..num_vertices {
                if self.next_i32(0, 2) == 0 {
                    vertex_exprs.push(None);
                } else {
                    let i = self.next_u32(int_vars.len() as u32) as usize;
                    if used_vars[i] {
                        vertex_exprs.push(None);
                    } else {
                        used_vars[i] = true;
                        let v = int_vars[i].expr();
                        vertex_exprs.push(Some(v));
                    }
                }
            }
        } else {
            for _ in 0..num_vertices {
                if self.next_i32(0, 2) == 0 {
                    vertex_exprs.push(None);
                } else {
                    let v = int_vars[self.next_u32(int_vars.len() as u32) as usize].expr();
                    vertex_exprs.push(Some(v));
                }
            }
        }

        let mut edges = vec![];
        for _ in 0..num_edges {
            loop {
                let u = self.next_u32(num_vertices as u32) as usize;
                let v = self.next_u32(num_vertices as u32) as usize;
                if u != v {
                    edges.push((u, v));
                    break;
                }
            }
        }

        let mut edge_exprs: Vec<BoolExpr> = vec![];
        if simple_only {
            let mut used_vars = vec![false; bool_vars.len()];
            for _ in 0..num_edges {
                let idx = loop {
                    let idx = self.next_u32(bool_vars.len() as u32) as usize;
                    if !used_vars[idx] {
                        used_vars[idx] = true;
                        break idx;
                    }
                };
                let expr = bool_vars[idx].expr();
                edge_exprs.push(expr);
            }
        } else {
            for _ in 0..num_edges {
                let c = self.next_u32(max_complexity / 2 + 1);
                let expr = self.random_bool_expr(bool_vars, int_vars, c);
                edge_exprs.push(expr);
            }
        }

        let opts = Default::default();

        Stmt::GraphDivision(vertex_exprs, edges, edge_exprs, opts)
    }

    #[cfg(feature = "csp-extra-constraints")]
    fn random_extension_supports_stmt(
        &mut self,
        _bool_vars: &[BoolVar],
        int_vars: &[IntVar],
    ) -> Stmt {
        if int_vars.is_empty() {
            return Stmt::Expr(BoolExpr::Const(true));
        }
        // 1 to 3 variables in the scope
        let n = (self.next_u32(3) as usize + 1).min(int_vars.len());
        let exprs: Vec<IntExpr> = (0..n)
            .map(|_| {
                let idx = self.next_u32(int_vars.len() as u32) as usize;
                int_vars[idx].expr()
            })
            .collect();
        // 1 to 6 support tuples; values in -3..=3 to match int var domains
        let n_tuples = self.next_u32(6) as usize + 1;
        let mut supports = vec![];
        for _ in 0..n_tuples {
            let tuple: Vec<Option<i32>> = (0..n)
                .map(|_| {
                    if self.next_u32(5) == 0 {
                        None // wildcard
                    } else {
                        Some(self.next_i32(-3, 4))
                    }
                })
                .collect();
            supports.push(tuple);
        }
        Stmt::ExtensionSupports(exprs, supports)
    }

    fn random_bool_expr(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        complexity: u32,
    ) -> BoolExpr {
        if complexity == 0 {
            let idx = self.next_i32(-1, bool_vars.len() as i32);
            if idx < 0 {
                return BoolExpr::Const(self.next_u32(2) == 0);
            } else {
                return bool_vars[idx as usize].expr();
            }
        }

        let mode = self.next_u32(7);
        match mode {
            0 => BoolExpr::Not(Box::new(self.random_bool_expr(
                bool_vars,
                int_vars,
                complexity - 1,
            ))),
            1 | 2 | 3 | 4 | 5 => {
                let left_complexity = self.next_u32(complexity);
                let right_complexity = complexity - left_complexity - 1;

                let lhs = Box::new(self.random_bool_expr(bool_vars, int_vars, left_complexity));
                let rhs = Box::new(self.random_bool_expr(bool_vars, int_vars, right_complexity));

                match mode {
                    1 => BoolExpr::And(vec![lhs, rhs]),
                    2 => BoolExpr::Or(vec![lhs, rhs]),
                    3 => BoolExpr::Xor(lhs, rhs),
                    4 => BoolExpr::Iff(lhs, rhs),
                    5 => BoolExpr::Imp(lhs, rhs),
                    _ => unreachable!(),
                }
            }
            6 => {
                let left_complexity = self.next_u32(complexity);
                let right_complexity = complexity - left_complexity - 1;

                let op = match self.next_u32(6) {
                    0 => CmpOp::Eq,
                    1 => CmpOp::Ne,
                    2 => CmpOp::Le,
                    3 => CmpOp::Ge,
                    4 => CmpOp::Lt,
                    5 => CmpOp::Gt,
                    _ => unreachable!(),
                };

                let lhs = Box::new(self.random_int_expr(bool_vars, int_vars, left_complexity));
                let rhs = Box::new(self.random_int_expr(bool_vars, int_vars, right_complexity));

                BoolExpr::Cmp(op, lhs, rhs)
            }
            _ => unreachable!(),
        }
    }

    fn random_int_expr(
        &mut self,
        bool_vars: &[BoolVar],
        int_vars: &[IntVar],
        complexity: u32,
    ) -> IntExpr {
        if complexity == 0 {
            let idx = self.next_i32(-1, int_vars.len() as i32);
            if idx < 0 {
                return IntExpr::Const(self.next_i32(-3, 4));
            } else {
                return int_vars[idx as usize].expr();
            }
        }

        #[cfg(feature = "csp-extra-constraints")]
        let mode = self.next_u32(5);
        #[cfg(not(feature = "csp-extra-constraints"))]
        let mode = self.next_u32(4);

        match mode {
            0 => {
                let cond_complexity = self.next_u32(complexity);
                let t_complexity = self.next_u32(complexity - cond_complexity);
                let f_complexity = complexity - cond_complexity - t_complexity - 1;

                let cond = Box::new(self.random_bool_expr(bool_vars, int_vars, cond_complexity));
                let t_expr = Box::new(self.random_int_expr(bool_vars, int_vars, t_complexity));
                let f_expr = Box::new(self.random_int_expr(bool_vars, int_vars, f_complexity));

                IntExpr::If(cond, t_expr, f_expr)
            }
            1 => IntExpr::Abs(Box::new(self.random_int_expr(
                bool_vars,
                int_vars,
                complexity - 1,
            ))),
            2 => {
                let scale = self.next_i32(-3, 4);
                IntExpr::Linear(vec![(
                    Box::new(self.random_int_expr(bool_vars, int_vars, complexity - 1)),
                    scale,
                )])
            }
            3 => {
                let t1_complexity = self.next_u32(complexity);
                let t2_complexity = complexity - t1_complexity - 1;

                let t1 = Box::new(self.random_int_expr(bool_vars, int_vars, t1_complexity));
                let t2 = Box::new(self.random_int_expr(bool_vars, int_vars, t2_complexity));

                let scale1 = self.next_i32(-3, 4);
                let scale2 = self.next_i32(-3, 4);

                IntExpr::Linear(vec![(t1, scale1), (t2, scale2)])
            }
            #[cfg(feature = "csp-extra-constraints")]
            4 => {
                let left_complexity = self.next_u32(complexity);
                let right_complexity = complexity - left_complexity - 1;

                let lhs = Box::new(self.random_int_expr(bool_vars, int_vars, left_complexity));
                let rhs = Box::new(self.random_int_expr(bool_vars, int_vars, right_complexity));

                IntExpr::Mul(lhs, rhs)
            }
            _ => unreachable!(),
        }
    }
}

const DEFAULT_FUZZ_PARALLELISM: usize = 4;
const FUZZ_PARALLELISM_ENV: &str = "CSPUZ_FUZZ_PARALLELISM";

fn fuzz_parallelism() -> usize {
    env::var(FUZZ_PARALLELISM_ENV)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .filter(|v| *v > 0)
        .unwrap_or(DEFAULT_FUZZ_PARALLELISM)
}

fn generate_seeds(base_seed: u64, num_trials: usize) -> Vec<u64> {
    let mut seed_gen = Fuzzer::new(base_seed);
    (0..num_trials).map(|_| seed_gen.next_random()).collect()
}

#[derive(Debug, Clone, Copy)]
struct FuzzTrialConfig {
    mode: FuzzerLogEncodingMode,
    long_mode: bool,
    graph_division_mode: FuzzerGraphDivisionMode,
    encode_only: bool,
}

fn run_single_fuzz_trial(seed: u64, config: FuzzTrialConfig) {
    let FuzzTrialConfig {
        mode: log_encoding_mode,
        long_mode,
        graph_division_mode,
        encode_only,
    } = config;
    let mut fuzzer = Fuzzer::new(seed);
    let (num_bool_vars, num_int_vars, num_exprs, max_complexity) =
        match (log_encoding_mode, graph_division_mode, long_mode) {
            (_, FuzzerGraphDivisionMode::CppImpl, _)
            | (_, FuzzerGraphDivisionMode::RustImpl, _) => (
                10,
                fuzzer.next_i32(1, 3) as usize,
                fuzzer.next_i32(2, 5) as usize,
                5,
            ),
            (FuzzerLogEncodingMode::Force, _, false) => (
                fuzzer.next_i32(3, 6) as usize,
                fuzzer.next_i32(1, 4) as usize,
                fuzzer.next_i32(2, 8) as usize,
                7,
            ),
            (FuzzerLogEncodingMode::Force, _, true) => (
                fuzzer.next_i32(3, 6) as usize,
                fuzzer.next_i32(1, 4) as usize,
                fuzzer.next_i32(2, 12) as usize,
                7,
            ),
            (_, _, false) => (
                fuzzer.next_i32(3, 6) as usize,
                fuzzer.next_i32(1, 4) as usize,
                fuzzer.next_i32(2, 11) as usize,
                7,
            ),
            (_, _, true) => (
                fuzzer.next_i32(3, 7) as usize,
                fuzzer.next_i32(1, 5) as usize,
                fuzzer.next_i32(2, 12) as usize,
                10,
            ),
        };

    fuzzer.run_single_trial(
        num_bool_vars,
        num_int_vars,
        num_exprs,
        max_complexity,
        log_encoding_mode,
        graph_division_mode,
        encode_only,
    );
}

fn run_fuzz_trials_parallel(base_seed: u64, num_trials: usize, config: FuzzTrialConfig) {
    if num_trials == 0 {
        return;
    }
    let queue = Arc::new(Mutex::new(VecDeque::from(generate_seeds(
        base_seed, num_trials,
    ))));
    let num_workers = fuzz_parallelism().min(num_trials);
    let mut handles = vec![];

    for _ in 0..num_workers {
        let queue = Arc::clone(&queue);
        handles.push(thread::spawn(move || loop {
            let seed = {
                let mut queue = queue.lock().unwrap();
                queue.pop_front()
            };
            let Some(seed) = seed else {
                break;
            };
            run_single_fuzz_trial(seed, config);
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

#[test]
fn test_integration_fuzz_quick_without_log_encoding() {
    run_fuzz_trials_parallel(
        0x9f6abcde12345678,
        1000,
        FuzzTrialConfig {
            mode: FuzzerLogEncodingMode::Never,
            long_mode: false,
            graph_division_mode: FuzzerGraphDivisionMode::None,
            encode_only: false,
        },
    );
}

#[test]
fn test_integration_fuzz_quick_with_log_encoding() {
    run_fuzz_trials_parallel(
        0x3b1dd8e4a5f9c217,
        100,
        FuzzTrialConfig {
            mode: FuzzerLogEncodingMode::Force,
            long_mode: false,
            graph_division_mode: FuzzerGraphDivisionMode::None,
            encode_only: false,
        },
    );
}

#[test]
fn test_integration_fuzz_quick_with_log_encoding_encode_only() {
    run_fuzz_trials_parallel(
        0x79fa3908126dbec3,
        1000,
        FuzzTrialConfig {
            mode: FuzzerLogEncodingMode::Force,
            long_mode: false,
            graph_division_mode: FuzzerGraphDivisionMode::None,
            encode_only: true,
        },
    );
}

#[test]
fn test_integration_fuzz_quick_graph_division_cpp() {
    run_fuzz_trials_parallel(
        0x9f6abcde12345678,
        1000,
        FuzzTrialConfig {
            mode: FuzzerLogEncodingMode::Never,
            long_mode: false,
            graph_division_mode: FuzzerGraphDivisionMode::CppImpl,
            encode_only: false,
        },
    );
}

#[test]
fn test_integration_fuzz_quick_graph_division_rust() {
    run_fuzz_trials_parallel(
        0x9f6abcde12345678,
        1000,
        FuzzTrialConfig {
            mode: FuzzerLogEncodingMode::Never,
            long_mode: false,
            graph_division_mode: FuzzerGraphDivisionMode::RustImpl,
            encode_only: false,
        },
    );
}

#[test]
#[ignore] // This test can take a long time to run
fn test_integration_fuzz_long() {
    for (i, (mode, rep)) in [
        (FuzzerLogEncodingMode::Never, 100000),
        (FuzzerLogEncodingMode::Allow, 50000),
        (FuzzerLogEncodingMode::Force, 10000),
    ]
    .into_iter()
    .enumerate()
    {
        run_fuzz_trials_parallel(
            0x6ad0c8f1e2457b39 ^ i as u64,
            rep,
            FuzzTrialConfig {
                mode,
                long_mode: true,
                graph_division_mode: FuzzerGraphDivisionMode::None,
                encode_only: false,
            },
        );
    }
}
