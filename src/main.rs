use {
	calc::{run::run, state::State},
	regex::RegexBuilder,
};
fn main() {
	let mut state = State {
		running: true,
		re: RegexBuilder::new(&format!(
			"(?<number>{})|(?<sign>{})|(?<open>{})|(?<close>{})",
			r"\d+(?:\.\d+)?", r"//|<<|>>|[+-/*^%]", r"\(", r"\)"
		))
		.build()
		.expect("Invalid Regex"),
	};
	loop {
		run(&mut state);
		if !state.running {
			break;
		}
	}
}
