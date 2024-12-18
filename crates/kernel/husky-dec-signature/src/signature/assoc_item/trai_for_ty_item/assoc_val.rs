use super::*;
use husky_syn_decl::decl::assoc_item::trai_for_ty_item::assoc_val::TraitForTypeAssocValSynDecl;

#[salsa::interned]
pub struct TraitForTypeAssocValDecTemplate {}

impl TraitForTypeAssocValDecTemplate {
    pub(super) fn from_decl(
        db: &::salsa::Db,
        decl: TraitForTypeAssocValSynDecl,
    ) -> DecSignatureResult<TraitForTypeAssocValDecTemplate> {
        let syn_expr_region = decl.syn_expr_region(db);
        let _dec_term_region = syn_expr_dec_term_region(db, syn_expr_region);
        let _dec_term_menu = db.dec_term_menu(syn_expr_region.toolchain(db)).unwrap();
        Ok(TraitForTypeAssocValDecTemplate::new(db))
    }
}
