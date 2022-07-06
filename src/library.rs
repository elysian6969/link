use crate::{ffi, Cache, OpenError};
use libloading::os::unix;
use std::ffi::OsStr;
use std::mem;
use std::mem::ManuallyDrop;
use std::ptr::NonNull;

pub struct Library {
    name: Box<OsStr>,
    handle: NonNull<u8>,
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

        // ensure library cache is updated
        let cache = Cache::load();

        cache.update();

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

    /// Returns the name of this library.
    #[inline]
    pub fn name(&self) -> &OsStr {
        &*self.name
    }

    /// Return a slice to the memory this libray consumes.
    ///
    /// # Safety
    ///
    /// The memory this slice points to may not be valid, nor may it be accessible.
    #[inline]
    pub unsafe fn bytes(&self) -> &'static [u8] {
        let cache = Cache::load();

        cache.get(self.name())
    }

    /// Returns the symbol identified by `name`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// type MallocFn = unsafe extern "C" fn(len: usize) -> *mut u8;
    ///
    /// let malloc: MallocFn = library.symbol("malloc")?;
    /// let alloc = malloc(4);
    /// ```
    ///
    /// # Safety
    ///
    /// Caller is responsible for ensuring `T` is the correct type.
    #[inline]
    pub unsafe fn symbol<S, T>(&self, name: S) -> Option<T>
    where
        S: AsRef<OsStr>,
    {
        let name = name.as_ref();

        symbol(self.handle, name)
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
unsafe fn load(name: &OsStr) -> Result<NonNull<u8>, OpenError> {
    unix::Library::open(Some(name), LOAD)
        .map(into_handle)
        .map_err(OpenError::from_libloading)
}

#[inline]
unsafe fn is_loaded(library: &OsStr) -> bool {
    unix::Library::open(Some(library), IS_LOADED)
        .map(move |_| true)
        .unwrap_or(false)
}

#[inline]
unsafe fn into_libloading(handle: NonNull<u8>) -> ManuallyDrop<unix::Library> {
    ManuallyDrop::new(unix::Library::from_raw(handle.as_ptr().cast()))
}

#[inline]
unsafe fn symbol<T>(handle: NonNull<u8>, name: &OsStr) -> Option<T> {
    let result = ffi::with_osstr_to_cstr(name, |cstr| {
        let library = into_libloading(handle);

        match library.get::<Option<T>>(cstr.to_bytes_with_nul()) {
            Ok(symbol) => {
                // check if the pointer is null
                let symbol = symbol.lift_option()?;

                // convert to T
                let data = symbol.into_raw();
                let data = mem::transmute_copy(&data);

                Some(data)
            }
            Err(_error) => None,
        }
    });

    result?
}
