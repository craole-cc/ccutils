use std::{
	cmp::Ordering, fs::Metadata, path::PathBuf, time::SystemTime,
};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Sort {
	#[default]
	Name,
	Size,
	Date,
	Extension,
	Modified,
	Created,
	Accessed,
}

impl Sort {
	pub fn sort(&self, paths: &mut [PathBuf]) {
		match self {
			Sort::Name => paths.sort(),
			Sort::Size => paths
				.sort_by(|a, b| sort_by_metadata(a, b, |m| m.len())),
			Sort::Date => paths.sort_by(|a, b| {
				sort_by_metadata(a, b, |m| {
					m.modified().unwrap_or(SystemTime::UNIX_EPOCH)
				})
			}),
			Sort::Extension => paths.sort_by(|a, b| {
				let ext_a = a
					.extension()
					.and_then(|e| e.to_str())
					.unwrap_or("");
				let ext_b = b
					.extension()
					.and_then(|e| e.to_str())
					.unwrap_or("");
				ext_a.cmp(ext_b)
			}),
			Sort::Modified => paths.sort_by(|a, b| {
				sort_by_metadata(a, b, |m| {
					m.modified().unwrap_or(SystemTime::UNIX_EPOCH)
				})
			}),
			Sort::Created => paths.sort_by(|a, b| {
				sort_by_metadata(a, b, |m| {
					m.created().unwrap_or(SystemTime::UNIX_EPOCH)
				})
			}),
			Sort::Accessed => paths.sort_by(|a, b| {
				sort_by_metadata(a, b, |m| {
					m.accessed().unwrap_or(SystemTime::UNIX_EPOCH)
				})
			}),
		}
	}

	pub fn is_none(&self) -> bool {
		*self == Sort::Name
	}
}

fn sort_by_metadata<F, T>(a: &PathBuf, b: &PathBuf, f: F) -> Ordering
where
	F: Fn(&Metadata) -> T,
	T: Ord,
{
	match (a.metadata(), b.metadata()) {
		(Ok(meta_a), Ok(meta_b)) => f(&meta_a).cmp(&f(&meta_b)),
		(Ok(_), Err(_)) => Ordering::Less,
		(Err(_), Ok(_)) => Ordering::Greater,
		(Err(_), Err(_)) => a.cmp(b),
	}
}
