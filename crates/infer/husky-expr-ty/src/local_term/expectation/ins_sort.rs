use super::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectInsSort {
    smallest_universe: TermUniverse,
}

impl const ProvideEntityPathTypeExpectation for ExpectInsSort {
    fn entity_path_ty_expectation(
        &self,
        db: &dyn ExprTypeDb,
        unresolved_terms: &UnresolvedTerms,
    ) -> EntityPathTypeExpectation {
        todo!()
    }
}

impl ExpectLocalTerm for ExpectInsSort {
    type Outcome = ExpectInsSortOutcome;

    fn destination(&self) -> Option<LocalTerm> {
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
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectInsSortOutcome {
    destination: LocalTerm,
}

impl ExpectInsSortOutcome {
    pub(crate) fn resolved(&self) -> Option<ReducedTerm> {
        todo!()
    }
}

impl ExpectLocalTermOutcome for ExpectInsSortOutcome {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationOutcome) -> &Self {
        match resolved_ok {
            LocalTermExpectationOutcome::InsSort(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl<'a> ExprTypeEngine<'a> {
    /// try to tell if a term is an instance of `Type u` for some universe u
    pub(super) fn resolve_ins_sort_expectation(
        &self,
        expectee: LocalTerm,
        expectation: &ExpectInsSort,
        unresolved_terms: &mut UnresolvedTerms,
    ) -> Option<LocalTermExpectationEffect> {
        match expectee {
            LocalTerm::Resolved(resolved_expectee) => {
                let expectee_ty = term_ty(
                    self.db(),
                    todo!(),
                    resolved_expectee,
                    self.toolchain(),
                    self.reduced_term_menu(),
                );
                Some(match expectee_ty {
                    Ok(expectee_ty) => match expectee_ty.term() {
                        Term::Category(cat) => {
                            match cat.universe() >= expectation.smallest_universe {
                                true => LocalTermExpectationEffect {
                                    result: Ok(LocalTermExpectationOutcome::InsSort(
                                        ExpectInsSortOutcome {
                                            destination: expectee,
                                        },
                                    )),
                                    actions: vec![],
                                },
                                false => LocalTermExpectationEffect {
                                    result: Err(todo!()),
                                    actions: vec![],
                                },
                            }
                        }
                        _ => LocalTermExpectationEffect {
                            result: Err(todo!()),
                            actions: vec![],
                        },
                    },
                    Err(error) => LocalTermExpectationEffect {
                        result: Err(match error {
                            TypeError::Original(_) => {
                                OriginalLocalTermExpectationError::TermTypeError {
                                    term: resolved_expectee.term(),
                                    error,
                                }
                                .into()
                            }
                            TypeError::Derived(_) => {
                                DerivedLocalTermExpectationError::TermTypeError {
                                    term: resolved_expectee.term(),
                                    error,
                                }
                                .into()
                            }
                        }),
                        actions: vec![],
                    },
                })
            }
            LocalTerm::Unresolved(_) => None,
        }
    }
}
