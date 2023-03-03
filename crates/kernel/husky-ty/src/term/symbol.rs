use vec_like::VecSet;

use super::*;

#[salsa::tracked(db = TypeDb, jar = TypeJar)]
pub struct TermSymbols {
    #[return_ref]
    data: VecSet<TermSymbol>,
}

impl TermSymbols {
    pub(crate) fn contains(self, db: &dyn TypeDb, symbol: TermSymbol) -> bool {
        self.data(db).has(symbol)
    }

    fn merge(fst: impl Into<Option<Self>>, snd: impl Into<Option<Self>>) -> Option<Self> {
        let fst: Option<_> = fst.into();
        let snd: Option<_> = snd.into();
        match (fst, snd) {
            (None, None) => None,
            (None, Some(snd)) => Some(snd),
            (Some(fst), None) => Some(fst),
            (Some(fst), Some(snd)) => todo!(),
        }
    }
}

pub(crate) fn calc_term_symbols(db: &dyn TypeDb, term: Term) -> Option<TermSymbols> {
    match term {
        Term::Literal(_) => todo!(),
        Term::Symbol(symbol) => Some(TermSymbols::new(db, VecSet::new_one_elem_set(symbol))),
        Term::Entity(path) => match path {
            EntityPath::Module(_) => None,
            EntityPath::ModuleItem(path) => match path {
                ModuleItemPath::Type(_) => None,
                ModuleItemPath::Trait(_) => None,
                ModuleItemPath::Form(_) => todo!(),
            },
            EntityPath::AssociatedItem(_) => todo!(),
            EntityPath::Variant(_) => todo!(),
        },
        Term::Category(_) => None,
        Term::Universe(_) => None,
        Term::Curry(term) => term_curry_symbols(db, term),
        Term::Ritchie(term) => term_ritchie_symbols(db, term),
        Term::Abstraction(_) => todo!(),
        Term::Application(term) => term_application_symbols(db, term),
        Term::Composition(_) => todo!(),
        Term::Subentity(_) => todo!(),
        Term::AsTraitSubentity(_) => todo!(),
        Term::TraitConstraint(_) => todo!(),
    }
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn term_curry_symbols(db: &dyn TypeDb, term: TermCurry) -> Option<TermSymbols> {
    todo!()
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn term_ritchie_symbols(db: &dyn TypeDb, term: TermRitchie) -> Option<TermSymbols> {
    let mut symbols: Option<TermSymbols> = None;
    for parameter_ty in term.parameter_tys(db) {
        symbols = TermSymbols::merge(symbols, calc_term_symbols(db, parameter_ty.ty()))
    }
    TermSymbols::merge(symbols, calc_term_symbols(db, term.return_ty(db)))
}

#[salsa::tracked(jar = TypeJar)]
pub(crate) fn term_application_symbols(
    db: &dyn TypeDb,
    term: TermApplication,
) -> Option<TermSymbols> {
    TermSymbols::merge(
        calc_term_symbols(db, term.function(db)),
        calc_term_symbols(db, term.argument(db)),
    )
}