use super::*;

#[derive(Debug, Clone, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct ExpectEqsFunctionType {
    final_destination: FinalDestination,
}

impl ExpectEqsFunctionType {
    pub fn new(final_destination: FinalDestination) -> Self {
        Self { final_destination }
    }
}

impl ExpectFluffyTerm for ExpectEqsFunctionType {
    type Outcome = ExpectEqsFunctionTypeOutcome;

    #[inline(always)]
    fn retrieve_outcome(outcome: &ExpectationOutcome) -> &Self::Outcome {
        match outcome {
            ExpectationOutcome::EqsFunctionCallType(outcome) => outcome,
            _ => unreachable!(),
        }
    }

    #[inline(always)]
    fn final_destination_inner(&self, db: &::salsa::Db, terms: &FluffyTerms) -> FinalDestination {
        self.final_destination
    }

    #[inline(always)]
    fn destination(&self) -> Option<FluffyTerm> {
        None
    }

    fn resolve(
        &self,
        db: &::salsa::Db,
        fluffy_terms: &mut FluffyTerms,
        state: &mut ExpectationState,
    ) -> AltOption<FluffyTermEffect> {
        // todo: move these to aux
        match state.expectee().data_inner(db, fluffy_terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ..
            } => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedFunctionType,
                smallvec![],
            ),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => {
                if let Some(parameter_rune) = parameter_rune {
                    match parameter_rune.base_ty_data_inner(db, fluffy_terms) {
                        FluffyBaseTypeData::TypeOntology {
                            ty_path,
                            refined_ty_path,
                            ty_arguments,
                            ty_ethereal_term,
                        } => todo!(),
                        FluffyBaseTypeData::Curry {
                            curry_kind,
                            variance,
                            parameter_rune,
                            parameter_ty,
                            return_ty,
                            ty_ethereal_term,
                        } => todo!(),
                        FluffyBaseTypeData::Hole(_, _) => todo!(),
                        FluffyBaseTypeData::Category(_) => todo!(),
                        FluffyBaseTypeData::Ritchie {
                            ritchie_kind,
                            parameter_contracted_tys,
                            return_ty,
                        } => todo!(),
                        FluffyBaseTypeData::Symbol { symbol } => todo!(),
                        FluffyBaseTypeData::Rune { rune } => (),
                    }
                }
                self.resolve_curry(
                    db,
                    state,
                    fluffy_terms,
                    curry_kind,
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                )
            }
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedFunctionType,
                smallvec![],
            ),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
                ..
            } => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    template_parameter_substitutions: smallvec![],
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys: parameter_contracted_tys.to_vec(),
                    },
                },
                smallvec![],
            ),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub struct ExpectEqsFunctionTypeOutcome {
    pub(crate) template_parameter_substitutions: SmallVec<[ImplicitParameterSubstitution; 2]>,
    pub(crate) return_ty: FluffyTerm,
    pub(crate) variant: ExpectEqsFunctionTypeOutcomeData,
}

impl ExpectEqsFunctionTypeOutcome {
    pub fn variant(&self) -> &ExpectEqsFunctionTypeOutcomeData {
        &self.variant
    }

