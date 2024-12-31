pub struct Config {
	/// The pattern to search for.
	///
	/// The default pattern is `.*`, which matches all files and directories.
	/// To specify a different pattern, use the `pattern` field.
	pub pattern: String,

  /// The type of path to search for.
  ///
  /// The default path type is `Path::File`.
  /// To specify a different path type, use the `path_type` field.
	path_type: Path,
	directory: Directory,
	direction: Direction,
	sort: Sort,

	/// Specifies if the search should be case-sensitive.
	///
	/// - If `true`, the search is case-sensitive (e.g., "File.txt" and "file.txt" are different).
	/// - If `false`, the search is case-insensitive (e.g., "File.txt" and "file.txt" are the same).
	///
	/// By default, `case_sensitive` is `false`, making the search case-insensitive.
	pub case_sensitive: bool,

	/// The maximum number of results to return.
	///
	///   If `None`, there is no limit to the number of results.
	///   If `Some(limit)`, the search will return at most `limit` results.
	pub limit: Option<usize>,

	/// The maximum search depth.
	///
	/// If `None`, there is no limit to the search depth.
	/// If `Some(depth)`, the search will not go deeper than the specified depth.
	///
	/// A depth of `1` includes all files and directories directly under the current directory.
	/// A depth of `2` includes all files and directories under subdirectories of the current directory, and so on.
	pub max_depth: Option<usize>,

	/// The minimum depth for reported entries.
	///
	/// If `Some(depth)`, only entries at or below the specified depth will be reported.
	/// If `None`, there is no minimum depth limit.
	///
	/// A depth of `1` means that only files and directories directly under the current directory will be reported.
	/// A depth of `2` means that files and directories under subdirectories of the current directory will be reported, and so on.
	pub min_depth: Option<usize>,

	/// Include hidden files and directories.
	///
	/// If set to `true`, the search will include hidden files and directories.
	/// If set to `false`, the search will exclude hidden files and directories.
	/// The default value is `false`.
	pub include_hidden: bool,

	/// A list of glob patterns that should be excluded from the search.
	///
	/// This is a list of glob patterns that should be excluded from the search.
	/// The search will respect these patterns and exclude any matching entries from the results.
	pub exclude: Vec<String>,

	/// Ignore files to read.
	///
	/// This includes entries in ignore files such as `.gitignore`, `.fdignore`, and `.ignore`.
	/// The search will respect these files and exclude any matching entries from the results.
	pub ignore_files: Vec<String>,

	/// Override the default ignore behavior
	///
	/// If set to `true`, the search will include results from files and directories that would otherwise be
	/// ignored by ignore files including the defaults.
	///
	/// This allows the user to include these files in the search results, overriding the default ignore behavior.
	pub no_ignore: bool,

	/// Quiet mode
	///
	/// If set to `true`, the search returns only an exit code (0 or 1).
	/// The default value is `false`.
	pub quiet: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Path {
	Executable,
	Empty,
	BlockDevice,
	CharacterDevice,
	Directory,
	Pipe,
	File,
	SymbolicLink,
	Socket,
	#[default]
	Any,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum Directory {
	Executable,
	#[default]
	Current,
	Parent,
	Root,
	Config,
	Data,
	Cache,
	Home,
	Fonts,
	Downloads,
	Documents,
	Path(PathBuf),
	Env(String),
}
