use crate::{
    helpers::paths::module_item_syn_node_paths,
    node::{
        assoc_item::AssocItemSynNodePath, impl_block::ImplBlockSynNodePath, HasSynNodePath,
        ItemSynNodePath, ItemSynNodePathId,
    },
    region_path::SynNodeRegionPath,
    Jar,
};
use husky_entity_path::path::{
    impl_block::TypeSketch, major_item::ty::TypePath, ItemPath, ItemPathId,
};
use husky_path_utils::{Path, PathBuf};
use husky_vfs::{
    path::{crate_path::CratePath, module_path::ModulePath, package_path::PackagePath},
    test_utils::*,
};

impl IsVfsTestUnit<Jar> for ItemSynNodePath {
    fn collect_from_package_path_aux(
        db: &salsa::Db,
        package_path: PackagePath,
    ) -> impl Iterator<Item = Self> {
        ModulePath::collect_from_package_path_aux(db, package_path)
            .into_iter()
            .map(|module_path| module_item_syn_node_paths(db, module_path))
            .flatten()
            .copied()
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &std::path::Path,
        config: &VfsTestConfig,
    ) -> std::path::PathBuf {
        let stem = self.stem(package_expect_files_dir, config.test_name(), db);
        stem.with_extension(config.expect_file_extension().str())
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &std::path::Path,
        config: &VfsTestConfig,
    ) -> Option<std::path::PathBuf> {
        let stem = self.stem(package_adversarials_dir, config.test_name(), db);
        Some(stem.with_extension(format!(
            "{}.{}",
            adversarial_kind.as_str(),
            config.adversarial_extension()
        )))
    }

    fn vfs_test_unit_downcast_as_module_path(
        self,
    ) -> Option<husky_vfs::path::module_path::ModulePath> {
        None
    }
}

impl IsVfsTestUnit<Jar> for SynNodeRegionPath {
    fn collect_from_package_path_aux(
        db: &salsa::Db,
        package_path: PackagePath,
    ) -> impl Iterator<Item = Self> {
        CratePath::collect_from_package_path_aux(db, package_path)
            .map(|crate_path| SynNodeRegionPath::CrateDecl(crate_path))
            .chain(
                ItemSynNodePath::collect_from_package_path_aux(db, package_path)
                    .into_iter()
                    .map(|syn_node_path| {
                        [
                            SynNodeRegionPath::ItemDecl(syn_node_path),
                            SynNodeRegionPath::ItemDefn(syn_node_path),
                        ]
                    })
                    .flatten(),
            )
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &husky_path_utils::Path,
        config: &VfsTestConfig,
    ) -> husky_path_utils::PathBuf {
        match self {
            SynNodeRegionPath::CrateDecl(crate_path) => {
                crate_path.determine_expect_file_path(db, package_expect_files_dir, config)
            }
            SynNodeRegionPath::ItemDecl(syn_node_path) => syn_node_path
                .stem(package_expect_files_dir, config.test_name(), db)
                .with_extension(format!("decl.{}", config.expect_file_extension().str())),
            SynNodeRegionPath::ItemDefn(syn_node_path) => syn_node_path
                .stem(package_expect_files_dir, config.test_name(), db)
                .with_extension(format!("defn.{}", config.expect_file_extension().str())),
        }
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &husky_path_utils::Path,
        config: &VfsTestConfig,
    ) -> Option<husky_path_utils::PathBuf> {
        match self {
            SynNodeRegionPath::CrateDecl(crate_path) => crate_path.determine_adversarial_path(
                db,
                adversarial_kind,
                package_adversarials_dir,
                config,
            ),
            SynNodeRegionPath::ItemDecl(syn_node_path) => Some(
                syn_node_path
                    .stem(package_adversarials_dir, config.test_name(), db)
                    .with_extension(format!(
                        "decl.{}.{}",
                        adversarial_kind.as_str(),
                        config.adversarial_extension(),
                    )),
            ),
            SynNodeRegionPath::ItemDefn(syn_node_path) => Some(
                syn_node_path
                    .stem(package_adversarials_dir, config.test_name(), db)
                    .with_extension(format!(
                        "defn.{}.{}",
                        adversarial_kind.as_str(),
                        config.adversarial_extension(),
                    )),
            ),
        }
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        None
    }
}

impl IsVfsTestUnit<Jar> for ItemPath {
    fn collect_from_package_path_aux(
        db: &salsa::Db,
        package_path: PackagePath,
    ) -> impl Iterator<Item = Self> {
        ItemSynNodePath::collect_from_package_path_aux(db, package_path)
            .filter_map(|item_syn_node_path| item_syn_node_path.unambiguous_item_path(db))
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        self.syn_node_path(db)
            .determine_expect_file_path(db, package_expect_files_dir, config)
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &Path,
        config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        self.syn_node_path(db).determine_adversarial_path(
            db,
            adversarial_kind,
            package_adversarials_dir,
            config,
        )
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        None
    }
}

impl IsVfsTestUnit<Jar> for ItemPathId {
    fn collect_from_package_path_aux(
        db: &salsa::Db,
        package_path: PackagePath,
    ) -> impl Iterator<Item = Self> {
        ItemPath::collect_from_package_path_aux(db, package_path).map(|item_path| *item_path)
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        self.item_path(db)
            .determine_expect_file_path(db, package_expect_files_dir, config)
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &Path,
        config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        self.item_path(db).determine_adversarial_path(
            db,
            adversarial_kind,
            package_adversarials_dir,
            config,
        )
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        None
    }
}

impl IsVfsTestUnit<Jar> for TypePath {
    fn collect_from_package_path_aux(
        db: &salsa::Db,
        package_path: PackagePath,
    ) -> impl Iterator<Item = Self> {
        ItemPath::collect_from_package_path_aux(db, package_path)
            .filter_map(|item_path| item_path.ty_path())
    }

    fn determine_expect_file_path(
        self,
        db: &salsa::Db,
        package_expect_files_dir: &Path,
        config: &VfsTestConfig,
    ) -> PathBuf {
        let slf: ItemPath = self.into();
        slf.determine_expect_file_path(db, package_expect_files_dir, config)
    }

    fn determine_adversarial_path(
        self,
        db: &salsa::Db,
        adversarial_kind: AdversarialKind,
        package_adversarials_dir: &Path,
        config: &VfsTestConfig,
    ) -> Option<PathBuf> {
        let slf: ItemPath = self.into();
        slf.determine_adversarial_path(db, adversarial_kind, package_adversarials_dir, config)
    }

    fn vfs_test_unit_downcast_as_module_path(self) -> Option<ModulePath> {
        None
    }
}
