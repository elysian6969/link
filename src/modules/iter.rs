use std::collections::hash_map;
use std::ffi::OsStr;

pub struct Iter<'a> {
    iter: hash_map::Iter<'a, Box<OsStr>, &'static [u8]>,
}

impl<'a> Iter<'a> {
    #[inline]
    pub(crate) fn new(iter: hash_map::Iter<'a, Box<OsStr>, &'static [u8]>) -> Self {
        Self { iter }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (&'a OsStr, &'static [u8]);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(name, bytes)| (&**name, *bytes))
    }
}
