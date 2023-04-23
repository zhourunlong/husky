use husky_entity_tree::{
    EntityTreeError, IllFormedImplBlock, OriginalEntityTreeError, UseExprRuleState,
};

use super::*;

#[salsa::tracked(db = DiagnosticsDb, jar = DiagnosticsJar)]
pub struct EntityTreeDiagnosticSheet {
    #[return_ref]
    pub diagnostics: Vec<Diagnostic>,
}

#[salsa::tracked(jar = DiagnosticsJar)]
pub(crate) fn entity_tree_diagnostic_sheet(
    db: &dyn DiagnosticsDb,
    module_path: ModulePath,
) -> EntityTreeDiagnosticSheet {
    let mut diagnostics = vec![];
    let ctx = SheetDiagnosticsContext::new(db, module_path);
    if let Ok(entity_tree_sheet) = db.entity_tree_sheet(module_path) {
        for e in entity_tree_sheet.errors() {
            match e {
                EntityTreeError::Original(e) => diagnostics.push(e.to_diagnostic(&ctx)),
                EntityTreeError::Derived(_) => (),
            }
        }
        for ill_formed_impl_block in entity_tree_sheet.all_ill_formed_impl_blocks() {
            diagnostics.push(ill_formed_impl_block.to_diagnostic(&ctx))
        }
    }
    // todo
    EntityTreeDiagnosticSheet::new(db, diagnostics)
}

impl Diagnose for OriginalEntityTreeError {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, db: &Self::Context<'_>) -> String {
        match self {
            OriginalEntityTreeError::UnresolvedIdent(_) => format!("unresolved identifier"),
            OriginalEntityTreeError::SymbolExistsButNotAccessible(_) => {
                format!("SymbolNotAccessible")
            }
            OriginalEntityTreeError::NoSubentity => format!("NoSubentity"),
            OriginalEntityTreeError::EntitySymbolAlreadyDefined { old, new } => {
                format!("EntitySymbolAlreadyDefined")
            }
            OriginalEntityTreeError::ExpectIdentAfterKeyword => {
                format!("ExpectIdentAfterKeyword")
            }
            OriginalEntityTreeError::InvalidTypePath(_) => todo!(),
        }
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        match self {
            OriginalEntityTreeError::UnresolvedIdent(ident_token)
            | OriginalEntityTreeError::SymbolExistsButNotAccessible(ident_token) => ctx
                .ranged_token_sheet()
                .token_text_range(ident_token.token_idx()),
            OriginalEntityTreeError::NoSubentity => todo!(),
            OriginalEntityTreeError::EntitySymbolAlreadyDefined { old, new } => ctx
                .ranged_token_sheet()
                .token_text_range(new.ident_token(ctx.db()).token_idx()),
            OriginalEntityTreeError::ExpectIdentAfterKeyword => todo!(),
            OriginalEntityTreeError::InvalidTypePath(_) => todo!(),
        }
    }
}

impl Diagnose for IllFormedImplBlock {
    type Context<'a> = SheetDiagnosticsContext<'a>;

    fn message(&self, db: &Self::Context<'_>) -> String {
        "IllFormedImplBlock".to_string()
    }

    fn severity(&self) -> DiagnosticSeverity {
        DiagnosticSeverity::Error
    }

    fn range(&self, ctx: &Self::Context<'_>) -> TextRange {
        ctx.ranged_token_sheet()
            .tokens_text_range(ctx.ast_token_idx_range_sheet()[self.ast_idx(ctx.db())])
    }
}
