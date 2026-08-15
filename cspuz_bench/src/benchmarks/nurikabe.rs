use cspuz_rs_puzzles::puzzles::nurikabe::{deserialize_problem, solve_nurikabe};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NurikabeTask {
    pub clues: Vec<Vec<Option<i32>>>,
    pub expectation: Option<Vec<Vec<Option<bool>>>>,
}

pub(super) fn run_benchmark(
    task: &NurikabeTask,
) -> Result<super::BenchResult, super::BenchmarkError> {
    let clues = task.clues.clone();
    let expectation = task.expectation.clone();

    let (answer, bench_result) = super::run_with_bench_result(|| solve_nurikabe(&clues));
    if answer != expectation {
        Err(super::BenchmarkError::AnswerMismatch)
    } else {
        Ok(bench_result)
    }
}

pub(super) fn materialize_solve_task(url: &str) -> super::Task {
    let clues = deserialize_problem(url).expect("Failed to deserialize problem");
    let expectation = solve_nurikabe(&clues);

    super::Task::Nurikabe(NurikabeTask { clues, expectation })
}
