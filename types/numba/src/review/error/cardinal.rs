use erks::thiserror;

#[derive(thiserror::Error, Debug)]
pub enum Error {
	#[error("Unknown word representation: '{0}'")]
	FailedToParseWords(String),

	#[error("Failed to parse input: '{0}'")]
	ParseFailure(String),

	#[error("Unknown cardinal encountered: '{0}'")]
	Unknown(String),

	#[error("This is not a valid number: '{0}'")]
	InvalidNumber(String),

	#[error("Invalid order")]
	InvalidOrder,

	#[error("Invalid digit")]
	InvalidDigit,

	#[error("Invalid scale")]
	InvalidScale,

	#[error("Invalid scale")]
	KnownRangeExceeded,
}
