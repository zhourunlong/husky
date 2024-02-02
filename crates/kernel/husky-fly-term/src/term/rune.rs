use husky_eth_term::term::rune::EthRune;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RuneFlyTerm(FlyTerm);

impl std::ops::Deref for RuneFlyTerm {
    type Target = FlyTerm;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RuneFlyTerm {
    pub(crate) fn rewrite_inner(
        self,
        db: &::salsa::Db,
        terms: &mut FlyTerms,
        src: HoleSource,
        substitution_rules: &[FlyTermSubstitution],
    ) -> Self {
        let slf = (*self).rewrite_inner(db, terms, src, substitution_rules);
        match slf.base_ty_data_inner(db, terms) {
            FlyBaseTypeData::TypeOntology {
                ty_path,
                refined_ty_path,
                ty_arguments,
                ty_ethereal_term,
            } => todo!(),
            FlyBaseTypeData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => todo!(),
            FlyBaseTypeData::Hole(_, _) => todo!(),
            FlyBaseTypeData::Category(_) => todo!(),
            FlyBaseTypeData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => todo!(),
            FlyBaseTypeData::Symbol { symbol } => todo!(),
            FlyBaseTypeData::Rune { rune } => (),
        }
        Self(slf)
    }
}

impl From<EthRune> for RuneFlyTerm {
    fn from(value: EthRune) -> Self {
        RuneFlyTerm(value.into())
    }
}
