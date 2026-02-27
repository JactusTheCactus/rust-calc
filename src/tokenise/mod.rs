use crate::{
	state::State,
	tokenise::token::{NullToken, Token},
};
pub mod token;
pub fn tokenise(state: &State, line: &str) -> Result<Vec<Token>, ()> {
	let mut elements = vec![];
	for str in line.split_whitespace() {
		for cap in state.re.captures_iter(str) {
			elements.push(if let Some(number) = cap.name("number") {
				NullToken::Some(Token::Number(
					number.as_str().parse().expect("Invalid number"),
				))
			} else if let Some(string) = cap.name("sign") {
				NullToken::Some(Token::Sign(string.as_str().into()))
			} else if cap.name("open").is_some() {
				NullToken::Some(Token::Open)
			} else if cap.name("close").is_some() {
				NullToken::Some(Token::Close)
			} else {
				NullToken::None
			})
		}
	}
	Ok(elements
		.iter()
		.filter_map(|e| match e {
			NullToken::Some(t) => Some(t.clone()),
			NullToken::None => None,
		})
		.collect())
}
