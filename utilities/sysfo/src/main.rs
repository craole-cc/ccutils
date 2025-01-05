fn main() -> anyhow::Result<()> {
    logline::init();
    sysfo::test();

    Ok(())
}
