mod id;
mod impl_feature_expr;
mod impl_feature_stmt;
mod impl_routine_call;
mod indicator;

pub(crate) use id::FeatureEvalId;
pub use indicator::FeatureEvalIndicator;

use vm::{AnyValueDyn, BasicInterpreter, Instruction, Interpreter, VMResult};

use crate::{sheet::FeatureSheet, *};
use vm::{Conditional, EvalValue, StackValue};

pub fn eval_feature_block<'eval>(
    block: &FeatureBlock,
    input: Arc<dyn AnyValueDyn>,
    sheet: &mut FeatureSheet<'eval>,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator { input, sheet };
    evaluator.eval_feature_block(block)
}

pub fn eval_feature_stmt<'eval>(
    stmt: &FeatureStmt,
    input: Arc<dyn AnyValueDyn>,
    sheet: &mut FeatureSheet<'eval>,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator { input, sheet };
    evaluator.eval_feature_stmt(stmt)
}

pub fn eval_feature_expr<'eval>(
    expr: &FeatureExpr,
    input: Arc<dyn AnyValueDyn>,
    sheet: &mut FeatureSheet<'eval>,
) -> EvalValue<'eval, 'eval> {
    let mut evaluator = FeatureEvaluator { input, sheet };
    evaluator.eval_feature_expr(expr)
}

pub struct FeatureEvaluator<'a, 'eval: 'a> {
    input: Arc<dyn AnyValueDyn>,
    sheet: &'a mut FeatureSheet<'eval>,
}

impl<'a, 'eval: 'a> FeatureEvaluator<'a, 'eval> {
    fn eval_feature_block(&mut self, block: &FeatureBlock) -> EvalValue<'eval, 'eval> {
        self.cache(block.feature, |this: &mut Self| {
            for stmt in block.stmts.iter() {
                let value = this.eval_feature_stmt(stmt)?;
                match value {
                    Conditional::Defined(_) => return Ok(value),
                    Conditional::Undefined => (),
                }
            }
            Ok(Conditional::Undefined)
        })
    }

    fn cache(
        &mut self,
        feature: FeaturePtr,
        compute_value: impl FnOnce(&mut Self) -> EvalValue<'eval, 'eval>,
    ) -> EvalValue<'eval, 'eval> {
        if let Some(value) = self.sheet.cached_value(feature) {
            value
        } else {
            let value = compute_value(self);
            self.sheet.cache(feature, value)
        }
    }
}
