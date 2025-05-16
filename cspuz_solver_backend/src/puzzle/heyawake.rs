pub use crate::puzzle::heyawake_internal::enumerate_answers_heyawake;

pub fn solve(url: &str) -> Result<crate::board::Board, &'static str> {
    crate::puzzle::heyawake_internal::solve(url, false)
}
