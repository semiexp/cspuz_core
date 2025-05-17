use cspuz_solver_backend::{list_puzzles_for_enumerate, list_puzzles_for_solve};

fn main() {
    let puzzles_solve = list_puzzles_for_solve();
    let puzzles_enumerate = list_puzzles_for_enumerate();

    println!("{{");
    {
        println!("  \"solve\": [");
        for i in 0..puzzles_solve.len() {
            let (en_name, ja_name) = &puzzles_solve[i];
            print!(
                "    {{\n      \"en\": \"{}\",\n      \"ja\": \"{}\"\n    }}",
                en_name, ja_name
            );
            if i != puzzles_solve.len() - 1 {
                println!(",");
            } else {
                println!();
            }
        }
        println!("  ],");
    }
    {
        println!("  \"enumerate\": [");
        for i in 0..puzzles_enumerate.len() {
            let (en_name, ja_name) = &puzzles_enumerate[i];
            print!(
                "    {{\n      \"en\": \"{}\",\n      \"ja\": \"{}\"\n    }}",
                en_name, ja_name
            );
            if i != puzzles_enumerate.len() - 1 {
                println!(",");
            } else {
                println!();
            }
        }
        println!("  ]");
    }
    println!("}}");
}
