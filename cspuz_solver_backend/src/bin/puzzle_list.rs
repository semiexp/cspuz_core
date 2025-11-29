use cspuz_solver_backend::{
    list_penpa_edit_puzzles, list_puzzles_for_enumerate, list_puzzles_for_solve,
};

fn main() {
    let puzzles_solve = list_puzzles_for_solve();
    let puzzles_enumerate = list_puzzles_for_enumerate();
    let puzzles_penpa_edit = list_penpa_edit_puzzles();

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
        println!("  ],");
    }
    {
        println!("  \"penpa_edit\": [");
        for i in 0..puzzles_penpa_edit.len() {
            let (key, en_name, ja_name) = &puzzles_penpa_edit[i];
            print!(
                "    {{\n      \"key\": \"{}\",\n      \"en\": \"{}\",\n      \"ja\": \"{}\"\n    }}",
                key, en_name, ja_name
            );
            if i != puzzles_penpa_edit.len() - 1 {
                println!(",");
            } else {
                println!();
            }
        }
        println!("  ]");
    }
    println!("}}");
}
