// TODO: actual errors, lol !

use std::fmt;

pub enum OpenError {
    UndefinedSymbol, //(Box<CStr>),
}

impl OpenError {
    pub(crate) fn from_libloading(_error: libloading::Error) -> OpenError {
        OpenError::UndefinedSymbol //(Default::default())
    }
}

impl fmt::Debug for OpenError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenError::UndefinedSymbol => write!(fmt, "undefined symbol"),
        }
    }
}