    pub fn return_ty(&self) -> FluffyTerm {
        self.return_ty
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::debug_with_db(db = FluffyTermDb, jar = FluffyTermJar)]
pub enum ExpectEqsFunctionTypeOutcomeData {
    Ritchie {
        ritchie_kind: RitchieKind,
        parameter_contracted_tys: Vec<FluffyRitchieParameter>,
    },
    Curry {
        variance: Variance,
        parameter_rune: Option<FluffyTermRune>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    },
}

impl ExpectEqsFunctionType {
    fn resolve_curry(
        &self,
        db: &::salsa::Db,
        state: &mut ExpectationState,
        terms: &mut FluffyTerms,
        curry_kind: CurryKind,
        variance: Variance,
        parameter_rune: Option<FluffyTermRune>,
        parameter_ty: FluffyTerm,
        return_ty: FluffyTerm,
    ) -> AltOption<FluffyTermEffect> {
        if let Some(parameter_rune) = parameter_rune {
            match parameter_rune.base_ty_data_inner(db, terms) {
                FluffyBaseTypeData::TypeOntology {
                    ty_path,
                    refined_ty_path,
                    ty_arguments,
                    ty_ethereal_term,
                } => todo!(),
                FluffyBaseTypeData::Curry {
                    curry_kind,
                    variance,
                    parameter_rune,
                    parameter_ty,
                    return_ty,
                    ty_ethereal_term,
                } => todo!(),
                FluffyBaseTypeData::Hole(_, _) => todo!(),
                FluffyBaseTypeData::Category(_) => todo!(),
                FluffyBaseTypeData::Ritchie {
                    ritchie_kind,
                    parameter_contracted_tys,
                    return_ty,
                } => todo!(),
                FluffyBaseTypeData::Symbol { symbol } => todo!(),
                FluffyBaseTypeData::Rune { rune } => (),
            }
        }
        match curry_kind {
            CurryKind::Explicit => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    template_parameter_substitutions: smallvec![],
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeData::Curry {
                        variance,
                        parameter_rune,
                        parameter_ty,
                        return_ty,
                    },
                },
                smallvec![],
            ),
            CurryKind::Implicit => match parameter_rune {
                Some(parameter_rune) => {
                    let implicit_symbol = terms.new_hole_from_parameter_rune(
                        db,
                        HoleSource::Expectation(state.idx()),
                        parameter_rune,
                    );
                    let mut template_parameter_substitutions =
                        smallvec![ImplicitParameterSubstitution::new(
                            parameter_rune,
                            implicit_symbol,
                        )];
                    let expectee = return_ty;
                    let expectee = expectee.rewrite_inner(
                        db,
                        terms,
                        HoleSource::Expectation(state.idx()),
                        &template_parameter_substitutions,
                    );
                    self.resolve_aux(db, state, terms, expectee, template_parameter_substitutions)
                }
                None => self.resolve_aux(db, state, terms, return_ty, smallvec![]), // ad hoc
            },
        }
    }

    fn resolve_aux(
        &self,
        db: &::salsa::Db,
        state: &mut ExpectationState,
        terms: &mut FluffyTerms,
        expectee: FluffyTerm,
        mut substitution_rules: SmallVec<[ImplicitParameterSubstitution; 2]>,
    ) -> AltOption<FluffyTermEffect> {
        match expectee.data_inner(db, terms) {
            FluffyTermData::Literal(_) => todo!(),
            FluffyTermData::TypeOntology {
                ty_path: path,
                refined_ty_path: refined_path,
                ty_arguments: arguments,
                ..
            } => state.set_err(
                OriginalFluffyTermExpectationError::ExpectedFunctionType,
                smallvec![],
            ),
            FluffyTermData::Curry {
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
                ty_ethereal_term,
            } => self.resolve_curry(
                db,
                state,
                terms,
                curry_kind,
                variance,
                parameter_rune,
                parameter_ty,
                return_ty,
            ),
            FluffyTermData::Hole(_, _) => todo!(),
            FluffyTermData::Category(_) => todo!(),
            FluffyTermData::Ritchie {
                ritchie_kind,
                parameter_contracted_tys,
                return_ty,
            } => state.set_ok(
                ExpectEqsFunctionTypeOutcome {
                    // todo: is this really correct?
                    template_parameter_substitutions: substitution_rules,
                    return_ty,
                    variant: ExpectEqsFunctionTypeOutcomeData::Ritchie {
                        ritchie_kind,
                        parameter_contracted_tys: parameter_contracted_tys.to_vec(),
                    },
                },
                smallvec![],
            ),
            FluffyTermData::Symbol { .. } => todo!(),
            FluffyTermData::Rune { ty } => todo!(),
            FluffyTermData::TypeVariant { path } => todo!(),
        }
    }
}
