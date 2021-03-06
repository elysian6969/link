use crate::{cache, imp, Cache, OpenError};
use std::ffi::OsStr;
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
        let result = crate::ffi::with_osstr_to_cstr(name, |cstr| {
            let name = &*(cstr as *const std::ffi::CStr as *const [u8]);
            let name = &name[..(name.len() - 1)];
            let name = &*(name as *const [u8] as *const OsStr);
            let library = imp::load(name)?;

            // ensure library cache is updated
            let cache = Cache::load();

            cache.update();

            let name = cache::name_of(name).into();

            Ok(Self::from_raw(name, library))
        });

        result.unwrap()
    }

    /// Determine whether an executable object file is loaded.
    #[inline]
    pub fn is_loaded<S>(name: S) -> bool
    where
        S: AsRef<OsStr>,
    {
        let name = name.as_ref();

        unsafe { imp::is_loaded(name) }
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

    /// Returns a pointer to the symbol identified by `name`.
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
    pub unsafe fn symbol_ptr<S, T>(&self, name: S) -> Option<*const T>
    where
        S: AsRef<OsStr>,
    {
        let name = name.as_ref();
        let ptr = imp::symbol(self.handle, name)?;

        Some(ptr.cast())
    }

    /// Returns the value located at the symbol identified by `name`.
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
    pub unsafe fn symbol_read<S, T>(&self, name: S) -> Option<T>
    where
        S: AsRef<OsStr>,
    {
        let address: *const T = self.symbol_ptr(name)?;

        Some(address.read_unaligned())
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe {
            imp::close(self.handle);
        }

        // ensure library cache is updated
        let cache = Cache::load();

        cache.update();
    }
}
