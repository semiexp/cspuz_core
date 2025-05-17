use cspuz_solver_backend::list_puzzles_for_solve;

fn main() {
    let puzzles = list_puzzles_for_solve();

    println!("[");
    for i in 0..puzzles.len() {
        let (en_name, ja_name) = &puzzles[i];
        print!(
            "  {{\n    \"en\": \"{}\",\n    \"ja\": \"{}\"\n  }}",
            en_name, ja_name
        );
        if i != puzzles.len() - 1 {
            println!(",");
        } else {
            println!();
        }
    }
    println!("]");
}
