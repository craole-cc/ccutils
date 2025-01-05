mod process;

pub fn test() -> anyhow::Result<()> {
    let mut proc = process::Process::new();
    let parent_proc = proc.get_parent_proc()?;
    println!("{:?}", parent_proc);
    anyhow::bail!("test");
    Ok(())
}
