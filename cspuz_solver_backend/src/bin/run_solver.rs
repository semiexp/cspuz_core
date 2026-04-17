use cspuz_solver_backend::{decode_and_solve, Uniqueness};

fn main() {
    let mut url = None;
    let mut show_json = false;

    for arg in std::env::args().skip(1) {
        if arg == "--json" {
            show_json = true;
        } else if url.is_none() {
            url = Some(arg);
        } else {
            eprintln!("Unexpected argument: {}", arg);
            std::process::exit(1);
        }
    }

    if url.is_none() {
        eprintln!("Usage: run_solver [--json] <puzzle_url>");
        std::process::exit(1);
    }

    let url = url.unwrap();
    match decode_and_solve(url.as_bytes()) {
        Ok(board) => {
            if show_json {
                println!("{}", board.to_json());
            } else {
                let answer_status = match board.uniqueness {
                    Uniqueness::NoAnswer => "No Answer",
                    Uniqueness::Unique => "Unique",
                    Uniqueness::NonUnique => "Not unique",
                    _ => panic!(),
                };
                println!("Answer Status: {}", answer_status);
            }
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
