#[derive(Debug, Clone)]
pub enum Token {
	Number(f64),
	Sign(String),
	Open,
	Close,
}
pub enum NullToken {
	Some(Token),
	None,
}
