use thiserror::Error;

/// LunoError is the wrapper error type for this crate to help differentiate it from
/// more generic errors in your application.
#[derive(Error, Debug)]
pub enum LunoError {
	#[error("Network error encountered")]
	HttpError(reqwest::Error),
}

impl From<reqwest::Error> for LunoError {
	fn from(item: reqwest::Error) -> Self {
		LunoError::HttpError(item)
	}
}
