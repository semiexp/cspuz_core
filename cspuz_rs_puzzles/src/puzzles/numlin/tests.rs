use super::*;

fn problem_for_tests() -> Problem {
    vec![
        vec![None, None, None, None, Some(4), None],
        vec![None, Some(1), Some(4), None, None, None],
        vec![None, None, None, None, Some(2), None],
        vec![None, None, None, None, None, None],
        vec![None, Some(2), None, None, Some(1), Some(3)],
        vec![Some(3), None, None, None, None, None],
    ]
}

#[test]
fn test_numlin_problem() {
    let problem = problem_for_tests();
    let ans = enumerate_answers_numlin(&problem, 3);
    assert_eq!(ans.len(), 1);
    let ans = &ans[0];
    let expected = graph::BoolGridEdgesModel {
        horizontal: util::tests::to_bool_2d([
            [1, 1, 1, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 1, 1],
            [1, 0, 1, 1, 0],
            [1, 1, 1, 1, 1],
        ]),
        vertical: util::tests::to_bool_2d([
            [1, 0, 0, 1, 0, 1],
            [1, 1, 1, 0, 1, 1],
            [1, 1, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 1],
        ]),
    };
    assert_eq!(ans, &expected);
}

#[test]
fn test_numlin_serializer() {
    let problem = problem_for_tests();
    let url = "https://puzz.link/p?numlin/6/6/j4h14m2n2h133k";
    util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
}

fn answer_flat(answer: &graph::BoolGridEdgesModel) -> Vec<bool> {
    let mut result = vec![];
    let height = answer.horizontal.len();
    let width = answer.horizontal[0].len() + 1;

    for y in 0..height {
        for x in 0..(width - 1) {
            result.push(answer.horizontal[y][x]);
        }
    }
    for y in 0..(height - 1) {
        for x in 0..width {
            result.push(answer.vertical[y][x]);
        }
    }

    result
}

fn run_fuzz(height: usize, width: usize, seed: u64) {
    let problem =
        if let Some(problem) = instance_generator::generate_instance_by_csp(height, width, seed) {
            problem
        } else {
            return;
        };
    let csp_answers = enumerate_answers_numlin(&problem, usize::MAX);
    let no_propagator_answers = enumerate_answers_numlin_impl(&problem, usize::MAX, false);
    let numlin_answers = no_propagator_answers;

    assert_eq!(csp_answers.len(), numlin_answers.len());

    let mut csp_answers = csp_answers
        .iter()
        .map(|ans| answer_flat(ans))
        .collect::<Vec<_>>();
    csp_answers.sort();

    let mut numlin_answers = numlin_answers
        .iter()
        .map(|ans| answer_flat(ans))
        .collect::<Vec<_>>();
    numlin_answers.sort();

    assert_eq!(csp_answers, numlin_answers);
}

#[test]
fn test_numlin_fuzz_short() {
    for (height, width) in [(7, 8), (8, 7), (8, 8), (9, 9), (10, 10)] {
        let seed_start = height * 10000 + width * 100;
        for seed in 0..10 {
            run_fuzz(height, width, (seed_start + seed) as u64);
        }
    }
}

#[test]
#[ignore]
fn test_numlin_fuzz_long() {
    for (height, width) in [(7, 8), (8, 7), (8, 8), (9, 9), (10, 10)] {
        let seed_start = height * 100000 + width * 1000;
        for seed in 0..1000 {
            run_fuzz(height, width, (seed_start + seed) as u64);
        }
    }
}
