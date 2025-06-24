use erks::thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Invalid word representation")]
	InvalidWordRepresentation,

	#[error("Failed to parse '{}' a known numeral", _0)]
	UnknownNumeral(String),
}
