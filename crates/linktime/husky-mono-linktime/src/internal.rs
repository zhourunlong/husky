mod libgen;
mod mapgen;

use crate::*;
use husky_hir_deps::HasDeps;
use husky_vfs::CratePath;

use self::libgen::generate_library;
use self::mapgen::generate_map;

pub struct MonoLinkTimeInternal<ComptimeDb, Linkage>
where
    ComptimeDb: HirDepsDb,
    Linkage: IsLinkage,
{
    target_crate: CratePath,
    library_storage: MonoLibraryStorage,
    map: HashMap<LinkagePath, (HirLinkageDeps, Linkage)>,
    _marker: PhantomData<ComptimeDb>,
}

impl<Db, Linkage> Default for MonoLinkTimeInternal<Db, Linkage>
where
    Db: HirDepsDb,
    Linkage: IsLinkage,
{
    fn default() -> Self {
        Self {
            target_crate: None,
            library_storage: todo!(),
            map: todo!(),
            _marker: PhantomData,
        }
    }
}

pub struct MonoLibraryStorage {}

impl<ComptimeDb, Linkage: IsLinkage> MonoLinkTimeInternal<ComptimeDb, Linkage>
where
    ComptimeDb: HirDepsDb,
    Linkage: IsLinkage,
{
    pub(crate) fn new(target_crate: CratePath, db: &ComptimeDb) -> Self {
        let library_storage = generate_library(target_crate, db);
        let map = generate_map(target_crate, &library_storage, db);
        Self {
            target_crate: Some(target_crate),
            library_storage,
            map,
            _marker: PhantomData,
        }
    }

    pub(crate) fn get_linkage(&self, key: LinkagePath, db: &ComptimeDb) -> Option<Linkage> {
        let (deps, linkage) = self.map.get(&key).copied().expect("todo");
        (deps == key.deps(db)).then_some(linkage)
    }

    /// still need the key to avoid redundant reload when two attempts simultaneously want to lock
    pub(crate) fn get_linkage_with_reload(&mut self, key: LinkagePath, db: &ComptimeDb) -> Linkage {
        let (deps, linkage) = self.map.get(&key).copied().expect("todo");
        if deps == key.deps(db) {
            return linkage;
        }
        todo!("reload")
    }

    fn reload(&mut self, db: &dyn HirDepsDb) {
        self.library_storage = generate_library(self.target_crate, db);
        self.map = generate_map(self.target_crate, &self.library_storage, db)
    }
}
