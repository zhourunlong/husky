use salsa::Database;

use super::*;

#[salsa::interned(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermAbstraction {
    x: EtherealTermRune,
    m: EtherealTerm,
}

#[test]
fn term_abstraction_size_works() {
    assert_eq!(
        std::mem::size_of::<EtherealTermAbstraction>(),
        std::mem::size_of::<u32>()
    );
}

impl EtherealTermAbstraction {
    pub(crate) fn from_declarative(
        db: &dyn EtherealTermDb,
        precise_term: DeclarativeTermAbstraction,
        term_ty_expectation: TermTypeExpectation,
    ) -> EtherealTermResult<Self> {
        todo!()
    }

    pub fn ty(&self) -> EtherealTerm {
        todo!()
    }

    #[inline(never)]
    pub(crate) fn show_with_db_fmt(
        self,
        f: &mut std::fmt::Formatter<'_>,
        db: &dyn EtherealTermDb,
        ctx: &mut TermShowContext,
    ) -> std::fmt::Result {
        todo!()
    }
}

impl EtherealTermAbstraction {
    fn substitute(self, db: &dyn EtherealTermDb, substituation: &TermSubstitution) -> EtherealTerm {
        todo!()
    }
}

impl salsa::DisplayWithDb for EtherealTermAbstraction {
    fn display_with_db_fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        // use std::fmt::Write;
        // f.write_char(husky_unicode_symbols::greek::GREEK_LETTER_LOWERCASE_LAMBDA);
        // todo!()
        self.show_with_db_fmt(
            f,
            db.as_jar_db_dyn::<EtherealTermJar>(),
            &mut Default::default(),
        )
    }
}
