use crate::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub(crate) fn ty_as_trai_associated_function_raw_signature(
    db: &dyn RawSignatureDb,
    decl: TypeAsTraitAssociatedFunctionDecl,
) -> RawSignatureResult<TypeAsTraitAssociatedFunctionRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    Ok(TypeAsTraitAssociatedFunctionRawSignature::new(db, todo!()))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct TypeAsTraitAssociatedFunctionRawSignature {
    pub return_ty: RawTerm,
}