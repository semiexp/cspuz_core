pub fn solve(url: &str) -> Result<crate::board::Board, &'static str> {
    crate::puzzle::heyawake_internal::solve(url, false)
}

pub fn enumerate(url: &str, num_max_answers: usize) -> Result<(crate::board::Board, Vec<crate::board::Board>), &'static str> {
    crate::puzzle::heyawake_internal::enumerate_answers_heyawake(url, num_max_answers)
}
