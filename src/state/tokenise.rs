use crate::{state::State, token::Token};
pub fn tokenise(state: &State, line: &str) -> Vec<Token> {
	let mut elements = vec![];
	for str in line.split_whitespace() {
		for cap in state.re.captures_iter(str) {
			elements.push(if let Some(number) = cap.name("number") {
				Token::Number(
					number.as_str().parse::<i32>().expect("Invalid number"),
				)
			} else if let Some(string) = cap.name("sign") {
				Token::Sign(string.as_str().into())
			} else if cap.name("open").is_some() {
				Token::Open
			} else if cap.name("close").is_some() {
				Token::Close
			} else {
				Token::None
			})
		}
	}
	elements
}
