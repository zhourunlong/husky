mod let_init;

use super::*;

impl<'a> ExprTypeEngine<'a> {
    pub(super) fn infer_new_block(
        &mut self,
        stmts: SynStmtIdxRange,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> (SemaStmtIdxRange, Option<FluffyTerm>) {
        let mut stmt_entries = vec![];
        for stmt in stmts.start()..(stmts.end() - 1) {
            let (data_result, ty_result) = self.infer_new_nonlast_stmt(stmt);
            stmt_entries.push(SemaStmtEntry::new(data_result, ty_result))
        }
        self.infer_new_stmt(stmts.end() - 1, expr_expectation);
        todo!()
    }

    fn infer_new_nonlast_stmt(&mut self, stmt_idx: SynStmtIdx) -> SemaStmtEntry {
        let expect_unit = self.expect_unit();
        self.calc_stmt(stmt_idx, expect_unit);
    }

    fn infer_new_stmt(
        &mut self,
        stmt_idx: SynStmtIdx,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        self.calc_stmt(stmt_idx, expr_expectation)
    }

    fn calc_stmt(
        &mut self,
        stmt_idx: SynStmtIdx,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> (
        SemaExprDataResult<SemaStmtData>,
        SemaExprTypeResult<FluffyTerm>,
    ) {
        match self.expr_region_data[stmt_idx] {
            SynStmtData::Let {
                let_token,
                ref let_variables_pattern,
                initial_value,
                ..
            } => self.calc_let_stmt(let_variables_pattern, initial_value),
            SynStmtData::Return { result, .. } => {
                match self.return_ty {
                    Some(return_ty) => {
                        self.build_new_expr_ty_discarded(
                            result,
                            ExpectCoersion::new_move(return_ty.into()),
                        );
                    }
                    None => {
                        self.build_new_expr_ty_discarded(result, ExpectAnyDerived);
                    }
                };
                (todo!(), Ok(self.term_menu.never().into()))
            }
            SynStmtData::Require { condition, .. } => {
                self.build_new_expr_ty_discarded(condition, ExpectConditionType);
                (todo!(), Ok(self.term_menu.unit_ty_ontology().into()))
            }
            SynStmtData::Assert { condition, .. } => {
                self.build_new_expr_ty_discarded(condition, ExpectConditionType);
                (todo!(), Ok(self.term_menu.unit_ty_ontology().into()))
            }
            SynStmtData::Break { .. } => (todo!(), Ok(self.term_menu.never().into())),
            SynStmtData::Eval {
                expr_idx,
                eol_semicolon,
            } => {
                let (sema_expr_idx, ty) = match eol_semicolon {
                    Ok(None) => self.build_new_expr_ty(expr_idx, expr_expectation),
                    Ok(Some(_)) => {
                        let (sema_expr_idx, expr_ty) =
                            self.build_new_expr_ty(expr_idx, ExpectAnyOriginal);
                        let ty = match expr_ty {
                            Some(_) => todo!(),
                            None => todo!(),
                        };
                        todo!("unit or never")
                    }
                    Err(_) => self.build_new_expr_ty(expr_idx, ExpectAnyDerived),
                };
                (
                    todo!(),
                    ty.ok_or(DerivedSemaExprTypeError::EvalExprTypeNotInferred.into()),
                )
            }
            SynStmtData::ForBetween {
                ref particulars,
                frame_var_symbol_idx,
                ref block,
                ..
            } => {
                let mut expected_frame_var_ty: Option<FluffyTerm> = None;
                let Ok(ref range) = particulars.range else {
                    todo!()
                };
                if let Some(bound_expr) = range.initial_boundary.bound_expr {
                    match self.build_new_sema_expr_with_outcome(bound_expr, ExpectIntType) {
                        Some(num_ty_outcome) => {
                            expected_frame_var_ty = Some(num_ty_outcome.placeless_num_ty())
                        }
                        None => (),
                    }
                }
                if let Some(bound_expr) = range.final_boundary.bound_expr {
                    match expected_frame_var_ty {
                        Some(expected_frame_var_ty) => {
                            self.build_new_expr_ty_discarded(
                                bound_expr,
                                ExpectCoersion::new_pure(self, expected_frame_var_ty),
                            );
                        }
                        None => {
                            if let Some(ty) = self.build_new_expr_ty(bound_expr, ExpectAnyOriginal)
                            {
                                expected_frame_var_ty = Some(ty)
                            }
                        }
                    }
                }
                if let Some(expected_frame_var_ty) = expected_frame_var_ty {
                    let place = Place::ImmutableStackOwned {
                        location: frame_var_symbol_idx
                            .into_local_symbol_idx(self.expr_region_data)
                            .into(),
                    };
                    let frame_var_symbol_ty =
                        SymbolType::new(self, frame_var_symbol_idx, expected_frame_var_ty);
                    self.symbol_tys
                        .insert_new(frame_var_symbol_idx, frame_var_symbol_ty)
                }
                let expr_expectation = self.expect_unit();
                self.infer_new_block(*block, expr_expectation);
                Some(self.term_menu.unit_ty_ontology().into())
            }
            SynStmtData::ForIn {
                ref condition,
                block,
                ..
            } => todo!(),
            SynStmtData::ForExt {
                ref particulars,
                block,
                ..
            } => {
                let Some(forext_loop_var_ty) =
                    self.build_new_expr_ty(particulars.forext_loop_var_expr_idx, ExpectIntType)
                else {
                    todo!()
                };
                self.build_new_expr_ty_discarded(
                    particulars.bound_expr,
                    ExpectCoersion::new_pure(self, forext_loop_var_ty),
                );
                let expr_expectation = self.expect_unit();
                self.infer_new_block(block, expr_expectation);
                Some(self.term_menu.unit_ty_ontology().into())
            }
            SynStmtData::While {
                ref condition,
                block,
                ..
            }
            | SynStmtData::DoWhile {
                ref condition,
                block,
                ..
            } => {
                condition.as_ref().copied().map(|condition| {
                    self.build_new_expr_ty_discarded(condition, ExpectConditionType)
                });
                let expect_unit = self.expect_unit();
                self.infer_new_block(block, expect_unit);
                Some(self.term_menu.unit_ty_ontology().into())
            }
            SynStmtData::IfElse {
                ref if_branch,
                ref elif_branches,
                ref else_branch,
            } => self.calc_if_else_stmt(
                if_branch,
                elif_branches,
                else_branch.as_ref(),
                expr_expectation,
            ),
            SynStmtData::Match { .. } => {
                // todo: match
                None
            }
        }
    }

    fn calc_if_else_stmt(
        &mut self,
        if_branch: &SynIfBranch,
        elif_branches: &[SynElifBranch],
        else_branch: Option<&SynElseBranch>,
        expr_expectation: impl ExpectFluffyTerm,
    ) -> Option<FluffyTerm> {
        let mut branch_tys = BranchTypes::new(expr_expectation);
        if_branch
            .condition
            .as_ref()
            .copied()
            .map(|condition| self.build_new_expr_ty(condition, ExpectConditionType));
        branch_tys.visit_branch(self, if_branch.stmts);
        for elif_branch in elif_branches {
            elif_branch
                .condition
                .as_ref()
                .copied()
                .map(|condition| self.build_new_expr_ty_discarded(condition, ExpectConditionType));
            branch_tys.visit_branch(self, elif_branch.stmts);
        }
        if let Some(else_branch) = else_branch {
            branch_tys.visit_branch(self, else_branch.stmts);
        }
        // exhaustive iff else branch exists
        branch_tys.merge(else_branch.is_some(), &self.term_menu)
    }
}

struct BranchTypes<Expectation: ExpectFluffyTerm> {
    /// this is true if the type of one of the branches cannot be inferred
    has_error: bool,
    /// this is true if the type of one of the branches is inferred to be not never
    ever_ty: Option<FluffyTerm>,
    expr_expectation: Expectation,
}

impl<Expectation: ExpectFluffyTerm> BranchTypes<Expectation> {
    fn new(expr_expectation: Expectation) -> Self {
        Self {
            has_error: false,
            ever_ty: None,
            expr_expectation,
        }
    }

    fn visit_branch(&mut self, engine: &mut ExprTypeEngine, block: SynStmtIdxRange) {
        let (stmts, new_block_ty) = engine.infer_new_block(block, self.expr_expectation.clone());
        match new_block_ty {
            Some(new_block_ty)
                if new_block_ty.base_resolved(engine)
                    == FluffyTermBase::Ethereal(EtherealTerm::EntityPath(
                        TermEntityPath::TypeOntology(engine.item_path_menu.never_ty_path()),
                    )) =>
            {
                ()
            }
            Some(new_block_ty) => {
                if self.ever_ty.is_none() {
                    self.ever_ty = Some(new_block_ty)
                }
            }
            None => self.has_error = true,
        }
    }

    fn merge(self, exhaustive: bool, menu: &EtherealTermMenu) -> Option<FluffyTerm> {
        if let Some(ever_ty) = self.ever_ty {
            return ever_ty.into();
        }
        (!self.has_error).then_some(menu.never().into())
    }
}