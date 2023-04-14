use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectInsSort {
    smallest_universe: TermUniverse,
}

impl ExpectFluffyTerm for ExpectInsSort {
    type Outcome = ExpectInsSortOutcome;

    fn retrieve_outcome(outcome: &FluffyTermExpectationOutcome) -> &Self::Outcome {
        match outcome {
            FluffyTermExpectationOutcome::InsSort(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(
        &self,
        db: &dyn FluffyTermDb,
        terms: &FluffyTerms,
    ) -> FinalDestination {
        todo!()
    }

    fn destination(&self) -> Option<FluffyTerm> {
        None
    }
}

impl ExpectInsSort {
    pub(crate) fn new(u: u8) -> Self {
        ExpectInsSort {
            smallest_universe: u.into(),
        }
    }

    pub(crate) fn smallest_universe(&self) -> TermUniverse {
        self.smallest_universe
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = FluffyTermDb)]
pub struct ExpectInsSortOutcome {
    destination: FluffyTerm,
}

impl ExpectInsSortOutcome {
    pub(crate) fn resolved(&self) -> Option<Term> {
        todo!()
    }
}

impl ExpectInsSort {
    /// try to tell if a term is an instance of `Type u` for some universe u
    pub(super) fn resolve(
        &self,
        db: &dyn FluffyTermDb,
        porous_terms: &mut FluffyTerms,
        expectee: FluffyTerm,
    ) -> Option<FluffyTermExpectationEffect> {
        todo!()
        // match expectee {
        //     FluffyTerm::Term(resolved_expectee) => {
        //         todo!()
        //         // let expectee_ty = term_ty(
        //         //     self.db(),
        //         //     todo!(),
        //         //     resolved_expectee,
        //         //     self.toolchain(),
        //         //     self.term_menu(),
        //         // );
        //         // Some(match expectee_ty {
        //         //     Ok(expectee_ty) => match expectee_ty {
        //         //         Term::Category(cat) => {
        //         //             match cat.universe() >= expectation.smallest_universe {
        //         //                 true => FluffyTermExpectationEffect {
        //         //                     result: Ok(FluffyTermExpectationOutcome::InsSort(
        //         //                         ExpectInsSortOutcome {
        //         //                             destination: expectee,
        //         //                         },
        //         //                     )),
        //         //                     actions: vec![],
        //         //                 },
        //         //                 false => FluffyTermExpectationEffect {
        //         //                     result: Err(todo!()),
        //         //                     actions: vec![],
        //         //                 },
        //         //             }
        //         //         }
        //         //         _ => FluffyTermExpectationEffect {
        //         //             result: Err(todo!()),
        //         //             actions: vec![],
        //         //         },
        //         //     },
        //         //     Err(error) => FluffyTermExpectationEffect {
        //         //         result: Err(DerivedFluffyTermExpectationError::TermTypeError {
        //         //             term: resolved_expectee,
        //         //             error,
        //         //         }
        //         //         .into()),
        //         //         actions: vec![],
        //         //     },
        //         // })
        //     }
        //     FluffyTerm::Unresolved(_) => None,
        //     _ => todo!(),
        // }
    }
}
