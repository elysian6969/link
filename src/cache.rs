use findshlibs::{SharedLibrary, TargetSharedLibrary};
use map::Map;
use once_cell::sync::Lazy;
use std::ffi::OsStr;
use std::path::Path;
use std::{ptr, slice};

mod map;

static CACHE: Cache = Cache::new();

/// Library cache.
pub(crate) struct Cache {
    cache: Lazy<Map>,
}

impl Cache {
    #[inline]
    const fn new() -> Self {
        let cache = Lazy::new(Map::new as fn() -> Map);

        Self { cache }
    }

    #[inline]
    pub(crate) fn load() -> &'static Map {
        Lazy::force(&CACHE.cache)
    }
}

/// Iterate loaded libraries.
#[inline]
pub(crate) fn iterate<F>(mut f: F)
where
    F: FnMut(Box<OsStr>, &'static [u8]),
{
    TargetSharedLibrary::each(|library| {
        let name = name_of(library.name());
        let address = ptr::from_exposed_addr(library.actual_load_addr().0);
        let len = library.len();
        let bytes = unsafe { slice::from_raw_parts(address, len) };

        f(name, bytes);
    });
}

/// Returns the file name of a library path as a `Box<OsStr>`.
#[inline]
fn name_of(path: &OsStr) -> Box<OsStr> {
    let path = Path::new(path);

    // SAFETY: Library paths *always* have a file name.
    let name = unsafe { path.file_name().unwrap_unchecked() };

    Box::from(name)
}
