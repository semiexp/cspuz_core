#![allow(static_mut_refs)] // TODO: remove this

extern crate cspuz_rs;

pub mod board;
mod puzzle;
mod uniqueness;

use board::Board;
use cspuz_rs::serializer::{get_kudamono_url_info_detailed, url_to_puzzle_kind};

static mut SHARED_ARRAY: Vec<u8> = vec![];

fn solve_puzz_link(puzzle_kind: String, url: &str) -> Result<Board, &'static str> {
    if puzzle_kind == "nurikabe" {
        puzzle::nurikabe::solve_nurikabe(url)
    } else if puzzle_kind == "yajilin" || puzzle_kind == "yajirin" {
        puzzle::yajilin::solve_yajilin(url)
    } else if puzzle_kind == "heyawake" {
        puzzle::heyawake::solve_heyawake(url, false)
    } else if puzzle_kind == "ayeheya" {
        puzzle::heyawake::solve_heyawake(url, true)
    } else if puzzle_kind == "slither" || puzzle_kind == "slitherlink" {
        puzzle::slitherlink::solve_slitherlink(url)
    } else if puzzle_kind == "slalom" {
        puzzle::slalom::solve_slalom(url)
    } else if puzzle_kind == "nurimisaki" {
        puzzle::nurimisaki::solve_nurimisaki(url)
    } else if puzzle_kind == "compass" {
        puzzle::compass::solve_compass(url)
    } else if puzzle_kind == "akari" {
        puzzle::akari::solve_akari(url)
    } else if puzzle_kind == "lits" {
        puzzle::lits::solve_lits(url)
    } else if puzzle_kind == "masyu" || puzzle_kind == "mashu" {
        puzzle::masyu::solve_masyu(url)
    } else if puzzle_kind == "shakashaka" {
        puzzle::shakashaka::solve_shakashaka(url)
    } else if puzzle_kind == "araf" {
        puzzle::araf::solve_araf(url)
    } else if puzzle_kind == "aqre" {
        puzzle::aqre::solve_aqre(url)
    } else if puzzle_kind == "tapa" {
        puzzle::tapa::solve_tapa(url)
    } else if puzzle_kind == "simpleloop" {
        puzzle::simpleloop::solve_simpleloop(url)
    } else if puzzle_kind == "yajilin-regions" {
        puzzle::yajilin_regions::solve_yajilin_regions(url)
    } else if puzzle_kind == "kropki" {
        puzzle::kropki::solve_kropki(url)
    } else if puzzle_kind == "kurotto" {
        puzzle::kurotto::solve_kurotto(url)
    } else if puzzle_kind == "castle" {
        puzzle::castle_wall::solve_castle_wall(url)
    } else if puzzle_kind == "shimaguni" {
        puzzle::shimaguni::solve_shimaguni(url)
    } else if puzzle_kind == "norinori" {
        puzzle::norinori::solve_norinori(url)
    } else if puzzle_kind == "coral" {
        puzzle::coral::solve_coral(url)
    } else if puzzle_kind == "cave" {
        puzzle::cave::solve_cave(url)
    } else if puzzle_kind == "curvedata" {
        puzzle::curvedata::solve_curvedata(url)
    } else if puzzle_kind == "shikaku" {
        puzzle::shikaku::solve_shikaku(url)
    } else if puzzle_kind == "sudoku" {
        puzzle::sudoku::solve_sudoku(url)
    } else if puzzle_kind == "sashigane" {
        puzzle::sashigane::solve_sashigane(url)
    } else if puzzle_kind == "lohkous" {
        puzzle::lohkous::solve_lohkous(url)
    } else if puzzle_kind == "hashi" {
        puzzle::hashi::solve_hashi(url)
    } else if puzzle_kind == "herugolf" {
        puzzle::herugolf::solve_herugolf(url)
    } else if puzzle_kind == "slashpack" {
        puzzle::slashpack::solve_slashpack(url)
    } else if puzzle_kind == "moonsun" {
        puzzle::moonsun::solve_moonsun(url)
    } else if puzzle_kind == "fillomino" {
        puzzle::fillomino::solve_fillomino(url)
    } else if puzzle_kind == "cbanana" {
        puzzle::chocobanana::solve_chocobanana(url)
    } else if puzzle_kind == "fivecells" {
        puzzle::fivecells::solve_fivecells(url)
    } else if puzzle_kind == "cocktail" {
        puzzle::cocktail::solve_cocktail(url)
    } else if puzzle_kind == "stostone" {
        puzzle::stostone::solve_stostone(url)
    } else if puzzle_kind == "pencils" {
        puzzle::pencils::solve_pencils(url)
    } else if puzzle_kind == "barns" {
        puzzle::barns::solve_barns(url)
    } else if puzzle_kind == "reflect" {
        puzzle::reflect::solve_reflect_link(url)
    } else if puzzle_kind == "ringring" {
        puzzle::ringring::solve_ringring(url)
    } else if puzzle_kind == "loopsp" {
        puzzle::loop_special::solve_loop_speical(url)
    } else if puzzle_kind == "nagenawa" {
        puzzle::nagenawa::solve_nagenawa(url)
    } else if puzzle_kind == "icewalk" {
        puzzle::icewalk::solve_icewalk(url)
    } else if puzzle_kind == "forestwalk" {
        puzzle::forestwalk::solve_forestwalk(url)
    } else if puzzle_kind == "kouchoku" {
        puzzle::kouchoku::solve_kouchoku(url)
    } else if puzzle_kind == "creek" {
        puzzle::creek::solve_creek(url)
    } else if puzzle_kind == "squarejam" {
        puzzle::square_jam::solve_square_jam(url)
    } else if puzzle_kind == "firefly" {
        puzzle::firefly::solve_firefly(url)
    } else if puzzle_kind == "nothree" {
        puzzle::nothree::solve_nothree(url)
    } else if puzzle_kind == "invlitso" {
        puzzle::inverse_litso::solve_inverse_litso(url)
    } else if puzzle_kind == "tontonbeya" {
        puzzle::tontonbeya::solve_tontonbeya(url)
    } else if puzzle_kind == "guidearrow" {
        puzzle::guidearrow::solve_guidearrow(url)
    } else if puzzle_kind == "aquapelago" {
        puzzle::aquapelago::solve_aquapelago(url)
    } else if puzzle_kind == "pentominous" {
        puzzle::polyominous::solve_pentominous(url)
    } else if puzzle_kind == "akichi" {
        puzzle::akichiwake::solve_akichiwake(url)
    } else if puzzle_kind == "chainedb" {
        puzzle::chainedb::solve_chainedb(url)
    } else if puzzle_kind == "evolmino" {
        puzzle::evolmino::solve_evolmino(url)
    } else if puzzle_kind == "dbchoco" {
        puzzle::dbchoco::solve_doublechoco(url)
    } else if puzzle_kind == "statuepark" {
        puzzle::statue_park::solve_statue_park(url)
    } else if puzzle_kind == "kakuro" {
        puzzle::kakuro::solve_kakuro(url)
    } else if puzzle_kind == "doppelblock" {
        puzzle::doppelblock::solve_doppelblock(url)
    } else if puzzle_kind == "lither" {
        puzzle::litherslink::solve_litherslink(url)
    } else if puzzle_kind == "coffeemilk" {
        puzzle::coffeemilk::solve_coffeemilk(url)
    } else if puzzle_kind == "nurimaze" {
        puzzle::nurimaze::solve_nurimaze(url)
    } else if puzzle_kind == "firewalk" {
        puzzle::firewalk::solve_firewalk(url)
    } else if puzzle_kind == "nikoji" {
        puzzle::nikoji::solve_nikoji(url)
    } else if puzzle_kind == "archipelago" {
        puzzle::archipelago::solve_archipelago(url)
    } else if puzzle_kind == "timebomb" {
        puzzle::timebomb::solve_timebomb(url)
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
