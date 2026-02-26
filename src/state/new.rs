use {crate::state::State, regex::Regex};
pub fn new(re: Regex, help: &str) -> State {
	State { running: true, re, help: help.to_string() }
}
