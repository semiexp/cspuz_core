use cspuz_rs_puzzles::puzzles::dbchoco::{deserialize_problem, solve_doublechoco};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DoubleChocoTask {
    pub color: Vec<Vec<i32>>,
    pub num: Vec<Vec<Option<i32>>>,
    pub expectation: Option<super::BoolInnerGridEdgesIrrefutableFacts>,
}

pub(super) fn run_benchmark(
    task: &DoubleChocoTask,
) -> Result<super::BenchResult, super::BenchmarkError> {
    let color = task.color.clone();
    let num = task.num.clone();
    let expectation = task.expectation.as_ref().map(|e| e.to_cspuz_rs());

    let (answer, bench_result) = super::run_with_bench_result(|| solve_doublechoco(&color, &num));
    if answer != expectation {
        Err(super::BenchmarkError::AnswerMismatch)
    } else {
        Ok(bench_result)
    }
}

pub(super) fn materialize_solve_task(url: &str) -> super::Task {
    let (color, num) = deserialize_problem(url).expect("Failed to deserialize problem");
    let expectation = solve_doublechoco(&color, &num)
        .as_ref()
        .map(super::BoolInnerGridEdgesIrrefutableFacts::from_cspuz_rs);

    super::Task::DoubleChoco(DoubleChocoTask {
        color,
        num,
        expectation,
    })
}
