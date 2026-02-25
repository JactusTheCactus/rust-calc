use {
	crate::{el::El, state::State},
	std::io::{Write, stdin, stdout},
};
pub fn run(state: &mut State) {
	let mut buffer = String::new();
	let mut out = stdout();
	out.write(b"> ").unwrap();
	out.flush().unwrap();
	stdin().read_line(&mut buffer).unwrap();
	for line in buffer.lines() {
		match line {
			"h" | "help" => todo!(),
			"q" | "quit" => state.running = false,
			_ => {
				let mut elements = vec![];
				for str in line.split_whitespace() {
					for cap in state.re.captures_iter(str) {
						elements.push(if let Some(number) = cap.name("number") {
							El::Number(number.as_str().parse::<i32>().unwrap())
						} else if let Some(string) = cap.name("sign") {
							El::Sign(string.as_str().into())
						} else if cap.name("open").is_some() {
							El::Open
						} else if cap.name("close").is_some() {
							El::Close
						} else {
							El::None
						})
					}
				}
				println!(
					"{}",
					elements
						.iter()
						.filter_map(|e| match e {
							El::Number(n) => Some(n.to_string()),
							El::Sign(s) => Some(s.to_string()),
							El::Open => Some("(".to_string()),
							El::Close => Some(")".to_string()),
							El::None => None,
						})
						.collect::<Vec<String>>()
						.join(" ")
				);
			}
		}
	}
}
