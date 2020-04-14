use std::error;
use std::fmt;
use std::io;
use std::string;

use crate::enums;

#[derive(Debug)]
pub enum FlicError {
    Unmarshal(UnmarshalError),
    FlicD(io::Error),
    Generic(String),
}

impl fmt::Display for FlicError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            FlicError::Unmarshal(ref err) => write!(f, "failed during unmarshaling: {}", err),
            FlicError::FlicD(ref err) => write!(f, "failed communicating with flicd: {}", err),
            FlicError::Generic(ref err) => write!(f, "{}", err),
        }
    }
}

impl error::Error for FlicError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            FlicError::Unmarshal(ref err) => Some(err),
            FlicError::FlicD(ref err) => Some(err),
            FlicError::Generic(_) => None,
        }
    }
}

impl From<UnmarshalError> for FlicError {
    fn from(err: UnmarshalError) -> FlicError {
        FlicError::Unmarshal(err)
    }
}

impl From<io::Error> for FlicError {
    fn from(err: io::Error) -> FlicError {
        FlicError::FlicD(err)
    }
}

impl From<String> for FlicError {
    fn from(err: String) -> FlicError {
        FlicError::Generic(err)
    }
}

#[derive(Debug)]
pub enum UnmarshalError {
    BadLength(usize, usize),
    BadLengthAtLeast(usize, usize),
    BadString(string::FromUtf8Error),
    BadEnum(u8, String),
    BadOpcode(u8),
    BadClickType(enums::ClickType, String),
}

impl fmt::Display for UnmarshalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            UnmarshalError::BadLength(got_len, want_len) => {
                write!(f, "body length {}, expected {}", got_len, want_len)
            }
            UnmarshalError::BadLengthAtLeast(got_len, want_at_least_len) => write!(
                f,
                "body length {}, expected at least {}",
                got_len, want_at_least_len
            ),
            UnmarshalError::BadString(err) => write!(f, "string was not valid UTF-8: {}", err),
            UnmarshalError::BadEnum(field, val) => {
                write!(f, "enum value {} is invalid for enum {}", field, val)
            }
            UnmarshalError::BadOpcode(opcode) => write!(f, "unknown opcode {:?}", opcode),
            UnmarshalError::BadClickType(click_type, btn_evt) => {
                write!(f, "click type {:?} not valid for {}", click_type, btn_evt)
            }
        }
    }
}

impl error::Error for UnmarshalError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            UnmarshalError::BadString(ref err) => Some(err),
            _ => None,
        }
    }
}
