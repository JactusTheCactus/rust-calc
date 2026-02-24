use regex::Regex;
pub struct State {
	pub running: bool,
	pub re: Regex,
}
