use app::*;

fn main() -> anyhow::Result<()> {
    logline::init();
    cli::init();
    Ok(())
}
