mod utils;

pub(crate) use self::utils::*;

use super::*;

/// representing term `X -> Y` or dependent form `(a: X) -> Y(a)`
#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct CurryEtherealTerm {
    pub curry_kind: CurryKind,
    pub variance: Variance,
    /// a
    pub parameter_rune: Option<RuneEtherealTerm>,
    /// X
    pub parameter_ty: EtherealTerm,
    /// Y
    pub return_ty: EtherealTerm,
}

#[test]
fn term_curry_size_works() {
    assert_eq!(
        std::mem::size_of::<CurryEtherealTerm>(),
        std::mem::size_of::<u32>()
    );
}

impl CurryEtherealTerm {
    pub(crate) fn from_declarative(
        db: &::salsa::Db,
        declarative_term_curry: CurryDeclarativeTerm,
    ) -> EtherealTermResult<Self> {
        term_curry_from_declarative(db, declarative_term_curry)
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        let parameter_rune = self.parameter_rune(db);
        if parameter_rune.is_some() {
            f.write_str("(")?
        }
        f.write_str(self.variance(db).as_str())?;
        if let Some(parameter_rune) = parameter_rune {
            ctx.fmt_with_variable(db, parameter_rune, |ctx| {
                ctx.fmt_variable(db, parameter_rune, f)?;
                f.write_str(": ")?;
                self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
                f.write_str(") -> ")?;
                self.return_ty(db).show_with_db_fmt(f, db, ctx)
            })
        } else {
            self.parameter_ty(db).show_with_db_fmt(f, db, ctx)?;
            f.write_str(" -> ")?;
            self.return_ty(db).show_with_db_fmt(f, db, ctx)
        }
    }

    pub fn substitute(self, _db: &::salsa::Db, _substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_curry_from_declarative(
    db: &::salsa::Db,
    declarative_term_curry: CurryDeclarativeTerm,
) -> EtherealTermResult<CurryEtherealTerm> {
    let t = |declarative_ty| EtherealTerm::ty_from_declarative(db, declarative_ty);
    Ok(CurryEtherealTerm::new(
        db,
        declarative_term_curry.curry_kind(db),
        declarative_term_curry.variance(db),
        match declarative_term_curry.parameter_rune(db) {
            Some(parameter_rune) => Some(RuneEtherealTerm::from_declarative(db, parameter_rune)?),
            None => None,
        },
        t(declarative_term_curry.parameter_ty(db))?,
        t(declarative_term_curry.return_ty(db))?,
    ))
}

impl salsa::DisplayWithDb for CurryEtherealTerm {
    fn display_with_db_fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
    ) -> std::fmt::Result {
        self.show_with_db_fmt(f, db, &mut Default::default())
    }
}
