use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db]
pub struct DeclarativeRitchieKeyedParameter {
    key: Ident,
    contract: TermContract,
    ty: DecTerm,
    has_default: bool,
}

impl DeclarativeRitchieKeyedParameter {
    pub fn new(key: Ident, contract: TermContract, ty: DecTerm, has_default: bool) -> Self {
        Self {
            key,
            contract,
            ty,
            has_default,
        }
    }

    pub(crate) fn substitute_ty(self, f: impl Fn(DecTerm) -> DecTerm) -> Self {
        Self {
            key: self.key,
            contract: self.contract,
            ty: f(self.ty),
            has_default: self.has_default,
        }
    }

    pub fn key(&self) -> Ident {
        self.key
    }

    pub fn contract(&self) -> TermContract {
        self.contract
    }

    pub fn ty(&self) -> DecTerm {
        self.ty
    }

    pub fn has_default(&self) -> bool {
        self.has_default
    }

    #[inline(never)]
    pub(super) fn show_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut DecTermShowContext,
    ) -> std::fmt::Result {
        self.ty.show_with_db_fmt(f, db, ctx)
    }
}

impl salsa::DisplayWithDb for DeclarativeRitchieKeyedParameter {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        let mut ctx = Default::default();
        f.write_str(self.key.data(db))?;
        f.write_str(": ")?;
        self.ty.show_with_db_fmt(f, db, &mut ctx)?;
        f.write_str(" = ")?;
        match self.has_default {
            true => f.write_str("..."),
            false => f.write_str("_"),
        }
    }
}