use {
	crate::{
		state::{new::new, run::run, tokenise::tokenise},
		token::Token,
	},
	regex::Regex,
};
pub mod new;
pub mod run;
pub mod tokenise;
pub struct State {
	pub running: bool,
	pub re: Regex,
	pub help: String,
}
impl State {
	pub fn new(re: Regex, help: &str) -> Self {
		new(re, help)
	}
	pub fn run(&mut self) {
		run(self)
	}
	pub fn tokenise(&self, line: &str) -> Vec<Token> {
		tokenise(self, line)
	}
}
