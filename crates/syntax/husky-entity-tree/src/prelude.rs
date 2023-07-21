use crate::*;
use husky_manifest::{HasPackageManifest, ManifestError};
use husky_print_utils::p;
use husky_vfs::*;
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub enum PreludeError {
    #[error("{0}")]
    Toolchain(#[from] ToolchainError),
    #[error("core prelude")]
    CorePreludeEntityTreeSheet(Box<EntityTreeError>),
    #[error("manifest error")]
    ManifestError,
    #[error("vfs error {0}")]
    VfsError(#[from] VfsError),
}
pub type PreludeResult<T> = Result<T, PreludeError>;

impl From<&PreludeError> for PreludeError {
    fn from(value: &PreludeError) -> Self {
        todo!()
    }
}

impl From<&ManifestError> for PreludeError {
    fn from(value: &ManifestError) -> Self {
        todo!()
    }
}

#[test]
fn crate_symbol_context_works() {
    DB::default().ast_plain_test("crate-symbol-context", |db, crate_path| {
        let crate_symbol_context = CratePrelude::new(db, crate_path).unwrap();
        let root_module_path = crate_path.root_module_path(db);
        let t = |path: EntityPath| {
            let symbol =
                match crate_symbol_context.resolve_ident(db, root_module_path, path.ident(db)) {
                    Some(symbol) => symbol,
                    None => panic!(
                        r#"crate symbol context should contain {:?}
    crate symbol context is
    {:?}"#,
                        &path.debug(db),
                        crate_symbol_context.debug(db)
                    ),
                };
            if path != symbol.path(db).into() {
                panic!(
                    "symbol.path(db)({:?}) should be equal to path({:?})",
                    symbol.path(db).debug(db),
                    path.debug(db)
                )
            }
        };
        let toolchain = crate_path.toolchain(db);
        let _vfs_path_menu = db.vfs_path_menu(toolchain);
        let entity_path_menu = db.entity_path_menu(toolchain);
        t(entity_path_menu.bool_ty_path().into());
        t(entity_path_menu.i32_ty_path().into());
        t(entity_path_menu.i64_ty_path().into());
        t(entity_path_menu.f32_ty_path().into());
        t(entity_path_menu.f64_ty_path().into());
    })
}
