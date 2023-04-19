use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct TraitMethodFnSignature {
    pub return_ty: DeclarativeTerm,
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub(crate) fn trai_method_fn_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: TraitMethodFnDecl,
) -> DeclarativeSignatureResult<TraitMethodFnSignature> {
    let expr_region = decl.expr_region(db);
    let _declarative_term_region = declarative_term_region(db, expr_region);
    let _declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    Ok(TraitMethodFnSignature::new(db, todo!()))
}
