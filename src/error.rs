use failure::Fail;
use std::io;

/// Error type for kvs.
#[derive(Fail, Debug)]
pub enum KvsError {
  /// IO error.
  #[fail(display = "{}", _0)]
  Io(#[cause] io::Error),
  /// Serialization or deserialization error.
  #[fail(display = "{}", _0)]
  Serde(#[cause] serde_json::Error),
  /// Initial log error.
  #[fail(display = "{}", _0)]
  Log(#[cause] log::SetLoggerError),
  /// Removing non-existent key error.
  #[fail(display = "Key not found")]
  KeyNotFound,
  /// Unexpected command type error.
  /// It indicated a corrupted log or a program bug.
  #[fail(display = "Unexpected command type")]
  UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
  fn from(err: io::Error) -> KvsError {
    KvsError::Io(err)
  }
}

impl From<serde_json::Error> for KvsError {
  fn from(err: serde_json::Error) -> KvsError {
    KvsError::Serde(err)
  }
}

impl From<log::SetLoggerError> for KvsError {
  fn from(err: log::SetLoggerError) -> KvsError {
    KvsError::Log(err)
  }
}

/// Result type for kvs.
pub type Result<T> = std::result::Result<T, KvsError>;
