use super::*;

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar)]
pub struct TraitAssociatedValNodeDefn {
    #[id]
    pub syn_node_path: TraitItemSynNodePath,
    pub node_decl: TraitAssociatedValDecl,
    pub expr_region: SynExprRegion,
}

#[salsa::tracked(db = SynDefnDb, jar = SynDefnJar, constructor = new_inner)]
pub struct TraitAssociatedValDefn {
    #[id]
    pub syn_node_path: TraitItemPath,
    pub decl: TraitAssociatedValDecl,
    pub expr_region: SynExprRegion,
}
