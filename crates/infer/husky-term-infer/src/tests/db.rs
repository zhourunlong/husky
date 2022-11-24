mod init;
mod trivia;

use super::*;
use husky_entity_path::{
    EntityPath, EntityPathDb, EntityPathJar, EntityPathMenu, EntityPathMenuPlace,
};
use husky_expr_syntax::ExprIdx;
use husky_identifier::{IdentifierDb, IdentifierJar};
use husky_symbol_syntax::{Symbol, SymbolContext, SymbolKind};
use husky_term::{
    AskDecl, Decl, Term, TermDb, TermError, TermJar, TermMenu, TermResult, TermResultArc, TyDecl,
};
use std::{collections::HashMap, sync::Arc};
use upcast::Upcast;

#[salsa::db(TermJar, TermInferJar, EntityPathJar, IdentifierJar)]
pub(crate) struct TermInferTestsDb {
    storage: salsa::Storage<Self>,
    entity_tys: HashMap<EntityPath, Term>,
    decls: HashMap<EntityPath, Arc<Decl>>,
    prelude_symbols: Vec<Symbol>,
}

impl TermInferTestsDb {
    pub(crate) fn new() -> Self {
        let mut db = Self {
            storage: Default::default(),
            entity_tys: Default::default(),
            decls: Default::default(),
            prelude_symbols: Default::default(),
        };
        db.init();
        db
    }

    pub(super) fn parse_expr_from_text(&self, text: &str) -> (ExprArena, ExprIdx) {
        use husky_tokenize::Tokenize;
        let tokens = self.tokenize_line(text);
        let mut arena = ExprArena::new();
        todo!()
        // let expr = parse_expr(&mut symbol_ctx, &mut arena, &tokens);
        // (arena, expr)
    }
}

impl TyInferQueries for TermInferTestsDb {
    fn infer_entity_ty(&self, entity: EntityPath) -> Term {
        self.entity_tys[&entity]
    }
}

impl AskDecl for TermInferTestsDb {
    fn ask_namespace_decl(
        &self,
        namespace: husky_term::TermNamespace,
    ) -> TermResultArc<husky_term::NamespaceDecl> {
        todo!()
    }

    fn ask_ty_decl(&self, ty: Term) -> TermResultArc<TyDecl> {
        todo!()
    }

    fn ask_decl(&self, entity_path: EntityPath) -> TermResultArc<Decl> {
        self.decls.get(&entity_path).map_or(
            Err(TermError::NoDeclForEntityPath { entity_path }),
            |decl| Ok(decl.clone()),
        )
    }
}
