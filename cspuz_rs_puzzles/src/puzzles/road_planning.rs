use crate::penpa_editor::{decode_penpa_editor_url, Item, PenpaEditorPuzzle};
use cspuz_core::propagators::graph_division::GraphDivisionOptions;
use cspuz_rs::graph;
use cspuz_rs::solver::{count_true, Config, GraphDivisionMode, Solver};

pub fn solve_road_planning(
    clues: &[Vec<bool>],
) -> Option<graph::BoolInnerGridEdgesIrrefutableFacts> {
    let h = clues.len() - 1;
    let w = clues[0].len() - 1;

    let mut config = Config::default();
    config.graph_division_mode = GraphDivisionMode::Rust;

    let mut solver = Solver::with_config(config);
    let is_border = graph::BoolInnerGridEdges::new(&mut solver, (h, w));
    solver.add_answer_key_bool(&is_border.horizontal);
    solver.add_answer_key_bool(&is_border.vertical);

    let mut domain = vec![];
    for i in 1..=(h * w) {
        if h * w % i == 0 {
            domain.push(i as i32);
        }
    }

    let global_num = &solver.int_var_from_domain(domain.clone());
    let num = &solver.int_var_2d_from_domains((h, w), &vec![vec![domain; w]; h]);
    solver.add_expr(num.eq(global_num));

    let opts = GraphDivisionOptions {
        allow_extra_walls: true,
        ..Default::default()
    };
    graph::graph_division_2d_with_options(&mut solver, num, &is_border, opts);

    for y in 0..=h {
        for x in 0..=w {
            if (y == 0 || y == h) && (x == 0 || x == w) {
                if clues[y][x] {
                    return None;
                }
                continue;
            }

            if y == 0 {
                solver.add_expr(is_border.vertical.at((y, x - 1)).iff(clues[y][x]));
            } else if y == h {
                solver.add_expr(is_border.vertical.at((y - 1, x - 1)).iff(clues[y][x]));
            } else if x == 0 {
                solver.add_expr(is_border.horizontal.at((y - 1, x)).iff(clues[y][x]));
            } else if x == w {
                solver.add_expr(is_border.horizontal.at((y - 1, x - 1)).iff(clues[y][x]));
            } else {
                let adj = [
                    is_border.horizontal.at((y - 1, x)),
                    is_border.horizontal.at((y - 1, x - 1)),
                    is_border.vertical.at((y, x - 1)),
                    is_border.vertical.at((y - 1, x - 1)),
                ];
                if clues[y][x] {
                    solver.add_expr(count_true(&adj).ge(3));
                } else {
                    solver.add_expr(count_true(&adj).ne(1));
                    solver.add_expr(count_true(adj).le(2));
                }
            }
        }
    }

    solver.irrefutable_facts().map(|f| f.get(&is_border))
}

type Problem = Vec<Vec<bool>>;

pub fn deserialize_problem(url: &str) -> Option<Problem> {
    let decoded = decode_penpa_editor_url(url).ok()?;
    #[allow(unreachable_patterns)]
    let decoded = match decoded {
        PenpaEditorPuzzle::Square(s) => s,
        _ => return None,
    };

    let mut ret = vec![vec![false; decoded.width() + 1]; decoded.height() + 1];
    for y in 0..=decoded.height() {
        for x in 0..=decoded.width() {
            for item in decoded.get_vertex(y, x) {
                if let Item::Symbol(symbol) = item {
                    if symbol.name.starts_with("circle_") {
                        ret[y][x] = true;
                    }
                }
            }
        }
    }

    Some(ret)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn problem_for_tests() -> Problem {
        crate::util::tests::to_bool_2d([
            [0, 1, 1, 0, 0, 0],
            [0, 0, 0, 0, 0, 0],
            [0, 0, 1, 1, 1, 1],
            [1, 0, 0, 0, 0, 0],
            [0, 0, 0, 1, 0, 0],
        ])
    }

    #[test]
    fn test_road_planning_problem() {
        let problem = problem_for_tests();
        let ans = solve_road_planning(&problem);

        assert!(ans.is_some());
        let ans = ans.unwrap();

        let expected = graph::InnerGridEdges {
            horizontal: crate::util::tests::to_option_bool_2d([
                [0, 0, 1, 1, 0],
                [0, 1, 1, 1, 1],
                [1, 1, 0, 0, 0],
            ]),
            vertical: crate::util::tests::to_option_bool_2d([
                [1, 1, 0, 0],
                [1, 0, 0, 1],
                [0, 1, 1, 0],
                [0, 0, 1, 0],
            ]),
        };
        assert_eq!(ans, expected);
    }

    #[test]
    fn test_road_planning_serializer() {
        let problem = problem_for_tests();
        let url = "https://opt-pan.github.io/penpa-edit/#m=solve&p=tVRRb9owEH7Pr5j8fA+xgRb81nXrXjq2DqYKWREKkJaoAXdOslZG9Lf3fIkGJuZl2hT505fPF/vsuy/lrzo1GQygD70hxMDxEWIIfIT8ckAjbp9pXhWZ/ABXdbXWBgnAtzE8pEWZRaoNSiLFBAManCVvdvKmGAOeRDv7Q+7sXKpkD/bngQ4PdCJ3iGNCTjiTOzbsMak4sGVulkU2n0wYiATYsB+UeSzO6OFleHxuncEZfRTWBQ/oeIQbOoggnOI5wfYIPxHGhAPCW4r5THhPeE3YJ7ygmEt3U1GkRFMq92CRzjOsB149K3UxL2vzkC4zJqliQNq23iwy40mF1s9FvvXj8setNllwyonZ6jEUv9BmdbL6S1oUntA0oCc1N+hJlcm999QY/eIpm7Rae8IirbBdy3X+7K+UbSs/gSr1U0yf0pPdNocz7yP2ymgoAeICBHX2SNo7sF+k1/tg77C1v0o7c53duMAVWeGkaxms9B96T/OOXTcij5GPW450htTvOPtdKjsF5jb6SJ87yjb6N+ZK39H7Um8WeBrFju6jmSnrlX6q21juWvWqyXcSyLd3yNfRJl/HAvm65I7yvW0W+qfpjpJ9U4n4r/8r/8mar63btAkaDuWA51ANeqvVO/ZCvWMkt2HXS6gG7ITqqaNQ6poKxY6vUDtjLbfqqbtcVqcGc1t1POa2OrYZ/raIvQM=";
        assert_eq!(deserialize_problem(url).unwrap(), problem);
    }
}
