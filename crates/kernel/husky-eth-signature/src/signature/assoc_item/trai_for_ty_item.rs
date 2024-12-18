pub mod assoc_ritchie;
pub mod assoc_ty;
pub mod assoc_val;
pub mod method_curry;
pub mod method_ritchie;

use self::assoc_ty::*;
use self::assoc_val::*;
use self::method_ritchie::*;
use self::{
    assoc_ritchie::*,
    signature::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockEthSignatureBuilderItd,
};
use super::*;
use husky_dec_signature::signature::assoc_item::trai_for_ty_item::TraitForTypeItemDecTemplate;
use husky_entity_path::path::assoc_item::trai_for_ty_item::TraitForTypeItemPath;

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEthTemplate {
    AssocRitchie(TraitForTypeAssocRitchieEthTemplate),
    AssocVal(TraitForTypeAssocValEthTemplate),
    AssocType(TraitForTypeAssocTypeEthTemplate),
    MethodRitchie(TraitForTypeMethodRitchieEthTemplate),
}

impl TraitForTypeItemEthTemplate {
    pub fn path(self, db: &::salsa::Db) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemEthTemplate::AssocRitchie(slf) => slf.path(db),
            TraitForTypeItemEthTemplate::AssocVal(slf) => slf.path(db),
            TraitForTypeItemEthTemplate::AssocType(slf) => slf.path(db),
            TraitForTypeItemEthTemplate::MethodRitchie(slf) => slf.path(db),
        }
    }

    pub fn self_ty(self, db: &::salsa::Db) -> Option<EthTerm> {
        match self {
            TraitForTypeItemEthTemplate::AssocRitchie(_) => None,
            TraitForTypeItemEthTemplate::AssocVal(_) => None,
            TraitForTypeItemEthTemplate::AssocType(_) => None,
            TraitForTypeItemEthTemplate::MethodRitchie(template) => {
                // ad hoc
                Some(template.self_ty(db))
            }
        }
    }

    pub(crate) fn inherit_instantiation_builder(
        self,
        db: &::salsa::Db,
        impl_block_signature_builder: TraitForTypeImplBlockEthSignatureBuilderItd,
    ) -> TraitForTypeItemEthSignatureBuilder {
        match self {
            TraitForTypeItemEthTemplate::AssocType(item_template) => item_template
                .inherit_instantiation_builder(impl_block_signature_builder, db)
                .into(),
            TraitForTypeItemEthTemplate::MethodRitchie(item_template) => item_template
                .inherit_instantiation_builder(impl_block_signature_builder, db)
                .into(),
            TraitForTypeItemEthTemplate::AssocRitchie(item_template) => item_template
                .inherit_instantiation_builder(impl_block_signature_builder, db)
                .into(),
            TraitForTypeItemEthTemplate::AssocVal(_) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[enum_class::from_variants]
pub enum TraitForTypeItemEthSignatureBuilder {
    AssocType(TraitForTypeAssocTypeEthSignatureBuilder),
    AssocRitchie(TraitForTypeAssocRitchieEthSignatureBuilder),
    Method(TraitForTypeMethodRitchieEthSignatureBuilder),
}

impl HasEthTemplate for TraitForTypeItemPath {
    type EthTemplate = TraitForTypeItemEthTemplate;

    fn eth_template(self, db: &::salsa::Db) -> EthSignatureResult<Self::EthTemplate> {
        trai_for_ty_item_eth_template(db, self)
    }
}

#[salsa::tracked]
fn trai_for_ty_item_eth_template(
    db: &::salsa::Db,
    path: TraitForTypeItemPath,
) -> EthSignatureResult<TraitForTypeItemEthTemplate> {
    Ok(match path.dec_template(db)? {
        TraitForTypeItemDecTemplate::AssocRitchie(dec_template) => {
            TraitForTypeAssocRitchieEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TraitForTypeItemDecTemplate::MethodRitchie(dec_template) => {
            TraitForTypeMethodRitchieEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TraitForTypeItemDecTemplate::AssocType(dec_template) => {
            TraitForTypeAssocTypeEthTemplate::from_dec(db, path, dec_template)?.into()
        }
        TraitForTypeItemDecTemplate::AssocVal(_) => todo!(),
    })
}
