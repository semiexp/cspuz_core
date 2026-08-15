use cspuz_rs::generator;
use cspuz_rs_puzzles::puzzles::slitherlink::{deserialize_problem, solve_slitherlink};

use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub struct SlitherlinkTask {
    pub problem: Vec<Vec<Option<i32>>>,
    pub expectation: Option<super::BoolGridEdgesIrrefutableFacts>,
}

pub(super) fn run_benchmark(
    task: &SlitherlinkTask,
) -> Result<super::BenchResult, super::BenchmarkError> {
    let problem = task.problem.clone();
    let expectation = task.expectation.as_ref().map(|e| e.to_cspuz_rs());

    let (answer, bench_result) =
        super::run_with_bench_result(|| solve_slitherlink(false, &problem));
    if answer != expectation {
        Err(super::BenchmarkError::AnswerMismatch)
    } else {
        Ok(bench_result)
    }
}

pub(super) fn materialize_solve_task(url: &str) -> super::Task {
    let (full, problem) = deserialize_problem(url).expect("Failed to deserialize problem");
    assert_eq!(full, false);

    let expectation = solve_slitherlink(false, &problem)
        .map(|ans| super::BoolGridEdgesIrrefutableFacts::from_cspuz_rs(&ans));
    super::Task::Slitherlink(SlitherlinkTask {
        problem,
        expectation,
    })
}

pub(super) fn materialize_generate_task(
    base: super::BaseGenerateTaskSet,
) -> super::GenerateTaskSet {
    assert_eq!(base.puzzle_type, "slitherlink");

    let height = base.height;
    let width = base.width;
    let pattern =
        vec![
            vec![
                generator::Choice::new(vec![None, Some(0), Some(1), Some(2), Some(3)], None);
                width
            ];
            height
        ];

    let trajectory = RefCell::new(vec![]);
    let solve_with_record = |problem: &Vec<Vec<Option<i32>>>| {
        let answer = solve_slitherlink(false, problem);

        trajectory
            .borrow_mut()
            .push((problem.clone(), answer.clone()));

        answer
    };

    let mut rng = rand::rngs::StdRng::seed_from_u64(base.seed);
    let _ = generator::Generator::new(
        solve_with_record,
        pattern,
        generator::default_uniqueness_checker(),
        generator::default_scorer(None, 5.0),
    )
    .generate(&mut rng);

    let tasks = trajectory
        .into_inner()
        .into_iter()
        .map(|(problem, answer)| {
            let expectation =
                answer.map(|ans| super::BoolGridEdgesIrrefutableFacts::from_cspuz_rs(&ans));
            super::Task::Slitherlink(SlitherlinkTask {
                problem,
                expectation,
            })
        })
        .collect::<Vec<_>>();
    super::GenerateTaskSet {
        name: base.name,
        puzzle_type: base.puzzle_type,
        tasks,
    }
}
