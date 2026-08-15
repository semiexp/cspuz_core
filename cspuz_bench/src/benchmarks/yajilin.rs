use cspuz_rs::generator::{self, DefaultScorableAnswer};
use cspuz_rs::graph::BoolGridEdgesIrrefutableFacts as CspuzRsBoolGridEdgesIrrefutableFacts;
use cspuz_rs::items::{Arrow, NumberedArrow};
use cspuz_rs_puzzles::puzzles::yajilin::{deserialize_problem, solve_yajilin};
use rand::SeedableRng;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;

#[derive(Serialize, Deserialize)]
pub enum YajilinDirection {
    Unspecified,
    Up,
    Down,
    Left,
    Right,
}

impl YajilinDirection {
    fn to_cspuz_rs(&self) -> Arrow {
        match self {
            YajilinDirection::Unspecified => Arrow::Unspecified,
            YajilinDirection::Up => Arrow::Up,
            YajilinDirection::Down => Arrow::Down,
            YajilinDirection::Left => Arrow::Left,
            YajilinDirection::Right => Arrow::Right,
        }
    }

    fn from_cspuz_rs(direction: Arrow) -> Self {
        match direction {
            Arrow::Unspecified => YajilinDirection::Unspecified,
            Arrow::Up => YajilinDirection::Up,
            Arrow::Down => YajilinDirection::Down,
            Arrow::Left => YajilinDirection::Left,
            Arrow::Right => YajilinDirection::Right,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct YajilinIrrefutableFacts {
    pub is_line: super::BoolGridEdgesIrrefutableFacts,
    pub is_black: Vec<Vec<Option<bool>>>,
}

#[derive(Serialize, Deserialize)]
pub struct YajilinTask {
    pub outside: bool,
    pub clues: Vec<Vec<Option<(YajilinDirection, i32)>>>,
    pub expectation: Option<YajilinIrrefutableFacts>,
}

#[derive(Clone)]
struct YajilinAnswer {
    is_line: CspuzRsBoolGridEdgesIrrefutableFacts,
    is_black: Vec<Vec<Option<bool>>>,
}

impl DefaultScorableAnswer for YajilinAnswer {
    fn score(&self) -> f64 {
        self.is_line.score() + self.is_black.score()
    }

    fn fully_solved(&self) -> bool {
        self.is_line.fully_solved() && self.is_black.fully_solved()
    }
}

fn clues_to_cspuz_rs(
    clues: &[Vec<Option<(YajilinDirection, i32)>>],
) -> Vec<Vec<Option<NumberedArrow>>> {
    clues
        .iter()
        .map(|row| {
            row.iter()
                .map(|clue| {
                    clue.as_ref()
                        .map(|(direction, number)| (direction.to_cspuz_rs(), *number))
                })
                .collect()
        })
        .collect()
}

fn clues_from_cspuz_rs(
    clues: &[Vec<Option<NumberedArrow>>],
) -> Vec<Vec<Option<(YajilinDirection, i32)>>> {
    clues
        .iter()
        .map(|row| {
            row.iter()
                .map(|clue| {
                    clue.map(|(direction, number)| {
                        (YajilinDirection::from_cspuz_rs(direction), number)
                    })
                })
                .collect()
        })
        .collect()
}

pub(super) fn run_benchmark(
    task: &YajilinTask,
) -> Result<super::BenchResult, super::BenchmarkError> {
    let clues = clues_to_cspuz_rs(&task.clues);
    let expectation = task.expectation.as_ref().map(|expectation| {
        (
            expectation.is_line.to_cspuz_rs(),
            expectation.is_black.clone(),
        )
    });

    let (answer, bench_result) =
        super::run_with_bench_result(|| solve_yajilin(task.outside, &clues));
    if answer != expectation {
        Err(super::BenchmarkError::AnswerMismatch)
    } else {
        Ok(bench_result)
    }
}

pub(super) fn materialize_solve_task(url: &str) -> super::Task {
    let (outside, clues) = deserialize_problem(url).expect("Failed to deserialize problem");
    let expectation =
        solve_yajilin(outside, &clues).map(|(is_line, is_black)| YajilinIrrefutableFacts {
            is_line: super::BoolGridEdgesIrrefutableFacts::from_cspuz_rs(&is_line),
            is_black,
        });

    super::Task::Yajilin(YajilinTask {
        outside,
        clues: clues_from_cspuz_rs(&clues),
        expectation,
    })
}

fn clue_candidates(y: usize, x: usize, height: usize, width: usize) -> Vec<Option<NumberedArrow>> {
    let directions_and_lengths = [
        (Arrow::Up, y),
        (Arrow::Down, height - y - 1),
        (Arrow::Left, x),
        (Arrow::Right, width - x - 1),
    ];

    let mut candidates = vec![None];
    for (direction, length) in directions_and_lengths {
        for number in 0..=((length + 1) / 2) {
            candidates.push(Some((direction, number as i32)));
        }
    }
    candidates
}

pub(super) fn materialize_generate_task(
    base: super::BaseGenerateTaskSet,
) -> super::GenerateTaskSet {
    assert_eq!(base.puzzle_type, "yajilin");

    let pattern = (0..base.height)
        .map(|y| {
            (0..base.width)
                .map(|x| {
                    generator::Choice::new(clue_candidates(y, x, base.height, base.width), None)
                })
                .collect()
        })
        .collect::<Vec<Vec<_>>>();

    let trajectory = RefCell::new(vec![]);
    let solve_with_record = |problem: &Vec<Vec<Option<NumberedArrow>>>| {
        let answer = solve_yajilin(false, problem)
            .map(|(is_line, is_black)| YajilinAnswer { is_line, is_black });
        trajectory
            .borrow_mut()
            .push((problem.clone(), answer.clone()));
        answer
    };

    let mut rng = rand::rngs::StdRng::seed_from_u64(base.seed);
    let score_is_line = generator::default_scorer(None, 20.0);
    let _ = generator::Generator::new(
        solve_with_record,
        pattern,
        generator::default_uniqueness_checker(),
        |problem, answer: &YajilinAnswer| score_is_line(problem, &answer.is_line),
    )
    .generate(&mut rng);

    let tasks = trajectory
        .into_inner()
        .into_iter()
        .map(|(clues, answer)| {
            let expectation = answer.map(|answer| YajilinIrrefutableFacts {
                is_line: super::BoolGridEdgesIrrefutableFacts::from_cspuz_rs(&answer.is_line),
                is_black: answer.is_black,
            });
            super::Task::Yajilin(YajilinTask {
                outside: false,
                clues: clues_from_cspuz_rs(&clues),
                expectation,
            })
        })
        .collect();

    super::GenerateTaskSet {
        name: base.name,
        puzzle_type: base.puzzle_type,
        tasks,
    }
}
