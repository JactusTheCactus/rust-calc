use {crate::state::State, regex::Regex};
pub(super) fn new(re: Regex, help: &str) -> State {
	State { running: true, re, help: help.to_string() }
}
