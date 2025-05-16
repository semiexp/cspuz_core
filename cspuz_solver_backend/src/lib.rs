#![allow(static_mut_refs)] // TODO: remove this

extern crate cspuz_rs;

pub mod board;
mod puzzle;
mod uniqueness;

use board::Board;
use cspuz_rs::serializer::{get_kudamono_url_info_detailed, url_to_puzzle_kind};

static mut SHARED_ARRAY: Vec<u8> = vec![];

fn solve_puzz_link(puzzle_kind: String, url: &str) -> Result<Board, &'static str> {
    if let Some(board) = puzzle::dispatch_puzz_link_puzzle(&puzzle_kind, url) {
        return board;
    }

    if puzzle_kind == "heyawake" {
        puzzle::heyawake::solve_heyawake(url, false)
    } else if puzzle_kind == "ayeheya" {
        puzzle::heyawake::solve_heyawake(url, true)
    } else {
        Err("unknown puzzle type")
    }
}

fn decode_and_solve(url: &[u8]) -> Result<Board, &'static str> {
    let url = std::str::from_utf8(url).map_err(|_| "failed to decode URL as UTF-8")?;

    let puzzle_kind = url_to_puzzle_kind(url).ok_or("puzzle type not detected");

    match puzzle_kind {
        Ok(puzzle_kind) => solve_puzz_link(puzzle_kind, url),
        Err(_) => {
            let puzzle_info = get_kudamono_url_info_detailed(url).ok_or("failed to parse URL")?;

            let puzzle_kind = *puzzle_info.get("G").unwrap_or(&"");
            let puzzle_variant = *puzzle_info.get("V").unwrap_or(&"");

            if puzzle_kind == "tricklayer" {
                puzzle::tricklayer::solve_tricklayer(url)
            } else if puzzle_kind == "parrot-loop" {
                puzzle::parrot_loop::solve_parrot_loop(url)
            } else if puzzle_kind == "crosswall" {
                puzzle::crosswall::solve_crosswall(url)
            } else if puzzle_kind == "soulmates" {
                puzzle::soulmates::solve_soulmates(url)
            } else if puzzle_kind == "cross-border-parity-loop" {
                puzzle::cross_border_parity_loop::solve_cross_border_parity_loop(url)
            } else if puzzle_kind == "akari-regional" {
                puzzle::akari_regions::solve_akari_regions(url)
            } else if puzzle_kind == "akari-rgb" {
                puzzle::akari_rgb::solve_akari_rgb(url)
            } else if puzzle_kind == "milk-tea" {
                puzzle::milktea::solve_milktea(url)
            } else if puzzle_kind == "seiza" {
                puzzle::seiza::solve_seiza(url)
            } else if puzzle_kind == "spokes" {
                puzzle::spokes::solve_spokes(url)
            } else if puzzle_kind == "kropki-pairs" {
                puzzle::kropki_pairs::solve_kropki_pairs(url)
            } else if puzzle_kind == "letter-weights" {
                puzzle::letter_weights::solve_letter_weights(url)
            } else if puzzle_kind == "sniping-arrow" {
                puzzle::sniping_arrow::solve_sniping_arrow(url)
            } else if puzzle_kind == "multiplication-link" {
                puzzle::multiplication_link::solve_multiplication_link(url)
            } else if puzzle_kind == "hidoku" {
                puzzle::hidato::solve_hidato(url)
            } else if puzzle_kind == "the-longest" {
                puzzle::the_longest::solve_the_longest(url)
            } else if puzzle_kind == "slicy" {
                puzzle::slicy::solve_slicy(url)
            } else if puzzle_kind == "lits" && puzzle_variant == "double" {
                puzzle::double_lits::solve_double_lits(url)
            } else {
                Err("unknown puzzle type")
            }
        }
    }
}

fn decode_and_enumerate(
    url: &[u8],
    num_max_answers: usize,
) -> Result<(Board, Vec<Board>), &'static str> {
    let url = std::str::from_utf8(url).map_err(|_| "failed to decode URL as UTF-8")?;

    let puzzle_kind = url_to_puzzle_kind(url).ok_or("puzzle type not detected")?;

    if puzzle_kind == "heyawake" {
        puzzle::heyawake::enumerate_answers_heyawake(url, num_max_answers)
    } else if puzzle_kind == "slither" || puzzle_kind == "slitherlink" {
        puzzle::slitherlink::enumerate_answers_slitherlink(url, num_max_answers)
    } else if puzzle_kind == "nurikabe" {
        puzzle::nurikabe::enumerate_answers_nurikabe(url, num_max_answers)
    } else if puzzle_kind == "curvedata" {
        puzzle::curvedata::enumerate_answers_curvedata(url, num_max_answers)
    } else {
        Err("unsupported puzzle type")
    }
}

#[no_mangle]
fn solve_problem(url: *const u8, len: usize) -> *const u8 {
    let url = unsafe { std::slice::from_raw_parts(url, len) };
    let result = decode_and_solve(url);

    let ret_string = match result {
        Ok(board) => {
            format!("{{\"status\":\"ok\",\"description\":{}}}", board.to_json())
        }
        Err(err) => {
            // TODO: escape `err` if necessary
            format!("{{\"status\":\"error\",\"description\":\"{}\"}}", err)
        }
    };

    let ret_len = ret_string.len();
    unsafe {
        SHARED_ARRAY.clear();
        SHARED_ARRAY.reserve(4 + ret_len);
        SHARED_ARRAY.push((ret_len & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 8) & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 16) & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 24) & 0xff) as u8);
        SHARED_ARRAY.extend_from_slice(ret_string.as_bytes());
        SHARED_ARRAY.as_ptr()
    }
}

#[no_mangle]
fn enumerate_answers_problem(url: *const u8, len: usize, num_max_answers: usize) -> *const u8 {
    let url = unsafe { std::slice::from_raw_parts(url, len) };
    let result = decode_and_enumerate(url, num_max_answers);

    let ret_string = match result {
        Ok((common, per_answer)) => {
            format!(
                "{{\"status\":\"ok\",\"description\":{{\"common\":{},\"answers\":[{}]}}}}",
                common.to_json(),
                per_answer
                    .iter()
                    .map(|x| x.to_json())
                    .collect::<Vec<_>>()
                    .join(",")
            )
        }
        Err(err) => {
            // TODO: escape `err` if necessary
            format!("{{\"status\":\"error\",\"description\":\"{}\"}}", err)
        }
    };

    let ret_len = ret_string.len();
    unsafe {
        SHARED_ARRAY.clear();
        SHARED_ARRAY.reserve(4 + ret_len);
        SHARED_ARRAY.push((ret_len & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 8) & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 16) & 0xff) as u8);
        SHARED_ARRAY.push(((ret_len >> 24) & 0xff) as u8);
        SHARED_ARRAY.extend_from_slice(ret_string.as_bytes());
        SHARED_ARRAY.as_ptr()
    }
}
