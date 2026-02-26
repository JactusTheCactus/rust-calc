use {calc::state::State, regex::RegexBuilder};
fn main() {
	State::new(
		RegexBuilder::new(&format!(
			"(?<number>{})|(?<sign>{})|(?<open>{})|(?<close>{})",
			r"\d+(?:\.\d+)?", r"//|<<|>>|[+-/*^%]", r"\(", r"\)"
		))
		.build()
		.expect("Invalid Regex"),
		"Help",
	)
	.run();
}
