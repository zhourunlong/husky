use husky_crate_decl_ast::{CrateDeclAst, CrateDeclAstIdxRange};

use super::*;

#[salsa::tracked]
pub struct LibCrateSynNodeDecl {
    #[id]
    pub path: CratePath,
    #[return_ref]
    pub items: Vec<SynNodeDeclResult<()>>,
    pub syn_expr_region: SynExprRegion,
}

impl<'db> CrateDeclParser<'db> {
    pub(super) fn parse_lib_crate_syn_node_decl(mut self) -> LibCrateSynNodeDecl {
        let db = self.db();
        let crate_path = self.crate_path();
        use husky_print_utils::p;
        use salsa::DebugWithDb;
        p!(crate_path.debug(db));
        let crate_decl_tokra_region_data = self.crate_decl_tokra_region_data();
        let mut items = vec![];
        for ast_idx in crate_decl_tokra_region_data.root_body() {
            items.push(match crate_decl_tokra_region_data[ast_idx] {
                CrateDeclAst::Err => todo!(),
                CrateDeclAst::BasicStmtOrBranch {
                    regional_token_verse_idx,
                    body,
                } => self.parse_basic_stmt(regional_token_verse_idx, body),
                CrateDeclAst::IfElseStmts {
                    if_branch,
                    elif_branches,
                    else_branch,
                } => todo!(),
                CrateDeclAst::MatchStmt {
                    regional_token_verse_idx,
                    pattern_stmt,
                    case_branches,
                } => todo!(),
            })
        }
        let syn_expr_region = self.finish();
        LibCrateSynNodeDecl::new(db, crate_path, items, syn_expr_region)
    }

    fn parse_basic_stmt(
        &mut self,
        regional_token_verse_idx: RegionalTokenVerseIdx,
        body: Option<CrateDeclAstIdxRange>,
    ) -> SynNodeDeclResult<()> {
        let mut parser = self.token_verse_expr_parser(regional_token_verse_idx);
        if let Some(basic_stmt_kw) = parser.try_parse_option::<BasicStmtLeadRegionalToken>()? {
            match basic_stmt_kw {
                BasicStmtLeadRegionalToken::Let(_) => todo!(),
                BasicStmtLeadRegionalToken::Return(_) => todo!(),
                BasicStmtLeadRegionalToken::Require(_) => todo!(),
                BasicStmtLeadRegionalToken::Assert(_) => todo!(),
                BasicStmtLeadRegionalToken::Break(_) => todo!(),
                BasicStmtLeadRegionalToken::For(_) => todo!(),
                BasicStmtLeadRegionalToken::ForExt(_) => todo!(),
                BasicStmtLeadRegionalToken::While(_) => todo!(),
                BasicStmtLeadRegionalToken::Do(_) => todo!(),
                BasicStmtLeadRegionalToken::Narrate(narrate_token) => {
                    self.parse_narrative(narrate_token)
                }
            }
        } else {
            todo!()
        }
    }

    fn parse_narrative(&mut self, narrate_token: NarrateRegionalToken) -> SynNodeDeclResult<()> {
        Ok(())
    }
}
