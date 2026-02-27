use {calc::state::State, regex::RegexBuilder};
fn main() -> Result<(), ()> {
	State::new(
		RegexBuilder::new(
			&[
				("number", r"\d+(?:\.\d+)?(?:e\d+)?"),
				("sign", r"//|<<|>>|[+-/*^%]"),
				("open", r"\("),
				("close", r"\)"),
			]
			.map(|(l, r)| format!("(?<{l}>{r})"))
			.join("|"),
		)
		.build()
		.expect("Invalid Regex"),
		"Help",
	)
	.run()
}
