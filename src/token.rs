#[derive(Debug)]
pub enum Token {
	Number(i32),
	Sign(String),
	Open,
	Close,
	None,
}
