use cspuz_rs::graph;
use cspuz_rs::serializer::{
    problem_to_url_with_context, url_to_problem, Choice, Combinator, Context, ContextBasedGrid,
    HexInt, Optionalize, Rooms, Size, Spaces, Tuple2,
};
use cspuz_rs::solver::Solver;

pub fn solve_nanro(
    borders: &graph::InnerGridEdges<Vec<Vec<bool>>>,
    clues: &[Vec<Option<i32>>],
) -> Option<Vec<Vec<Option<i32>>>> {
    let (h, w) = borders.base_shape();
    let mut solver = Solver::new();

    let rooms = graph::borders_to_rooms(borders);
    let is_num = &solver.bool_var_2d((h, w));
    let mut ranges = vec![vec![(1, 1); w]; h];

    graph::active_vertices_connected_2d(&mut solver, is_num);

    solver.add_expr(!is_num.conv2d_and((2, 2)));


    for room in &rooms {
        for &(y, x) in room {
            ranges[y][x] = (-2, room.len() as i32);
        }
    }

    
    let num = &solver.int_var_2d_from_ranges((h, w), &ranges);
    solver.add_answer_key_int(num);

    solver.add_expr(num.ne(-1));
    solver.add_expr(num.ne(0));
    solver.add_expr(num.ge(1).iff(is_num));

    for y in 0..h {
        for x in 0..w {  
            if let Some(c) = clues[y][x] {
                if c == -1 {
                    solver.add_expr(num.at((y, x)).ne(-2));
                }
                else {
                    solver.add_expr(num.at((y, x)).eq(c));
                }
            }
        }
    }

    for room in &rooms {
        let cells = room.iter().map(|&p| is_black.at(p)).collect::<Vec<_>>();
        solver.add_expr(count_true(cells).ge(1));
    }

}