pub fn output(command: &str, format: &Format) -> Result<String, Error> {
  let locations = locate(command)?;

  if locations.is_empty() {
    return Err(Error::CommandNotFound(command.to_string()));
  }

  Ok(match format {
    Format::Plain => locations
      .into_iter()
      .next()
      .map(|loc| loc.to_string())
      .unwrap_or_default(),
    Format::Fetch => locations
      .into_iter()
      .map(|loc| format!("{:#}", loc))
      .collect::<Vec<_>>()
      .join("\n"),
    Format::Verbose => {
      let mut output = format!("Command '{}' found in {} location(s):", command, locations.len());
      for location in locations {
        output.push_str(&format!("\n  {:#}", location));
      }
      output
    }
  })
}
