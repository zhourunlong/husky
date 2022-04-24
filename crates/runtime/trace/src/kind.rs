use feature::*;
use vm::{History, InstructionSheet, LoopFrameSnapshot, StackValueSnapshot, VMControl};
use word::CustomIdentifier;

use crate::*;

#[derive(Debug, Clone)]
pub enum TraceVariant<'eval> {
    Main(Arc<FeatureBlock>),
    FeatureStmt(Arc<FeatureStmt>),
    FeatureBranch(Arc<FeatureBranch>),
    FeatureExpr(Arc<FeatureExpr>),
    FeatureCallInput {
        ident: CustomIdentifier,
        input: Arc<FeatureExpr>,
    },
    FuncStmt {
        stmt: Arc<FuncStmt>,
        history: Arc<History<'eval>>,
    },
    ProcStmt {
        stmt: Arc<ProcStmt>,
        history: Arc<History<'eval>>,
    },
    LoopFrame {
        loop_stmt: Arc<ProcStmt>,
        body_instruction_sheet: Arc<InstructionSheet>,
        body_stmts: Arc<Vec<Arc<ProcStmt>>>,
        loop_frame_snapshot: LoopFrameSnapshot<'eval>,
    },
    EagerExpr {
        expr: Arc<EagerExpr>,
        history: Arc<History<'eval>>,
    },
    CallHead {
        entity: Arc<EntityDefn>,
        tokens: Vec<TokenProps<'eval>>,
    },
}

impl<'eval> TraceVariant<'eval> {
    pub fn file_and_range(&self) -> (FilePtr, TextRange) {
        match self {
            TraceVariant::Main(ref block) => (block.file, block.range),
            TraceVariant::FeatureStmt(ref stmt) => (stmt.file, stmt.range),
            TraceVariant::FeatureExpr(ref expr) => (expr.expr.file, expr.expr.range),
            TraceVariant::FeatureBranch(ref branch) => (branch.block.file, branch.block.range),
            TraceVariant::FeatureCallInput { input, .. } => (input.expr.file, input.expr.range),
            TraceVariant::FuncStmt { ref stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::EagerExpr { ref expr, .. } => (expr.file, expr.range),
            TraceVariant::CallHead { ref entity, .. } => (entity.file, entity.range),
            TraceVariant::ProcStmt { stmt, .. } => (stmt.file, stmt.range),
            TraceVariant::LoopFrame { loop_stmt, .. } => (loop_stmt.file, loop_stmt.range),
        }
    }
}

impl<'eval> Serialize for TraceVariant<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(match self {
            TraceVariant::Main(_) => "Main",
            TraceVariant::FeatureStmt(_) => "FeatureStmt",
            TraceVariant::FeatureBranch(_) => "FeatureBranch",
            TraceVariant::FeatureExpr(_) => "FeatureExpr",
            TraceVariant::FeatureCallInput { .. } => "FeatureCallInput",
            TraceVariant::FuncStmt { .. } => "FuncStmt",
            TraceVariant::ProcStmt { .. } => "ProcStmt",
            TraceVariant::EagerExpr { .. } => "EagerExpr",
            TraceVariant::CallHead { .. } => "CallHead",
            TraceVariant::LoopFrame { .. } => "LoopFrame",
        })
    }
}
