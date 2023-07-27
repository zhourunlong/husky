use super::*;

#[salsa::interned(db = HirDeclDb, jar = HirDeclJar)]
pub struct TypeMemoizedFieldHirDecl {
    pub path: TypeItemPath,
    pub impl_block: TypeImplBlockHirDecl,
    pub return_ty: EtherealTerm,
    pub hir_expr_region: HirEagerExprRegion,
}
