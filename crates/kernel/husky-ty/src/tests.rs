use husky_valid_ty::ValidTypeJar;
pub(crate) use husky_vfs::VfsTestUtils;

use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_entity_path::{EntityPathJar, ModuleItemPath, TypePath};
use husky_entity_tree::{EntityTreeDb, EntityTreeJar};
use husky_expr::ExprJar;
use husky_manifest::ManifestJar;
use husky_precise_term::PreciseTermJar;
use husky_precise_ty::PreciseTypeJar;
use husky_print_utils::p;
use husky_raw_term::RawTermJar;
use husky_raw_ty::{RawTypeDb, RawTypeJar};
use husky_signature::SignatureJar;
use husky_term::TermJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_valid_term::ValidTermJar;
use husky_vfs::*;
use husky_word::WordJar;

#[salsa::db(
    WordJar,
    VfsJar,
    EntityPathJar,
    TokenJar,
    AstJar,
    EntityTreeJar,
    ManifestJar,
    ExprJar,
    DeclJar,
    TermPreludeJar,
    RawTermJar,
    SignatureJar,
    RawTypeJar,
    PreciseTermJar,
    PreciseTypeJar,
    ValidTermJar,
    ValidTypeJar,
    TermJar,
    TypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}

fn entity_tys(db: &DB, module_path: ModulePath) -> Vec<(EntityPath, TypeResult<Term>)> {
    let Ok(entity_tree_sheet) = db.entity_tree_sheet(module_path) else {
        return vec![]
    };
    entity_tree_sheet
        .module_item_path_iter(db)
        .map(|path| {
            (
                path.into(),
                entity_path_ty(db, TypePathDisambiguation::Ontology, path.into()),
            )
        })
        .collect()
}

#[test]
fn entity_tys_works() {
    DB::default().vfs_expect_test_debug_with_db("entity_tys", entity_tys)
}
