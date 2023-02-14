use super::*;

#[derive(Debug, Clone)]
pub(crate) struct ExpectEqsRitchieCallType;

impl ExpectLocalTerm for ExpectEqsRitchieCallType {
    type ResolvedOk = ExpectEqsRitchieCallTypeResolvedOk;

    fn destination(&self) -> Option<LocalTerm> {
        None
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub(crate) struct ExpectEqsRitchieCallTypeResolvedOk {
    destination: LocalTerm,
    parameter_liasoned_tys: (),
    return_ty: (),
}

impl ExpectLocalTermResolvedOk for ExpectEqsRitchieCallTypeResolvedOk {
    fn destination(&self) -> LocalTerm {
        self.destination
    }

    fn downcast_ref(resolved_ok: &LocalTermExpectationResolvedOk) -> &Self {
        match resolved_ok {
            LocalTermExpectationResolvedOk::EqsRitchieCallType(resolved_ok) => resolved_ok,
            _ => unreachable!(),
        }
    }
}

impl ExpectEqsRitchieCallTypeResolvedOk {
    pub(crate) fn expectee(&self) -> LocalTerm {
        self.destination
    }
}

impl From<ExpectEqsRitchieCallTypeResolvedOk> for LocalTermExpectationResolvedOk {
    fn from(value: ExpectEqsRitchieCallTypeResolvedOk) -> Self {
        LocalTermExpectationResolvedOk::EqsRitchieCallType(value)
    }
}

impl From<ExpectEqsRitchieCallType> for LocalTermExpectation {
    fn from(value: ExpectEqsRitchieCallType) -> Self {
        LocalTermExpectation::EqsRitchieCallTy
    }
}

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn resolve_eqs_richie_call_ty(
        &self,
        expectee: LocalTerm,
    ) -> Option<LocalTermExpectationResolvedOkM> {
        match expectee {
            LocalTerm::Resolved(expectee) => self.res_to(expectee),
            LocalTerm::Unresolved(_) => todo!(),
        }
    }

    fn res_to(&self, expectee: ReducedTerm) -> Option<LocalTermExpectationResolvedOkM> {
        match expectee.term() {
            Term::Literal(_) => todo!(),
            Term::Symbol(_) => todo!(),
            Term::Entity(_) => todo!(),
            Term::Category(_) => Some(LocalTermExpectationResolvedOkM {
                // ad hoc
                result: Err(OriginalLocalTermExpectationError::Todo.into()),
                actions: vec![],
            }),
            Term::Universe(_) => todo!(),
            Term::Curry(_) => todo!(),
            Term::Ritchie(term) => {
                let result = match term.ritchie_kind(self.db()) {
                    TermRitchieKind::Fp => Ok(ExpectEqsRitchieCallTypeResolvedOk {
                        destination: expectee.into(),
                        parameter_liasoned_tys: (),
                        return_ty: (),
                    }
                    .into()),
                    TermRitchieKind::Fn => todo!(),
                    TermRitchieKind::FnMut => todo!(),
                };
                Some(LocalTermExpectationResolvedOkM {
                    result,
                    actions: vec![],
                })
            }
            Term::Abstraction(_) => todo!(),
            Term::Application(_) => todo!(),
            Term::Subentity(_) => todo!(),
            Term::AsTraitSubentity(_) => todo!(),
            Term::TraitConstraint(_) => todo!(),
        }
    }
}