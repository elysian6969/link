use std::borrow::Cow;
use std::ffi::{CStr, CString, OsStr};
use std::mem;
use std::os::unix::ffi::OsStrExt;

const EMPTY: Cow<'static, CStr> =
    Cow::Borrowed(unsafe { CStr::from_bytes_with_nul_unchecked(b"\0") });

/// Convert bytes into a C string.
///
/// Returns `None` when the provided bytes contain a null not at the end.
#[inline]
pub fn bytes_to_cstr_cow<'a>(bytes: &'a [u8]) -> Option<Cow<'a, CStr>> {
    let cow = match bytes.last() {
        None => EMPTY,
        Some(&0) => {
            let cstr = CStr::from_bytes_with_nul(bytes).ok()?;

            Cow::Borrowed(cstr)
        }
        Some(_) => {
            let cstring = CString::new(bytes).ok()?;

            Cow::Owned(cstring)
        }
    };

    Some(cow)
}

/// Convert `string` to bytes.
#[inline]
pub fn osstr_as_bytes<'a, S>(string: S) -> &'a [u8]
where
    S: AsRef<OsStr> + 'a,
{
    unsafe { mem::transmute(string.as_ref().as_bytes()) }
}

/// Convert `string` to a C string.
///
/// Returns `None` when the provided bytes contain a null not at the end.
#[inline]
pub fn osstr_to_cstr_cow<'a, S>(string: S) -> Option<Cow<'a, CStr>>
where
    S: AsRef<OsStr> + 'a,
{
    let bytes = osstr_as_bytes(string);
    let cow = bytes_to_cstr_cow(bytes);

    cow
}

/// Convert `string` to a C string and invoke `f` with a reference to the C string.
#[inline]
pub fn with_osstr_to_cstr<S, F, O>(string: S, f: F) -> Option<O>
where
    S: AsRef<OsStr>,
    F: FnOnce(&CStr) -> O,
{
    let cow = osstr_to_cstr_cow(string)?;

    Some(f(&*cow))
}
