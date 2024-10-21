// mod cli;
use erks::AnyhowResult;
use whers::data::*;

fn main() -> AnyhowResult<()> {
	logline::init();
	Search::test();
	numeral::test();

	// let result = whers::types::Number;
	// logline::info!("{:#?}", result);

	// .with_pattern("cargo.toml")
	// .with_direction("both")
	// .with_limit(2)
	// .with_depth(1)
	// .exclude(vec!["target".to_string()]);
	// search = search.with_pattern("pop");

	// logline::warn!("{:#?}", search.execute()?);
	// cli::init();

	Ok(())
}
