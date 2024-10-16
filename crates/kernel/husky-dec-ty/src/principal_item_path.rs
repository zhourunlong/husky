mod form;
mod ty_instance_constructor;
pub mod ty_variant;
mod utils;

pub use self::form::*;
pub use self::ty_instance_constructor::*;

use self::utils::*;
use crate::*;
use husky_dec_signature::{jar::DecSignatureDb, signature::HasDecTemplate};
use husky_entity_path::path::{
    major_item::{trai::TraitPath, ty::TypePath, MajorItemPath},
    ItemPath,
};
use husky_syn_decl::decl::HasSynDecl;

#[inline(always)]
pub fn declarative_term_item_path_declarative_ty(
    _db: &::salsa::Db,
    path: DecItemPath,
) -> DeclarativeTypeResult<DecTerm> {
    match path {
        DecItemPath::Form(_) => todo!(),
        DecItemPath::Trait(_) => todo!(),
        DecItemPath::Type(_) => todo!(),
        DecItemPath::TypeVariant(_) => todo!(),
    }
}

#[salsa::tracked(jar = DecTypeJar)]
pub fn ty_ontology_path_declarative_ty(
    db: &::salsa::Db,
    path: TypePath,
) -> DeclarativeTypeResult<DecTerm> {
    let dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => return Err(DerivedDeclarativeTypeError::SignatureError.into()),
    };
    let Ok(variances) = ty_path_variances(db, path) else {
        todo!()
    };
    curry_from_template_parameters(
        db,
        path.toolchain(db),
        CurryKind::Explicit,
        variances,
        signature.template_parameters(db),
        dec_term_menu.ty0(),
    )
}

#[test]
fn ty_ontology_path_declarative_ty_works() {
    DB::ast_rich_test_debug_with_db(
        |db, module_path: husky_vfs::path::module_path::ModulePath| {
            husky_entity_tree::helpers::paths::module_item_paths(db, module_path)
                .iter()
                .filter_map(|&module_item_path| match module_item_path {
                    ItemPath::MajorItem(MajorItemPath::Type(ty_path)) => Some((
                        ty_path,
                        ty_ontology_path_declarative_ty(db, ty_path).map(|t| {
                            let name_map = db
                                .syn_expr_dec_term_region(
                                    ty_path
                                        .syn_decl(db)
                                        .expect("should be okay at this branch")
                                        .syn_expr_region(db),
                                )
                                .symbolic_variable_region()
                                .symbol_name_map();
                            t.with_symbol_source_map(name_map)
                        }),
                    )),
                    _ => None,
                })
                .collect::<Vec<_>>()
        },
        &AstTestConfig::new(
            "ty_ontology_path_declarative_ty",
            FileExtensionConfig::Markdown,
            TestDomainsConfig::KERNEL,
        ),
    );
}

#[salsa::tracked(jar = DecTypeJar)]
pub fn trai_path_declarative_ty(
    db: &::salsa::Db,
    path: TraitPath,
) -> DeclarativeTypeResult<DecTerm> {
    let dec_term_menu = db.dec_term_menu(path.toolchain(db)).unwrap();
    let Ok(variances) = trai_item_variances(db, path) else {
        todo!()
    };
    let signature = match path.dec_template(db) {
        Ok(signature) => signature,
        Err(_) => todo!(),
    };
    curry_from_template_parameters(
        db,
        path.toolchain(db),
        CurryKind::Explicit,
        variances,
        signature.template_parameters_without_self_ty(db),
        dec_term_menu.trai_ty(),
    )
}
