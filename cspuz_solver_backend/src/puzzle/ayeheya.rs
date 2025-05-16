pub fn solve(url: &str) -> Result<crate::board::Board, &'static str> {
    crate::puzzle::heyawake_internal::solve(url, true)
}
