use std::error;
use std::io;
use std::fmt;
use std::string;
use std::num;
use std::result;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq)]
pub struct Error {
    internal: ErrorKind,
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Error {
        Error {
            internal: kind
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::from(ErrorKind::IOError(err))
    }
}

impl From<num::ParseIntError> for Error {
    fn from(err: num::ParseIntError) -> Error {
        Error::from(ErrorKind::IntParseError(err))
    }
}

impl From<string::FromUtf8Error> for Error {
    fn from(err: string::FromUtf8Error) -> Error {
        Error::from(ErrorKind::NotUTF8(err))
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmter: &mut fmt::Formatter) -> fmt::Result {
        match self.internal {
            ErrorKind::IntParseError(ref e) => e.fmt(fmter),
            ErrorKind::IOError(ref e) => e.fmt(fmter),
            ErrorKind::NotUTF8(ref e) => e.fmt(fmter),

            ErrorKind::BufferDoesntContainDelimiter => write!(fmter, "The acclimater buffer doesn't contain the delimiter"),
            ErrorKind::BeginningDoesntMatch => write!(fmter, "The beginning segments do not match"),
            ErrorKind::DelimiterDoesntMatch => write!(fmter, "The delimiters do not match"),
            ErrorKind::BufferEmpty => write!(fmter, "The buffer is empty"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&error::Error> {
        None
    }

    fn description(&self) -> &str {
        "Error caused by message processing"
    }
}

#[derive(Debug)]
pub enum ErrorKind {
    BufferDoesntContainDelimiter,
    BeginningDoesntMatch,
    DelimiterDoesntMatch,
    BufferEmpty,
    NotUTF8(string::FromUtf8Error),
    IOError(io::Error),
    IntParseError(num::ParseIntError),
}

impl PartialEq for ErrorKind {
    fn eq(&self, other: &Self) -> bool {
        let me = match *self {
            ErrorKind::IntParseError(_) => 0,
            ErrorKind::IOError(_) => 1,
            ErrorKind::NotUTF8(_) => 2,

            ErrorKind::BufferDoesntContainDelimiter => 3,
            ErrorKind::BeginningDoesntMatch => 4,
            ErrorKind::DelimiterDoesntMatch => 5,
            ErrorKind::BufferEmpty => 6,
        };
        let them = match *other {
            ErrorKind::IntParseError(_) => 0,
            ErrorKind::IOError(_) => 1,
            ErrorKind::NotUTF8(_) => 2,

            ErrorKind::BufferDoesntContainDelimiter => 3,
            ErrorKind::BeginningDoesntMatch => 4,
            ErrorKind::DelimiterDoesntMatch => 5,
            ErrorKind::BufferEmpty => 6,
        };
        me == them
    }
}