use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context_pzprxs, url_to_problem, Choice, Combinator, Context,
    ContextBasedGrid, HexInt, Map, MultiDigit, Optionalize, Rooms, Size, Spaces, Tuple3,
};
use cspuz_rs::solver::Solver;

pub fn solve_twinarea(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
    is_gray: &[Vec<bool>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();
    let rooms = graph::borders_to_rooms(borders);
    let max_number = rooms.iter().map(|room| room.len()).max().unwrap() as i32;

    let mut solver = Solver::new();
    // Zero represents an empty cell.
    let num = &solver.int_var_2d((h, w), 0, max_number);
    solver.add_answer_key_int(num);

    for room in &rooms {
        let room_nums = num.select(room);
        solver.add_expr(room_nums.eq(room.len() as i32) | room_nums.eq(0));
        solver.add_expr(room_nums.ne(0).count_true().eq(1));
    }

    solver.add_expr(!(num.slice((..(h - 1), ..)).ne(0) & num.slice((1.., ..)).ne(0)));
    solver.add_expr(!(num.slice((.., ..(w - 1))).ne(0) & num.slice((.., 1..)).ne(0)));

    for y in 0..h {
        for x in 0..w {
            if let Some(n) = clues[y][x] {
                solver.add_expr(num.at((y, x)).eq(n));
            }
        }
    }

    let mut visited = vec![vec![false; w]; h];
    for y in 0..h {
        for x in 0..w {
            if !is_gray[y][x] || visited[y][x] {
                continue;
            }
            let mut component = vec![];
            let mut stack = vec![(y, x)];
            visited[y][x] = true;
            while let Some((cy, cx)) = stack.pop() {
                component.push((cy, cx));
                for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
                    let ny = cy as i32 + dy;
                    let nx = cx as i32 + dx;
                    if ny < 0 || ny >= h as i32 || nx < 0 || nx >= w as i32 {
                        continue;
                    }
                    let (ny, nx) = (ny as usize, nx as usize);
                    if is_gray[ny][nx] && !visited[ny][nx] {
                        visited[ny][nx] = true;
                        stack.push((ny, nx));
                    }
                }
            }
            // A positive sum also requires at least one number in the component.
            solver.add_expr(num.select(&component).sum().eq(component.len() as i32));
        }
    }

    solver.irrefutable_facts().map(|f| f.get(num))
}

pub type Problem = (
    graph::InnerGridEdges<Vec<Vec<bool>>>,
    Vec<Vec<Option<i32>>>,
    Vec<Vec<bool>>,
);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple3::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
        ])),
        ContextBasedGrid::new(Map::new(
            MultiDigit::new(2, 5),
            |x| Some(if x { 1 } else { 0 }),
            |x| Some(x == 1),
        )),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let (h, w) = problem.0.base_shape();
    problem_to_url_with_context_pzprxs(
        combinator(),
        "twinarea",
        problem.clone(),
        &Context::sized(h, w),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["twinarea"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        (
            graph::InnerGridEdges {
                horizontal: crate::util::tests::to_bool_2d([
                    [1, 1, 0, 0, 0],
                    [0, 1, 1, 0, 1],
                    [1, 0, 0, 1, 0],
                ]),
                vertical: crate::util::tests::to_bool_2d([
                    [0, 1, 1, 0],
                    [0, 1, 1, 0],
                    [1, 1, 1, 1],
                    [0, 1, 1, 0],
                ]),
            },
            vec![vec![None; 5]; 4],
            crate::util::tests::to_bool_2d([
                [0, 1, 1, 0, 1],
                [0, 1, 1, 0, 1],
                [1, 0, 0, 0, 1],
                [1, 1, 1, 1, 0],
            ]),
        )
    }

    #[test]
    fn test_twinarea_problem() {
        let (borders, clues, is_gray) = problem_for_tests();
        let expected = crate::util::tests::to_option_2d([
            [0, 2, 0, 5, 0],
            [3, 0, 2, 0, 0],
            [0, 0, 0, 0, 3],
            [3, 0, 2, 0, 0],
        ]);
        assert_eq!(solve_twinarea(&borders, &clues, &is_gray), Some(expected));
    }

    #[test]
    fn test_twinarea_serializer() {
        crate::util::tests::serializer_test(
            problem_for_tests(),
            "https://pzprxs.vercel.app/p?twinarea/5/4/crr0odizddhu",
            serialize_problem,
            deserialize_problem,
        );
    }
}
