use super::*;
use husky_eth_signature::signature::HasEthTemplate;
use husky_syn_decl::decl::assoc_item::trai_for_ty_item::method_ritchie::TraitForTypeMethodRitchieSynDecl;

#[salsa::interned]
pub struct TraitForTypeMethodRitchieHirDecl {
    pub path: TraitForTypeItemPath,
    #[return_ref]
    pub template_parameters: HirTemplateParameters,
    pub self_value_parameter: HirEagerSelfValueParameter,
    #[return_ref]
    pub parenate_parameters: HirEagerParenateParameters,
    pub return_ty: HirType,
    pub hir_eager_expr_region: HirEagerExprRegion,
}

impl TraitForTypeMethodRitchieHirDecl {
    pub(super) fn from_syn(
        path: TraitForTypeItemPath,
        syn_decl: TraitForTypeMethodRitchieSynDecl,
        db: &::salsa::Db,
    ) -> Self {
        let builder = HirDeclBuilder::new(syn_decl.syn_expr_region(db), db);
        let template_parameters =
            HirTemplateParameters::from_syn(syn_decl.template_parameters(db), &builder);
        let self_ty =
            HirType::from_eth(path.eth_template(db).unwrap().self_ty(db).unwrap(), db).unwrap();
        let self_value_parameter =
            HirEagerSelfValueParameter::from_syn(self_ty, syn_decl.self_value_parameter(db));
        let parenate_parameters =
            HirEagerParenateParameters::from_syn(syn_decl.parenate_parameters(db), &builder);
        let return_ty = builder.return_ty_before_colon(syn_decl.return_ty(db));
        TraitForTypeMethodRitchieHirDecl::new(
            db,
            path,
            template_parameters,
            self_value_parameter,
            parenate_parameters,
            return_ty,
            builder.finish().eager(),
        )
    }
}
