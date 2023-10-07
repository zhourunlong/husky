pub mod db;

use std::path::Path;

use self::db::*;
use husky_task::{helpers::DevLinkageTable, linkage::IsLinkTime, IsTask};
use husky_vfs::{CratePath, DiffPathBuf};

pub struct DevComptime<Task: IsTask> {
    db: ComptimeDb,
    linkage_table: DevLinkageTable<Task>,
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn new(target_crate: &Path) -> Self {
        let db = Default::default();
        Self {
            db,
            linkage_table: IsLinkTime::new_linkage_table(todo!(), todo!()),
        }
    }
}

impl<Task: IsTask> DevComptime<Task> {
    pub fn db(&self) -> &ComptimeDb {
        &self.db
    }
}

impl<Task: IsTask> Default for DevComptime<Task> {
    fn default() -> Self {
        Self {
            db: Default::default(),
            linkage_table: Default::default(),
        }
    }
}
