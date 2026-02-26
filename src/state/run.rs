use {
	crate::{state::State, token::Token},
	std::io::{Write, stdin, stdout},
};
pub fn run(state: &mut State) {
	loop {
		let mut buffer = String::new();
		let mut out = stdout();
		let _ = out.write(b"> ").expect("Could not write to StdOut");
		out.flush().expect("StdOut flush failed");
		stdin().read_line(&mut buffer).expect("Could not read line from StdIn");
		for line in buffer.lines() {
			match line {
				"h" | "help" => println!("{:?}", state.help),
				"q" | "quit" => state.running = false,
				_ => {
					let elements = state.tokenise(line);
					for i in &elements {
						println!("{i:?}");
					}
					println!(
						"{}",
						elements
							.iter()
							.filter_map(|e| match e {
								Token::Number(n) => Some(n.to_string()),
								Token::Sign(s) => Some(s.to_string()),
								Token::Open => Some("(".to_string()),
								Token::Close => Some(")".to_string()),
								Token::None => None,
							})
							.collect::<Vec<String>>()
							.join(" ")
					);
				}
			}
		}
		if !state.running {
			break;
		}
	}
}
