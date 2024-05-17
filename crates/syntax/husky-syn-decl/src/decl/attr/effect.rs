use super::*;
use parsec::PunctuatedSmallList;

#[salsa::tracked(constructor = new_inner)]
pub struct EffectAttrSynNodeDecl {
    #[id]
    pub syn_node_path: AttrSynNodePath,
    pub pound_token: PoundRegionalToken,
    pub effect_token: IdentRegionalToken,
    #[return_ref]
    pub lpar_token: SynNodeDeclResult<LparRegionalToken>,
    #[return_ref]
    pub effects: SynNodeDeclResult<
        PunctuatedSmallList<EffectSynNodeDecl, CommaRegionalToken, SynNodeDeclError, true, 8>,
    >,
    #[return_ref]
    pub rpar_token: SynNodeDeclResult<RparRegionalToken>,
    #[skip_fmt]
    pub syn_expr_region: SynExprRegion,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EffectSynNodeDecl {
    MajorStatic(),
}

/// # constructor

impl EffectAttrSynNodeDecl {
    pub(super) fn new(db: &::salsa::Db, syn_node_path: AttrSynNodePath) -> Self {
        todo!()
    }
}

/// # getters

impl EffectAttrSynNodeDecl {
    pub fn errors(self, db: &::salsa::Db) -> SynNodeDeclErrorRefs {
        todo!()
    }
}

#[salsa::tracked]
pub struct EffectAttrSynDecl {
    #[id]
    pub path: AttrItemPath,
    // todo: effects
    pub syn_expr_region: SynExprRegion,
}

impl EffectAttrSynDecl {
    pub(super) fn from_node(
        path: AttrItemPath,
        syn_node_decl: EffectAttrSynNodeDecl,
        db: &::salsa::Db,
    ) -> SynDeclResult<Self> {
        let syn_expr_region = syn_node_decl.syn_expr_region(db);
        Ok(Self::new(db, path, syn_expr_region))
    }
}
