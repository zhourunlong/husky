use super::*;
use husky_expr::stmt::{LoopBoundaryKind, LoopStep};
use husky_hir_opr::suffix::HirSuffixOpr;

impl TranspileToRust<HirEagerExprRegion> for (IsLastStmt, HirEagerStmtIdx) {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let &(IsLastStmt(is_last_stmt), slf) = self;
        match *slf.data(builder.hir_eager_stmt_arena()) {
            HirEagerStmt::Let {
                pattern,
                initial_value,
            } => builder.on_new_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Let);
                pattern.transpile_to_rust(builder);
                builder.opr(RustOpr::Assign);
                any_precedence(initial_value).transpile_to_rust(builder)
            }),
            HirEagerStmt::Return { result } => builder.on_new_semicolon_line(|builder| {
                builder.keyword(RustKeyword::Return);
                any_precedence(result).transpile_to_rust(builder)
            }),
            HirEagerStmt::Require { condition } => builder.on_new_semicolon_line(|builder| {
                builder.macro_name(RustMacroName::Require);
                builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
                    condition.transpile_to_rust(builder)
                })
            }),
            HirEagerStmt::Assert { condition } => builder.on_new_semicolon_line(|builder| {
                builder.macro_name(RustMacroName::Assert);
                builder.heterogeneous_bracketed_comma_list(RustBracket::Par, |builder| {
                    condition.transpile_to_rust(builder)
                })
            }),
            HirEagerStmt::Break => {
                builder.on_new_semicolon_line(|builder| builder.keyword(RustKeyword::Break))
            }
            HirEagerStmt::Eval {
                expr_idx,
                discarded,
            } => match discarded || !is_last_stmt {
                true => builder.on_new_semicolon_line(|builder| {
                    any_precedence(expr_idx).transpile_to_rust(builder);
                }),
                false => builder.on_new_line(|builder| {
                    any_precedence(expr_idx).transpile_to_rust(builder);
                }),
            },
            HirEagerStmt::ForBetween {
                ref particulars,
                block,
            } => builder.on_new_line(|builder| {
                builder.keyword(RustKeyword::For);
                particulars.frame_var_ident.transpile_to_rust(builder);
                builder.keyword(RustKeyword::In);
                let range = &particulars.range;
                let t = |opd| (RustPrecedenceRange::Greater(RustPrecedence::Range), opd);
                match range.step {
                    LoopStep::Constant(step) => match step {
                        1 => {
                            match range.initial_boundary.kind {
                                LoopBoundaryKind::UpperOpen => unreachable!(),
                                LoopBoundaryKind::UpperClosed => unreachable!(),
                                LoopBoundaryKind::LowerOpen => todo!(),
                                LoopBoundaryKind::LowerClosed => {
                                    match range.initial_boundary.bound_expr {
                                        Some(initial_bound) => {
                                            t(initial_bound).transpile_to_rust(builder)
                                        }
                                        None => builder.zero(),
                                    }
                                }
                            }
                            builder.opr(match range.final_boundary.kind {
                                LoopBoundaryKind::UpperOpen => RustOpr::DotDot,
                                LoopBoundaryKind::UpperClosed => RustOpr::DotDotEq,
                                LoopBoundaryKind::LowerOpen => unreachable!(),
                                LoopBoundaryKind::LowerClosed => unreachable!(),
                            });
                            match range.final_boundary.bound_expr {
                                Some(final_bound) => t(final_bound).transpile_to_rust(builder),
                                None => builder.zero(), // ad hoc, todo: use Default::default()
                            }
                        }
                        -1 => todo!(),
                        _ => todo!(),
                    },
                }
                block.transpile_to_rust(builder)
            }),
            HirEagerStmt::Forext { particulars, block } => builder.on_new_line(|builder| {
                builder.keyword(RustKeyword::While);
                particulars.forext_loop_var_ident.transpile_to_rust(builder);
                match particulars.boundary_kind {
                    LoopBoundaryKind::UpperOpen => builder.opr(RustOpr::Less),
                    LoopBoundaryKind::UpperClosed => builder.opr(RustOpr::Leq),
                    LoopBoundaryKind::LowerOpen => builder.opr(RustOpr::Greater),
                    LoopBoundaryKind::LowerClosed => builder.opr(RustOpr::Geq),
                }
                (
                    RustPrecedenceRange::Greater(RustPrecedence::OrdComparison),
                    particulars.bound_expr_hir_eager_expr_idx,
                )
                    .transpile_to_rust(builder);
                builder.curly_block(|builder| {
                    builder.on_new_line(|builder| block.transpile_to_rust(builder));
                    builder.on_new_line(|builder| {
                        particulars.forext_loop_var_ident.transpile_to_rust(builder);
                        builder.opr(RustOpr::AddAssign);
                        match particulars.boundary_kind {
                            LoopBoundaryKind::UpperOpen | LoopBoundaryKind::UpperClosed => {
                                HirSuffixOpr::Incr.transpile_to_rust(builder)
                            }
                            LoopBoundaryKind::LowerOpen | LoopBoundaryKind::LowerClosed => {
                                HirSuffixOpr::Decr.transpile_to_rust(builder)
                            }
                        }
                    })
                })
            }),
            HirEagerStmt::ForIn {
                condition: _,
                block: _,
            } => todo!(),
            HirEagerStmt::While { condition, stmts } => builder.on_new_line(|builder| {
                builder.keyword(RustKeyword::While);
                condition.transpile_to_rust(builder);
                stmts.transpile_to_rust(builder)
            }),
            HirEagerStmt::DoWhile {
                condition: _,
                block: _,
            } => {
                builder.comment("DoWhile incomplete");
                builder.on_new_line(|builder| {
                    builder.keyword(RustKeyword::While);
                    true.transpile_to_rust(builder);
                })
                // block.transpile_to_rust(builder)
            }
            HirEagerStmt::IfElse {
                if_branch,
                ref elif_branches,
                else_branch,
            } => builder.on_new_line(|builder| {
                if_branch.transpile_to_rust(builder);
                for elif_branch in elif_branches {
                    elif_branch.transpile_to_rust(builder)
                }
                else_branch.transpile_to_rust(builder)
            }),
            HirEagerStmt::Match {} => {
                builder.on_new_line(|builder| builder.keyword(RustKeyword::Match))
            }
        }
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerCondition {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        any_precedence(self.hir_eager_expr_idx()).transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerLetVariablesPattern {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        self.pattern_expr_idx().transpile_to_rust(builder);
        if let Some(ty) = self.ty() {
            builder.opr(RustOpr::Colon);
            ty.transpile_to_rust(builder)
        }
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerIfBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::If);
        self.condition.transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerElifBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::Else);
        builder.keyword(RustKeyword::If);
        self.condition.transpile_to_rust(builder);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerElseBranch {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        builder.keyword(RustKeyword::Else);
        self.stmts.transpile_to_rust(builder)
    }
}

impl TranspileToRust<HirEagerExprRegion> for HirEagerStmtIdxRange {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let end = self.end();
        builder.curly_block(|builder| {
            for stmt in self {
                (IsLastStmt((stmt + 1) == end), stmt).transpile_to_rust(builder)
            }
        })
    }
}

struct IsLastStmt(bool);
