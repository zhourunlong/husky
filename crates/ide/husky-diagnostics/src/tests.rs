use crate::*;
use husky_ast::AstJar;
use husky_decl::DeclJar;
use husky_defn::DefnJar;
use husky_entity_path::{EntityPathDb, EntityPathJar};
use husky_entity_tree::EntityTreeJar;
use husky_expr::ExprJar;
use husky_expr_ty::ExprTypeJar;
use husky_manifest::ManifestJar;
use husky_precise_term::PreciseTermJar;
use husky_precise_ty::PreciseTypeJar;
use husky_raw_term::RawTermJar;
use husky_raw_ty::RawTypeJar;
use husky_signature::SignatureJar;
use husky_term::TermJar;
use husky_term_prelude::TermPreludeJar;
use husky_token::TokenJar;
use husky_ty::TypeJar;
use husky_valid_term::ValidTermJar;
use husky_valid_ty::ValidTypeJar;
use husky_word::WordJar;

#[salsa::db(
    VfsJar,
    WordJar,
    TokenJar,
    EntityPathJar,
    ManifestJar,
    AstJar,
    EntityTreeJar,
    DeclJar,
    DefnJar,
    ExprJar,
    DiagnosticsJar,
    TermPreludeJar,
    RawTermJar,
    RawTypeJar,
    PreciseTermJar,
    PreciseTypeJar,
    ValidTermJar,
    ValidTypeJar,
    TermJar,
    TypeJar,
    SignatureJar,
    ExprTypeJar
)]
#[derive(Default)]
pub(crate) struct DB {
    storage: salsa::Storage<Self>,
}

impl salsa::Database for DB {}
