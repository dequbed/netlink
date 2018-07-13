use core::{self, fmt};
use std::error::Error as StdError;
use std::io;

/// The error type for the netlink packet parser
#[derive(Debug)]
pub enum Error {
    Exhausted,
    Truncated,
    Malformed,
    Io(io::Error),
    #[doc(hidden)]
    __Nonexhaustive,
}

pub type Result<T> = core::result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Exhausted => "buffer space exhausted",
            Error::Truncated => "truncated packet",
            Error::Malformed => "malformed packet",
            Error::Io(_) => "failed to read or write a packet due to an IO error",
            Error::__Nonexhaustive => unreachable!(),
        }
    }

    fn cause(&self) -> Option<&StdError> {
        None
    }
}

impl From<io::Error> for Error {
    fn from(io_err: io::Error) -> Error {
        Error::Io(io_err)
    }
}
