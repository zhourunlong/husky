use crate::*;

#[salsa::interned(db = DeclarativeSignatureDb, jar = DeclarativeSignatureJar)]
pub struct FnDeclarativeSignature {
    #[return_ref]
    pub implicit_parameters: ImplicitParameterDeclarativeSignatures,
    #[return_ref]
    pub parameters: ExplicitParameterDeclarativeSignatures,
    pub return_ty: DeclarativeTerm,
}

impl HasDeclarativeSignature for FnDecl {
    type DeclarativeSignature = FnDeclarativeSignature;

    fn declarative_signature(
        self,
        db: &dyn DeclarativeSignatureDb,
    ) -> DeclarativeSignatureResult<Self::DeclarativeSignature> {
        fn_declarative_signature(db, self)
    }
}

#[salsa::tracked(jar = DeclarativeSignatureJar)]
pub fn fn_declarative_signature(
    db: &dyn DeclarativeSignatureDb,
    decl: FnDecl,
) -> DeclarativeSignatureResult<FnDeclarativeSignature> {
    let expr_region = decl.expr_region(db);
    let expr_region_data = expr_region.data(db);
    let declarative_term_region = declarative_term_region(db, expr_region);
    let declarative_term_menu = db.declarative_term_menu(expr_region.toolchain(db)).unwrap();
    let implicit_parameters = ImplicitParameterDeclarativeSignatures::from_decl(
        decl.implicit_parameters(db),
        declarative_term_region,
        declarative_term_menu,
    );
    let parameters = ExplicitParameterDeclarativeSignatures::from_decl(
        decl.parameters(db),
        expr_region_data,
        declarative_term_region,
    )?;
    let return_ty = match decl.return_ty(db) {
        Some(return_ty) => declarative_term_region.expr_term(return_ty.expr())?,
        None => declarative_term_menu.unit(),
    };
    Ok(FnDeclarativeSignature::new(
        db,
        implicit_parameters,
        parameters,
        return_ty,
    ))
}

impl FnDeclarativeSignature {}
