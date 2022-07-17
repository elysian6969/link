// TODO: actual errors, lol !

use std::ffi::{OsStr, OsString};
use std::fmt;

pub enum OpenError {
    InvalidArchitecture(Box<OsStr>),
    MissingDependency(Box<OsStr>),
    UndefinedSymbol(Box<OsStr>),
}

const CANNOT_OPEN_SHARED_OBJECT: &str = "cannot open shared object file";
const NO_SUCH_FILE_OR_DIRECTORY: &str = "No such file or directory";
const WRONG_ELF_CLASS: &str = "wrong ELF class";
const ELF_CLASS_32: &str = "ELFCLASS32";

fn into_boxed_osstr<S>(string: S) -> Box<OsStr>
where
    S: Into<OsString>,
{
    string.into().into_boxed_os_str()
}

fn from_desc(desc: Box<str>) -> Option<OpenError> {
    let mut iter = desc.rsplitn(3, ':');

    match (iter.next(), iter.next(), iter.next()) {
        (Some(error), Some(why), Some(library)) => {
            let error = error.trim();
            let why = why.trim();
            let library = into_boxed_osstr(library);

            //println!("{why} | {error} | {library:?}");

            match (why, error) {
                (CANNOT_OPEN_SHARED_OBJECT, NO_SUCH_FILE_OR_DIRECTORY) => {
                    return Some(OpenError::MissingDependency(library));
                }
                (WRONG_ELF_CLASS, ELF_CLASS_32) => {
                    return Some(OpenError::InvalidArchitecture(library));
                }
                _ => {}
            }
        }
        _ => {}
    }

    None
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Kind {
    Open,
}

fn map_error(error: libloading::Error) -> (Kind, Box<str>) {
    let desc = error.to_string().into_boxed_str();

    match error {
        libloading::Error::DlOpen { .. } => (Kind::Open, desc),
        _ => unimplemented!(),
    }
}

impl OpenError {
    pub(crate) fn from_libloading(error: libloading::Error) -> OpenError {
        let (kind, desc) = map_error(error);

        //println!("{kind:?} {desc:?}");

        match kind {
            Kind::Open => match from_desc(desc) {
                Some(error) => error,
                None => todo!(),
            },
            //_ => todo!("{kind:?} {desc}"),
        }
    }
}

impl fmt::Debug for OpenError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            OpenError::InvalidArchitecture(dependency) => {
                write!(fmt, "invalid architecture: {dependency:?}")
            }
            OpenError::MissingDependency(dependency) => {
                write!(fmt, "missing dependency: {dependency:?}")
            }
            OpenError::UndefinedSymbol(symbol) => write!(fmt, "undefined symbol: {symbol:?}"),
        }
    }
}
