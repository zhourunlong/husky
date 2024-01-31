use super::*;
use vec_like::VecSet;

#[salsa::tracked(db = EtherealTermDb, jar = EtherealTermJar)]
pub struct EtherealTermSymbols {
    #[return_ref]
    pub(crate) data: VecSet<SymbolEtherealTerm>,
}

impl EtherealTermSymbols {
    pub(crate) fn contains(self, db: &::salsa::Db, symbol: SymbolEtherealTerm) -> bool {
        self.data(db).has(symbol)
    }

    fn merge(fst: impl Into<Option<Self>>, snd: impl Into<Option<Self>>) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(_fst), Some(_snd)) => todo!(),
        }
    }

    fn remove(
        symbols: impl Into<Option<Self>>,
        _symbol: impl Into<Option<SymbolEtherealTerm>>,
    ) -> Option<Self> {
        let _symbols = symbols.into()?;
        todo!()
    }
}

impl EtherealTerm {
    pub(crate) fn symbols(self, db: &::salsa::Db) -> Option<EtherealTermSymbols> {
        match self {
            EtherealTerm::Literal(_)
            | EtherealTerm::Rune(_)
            | EtherealTerm::EntityPath(_)
            | EtherealTerm::Category(_) => None,
            EtherealTerm::Universe(_) => None, // ad hoc
            EtherealTerm::Symbol(symbol) => Some(EtherealTermSymbols::new(
                db,
                VecSet::new_one_elem_set(symbol),
            )),
            EtherealTerm::Curry(term) => term_curry_symbols(db, term),
            EtherealTerm::Ritchie(term) => term_ritchie_symbols(db, term),
            EtherealTerm::Abstraction(_) => todo!(),
            EtherealTerm::Application(term) => term_application_symbols(db, term),
            EtherealTerm::TypeAsTraitItem(_) => todo!(),
            EtherealTerm::TraitConstraint(_) => todo!(),
        }
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_curry_symbols(
    db: &::salsa::Db,
    term: CurryEtherealTerm,
) -> Option<EtherealTermSymbols> {
    let parameter_ty_symbols = term.parameter_ty(db).symbols(db);
    let return_ty_symbols = term.return_ty(db).symbols(db);
    EtherealTermSymbols::merge(parameter_ty_symbols, return_ty_symbols)
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_ritchie_symbols(
    db: &::salsa::Db,
    term: RitchieEtherealTerm,
) -> Option<EtherealTermSymbols> {
    let mut symbols: Option<EtherealTermSymbols> = None;
    for parameter_ty in term.parameter_contracted_tys(db) {
        symbols = EtherealTermSymbols::merge(symbols, parameter_ty.ty().symbols(db))
    }
    EtherealTermSymbols::merge(symbols, term.return_ty(db).symbols(db))
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn term_application_symbols(
    db: &::salsa::Db,
    term: ApplicationEtherealTerm,
) -> Option<EtherealTermSymbols> {
    EtherealTermSymbols::merge(term.function(db).symbols(db), term.argument(db).symbols(db))
}
