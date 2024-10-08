use super::*;
use husky_dec_signature::signature::major_item::ty::inductive::InductiveDecTemplate;

#[salsa::interned]
pub struct InductiveTypeEthTemplate {
    pub path: TypePath,
    #[return_ref]
    pub template_parameters: EthTemplateParameters,
}

impl InductiveTypeEthTemplate {
    pub(super) fn from_dec(
        db: &::salsa::Db,
        path: TypePath,
        dec_template: InductiveDecTemplate,
    ) -> EthSignatureResult<Self> {
        let template_parameters =
            EthTemplateParameters::from_dec(db, dec_template.template_parameters(db))?;
        Ok(Self::new(db, path, template_parameters))
    }
}
