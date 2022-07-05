use crate::{Modules, OpenError};
use libloading::os::unix;
use std::ffi::OsStr;
use std::ptr::NonNull;

pub struct Library {
    pub name: Box<OsStr>,
    pub handle: NonNull<u8>,
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Library {
    /// Find and load an executable object file.
    ///
    /// # Safety
    ///
    /// No safety guarentees can be made due to the execution of library initialisation routines.
    #[inline]
    pub unsafe fn load<S>(name: S) -> Result<Self, OpenError>
    where
        S: AsRef<OsStr>,
    {
        let name = name.as_ref();
        let library = load(name)?;

        Ok(Self::from_raw(name.into(), library))
    }

    /// Determine whether an executable object file is loaded.
    #[inline]
    pub fn is_loaded<S>(name: S) -> bool
    where
        S: AsRef<OsStr>,
    {
        let name = name.as_ref();

        unsafe { is_loaded(name) }
    }

    /// Convert a raw handle to a `Library`.
    ///
    /// # Safety
    ///
    /// The raw handle must have been returned by a successful call to `dlopen`.
    #[inline]
    pub unsafe fn from_raw(name: Box<OsStr>, handle: NonNull<u8>) -> Self {
        Self { name, handle }
    }

    pub fn name(&self) -> &OsStr {
        &*self.name
    }

    /// Return a slice to the memory this libray consumes.
    ///
    /// # Safety
    ///
    /// The memory this slice points to may not be valid, nor may it be accessible.
    #[inline]
    pub unsafe fn bytes(&self) -> &[u8] {
        let modules = Modules::force().read();
        let this_name = self.name();

        for (name, bytes) in modules.iter() {
            if this_name == name {
                return bytes;
            }
        }

        &[]
    }
}

#[inline]
fn into_handle(handle: unix::Library) -> NonNull<u8> {
    let handle = handle.into_raw().cast::<u8>();
    // SAFETY: `handle.into_raw()` will pretty much always be non-null
    let handle = unsafe { NonNull::new_unchecked(handle) };

    handle
}

const IS_LOADED: libc::c_int = libc::RTLD_NOLOAD;
const LOAD: libc::c_int = unix::RTLD_LAZY | unix::RTLD_LOCAL;

#[inline]
unsafe fn load(library: &OsStr) -> Result<NonNull<u8>, OpenError> {
    unix::Library::open(Some(library), LOAD)
        .map(into_handle)
        .map_err(OpenError::from_libloading)
}

#[inline]
unsafe fn is_loaded(library: &OsStr) -> bool {
    unix::Library::open(Some(library), IS_LOADED)
        .map(move |_| true)
        .unwrap_or(false)
}
