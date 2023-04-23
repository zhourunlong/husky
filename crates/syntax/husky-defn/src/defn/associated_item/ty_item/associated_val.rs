use crate::*;

#[salsa::tracked(db = DefnDb, jar = DefnJar)]
pub struct TypeAssociatedValDefn {
    #[id]
    pub entity_path: EntityPath,
    pub expr_region: ExprRegion,
    pub decl: TypeAssociatedValDecl,
}

#[salsa::tracked(jar = DefnJar)]
pub(crate) fn ty_associated_val_defn(
    _db: &dyn DefnDb,
    _decl: TypeAssociatedValDecl,
) -> TypeAssociatedValDefn {
    todo!()
}