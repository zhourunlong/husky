use super::*;

#[salsa::tracked(jar = RawSignatureJar,return_ref)]
pub fn union_ty_raw_signature(
    db: &dyn RawSignatureDb,
    decl: UnionTypeDecl,
) -> RawSignatureResult<UnionTypeRawSignature> {
    let expr_region = decl.expr_region(db);
    let raw_signature_term_region = raw_signature_term_region(db, expr_region);
    let raw_term_menu = db
        .raw_term_menu(expr_region.toolchain(db))
        .as_ref()
        .unwrap();
    let implicit_parameters = ImplicitParameterRawSignatures::from_decl(
        decl.implicit_parameters(db)?,
        &raw_signature_term_region,
        raw_term_menu,
    );
    Ok(UnionTypeRawSignature::new(db, implicit_parameters))
}

#[salsa::interned(db = RawSignatureDb, jar = RawSignatureJar)]
pub struct UnionTypeRawSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterRawSignatures,
}

impl UnionTypeRawSignature {}