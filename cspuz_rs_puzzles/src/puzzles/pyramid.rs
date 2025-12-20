use crate::penpa_editor::{decode_penpa_editor_url, Item, PenpaEditorPuzzle};
use cspuz_rs::solver::{any, Solver};

pub fn solve_pyramid(
    is_shaded: &[bool],
    clues: &[Vec<Option<i32>>],
    min_value: i32,
    max_value: i32,
) -> Option<Vec<Vec<Option<i32>>>> {
    let n = clues.len();

    let mut solver = Solver::new();
    let mut ans = vec![];
    for i in 0..n {
        ans.push(solver.int_var_1d(i + 1, min_value, max_value));
        solver.add_answer_key_int(&ans[i]);
    }

    for i in 0..n {
        assert_eq!(clues[i].len(), i + 1);
        for j in 0..=i {
            if let Some(n) = clues[i][j] {
                solver.add_expr(ans[i].at(j).eq(n));
            }
        }
    }

    for i in 0..n {
        if is_shaded[i] {
            for x1 in 0..=i {
                for x2 in 0..x1 {
                    solver.add_expr(ans[i].at(x1).ne(ans[i].at(x2)));
                }
            }
        } else {
            let mut cond = vec![];
            for x1 in 0..=i {
                for x2 in 0..x1 {
                    cond.push(ans[i].at(x1).eq(ans[i].at(x2)));
                }
            }
            solver.add_expr(any(cond));
        }
    }

    for i in 0..(n - 1) {
        for j in 0..=i {
            solver.add_expr(
                ans[i].at(j).eq(ans[i + 1].at(j) + ans[i + 1].at(j + 1))
                    | ans[i].at(j).eq(ans[i + 1].at(j) - ans[i + 1].at(j + 1))
                    | ans[i].at(j).eq(ans[i + 1].at(j + 1) - ans[i + 1].at(j)),
            );
        }
    }

    solver.irrefutable_facts().map(|f| {
        let mut result = vec![];
        for i in 0..n {
            result.push(f.get(&ans[i]));
        }
        result
    })
}

type Problem = (Vec<bool>, Vec<Vec<Option<i32>>>, i32, i32);

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let decoded = decode_penpa_editor_url(url).ok()?;
    let decoded = match decoded {
        PenpaEditorPuzzle::Pyramid(p) => p,
        _ => return None,
    };

    let size = decoded.size();
    let mut n_shaded = vec![0; size];
    let mut clues = vec![];
    for i in 0..size {
        clues.push(vec![None; i + 1]);
    }
    let mut min_value = 1;
    let mut max_value = 9;

    for item in decoded.get_outside() {
        if let Item::Text(text) = item {
            // Parses min-max value like "[1-9]" or "[2~8]"
            if let Some(s) = text
                .text
                .strip_prefix('[')
                .and_then(|s| s.strip_suffix(']'))
            {
                let parts: Vec<&str> = s.split(|c| c == '-' || c == '~' || c == '～').collect();
                if parts.len() == 2 {
                    if let Ok(min_v) = parts[0].parse::<i32>() {
                        if let Ok(max_v) = parts[1].parse::<i32>() {
                            min_value = min_v;
                            max_value = max_v;
                        }
                    }
                }
            }
        }
    }

    for y in 0..size {
        for x in 0..=y {
            let mut is_shaded = false;

            for item in decoded.get_cell(y, x) {
                if let &Item::Fill(fill) = item {
                    if fill == 1 || fill == 3 || fill == 8 {
                        is_shaded = true;
                    }
                } else if let Item::Text(text) = item {
                    if let Ok(n) = text.text.parse::<i32>() {
                        clues[y][x] = Some(n);
                    }
                }
            }

            n_shaded[y] += if is_shaded { 1 } else { 0 };
        }
    }

    let mut is_shaded = vec![false; size];
    for i in 0..size {
        if n_shaded[i] == 0 {
            is_shaded[i] = false;
        } else if n_shaded[i] == i + 1 {
            is_shaded[i] = true;
        } else {
            return None;
        }
    }

    Some((is_shaded, clues, min_value, max_value))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        // https://puzsq.logicpuzzle.app/puzzle/166721
        let is_shaded = vec![true, false, true, false];
        let clues = vec![
            vec![None],
            vec![None, None],
            vec![None, None, None],
            vec![Some(5), Some(3), Some(1), None],
        ];
        let min_value = 1;
        let max_value = 7;
        (is_shaded, clues, min_value, max_value)
    }

    #[test]
    fn test_pyramid_problem() {
        let (is_shaded, clues, min_value, max_value) = problem_for_tests();
        let ans = solve_pyramid(&is_shaded, &clues, min_value, max_value);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = vec![
            vec![Some(4)],
            vec![Some(2), Some(2)],
            vec![Some(2), Some(4), Some(6)],
            vec![Some(5), Some(3), Some(1), Some(5)],
        ];
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_exercise_serializer() {
        let problem = problem_for_tests();
        let url = "https://opt-pan.github.io/penpa-edit/?m=solve&p=vVNNb5tAEL3zK6o5byUDBjd7c9O6l9T9chVFK2StbRKjgHEXaFws57dnZsC1F1ypl1ZoH4/HwDyWedtfRmfJSoR4BAMxEC4ePjJaXjDkRTods6RMY/lKjKtynRskQnyaTMS9TovYUW7kKHBBgIfLhei5nj6z4EXOvv4q9/Vcqugg6u8n+uZEv8k9DAOQvoBw2JzaqxBPWDDFgpEPUkEAaJJaCBhhqQL/TMCHuOtR8H0uUe7rUcRqCNTOlXvEO8YJo8c4Qzei9hnfMQ4YA8YbrnnPeMt4zThkDLlmRN/jOGoYiCvsR8sVI0baItwMKPJ0XlTmXi9jkLx9grVNlS1iA7I0Vaukeb5Nk41dljxschNfvEVivHq4VL/IzYpefnbjSaepJRQ/Km3sh5eJWaa2VJrEutbG5E+WkulybQkLXeLsFOtka78p3pS2gVLbFvWj7nTLTt98cGAHvJQnvFB4uMH7+krWY1F/aMbgOI2i/oKz9lHWUxo1BSB8/p/thNJP/U1v+T6x63aWBsinyEPkSO+QNvsyv2mUz1LVMwHU5y0/TRSy/CdabXzQ9TLPFvgxCui/7VqxqFb5Y3WcWprNcccpNWidkunWKdHGKbGu0/ZT/p3Tq+jQbP/gL9PdRPg/RG/XJiw3F0OG8jFntnoxUK3eyxTqvfRQw36AUL2QIVS7MUKpnyQUe2FC7Q95ord2I0WuuqmiVr1gUavzbKnIeQE=&a=RcvBCUAxDALQXTx7+kQ7TMj+a6T5ORSEh4iZxReEGCAU/Ab9eJt3s+nLMYV5VQM=";
        assert_eq!(deserialize_problem(url).unwrap(), problem);
    }
}
