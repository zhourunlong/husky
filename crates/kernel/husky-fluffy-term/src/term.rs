mod application;
mod curry;
mod hole;
mod ritchie;
pub mod rune;
pub mod symbol_ty;
mod utils;

pub use self::hole::*;
pub use self::ritchie::*;
pub use self::rune::*;
pub use self::symbol_ty::*;

use crate::*;
use husky_term_prelude::literal::TermLiteral;

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct FluffyTerm {
    place: Option<FluffyPlace>,
    base: FluffyTermBase,
}

impl FluffyTerm {
    pub(crate) fn new_ethereal(place: FluffyPlace, ethereal_term: EtherealTerm) -> Self {
        Self {
            place: Some(place),
            base: ethereal_term.into(),
        }
    }

    pub fn with_place(self, place: FluffyPlace) -> Self {
        Self {
            place: Some(place),
            base: self.base,
        }
    }
}

#[salsa::debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[enum_class::from_variants]
pub enum FluffyTermBase {
    Ethereal(EtherealTerm),
    Solid(SolidTerm),
    Hollow(HollowTerm),
    Place,
}

impl From<FluffyPlace> for FluffyTerm {
    fn from(place: FluffyPlace) -> Self {
        FluffyTerm {
            place: Some(place),
            base: FluffyTermBase::Place,
        }
    }
}

impl From<EtherealTerm> for FluffyTerm {
    #[inline(always)]
    fn from(term: EtherealTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

impl From<TermLiteral> for FluffyTerm {
    fn from(value: TermLiteral) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<ItemPathTerm> for FluffyTerm {
    fn from(value: ItemPathTerm) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<ApplicationEtherealTerm> for FluffyTerm {
    fn from(value: ApplicationEtherealTerm) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<CurryEtherealTerm> for FluffyTerm {
    fn from(value: CurryEtherealTerm) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<CategoryTerm> for FluffyTerm {
    fn from(value: CategoryTerm) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<SymbolEtherealTerm> for FluffyTerm {
    fn from(value: SymbolEtherealTerm) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<RuneEtherealTerm> for FluffyTerm {
    fn from(value: RuneEtherealTerm) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<RitchieEtherealTerm> for FluffyTerm {
    fn from(value: RitchieEtherealTerm) -> Self {
        Into::<EtherealTerm>::into(value).into()
    }
}

impl From<SolidTerm> for FluffyTerm {
    #[inline(always)]
    fn from(term: SolidTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

impl From<HollowTerm> for FluffyTerm {
    #[inline(always)]
    fn from(term: HollowTerm) -> Self {
        Self {
            place: None,
            base: term.into(),
        }
    }
}

#[test]
fn term_to_fluffy_term_works() {
    fn t(a: impl Copy + Into<EtherealTerm> + Into<FluffyTerm>) {
        let term: EtherealTerm = a.into();
        let fluffy_term: FluffyTerm = a.into();
        let term_to_fluffy_term: FluffyTerm = term.into();
        assert_eq!(fluffy_term, term_to_fluffy_term)
    }
    let db = DB::default();
    let toolchain = db.dev_toolchain().unwrap();
    let term_menu = db.ethereal_term_menu(toolchain);
    t(TermLiteral::I8(1))
}

impl FluffyTerm {
    #[deprecated(note = "should return place or panic")]
    pub fn place(self) -> Option<FluffyPlace> {
        self.place
    }

    pub fn base_resolved(self, engine: &impl FluffyTermEngine) -> FluffyTermBase {
        self.base_resolved_inner(engine.fluffy_terms())
    }

    pub fn base_resolved_inner(
        self,
        fluffy_terms: &impl std::borrow::Borrow<HollowTerms>,
    ) -> FluffyTermBase {
        match self.base {
            FluffyTermBase::Ethereal(_) | FluffyTermBase::Solid(_) => self.base,
            FluffyTermBase::Hollow(term) => match term.resolve_progress(fluffy_terms.borrow()) {
                TermResolveProgress::UnresolvedHollow => self.base,
                TermResolveProgress::ResolvedEthereal(term) => term.into(),
                TermResolveProgress::ResolvedSolid(term) => term.into(),
                TermResolveProgress::Err => todo!(),
            },
            FluffyTermBase::Place => self.base,
        }
    }

    #[inline(never)]
    pub fn show(self, db: &::salsa::Db, terms: &FluffyTerms) -> String {
        self.data_inner(db, terms).show(db, terms)
    }

    pub(crate) fn base(&self) -> FluffyTermBase {
        self.base
    }
}
