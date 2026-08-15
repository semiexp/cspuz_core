use cspuz_rs::items::{Arrow, NumberedArrow};
use cspuz_rs_puzzles::puzzles::yajilin::{deserialize_problem, solve_yajilin};
use serde::{Deserialize, Serialize};

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
