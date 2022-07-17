use super::*;
use husky_ast::{CasePattern, CasePatternVariant};
use husky_eager_semantics::{
    Boundary, FuncConditionFlowBranchVariant, FuncPatternBranchVariant, FuncStmt, FuncStmtVariant,
    LoopVariant, ProcConditionFlowBranchVariant, ProcPatternBranchVariant, ProcStmt,
    ProcStmtVariant,
};
use husky_entity_semantics::{EntityDefn, EntityDefnVariant, FieldDefnVariant};

impl<'a> LinkageCollector<'a> {
    pub(crate) fn collect_from_func_stmts(&mut self, stmts: &[Arc<FuncStmt>]) {
        for stmt in stmts {
            match stmt.variant {
                FuncStmtVariant::Init {
                    ref initial_value, ..
                } => self.collect_from_eager_expr(initial_value),
                FuncStmtVariant::Assert { ref condition } => {
                    self.collect_from_eager_expr(condition)
                }
                FuncStmtVariant::Return { ref result, .. } => self.collect_from_eager_expr(result),
                FuncStmtVariant::ConditionFlow { ref branches } => {
                    for branch in branches {
                        match branch.variant {
                            FuncConditionFlowBranchVariant::If { ref condition } => {
                                self.collect_from_eager_expr(condition)
                            }
                            FuncConditionFlowBranchVariant::Elif { ref condition } => {
                                self.collect_from_eager_expr(condition)
                            }
                            FuncConditionFlowBranchVariant::Else => (),
                        }
                        self.collect_from_func_stmts(&branch.stmts)
                    }
                }
                FuncStmtVariant::Match {
                    ref match_expr,
                    ref branches,
                } => {
                    self.collect_from_eager_expr(match_expr);
                    for branch in branches {
                        self.collect_from_func_stmts(&branch.stmts)
                    }
                }
            }
        }
    }

    pub(crate) fn collect_from_proc_stmts(&mut self, stmts: &[Arc<ProcStmt>]) {
        for stmt in stmts {
            match stmt.variant {
                ProcStmtVariant::Init {
                    ref initial_value, ..
                } => self.collect_from_eager_expr(initial_value),
                ProcStmtVariant::Assert { ref condition } => {
                    self.collect_from_eager_expr(condition)
                }
                ProcStmtVariant::Execute { ref expr } => self.collect_from_eager_expr(expr),
                ProcStmtVariant::ConditionFlow { ref branches } => {
                    for branch in branches {
                        match branch.variant {
                            ProcConditionFlowBranchVariant::If { ref condition } => {
                                self.collect_from_eager_expr(condition)
                            }
                            ProcConditionFlowBranchVariant::Elif { ref condition } => {
                                self.collect_from_eager_expr(condition)
                            }
                            ProcConditionFlowBranchVariant::Else => (),
                        }
                        self.collect_from_proc_stmts(&branch.stmts)
                    }
                }
                ProcStmtVariant::Loop {
                    ref loop_variant,
                    ref stmts,
                } => {
                    match loop_variant {
                        LoopVariant::For {
                            frame_var,
                            initial_boundary,
                            final_boundary,
                            step,
                        } => {
                            self.collect_from_boundary(initial_boundary);
                            self.collect_from_boundary(final_boundary)
                        }
                        LoopVariant::ForExt {
                            frame_var,
                            final_boundary,
                            step,
                        } => self.collect_from_boundary(final_boundary),
                        LoopVariant::While { condition } => self.collect_from_eager_expr(condition),
                        LoopVariant::DoWhile { condition } => {
                            self.collect_from_eager_expr(condition)
                        }
                    }
                    self.collect_from_proc_stmts(stmts)
                }
                ProcStmtVariant::Break => (),
                ProcStmtVariant::Return { ref result, .. } => self.collect_from_eager_expr(result),
                ProcStmtVariant::Match {
                    ref match_expr,
                    ref branches,
                } => {
                    self.collect_from_eager_expr(match_expr);
                    for branch in branches {
                        self.collect_from_proc_stmts(&branch.stmts)
                    }
                }
            }
        }
    }

    fn collect_from_boundary(&mut self, boundary: &Boundary) {
        if let Some(ref bound) = boundary.opt_bound {
            self.collect_from_eager_expr(bound)
        }
    }
}
