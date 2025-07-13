use super::{Error, process, terminal, time, traits::FormatHelpers};
use sysinfo::System;

#[derive(Debug)]
pub struct Fetcher {
  pub time: time::Info,
  pub process: process::Info
}

impl Default for Fetcher {
  fn default() -> Self {
    Self::new().unwrap_or_else(|e| {
      logline::error!("Failed to create Fetcher: {}", e);
      Self {
        time: time::Info::default(),
        process: process::Info::default()
      }
    })
  }
}

impl Fetcher {
  pub fn new() -> Result<Self, Error> {
    let mut system = System::new_all();
    system.refresh_all();

    Ok(Self {
      time: time::Info::default(),
      process: process::Info::new(&system)?
    })
  }

  pub fn fetcher(&self) -> String {
    let mut output = String::from("System Information Report\n");
    output.push_str(&"=".repeat(80));
    output.push('\n');

    // Time section
    output.push_str("\nSystem Time Status\n");
    output.push_str(&"-".repeat(40));
    output.push('\n');
    output.push_str(&format!("Current Time : {}\n", self.time.current_fmt()));
    output.push_str(&format!("System Uptime: {}\n", self.time.uptime_fmt()));
    output.push_str(&format!("Time Zone    : {}\n", self.time.timezone));

    // Process section
    output.push_str("\nActive Process\n");
    output.push_str(&"-".repeat(40));
    output.push('\n');
    output.push_str(&format!("ID           : {}\n", self.process.id));
    output.push_str(&format!("Name         : {}\n", self.process.name));
    output.push_str(&format!("User         : {}\n", self.process.user));
    output.push_str(&format!("Path         : {}\n", self.process.path.display()));
    output.push_str(&format!("Working Dir  : {}\n", self.process.cwd.display()));

    // Shell section
    output.push_str("\nShell Environment\n");
    output.push_str(&"-".repeat(40));
    output.push('\n');
    output.push_str(&format!("Shell        : {}\n", self.process.shell.name));
    output.push_str(&format!(
      "Version      : {}\n",
      self.process.shell.version.as_deref().unwrap_or("Unknown")
    ));
    output.push_str(&format!("Shell Path   : {}\n", self.process.shell.path.display()));

    // Shell configurations
    output.push_str("\nConfiguration Files:\n");
    for path in &self.process.shell.conf {
      output.push_str(&format!("  - {}\n", path.display()));
    }

    output
  }

  pub fn fetch(&self) -> String {
    let term = terminal::Info::new();
    let mut output = String::from("System Information Report\n");
    output.push_str(&term.separator_line());
    output.push('\n');

    // Time section
    output.push_str(&term.format_section("Time"));
    output.push_str(&term.format_field("Current", &self.time.current_fmt()));
    output.push_str(&term.format_field("Uptime", &self.time.uptime_fmt()));
    output.push_str(&term.format_field("Time Zone", &self.time.timezone));

    // Process Section
    output.push_str(&term.format_section("Process"));
    output.push_str(&term.format_field("ID", &self.process.id.to_string()));
    output.push_str(&term.format_field("Name", &self.process.name));
    output.push_str(&term.format_field("User", &self.process.user));
    output.push_str(&term.format_field("Path", &self.process.path.display().to_string()));
    output.push_str(&term.format_field("Working Dir", &self.process.cwd.display().to_string()));

    output.push_str(&term.format_section("Shell"));
    output.push_str(&term.format_field("ID", &self.process.shell.id.to_string()));
    output.push_str(&term.format_field("Name", &self.process.shell.name));
    output.push_str(&term.format_field("Path", &self.process.shell.path.display().to_string()));
    output.push_str(&term.format_field("Version", self.process.shell.version.as_deref().unwrap_or("Unknown")));

    output.push_str(&term.format_section("Shell Configurations"));
    for path in &self.process.shell.conf {
      output.push_str(&format!("  - {}\n", path.display()));
    }

    output
  }
}

pub fn init() -> Result<Fetcher, Error> {
  Fetcher::new()
}

// Update test function to handle potential errors
pub fn test() {
  match init() {
    Ok(info) => {
      let msg = format!("Testing Fetcher\n{}", info.fetch());
      logline::debug!("{}", msg);

      // logline::debug!("{}", info.time.fetch());
      // logline::debug!("{}", info.process.fetch());
    }
    Err(e) => {
      logline::error!("Failed to initialize fetcher: {}", e);
    }
  }
}
