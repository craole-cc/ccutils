// mod cli;
use erks::AnyhowResult;

fn main() -> AnyhowResult<()> {
	logline::init_warnings();
	let search = whers::data::Search::default()
		.with_pattern("init")
		.with_direction("Both")
		.exclude(vec!["target".to_string()]);

	logline::info!("{:#?}", search);

	logline::warn!("{:#?}", search.execute()?);
	// cli::init();

	Ok(())
}
