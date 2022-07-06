use super::iterate;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::ffi::OsStr;

/// Map of cached libraries.
pub struct Map {
    inner: RwLock<HashMap<Box<OsStr>, &'static [u8]>>,
}

impl Map {
    /// Construct a new map
    #[inline]
    pub fn new() -> Self {
        let mut map = HashMap::new();

        iterate(|name, bytes| {
            map.insert(name, bytes);
        });

        let inner = RwLock::new(map);

        Self { inner }
    }

    /// Update the contents of the map
    #[inline]
    pub fn update(&self) {
        let mut guard = self.inner.write();

        guard.clear();

        iterate(|name, bytes| {
            guard.insert(name, bytes);
        });
    }

    #[inline]
    pub fn get<S>(&self, name: S) -> &'static [u8]
    where
        S: AsRef<OsStr>,
    {
        const EMPTY: &[u8] = &[];

        self.inner.read().get(name.as_ref()).unwrap_or(&EMPTY)
    }
}
