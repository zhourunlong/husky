use super::*;

#[salsa::interned(db = EthTermDb, jar = EthTermJar, constructor = new_inner)]
pub struct RuneEthTerm {
    pub ty: EthTerm,
    /// this is the index for all symbols with the same type
    /// so that we have better cache hits
    pub index: RuneIndex,
}

impl RuneEthTerm {
    #[inline(always)]
    pub(crate) fn from_declarative(db: &::salsa::Db, variable: RuneDecTerm) -> EthTermResult<Self> {
        let ty = variable.ty(db)?;
        let ty = EthTerm::ty_from_declarative(db, ty)?;
        Ok(Self::new_inner(db, ty, variable.idx(db)))
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &::salsa::Db,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        ctx.fmt_variable(db, self, f)
    }
}

impl EthTerm {
    #[track_caller]
    pub fn rune(self) -> RuneEthTerm {
        match self {
            EthTerm::Rune(slf) => slf,
            _ => unreachable!(),
        }
    }
}

/// # rewrite

impl RuneEthTerm {
    pub fn substitute(self, substitution: EthTermSubstitution, db: &salsa::Db) -> EthTerm {
        if self == substitution.src() {
            return substitution.dst();
        }
        self.substitute_intact(substitution, db).into()
    }

    pub fn substitute_intact(
        self,
        substitution: EthTermSubstitution,
        db: &salsa::Db,
    ) -> RuneEthTerm {
        Self::new_inner(db, self.ty(db).substitute(substitution, db), self.index(db))
    }
}

impl EthTermInstantiate for RuneEthTerm {
    type Output = Self;

    fn instantiate(self, db: &::salsa::Db, instantiation: &EtherealInstantiation) -> Self::Output {
        // it's assumed that all symbols will be replaced by its map
        // otherwise it's illegal
        Self::new_inner(
            db,
            self.ty(db).instantiate(db, instantiation),
            self.index(db),
        )
    }
}

/// back to declarative
impl RuneEthTerm {
    pub(super) fn into_declarative(self, db: &salsa::Db) -> RuneDecTerm {
        RuneDecTerm::new(
            Ok(self.ty(db).into_declarative(db)),
            self.index(db).disambiguator(),
            db,
        )
    }
}