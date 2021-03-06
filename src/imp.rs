use crate::{ffi, OpenError};
use libloading::os::unix;
use std::ffi::OsStr;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

const IS_LOADED: libc::c_int = libc::RTLD_NOLOAD | unix::RTLD_LAZY;
const LOAD: libc::c_int = unix::RTLD_LAZY | unix::RTLD_GLOBAL;

#[inline]
fn into_handle(handle: unix::Library) -> NonNull<u8> {
    let handle = handle.into_raw().cast::<u8>();
    // SAFETY: `handle.into_raw()` will pretty much always be non-null
    let handle = unsafe { NonNull::new_unchecked(handle) };

    handle
}

#[inline]
pub unsafe fn into_libloading(handle: NonNull<u8>) -> ManuallyDrop<unix::Library> {
    ManuallyDrop::new(unix::Library::from_raw(handle.as_ptr().cast()))
}

#[inline]
pub unsafe fn is_loaded(library: &OsStr) -> bool {
    unix::Library::open(Some(library), IS_LOADED)
        .map(move |_| true)
        .unwrap_or(false)
}

#[inline]
pub unsafe fn load(name: &OsStr) -> Result<NonNull<u8>, OpenError> {
    unix::Library::open(Some(name), LOAD)
        .map(into_handle)
        .map_err(OpenError::from_libloading)
}

#[inline]
pub unsafe fn close(handle: NonNull<u8>) {
    let library = into_libloading(handle);
    let _library = ManuallyDrop::into_inner(library);
}

#[inline]
pub unsafe fn symbol(handle: NonNull<u8>, name: &OsStr) -> Option<*const u8> {
    let result = ffi::with_osstr_to_cstr(name, |cstr| {
        let library = into_libloading(handle);
        let name = cstr.to_bytes_with_nul();
        let symbol = library.get::<*const u8>(name).ok()?;
        let address = symbol.into_raw().as_const().cast::<u8>();
        let not_null = !address.is_null();

        not_null.then(|| address)
    });

    result?
}
