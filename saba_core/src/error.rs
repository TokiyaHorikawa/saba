use alloc::string::String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
  NetWork(String),
  UnexpectedInput(String),
  InvalidUI(String),
  Other(String),
}
