mod accessibility;
mod ancestry;

pub use accessibility::*;
pub use ancestry::*;

use super::*;

#[salsa::interned(jar = VfsJar)]
pub struct ModulePath {
    pub data: ModulePathData,
}

impl ModulePath {
    pub fn new_root(db: &dyn VfsDb, crate_path: CratePath) -> Self {
        Self::new(db, ModulePathData::Root(crate_path))
    }

    pub fn new_child(db: &dyn VfsDb, parent: ModulePath, ident: Identifier) -> Self {
        Self::new(db, ModulePathData::Child { parent, ident })
    }

    pub fn toolchain(self, db: &dyn VfsDb) -> Toolchain {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ModulePathData {
    Root(CratePath),
    Child {
        parent: ModulePath,
        ident: Identifier,
    },
}

impl<Db: VfsDb> salsa::DebugWithDb<Db> for ModulePath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &Db,
        include_all_fields: bool,
    ) -> std::fmt::Result {
        todo!()
    }
}
