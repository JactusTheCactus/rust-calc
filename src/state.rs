use {
	crate::{
		new::new,
		parse::{Node, parse},
		run::run,
		tokenise::{token::Token, tokenise},
	},
	regex::Regex,
};
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
	pub fn parse(&self, tokens: Vec<Token>) -> Node {
		parse(tokens)
	}
}
