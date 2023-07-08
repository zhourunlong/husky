use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeExprBeforeColon {
    expr: ExprIdx,
}

impl ReturnTypeExprBeforeColon {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for ReturnTypeExprBeforeColon {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) = ctx.parse_expr_root(None, ExprRootKind::ReturnType) {
            Ok(Some(ReturnTypeExprBeforeColon { expr }))
        } else {
            Ok(None)
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct ReturnTypeExprBeforeEq {
    expr: ExprIdx,
}

impl ReturnTypeExprBeforeEq {
    pub fn expr(&self) -> ExprIdx {
        self.expr
    }
}

impl<'a, 'b> TryParseOptionFromStream<ExprParseContext<'a, 'b>> for ReturnTypeExprBeforeEq {
    type Error = ExprError;

    fn try_parse_option_from_stream_without_guaranteed_rollback(
        ctx: &mut ExprParseContext<'a, 'b>,
    ) -> ExprResult<Option<Self>> {
        if let Some(expr) =
            ctx.parse_expr_root(ExprEnvironment::TypeBeforeEq, ExprRootKind::ReturnType)
        {
            Ok(Some(ReturnTypeExprBeforeEq { expr }))
        } else {
            Ok(None)
        }
    }
}
