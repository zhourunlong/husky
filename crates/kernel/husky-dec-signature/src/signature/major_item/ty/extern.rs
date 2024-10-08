use super::*;
use husky_syn_decl::decl::major_item::ty::r#extern::ExternSynDecl;

#[salsa::interned]
pub struct ExternDecTemplate {
    #[return_ref]
    pub template_parameters: DecTemplateParameters,
}

impl ExternDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        path: TypePath,
        decl: ExternSynDecl,
    ) -> DecSignatureResult<Self> {
        let syn_expr_region = decl.syn_expr_region(db);
        let dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        Ok(Self::new(
            db,
            DecTemplateParameters::from_decl(
                decl.template_parameters(db),
                &dec_term_region,
                dec_term_menu,
            ),
        ))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db]
pub struct ExternDecSignature {}
