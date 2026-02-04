//! Package scaffolding utilities.
//!
//! Creates new package directory structures with Cargo.toml and source files.

use crate::prelude::*;

/// Package scaffolding configuration.
///
/// Used to generate a new package structure with Cargo.toml and source files.
///
/// # Examples
/// ```no_run
/// use prjenv::prelude::*;
///
/// let scaffold = PackageScaffold::new("my-package")
///   .version("0.1.0")
///   .description("My new package")
///   .binary();
///
/// scaffold.create("packages/my-package")?;
/// # Ok::<(), std::io::Error>(())
/// ```
#[derive(Debug, Clone)]
pub struct Scaffold {
  pub name: String,
  pub version: String,
  pub description: String,
  pub edition: String,
  pub authors: Vec<String>,
  pub dependencies: Vec<(String, String)>,
  pub is_binary: bool,
}

impl Scaffold {
  /// Create a new package scaffold builder.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  ///
  /// let scaffold = PackageScaffold::new("my-cli");
  /// ```
  #[must_use]
  pub fn new(name: impl Into<String>) -> Self {
    Self {
      name: name.into(),
      version: String::from("0.1.0"),
      description: String::new(),
      edition: String::from("2024"),
      authors: Vec::new(),
      dependencies: Vec::new(),
      is_binary: false,
    }
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Builders                                                  ║
  //╚═══════════════════════════════════════════════════════════╝

  #[must_use]
  pub fn version(mut self, version: impl Into<String>) -> Self {
    self.version = version.into();
    self
  }

  #[must_use]
  pub fn description(mut self, desc: impl Into<String>) -> Self {
    self.description = desc.into();
    self
  }

  #[must_use]
  pub fn edition(mut self, edition: impl Into<String>) -> Self {
    self.edition = edition.into();
    self
  }

  #[must_use]
  pub fn author(mut self, author: impl Into<String>) -> Self {
    self.authors.push(author.into());
    self
  }

  #[must_use]
  pub fn dependency(mut self, name: impl Into<String>, version: impl Into<String>) -> Self {
    self.dependencies.push((name.into(), version.into()));
    self
  }

  #[must_use]
  pub const fn binary(mut self) -> Self {
    self.is_binary = true;
    self
  }

  #[must_use]
  pub const fn library(mut self) -> Self {
    self.is_binary = false;
    self
  }

  //╔═══════════════════════════════════════════════════════════╗
  //║ Scaffolding Operations                                    ║
  //╚═══════════════════════════════════════════════════════════╝

  /// Convert to TOML table for Cargo.toml.
  #[must_use]
  pub fn to_toml(&self) -> TomlTable {
    let mut table = TomlTable::new();
    let mut package = TomlTable::new();

    package.insert(String::from("name"), TomlValue::String(self.name.clone()));
    package.insert(
      String::from("version"),
      TomlValue::String(self.version.clone()),
    );
    package.insert(
      String::from("edition"),
      TomlValue::String(self.edition.clone()),
    );

    if !self.description.is_empty() {
      package.insert(
        String::from("description"),
        TomlValue::String(self.description.clone()),
      );
    }

    if !self.authors.is_empty() {
      let authors: Vec<TomlValue> = self
        .authors
        .iter()
        .map(|a| TomlValue::String(a.clone()))
        .collect();
      package.insert(String::from("authors"), TomlValue::Array(authors));
    }

    table.insert(String::from("package"), TomlValue::Table(package));

    // Add dependencies if any
    if !self.dependencies.is_empty() {
      let mut deps = TomlTable::new();
      for (name, version) in &self.dependencies {
        deps.insert(name.clone(), TomlValue::String(version.clone()));
      }
      table.insert(String::from("dependencies"), TomlValue::Table(deps));
    }

    table
  }

  /// Write Cargo.toml to file.
  ///
  /// # Errors
  /// Returns an error if TOML serialization or file writing fails.
  pub fn write_cargo_toml(&self, path: impl AsRef<Path>) -> Result<()> {
    let toml_string = to_toml_string_pretty(&self.to_toml())
      .map_err(|e| IOError::new(IOErrorKind::InvalidData, e))?;

    write(path, toml_string)?;
    Ok(())
  }

  /// Create complete package structure.
  ///
  /// Creates directories, Cargo.toml, and initial source file.
  ///
  /// # Errors
  /// Returns an error if directory creation or file writing fails.
  ///
  /// # Examples
  /// ```no_run
  /// use prjenv::prelude::*;
  ///
  /// let scaffold = PackageScaffold::new("my-cli")
  ///   .binary()
  ///   .description("My CLI tool");
  ///
  /// scaffold.create("packages")?;
  /// # Ok::<(), std::io::Error>(())
  /// ```
  pub fn create(self, base_path: impl AsRef<Path>) -> Result<PathBuf> {
    let pkg_path = base_path.as_ref().join(&self.name);

    // Create directories
    create_dir_all(&pkg_path)?;
    create_dir_all(pkg_path.join("src"))?;

    // Write Cargo.toml
    self.write_cargo_toml(pkg_path.join("Cargo.toml"))?;

    // Create source file
    let source_content = if self.is_binary {
      format!(
        "//! {}\n\nfn main() {{\n    println!(\"Hello from {}!\");\n}}\n",
        self.description, self.name
      )
    } else {
      format!(
        "//! {}\n\n#[cfg(test)]\nmod tests {{\n    #[test]\n    fn it_works() {{\n        assert_eq!(2 + 2, 4);\n    }}\n}}\n",
        self.description
      )
    };

    let source_file = if self.is_binary { "main.rs" } else { "lib.rs" };
    write(pkg_path.join("src").join(source_file), source_content)?;

    #[cfg(feature = "tracing")]
    info!("Scaffolded package: {}", pkg_path.display());

    Ok(pkg_path)
  }
}
