use crate::penpa_editor::{decode_penpa_editor_url, Item, PenpaEditorPuzzle};
use cspuz_rs::solver::Solver;

pub fn solve_pyramid_climbers(clues: &[Vec<String>]) -> Option<Vec<Vec<Option<bool>>>> {
    let n = clues.len();

    let mut solver = Solver::new();
    let mut ans = vec![];
    for i in 0..(n - 1) {
        ans.push(solver.bool_var_1d(2 * i + 2));
        solver.add_answer_key_bool(&ans[i]);
    }

    let mut seq = vec![];
    for i in 0..n {
        seq.push(solver.int_var_1d(i + 1, 0, n as i32 - 1));
    }

    for j in 0..n {
        solver.add_expr(seq[n - 1].at(j).eq(j as i32));
    }

    for i in 0..(n - 1) {
        for j in 0..=i {
            solver.add_expr(ans[i].at(j * 2) ^ ans[i].at(j * 2 + 1));
            if j > 0 {
                solver.add_expr(!(ans[i].at(j * 2) & ans[i].at(j * 2 - 1)));
            }
            solver.add_expr(ans[i].at(j * 2).imp(seq[i].at(j).eq(seq[i + 1].at(j))));
            solver.add_expr(
                ans[i]
                    .at(j * 2 + 1)
                    .imp(seq[i].at(j).eq(seq[i + 1].at(j + 1))),
            );
        }
    }

    for i1 in 0..(n - 1) {
        for i2 in (i1 + 1)..n {
            for j1 in 0..=i1 {
                for j2 in j1..=i2 {
                    if i2 - i1 < j2 - j1 {
                        continue;
                    }

                    if clues[i1][j1] == clues[i2][j2] {
                        solver.add_expr(seq[i1].at(j1).ne(seq[i2].at(j2)));
                    }
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| {
        let mut result = vec![];
        for i in 0..(n - 1) {
            result.push(f.get(&ans[i]));
        }
        result
    })
}

type Problem = Vec<Vec<String>>;

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let decoded = decode_penpa_editor_url(url).ok()?;
    let decoded = match decoded {
        PenpaEditorPuzzle::Pyramid(p) => p,
        _ => return None,
    };

    let size = decoded.size();

    let mut clues = vec![];
    for i in 0..size {
        clues.push(vec![String::new(); i + 1]);
    }

    for y in 0..size {
        for x in 0..=y {
            for item in decoded.get_cell(y, x) {
                if let Item::Text(text) = item {
                    clues[y][x] = text.text.clone();
                }
            }
        }
    }
    Some(clues)
}

#[cfg(test)]
mod tests {
    use crate::util;

    use super::*;

    fn problem_for_tests() -> Problem {
        // https://puzsq.logicpuzzle.app/puzzle/166598
        let base = vec![
            vec!["A"],
            vec!["G", "F"],
            vec!["B", "B", "C"],
            vec!["B", "A", "C", "D"],
            vec!["C", "D", "D", "D", "E"],
            vec!["A", "B", "C", "D", "E", "F"],
        ];
        base.iter()
            .map(|row| row.iter().map(|&s| s.to_string()).collect())
            .collect()
    }

    #[test]
    fn test_pyramid_climbers_problem() {
        let clues = problem_for_tests();
        let ans = solve_pyramid_climbers(&clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = util::tests::to_option_bool_2d(vec![
            vec![0, 1],
            vec![1, 0, 1, 0],
            vec![0, 1, 0, 1, 0, 1],
            vec![1, 0, 0, 1, 0, 1, 0, 1],
            vec![1, 0, 1, 0, 1, 0, 0, 1, 0, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_pyramid_climbers_serializer() {
        let problem = problem_for_tests();
        let url = "https://opt-pan.github.io/penpa-edit/#m=solve&p=vVRtb6pKEP7ur2j2aze5gCJKcj6g1b7c1toejbcSY1BRacHtQbAW0/72zgy2LGib3OTmhuw4PjM7b7vPPr+GTuDNeBU+XeEKV+Erg4ZL1Su0EMev50W+a5500z0nTd8LJm645lYcLUUIBm/1dCHEzHM557ftNp87/trlVw/L66awXs6sfza1aDhUz5X4Uhk8th9P74O/L71yqLY7te5N98bTFtZFs3FXbZ1Wu/G6H7mbu0BtPPaHvXl3sKhrr63OsJIMbxX9ajj/a2P1f5VsZVSymco402CpbPTOpiKYeOzdZr63csWW8fKotEvuzV0yNu3RG0/6mVrL1N/mDmTH3DFNZ6bNLAazwIiclSsInEsAebQzoEIeDQkgDxmoItDMAL1c8NAphpRWpxjyFopxlgFViiF5VCmG7EExZOAghoFAKwMMrVCHUazUoCxSWqOYxaAsclDK8jUxmLRK834g2SapkezBcfCkTPKMpEJSJ3lNPi2SA5JNkhWSVfIx8EBLJVvTeR3y4VK5QTL7Vbn+palwh+COsLXwx+s4nDtTd+xunWnEzPQOy5YctoqRAznIF+IZb96RCJ8mZkZhvMe8xUqEbmYpuLuzxXeR0JQD01ATEc4KJb04vp9v5U/shPnNUy+c+nkoCr3cfycMxUsOCZxomQMmTgSPxHrpPecjuavCLCMnX6Lz5BSyBdk43kpsy2jZZa7hce2SuplYPDmHSyWRnyd3QOgbM+kgn23GeA3uXhD7kTcVvoCUiKl0j2ijBmorUwdkR60JGgRVFdA7qQNuewA1ndT4OkW6pp30OMPcDdqNKgvEBopPa8P/6ZsEAB4yPUnQZDwTT/HeS0VKWFT83vmzA3T9oQMwf3aAatoBakc6wMb+kw7SV7XQQn30lh6U8q+e2//hKdjuuS3CH+idGYvwEZID+gPPJesx/BtOS9YifkBgLPaQw4AeoTGgRSYDdEhmAA/4DNg3lMaoRVZjVUViY6oDbmMqmd72qPQB&a=RY7BDQQhDAN74e3PksQpBtF/G/gw0kk8RpPxatfaWGMWovANjEhkmgppJ1F2EvUcUbxUAYYpQV8V8V0J9iVFPU2J9kJ5+8vK2wvl/Vvov/5vHw==";
        assert_eq!(deserialize_problem(url).unwrap(), problem);
    }
}
