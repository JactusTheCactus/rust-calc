use {
	crate::state::State,
	std::io::{Write, stdin, stdout},
};
pub(super) fn run(state: &mut State) {
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
					let tokens = state.tokenise(line);
					let _tree = state.parse(tokens);
				}
			}
		}
		if !state.running {
			break;
		}
	}
}
