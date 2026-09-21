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
