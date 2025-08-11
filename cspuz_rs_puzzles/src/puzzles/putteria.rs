use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    HexInt, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_putteria(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();

    let rooms = graph::borders_to_rooms(borders);
    let mut ranges = vec![vec![1; w]; h];
    let max_number = 0;

    for room in &rooms { // Make sure rooms can only content one type of number
        for &(y, x) in room {
            ranges[y][x] = (room.len() as i32);
            if max_number < room.len() as i32 {
                max_number = room.len();
            }
        }
    }

    let mut solver = Solver::new();
    let num = &solver.int_var_2d_from_ranges((h, w), (0, max_number));
    solver.add_answer_key_int(num);


    // Check no duplicates in rows
    for i in 0..h {
        for j in 1..max_number {  
        solver.add_expr(num.slice_fixed_x((.., i)).eq(j).count_true().le(1) );
        }
    }

    for i in 0..w {
        for j in 1..max_number {
            solver.add_expr(num.slice_fixed_y((i, ..)).eq(j).count_true().le(1));
        }
    }
    
    // Check no adjacent
    solver.add_expr(!(num.slice((..(height - 1), ..)).ne(0) & num.slice((1.., ..)).ne(0)));
    solver.add_expr(!(num.slice((.., ..(height - 1))).ne(0) & num.slice((.., 1..)).ne(0)));

    // Check no duplicate in rooms
   for room in &rooms {
        let room_nums = num.select(room);
        solver.add_expr(room_nums.eq(room.len() as i32).count_true().eq(1)); // One cell has the number 
        solver.add_expr(room_nums.eq(0).count_true().eq(room.len() - 1)); // The rest are empty
        
    }

    for y in 0..h {
        for x in 0..w {
            if let Some(c) = clues[y][x] {
                solver.add_expr(num.at((y, x)).eq(c));
            }
        }
    }

    

    solver.irrefutable_facts().map(|f| f.get(num))
}

pub type Problem = (graph::InnerGridEdges<Vec<Vec<bool>>>, Vec<Vec<Option<i32>>>);

fn combinator() -> impl Combinator<Problem> {
    Size::new(Tuple2::new(
        Rooms,
        ContextBasedGrid::new(Choice::new(vec![
            Box::new(Optionalize::new(HexInt)),
            Box::new(Spaces::new(None, 'g')),
        ])),
    ))
}

pub fn serialize_problem(problem: &Problem) -> Option<String> {
    let height = problem.0.vertical.len();
    let width = problem.0.vertical[0].len() + 1;
    problem_to_url_with_context(
        combinator(),
        "putteria",
        problem.clone(),
        &Context::sized(height, width),
    )
}

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    url_to_problem(combinator(), &["putteria"], url)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        let borders = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_bool_2d([
                [0, 0, 0, 1, 0],
                [1, 1, 1, 0, 0],
                [1, 1, 1, 1, 0],
            ]),
            vertical: crate::util::tests::to_bool_2d([
                [1, 0, 1, 0],
                [1, 0, 1, 1],
                [0, 1, 0, 1],
                [0, 0, 0, 1],
            ]),
        };

        let clues = vec![
            vec![None, Some(4), None, Some(3), None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, None],
            vec![None, None, None, None, Some(1)],
        ];

        (borders, clues)
    }

    #[test]
    fn test_ripple_problem() {
        let (borders, clues) = problem_for_tests();
        let ans = solve_ripple(&borders, &clues);
        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = crate::util::tests::to_option_2d([
            [1, 4, 2, 3, 5],
            [2, 3, 1, 2, 4],
            [1, 2, 3, 1, 2],
            [3, 1, 2, 4, 1],
        ]);
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_ripple_serializer() {
        let problem = problem_for_tests();
        let url = "https://puzz.link/p?ripple/5/4/ld8g2sug4g3u1";
        crate::util::tests::serializer_test(problem, url, serialize_problem, deserialize_problem);
    }
}
