use super::*;
use husky_coword::Ident;
use husky_expr::stmt::{LoopBoundaryKind, LoopStep};
use husky_opr::BinaryComparisonOpr;
use husky_regional_token::RegionalTokenIdx;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemForBetweenParticulars {
    for_between_loop_var_regional_token_idx: RegionalTokenIdx,
    for_between_loop_var_ident: Ident,
    for_between_loop_var_expr_idx: SemExprIdx,
    range: SemaForBetweenRange,
}

impl SemForBetweenParticulars {
    pub fn for_between_loop_var_regional_token_idx(&self) -> RegionalTokenIdx {
        self.for_between_loop_var_regional_token_idx
    }

    pub fn for_between_loop_var_ident(&self) -> Ident {
        self.for_between_loop_var_ident
    }

    pub fn for_between_loop_var_expr_idx(&self) -> SemExprIdx {
        self.for_between_loop_var_expr_idx
    }

    pub fn range(&self) -> &SemaForBetweenRange {
        &self.range
    }
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_sem_for_between_particulars(
        &mut self,
        particulars: &'a SynForBetweenParticulars,
        for_loop_varible_idx: CurrentVariableIdx,
    ) -> SynExprResultRef<'a, SemForBetweenParticulars> {
        let Ok(ref range) = particulars.range else {
            todo!()
        };
        let mut expected_for_loop_variable_ty: Option<FlyTerm> = None;
        let initial_bound_expr = match range.initial_boundary.bound_expr {
            Some(bound_expr) => {
                let (bound_sem_expr_idx, num_ty_outcome) =
                    self.build_expr_with_outcome(bound_expr, ExpectIntType);
                match num_ty_outcome {
                    Some(num_ty_outcome) => {
                        expected_for_loop_variable_ty = Some(num_ty_outcome.placeless_int_ty())
                    }
                    None => (),
                };
                Some(bound_sem_expr_idx)
            }
            None => None,
        };
        let final_bound_expr = match range.final_boundary.bound_expr {
            Some(bound_expr) => match expected_for_loop_variable_ty {
                Some(expected_for_loop_variable_ty) => Some(self.build_expr(
                    bound_expr,
                    ExpectCoercion::new_pure(self, expected_for_loop_variable_ty),
                )),
                None => {
                    let (final_bound_sem_expr_idx, ty) =
                        self.build_expr_with_ty(bound_expr, ExpectAnyOriginal);
                    if let Some(ty) = ty {
                        expected_for_loop_variable_ty = Some(ty)
                    }
                    Some(final_bound_sem_expr_idx)
                }
            },
            None => None,
        };
        let Some(expected_for_loop_variable_ty) = expected_for_loop_variable_ty else {
            todo!()
        };
        // let place = FlyPlace::ImmutableOnStack {
        //     location: for_loop_varible_idx
        //         .into_local_symbol_idx(self.syn_expr_region_data())
        //         .into(),
        // };
        let for_loop_variable_symbol_ty = SymbolType::new_variable_ty(
            self,
            for_loop_varible_idx,
            VariableModifier::Pure,
            expected_for_loop_variable_ty,
        )
        .unwrap();
        self.add_symbol_ty(for_loop_varible_idx, for_loop_variable_symbol_ty);
        let for_between_loop_var_expr_idx = self.build_expr(
            particulars.for_between_loop_var_expr_idx,
            ExpectCoercion::new_pure(self, for_loop_variable_symbol_ty.term()),
        );
        let range = SemaForBetweenRange {
            initial_boundary: SemaForBetweenLoopBoundary {
                bound_expr: initial_bound_expr,
                kind: range.initial_boundary.kind,
            },
            final_boundary: SemaForBetweenLoopBoundary {
                bound_expr: final_bound_expr,
                kind: range.final_boundary.kind,
            },
            step: range.step,
        };
        Ok(SemForBetweenParticulars {
            for_between_loop_var_regional_token_idx: particulars
                .for_between_loop_var_regional_token_idx,
            for_between_loop_var_ident: particulars.for_between_loop_var_ident,
            for_between_loop_var_expr_idx,
            range,
        })
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq)]
pub struct SemaForBetweenRange {
    pub initial_boundary: SemaForBetweenLoopBoundary,
    pub final_boundary: SemaForBetweenLoopBoundary,
    pub step: LoopStep,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct SemaForBetweenLoopBoundary {
    pub bound_expr: Option<SemExprIdx>,
    pub kind: LoopBoundaryKind,
}

impl Default for SemaForBetweenLoopBoundary {
    fn default() -> Self {
        Self {
            bound_expr: None,
            kind: LoopBoundaryKind::LowerClosed,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SemaForextParticulars {
    pub forext_loop_var_regional_token_idx: RegionalTokenIdx,
    pub forext_loop_var_ident: Ident,
    pub forext_loop_var_sem_expr_idx: SemExprIdx,
    pub bound_expr: SemExprIdx,
    pub boundary_kind: LoopBoundaryKind,
}

impl SemaForextParticulars {
    pub(crate) fn new(
        forext_loop_var_regional_token_idx: RegionalTokenIdx,
        forext_loop_var_ident: Ident,
        forext_loop_var_expr_idx: SemExprIdx,
        opr: BinaryComparisonOpr,
        bound_expr: SemExprIdx,
    ) -> Self {
        todo!()
    }
}

impl<'a> SemExprBuilder<'a> {
    pub(crate) fn build_sem_forext_particulars(
        &mut self,
        particulars: &'a SynForextParticulars,
        forext_loop_var_sem_expr_idx: SemExprIdx,
        forext_loop_var_ty: FlyTerm,
    ) -> SemaForextParticulars {
        let bound_expr_sem_expr_idx = self.build_expr(
            particulars.bound_expr,
            ExpectCoercion::new_pure(self, forext_loop_var_ty),
        );
        SemaForextParticulars {
            forext_loop_var_regional_token_idx: particulars.forext_loop_var_regional_token_idx,
            forext_loop_var_ident: particulars.forext_loop_var_ident,
            forext_loop_var_sem_expr_idx,
            bound_expr: bound_expr_sem_expr_idx,
            boundary_kind: particulars.boundary_kind,
        }
    }
}
