use findshlibs::{SharedLibrary, TargetSharedLibrary};
use iter::Iter;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;
use std::{ptr, slice};

mod iter;

static MODULES: Lazy<RwLock<Modules>> = Lazy::new(Modules::load);

pub struct Modules {
    pub modules: HashMap<Box<OsStr>, &'static [u8]>,
}

impl Modules {
    #[inline]
    fn update(&mut self) {
        iterate(|(name, bytes)| {
            self.modules.insert(name, bytes);
        });
    }

    #[inline]
    pub fn iter(&self) -> Iter<'_> {
        let iter = self.modules.iter();

        Iter::new(iter)
    }

    #[inline]
    pub fn force() -> &'static RwLock<Modules> {
        let has_loaded = Lazy::get(&MODULES).is_some();

        if has_loaded {
            let modules = Lazy::force(&MODULES);
            let mut writer = modules.write();

            writer.update();

            modules
        } else {
            Lazy::force(&MODULES)
        }
    }

    #[inline]
    pub(crate) fn load() -> RwLock<Modules> {
        let mut modules = HashMap::new();

        iterate(|(name, bytes)| {
            modules.insert(name, bytes);
        });

        RwLock::new(Modules { modules })
    }
}

fn iterate<F>(mut f: F)
where
    F: FnMut((Box<OsStr>, &'static [u8])),
{
    TargetSharedLibrary::each(|library| {
        let name = Path::new(library.name());
        let name = match name.file_name() {
            Some(name) => name,
            None => return,
        };

        let name = Box::from(name);
        let address = ptr::from_exposed_addr(library.actual_load_addr().0);
        let len = library.len();
        let bytes = unsafe { slice::from_raw_parts(address, len) };

        f((name, bytes));
    });
}
