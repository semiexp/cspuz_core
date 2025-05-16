#![allow(static_mut_refs)] // TODO: remove this

extern crate cspuz_rs;

pub mod board;
mod puzzle;
mod uniqueness;

use board::Board;
use cspuz_rs::serializer::{get_kudamono_url_info_detailed, url_to_puzzle_kind};

static mut SHARED_ARRAY: Vec<u8> = vec![];

fn decode_and_solve(url: &[u8]) -> Result<Board, &'static str> {
    let url = std::str::from_utf8(url).map_err(|_| "failed to decode URL as UTF-8")?;

    let puzzle_kind = url_to_puzzle_kind(url).ok_or("puzzle type not detected");

    match puzzle_kind {
        Ok(puzzle_kind) => {
            puzzle::dispatch_puzz_link(&puzzle_kind, url).unwrap_or(Err("unknown puzzle type"))
        }
        Err(_) => {
            let puzzle_info = get_kudamono_url_info_detailed(url).ok_or("failed to parse URL")?;

            let puzzle_kind = *puzzle_info.get("G").unwrap_or(&"");
            let puzzle_variant = *puzzle_info.get("V").unwrap_or(&"");

            if let Some(res) = puzzle::dispatch_kudamono(puzzle_kind, puzzle_variant, url) {
                return res;
            }

            Err("unknown puzzle type")
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
