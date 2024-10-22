fn main() -> erks::AnyhowResult<()> {
	logline::Logline::default()
		.with_level(logline::TRACE)
		.ugly()
		.show_line()
		// .show_target()
		.init();

	numeral::test();
	Ok(())
}
