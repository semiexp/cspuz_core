use crate::board::{Board, BoardKind, Item, ItemKind};
use crate::uniqueness::{Uniqueness, UniquenessCheckable};
use cspuz_rs_puzzles::puzzles::pyramid_climbers;

pub fn solve(url: &str) -> Result<Board, &'static str> {
    let clues = pyramid_climbers::deserialize_problem(url).ok_or("invalid url")?;
    let ans = pyramid_climbers::solve_pyramid_climbers(&clues);
    let size = clues.len();

    let mut board = Board::new(
        BoardKind::Empty,
        size,
        size * 2,
        ans.as_ref()
            .map_or(Uniqueness::NoAnswer, |a| if a.concat().is_unique() { Uniqueness::Unique } else { Uniqueness::NonUnique }),
    );

    // Clues
    for y in 0..size {
        for x in 0..=y {
            board.push(Item {
                y: 2 * y + 1,
                x: (size - y - 1 + 2 * x + 1) * 2,
                color: "black",
                kind: ItemKind::TextString(clues[y][x].clone()),
            });
        }
    }

    // Borders
    for y in 0..=size {
        let start = if y == size { 0 } else { size - y - 1 };
        let end = if y == size { size * 2 } else { size + y + 1 };

        for x in start..end {
            board.push(Item {
                y: y * 2,
                x: x * 2 + 1,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }
    for y in 0..size {
        for x in 0..=(y + 1) {
            board.push(Item {
                y: y * 2 + 1,
                x: (size - y - 1 + 2 * x) * 2,
                color: "black",
                kind: ItemKind::BoldWall,
            });
        }
    }

    // Answers
    if let Some(ans) = &ans {
        for y in 0..(size - 1) {
            for x in 0..=y {
                match ans[y][x * 2] {
                    Some(true) => {
                        board.push(Item {
                            y: 2 * y + 1,
                            x: (size - y - 1 + 2 * x) * 2 + 2,
                            color: "green",
                            kind: ItemKind::LineTo(
                                (2 * y + 3) as i32,
                                ((size - y - 1 + 2 * x) * 2) as i32,
                            ),
                        });
                    }
                    Some(false) => {
                        board.push(Item {
                            y: 2 * y + 2,
                            x: (size - y - 1 + 2 * x) * 2 + 1,
                            color: "green",
                            kind: ItemKind::Cross,
                        });
                    }
                    None => (),
                }
                match ans[y][x * 2 + 1] {
                    Some(true) => {
                        board.push(Item {
                            y: 2 * y + 1,
                            x: (size - y - 1 + 2 * x) * 2 + 2,
                            color: "green",
                            kind: ItemKind::LineTo(
                                (2 * y + 3) as i32,
                                ((size - y - 1 + 2 * x + 2) * 2) as i32,
                            ),
                        });
                    }
                    Some(false) => {
                        board.push(Item {
                            y: 2 * y + 2,
                            x: (size - y - 1 + 2 * x + 1) * 2 + 1,
                            color: "green",
                            kind: ItemKind::Cross,
                        });
                    }
                    None => (),
                }
            }
        }
    }
    Ok(board)
}

#[cfg(test)]
mod tests {
    use super::solve;
    use crate::board::*;
    use crate::compare_board_and_check_no_solution_case;
    use crate::uniqueness::Uniqueness;

    #[test]
    #[rustfmt::skip]
    fn test_solve() {
        compare_board_and_check_no_solution_case!(
            solve("https://opt-pan.github.io/penpa-edit/#m=solve&p=vVRtb6pKEP7ur2j2aze5gCJKcj6g1b7c1toejbcSY1BRacHtQbAW0/72zgy2LGib3OTmhuw4PjM7b7vPPr+GTuDNeBU+XeEKV+Erg4ZL1Su0EMev50W+a5500z0nTd8LJm645lYcLUUIBm/1dCHEzHM557ftNp87/trlVw/L66awXs6sfza1aDhUz5X4Uhk8th9P74O/L71yqLY7te5N98bTFtZFs3FXbZ1Wu/G6H7mbu0BtPPaHvXl3sKhrr63OsJIMbxX9ajj/a2P1f5VsZVSymco402CpbPTOpiKYeOzdZr63csWW8fKotEvuzV0yNu3RG0/6mVrL1N/mDmTH3DFNZ6bNLAazwIiclSsInEsAebQzoEIeDQkgDxmoItDMAL1c8NAphpRWpxjyFopxlgFViiF5VCmG7EExZOAghoFAKwMMrVCHUazUoCxSWqOYxaAsclDK8jUxmLRK834g2SapkezBcfCkTPKMpEJSJ3lNPi2SA5JNkhWSVfIx8EBLJVvTeR3y4VK5QTL7Vbn+palwh+COsLXwx+s4nDtTd+xunWnEzPQOy5YctoqRAznIF+IZb96RCJ8mZkZhvMe8xUqEbmYpuLuzxXeR0JQD01ATEc4KJb04vp9v5U/shPnNUy+c+nkoCr3cfycMxUsOCZxomQMmTgSPxHrpPecjuavCLCMnX6Lz5BSyBdk43kpsy2jZZa7hce2SuplYPDmHSyWRnyd3QOgbM+kgn23GeA3uXhD7kTcVvoCUiKl0j2ijBmorUwdkR60JGgRVFdA7qQNuewA1ndT4OkW6pp30OMPcDdqNKgvEBopPa8P/6ZsEAB4yPUnQZDwTT/HeS0VKWFT83vmzA3T9oQMwf3aAatoBakc6wMb+kw7SV7XQQn30lh6U8q+e2//hKdjuuS3CH+idGYvwEZID+gPPJesx/BtOS9YifkBgLPaQw4AeoTGgRSYDdEhmAA/4DNg3lMaoRVZjVUViY6oDbmMqmd72qPQB&a=RY7BDQQhDAN74e3PksQpBtF/G/gw0kk8RpPxatfaWGMWovANjEhkmgppJ1F2EvUcUbxUAYYpQV8V8V0J9iVFPU2J9kJ5+8vK2wvl/Vvov/5vHw=="),
            Board {
                kind: BoardKind::Empty,
                height: 6,
                width: 12,
                data: vec![
                    Item { y: 1, x: 12, color: "black", kind: ItemKind::TextString("A".to_string()) },
                    Item { y: 3, x: 10, color: "black", kind: ItemKind::TextString("G".to_string()) },
                    Item { y: 3, x: 14, color: "black", kind: ItemKind::TextString("F".to_string()) },
                    Item { y: 5, x: 8, color: "black", kind: ItemKind::TextString("B".to_string()) },
                    Item { y: 5, x: 12, color: "black", kind: ItemKind::TextString("B".to_string()) },
                    Item { y: 5, x: 16, color: "black", kind: ItemKind::TextString("C".to_string()) },
                    Item { y: 7, x: 6, color: "black", kind: ItemKind::TextString("B".to_string()) },
                    Item { y: 7, x: 10, color: "black", kind: ItemKind::TextString("A".to_string()) },
                    Item { y: 7, x: 14, color: "black", kind: ItemKind::TextString("C".to_string()) },
                    Item { y: 7, x: 18, color: "black", kind: ItemKind::TextString("D".to_string()) },
                    Item { y: 9, x: 4, color: "black", kind: ItemKind::TextString("C".to_string()) },
                    Item { y: 9, x: 8, color: "black", kind: ItemKind::TextString("D".to_string()) },
                    Item { y: 9, x: 12, color: "black", kind: ItemKind::TextString("D".to_string()) },
                    Item { y: 9, x: 16, color: "black", kind: ItemKind::TextString("D".to_string()) },
                    Item { y: 9, x: 20, color: "black", kind: ItemKind::TextString("E".to_string()) },
                    Item { y: 11, x: 2, color: "black", kind: ItemKind::TextString("A".to_string()) },
                    Item { y: 11, x: 6, color: "black", kind: ItemKind::TextString("B".to_string()) },
                    Item { y: 11, x: 10, color: "black", kind: ItemKind::TextString("C".to_string()) },
                    Item { y: 11, x: 14, color: "black", kind: ItemKind::TextString("D".to_string()) },
                    Item { y: 11, x: 18, color: "black", kind: ItemKind::TextString("E".to_string()) },
                    Item { y: 11, x: 22, color: "black", kind: ItemKind::TextString("F".to_string()) },
                    Item { y: 0, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 0, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 4, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 6, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 8, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 10, x: 23, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 1, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 3, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 5, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 7, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 9, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 11, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 13, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 15, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 17, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 19, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 21, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 12, x: 23, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 1, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 3, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 5, x: 18, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 7, x: 20, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 2, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 6, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 10, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 14, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 18, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 9, x: 22, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 0, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 4, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 8, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 12, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 16, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 20, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 11, x: 24, color: "black", kind: ItemKind::BoldWall },
                    Item { y: 2, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 1, x: 12, color: "green", kind: ItemKind::LineTo(3, 14) },
                    Item { y: 3, x: 10, color: "green", kind: ItemKind::LineTo(5, 8) },
                    Item { y: 4, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 3, x: 14, color: "green", kind: ItemKind::LineTo(5, 12) },
                    Item { y: 4, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 6, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 8, color: "green", kind: ItemKind::LineTo(7, 10) },
                    Item { y: 6, x: 11, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 12, color: "green", kind: ItemKind::LineTo(7, 14) },
                    Item { y: 6, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 5, x: 16, color: "green", kind: ItemKind::LineTo(7, 18) },
                    Item { y: 7, x: 6, color: "green", kind: ItemKind::LineTo(9, 4) },
                    Item { y: 8, x: 7, color: "green", kind: ItemKind::Cross },
                    Item { y: 8, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 10, color: "green", kind: ItemKind::LineTo(9, 12) },
                    Item { y: 8, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 14, color: "green", kind: ItemKind::LineTo(9, 16) },
                    Item { y: 8, x: 17, color: "green", kind: ItemKind::Cross },
                    Item { y: 7, x: 18, color: "green", kind: ItemKind::LineTo(9, 20) },
                    Item { y: 9, x: 4, color: "green", kind: ItemKind::LineTo(11, 2) },
                    Item { y: 10, x: 5, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 8, color: "green", kind: ItemKind::LineTo(11, 6) },
                    Item { y: 10, x: 9, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 12, color: "green", kind: ItemKind::LineTo(11, 10) },
                    Item { y: 10, x: 13, color: "green", kind: ItemKind::Cross },
                    Item { y: 10, x: 15, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 16, color: "green", kind: ItemKind::LineTo(11, 18) },
                    Item { y: 10, x: 19, color: "green", kind: ItemKind::Cross },
                    Item { y: 9, x: 20, color: "green", kind: ItemKind::LineTo(11, 22) },
                ],
                uniqueness: Uniqueness::Unique,
            },
        );
    }
}
