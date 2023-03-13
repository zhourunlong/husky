use super::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntityTreeDb)]
pub struct LetVariablesPattern {
    pattern: PatternExprIdx,
    variables: CurrentSymbolIdxRange,
    colon_token: ExprResult<Option<ColonToken>>,
    ty: Option<ExprIdx>,
}

impl<'a, 'b> ExprParseContext<'a, 'b> {
    pub(crate) fn parse_let_variable_pattern(
        &mut self,
        access_end: TokenIdxRangeEnd,
    ) -> ExprResult<LetVariablesPattern> {
        let state = self.state();
        if let Some(pattern) = self.parse_pattern_expr(PatternExprInfo::Let)? {
            let symbols = self.pattern_expr_region().pattern_symbol_map(pattern);
            let access_start = self.state();
            let symbols = symbols
                .iter()
                .map(|(ident, pattern_symbol)| {
                    CurrentSymbol::new(
                        access_start,
                        Some(access_end),
                        CurrentSymbolVariant::LetVariable {
                            ident: *ident,
                            pattern_symbol_idx: *pattern_symbol,
                        },
                    )
                })
                .collect::<Vec<_>>();
            let colon_token = self.parse::<ColonToken>().map_err(|e| e.into());
            let ty = match colon_token {
                Ok(Some(_)) => Some(self.parse_expr_expected2(
                    Some(ExprEnvironment::LetVariablesType),
                    OriginalExprError::ExpectedLetVariablesType,
                )),
                _ => None,
            };
            let ty_constraint = ty.map(|ty| PatternTypeConstraint::LetVariables { pattern, ty });
            let variables = self.define_symbols(symbols, ty_constraint);
            Ok(LetVariablesPattern {
                pattern,
                variables,
                colon_token,
                ty,
            })
        } else {
            Err(OriginalExprError::ExpectLetVariablePattern(state).into())
        }
    }
}

impl LetVariablesPattern {
    pub fn pattern_expr_idx(&self) -> ArenaIdx<PatternExpr> {
        self.pattern
    }

    pub fn ty(&self) -> Option<ExprIdx> {
        self.ty
    }

    pub fn pattern_expr(&self) -> PatternExprIdx {
        self.pattern
    }

    pub fn variables(&self) -> CurrentSymbolIdxRange {
        self.variables
    }
}
