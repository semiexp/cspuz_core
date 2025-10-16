use super::IntegrationTester;
use crate::arithmetic::CmpOp;
use crate::integration::*;

struct Fuzzer {
    random_state: u64,
}

impl Fuzzer {
    fn new() -> Self {
        Fuzzer {
            random_state: 0x123456789abcdef,
        }
    }

    fn next_random(&mut self) -> u64 {
        self.random_state = self.random_state.wrapping_mul(0x123456789);
        self.random_state
    }

    fn next_u32(&mut self, max: u32) -> u32 {
        assert!(0 < max);
        ((self.next_random() >> 16) % (max as u64)) as u32
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
    ) {
        let mut tester = IntegrationTester::with_config(Config {
            use_log_encoding: false, // do not use log encoding because negative numbers are not supported
            ..Config::default()
        });

        let mut bool_vars = vec![];
        for _ in 0..num_bool_vars {
            bool_vars.push(tester.new_bool_var());
        }

        let mut int_vars = vec![];
        let mut int_var_descs = vec![];
        for _ in 0..num_int_vars {
            if self.next_u32(2) == 0 {
                let a = self.next_i32(-3, 4);
                let b = self.next_i32(-3, 4);
                int_vars.push(tester.new_int_var(Domain::range(a.min(b), a.max(b))));
                int_var_descs.push(format!("{}..{}", a.min(b), a.max(b)));
            } else {
                let mut domain = vec![];
                for n in -3..=3 {
                    if self.next_u32(2) == 0 {
                        domain.push(n);
                    }
                }
                if domain.is_empty() {
                    domain.push(self.next_i32(-3, 4));
                }
                int_var_descs.push(format!("{:?}", domain));
                int_vars.push(tester.new_int_var_from_list(domain));
            }
        }

        let mut exprs = vec![];
        for _ in 0..num_exprs {
            let complexity = self.next_u32(max_complexity);

            let expr = self.random_bool_expr(&bool_vars, &int_vars, complexity);
            exprs.push(expr.clone());
            tester.add_expr(expr);
        }

        if !tester.check_internal(true) {
            eprintln!("Fuzzer failed!");
            eprintln!("Num bool vars: {}", num_bool_vars);
            eprintln!("Int vars:");
            for desc in &int_var_descs {
                eprintln!("- {}", desc);
            }
            eprintln!("Expressions:");
            for expr in exprs {
                eprint!("- ");
                let mut out_buf = vec![];
                let _ = expr.pretty_print(&mut out_buf);
                eprint!("{}", String::from_utf8(out_buf).unwrap());
                eprintln!();
            }
            panic!();
        }
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
            _ => unreachable!(),
        }
    }
}

#[test]
fn test_integration_fuzz_quick() {
    let mut fuzzer = Fuzzer::new();
    for _ in 0..1000 {
        let num_bool_vars = fuzzer.next_i32(3, 6) as usize;
        let num_int_vars = fuzzer.next_i32(1, 4) as usize;
        let num_exprs = fuzzer.next_i32(2, 11) as usize;
        let max_complexity = 7;

        fuzzer.run_single_trial(num_bool_vars, num_int_vars, num_exprs, max_complexity);
    }
}

#[test]
#[ignore] // This test can take a long time to run
fn test_integration_fuzz_long() {
    let mut fuzzer = Fuzzer::new();
    for _ in 0..100000 {
        let num_bool_vars = fuzzer.next_i32(3, 7) as usize;
        let num_int_vars = fuzzer.next_i32(1, 5) as usize;
        let num_exprs = fuzzer.next_i32(2, 12) as usize;
        let max_complexity = 10;

        fuzzer.run_single_trial(num_bool_vars, num_int_vars, num_exprs, max_complexity);
    }
}
