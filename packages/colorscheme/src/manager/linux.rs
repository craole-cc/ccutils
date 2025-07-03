use std::{env::var, process::Command};

pub struct Linux {
  desktop: Option<String>
}

impl Linux {
  pub fn new() -> Self {
    Self {
      desktop: var("XDG_CURRENT_DESKTOP").ok().map(|d| d.to_lowercase())
    }
  }

  fn is_kde(&self) -> bool {
    self.desktop.as_ref().map_or(false, |d| d.contains("kde"))
  }

  fn is_gnome(&self) -> bool {
    self.desktop.as_ref().map_or(false, |d| d.contains("gnome"))
  }
}

impl Manager for Linux {
  fn get(&self) -> Option<Mode> {
    if self.is_kde() {
      if let Ok(output) = Command::new("plasma-apply-colorscheme")
        .arg("--current")
        .output()
      {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Some(if stdout.contains("Light") {
          Mode::Light
        } else {
          Mode::Dark
        });
      }
    } else if self.is_gnome() {
      if let Ok(output) = Command::new("gsettings")
        .args(["get", "org.gnome.desktop.interface", "gtk-theme"])
        .output()
      {
        let stdout = String::from_utf8_lossy(&output.stdout);
        return Some(if stdout.to_lowercase().contains("light") {
          Mode::Light
        } else {
          Mode::Dark
        });
      }
    }
    None
  }

  fn set(&self, mode: &Mode) -> Result<(), Box<dyn std::error::Error>> {
    if self.is_kde() {
      let theme_name = match mode {
        Mode::Dark => "BreezeDark",
        Mode::Light => "BreezeLight"
      };
      Command::new("plasma-apply-colorscheme")
        .arg(theme_name)
        .output()?;
    } else if self.is_gnome() {
      let theme_name = match theme {
        Theme::Dark => "Adwaita-dark",
        Theme::Light => "Adwaita-light"
      };
      Command::new("gsettings")
        .args([
          "set",
          "org.gnome.desktop.interface",
          "gtk-theme",
          theme_name
        ])
        .output()?;
    } else if let Some(desktop) = &self.desktop {
      eprintln!("Unsupported Linux desktop environment: {}", desktop);
    }
    Ok(())
  }
}
