mod associated_fn;
mod associated_ty;
mod associated_val;
mod memoized_field;
mod method_fn;

pub use self::associated_fn::*;
pub use self::associated_ty::*;
pub use self::associated_val::*;
pub use self::memoized_field::*;
pub use self::method_fn::*;

use super::*;
use husky_ast::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemNodeDecl {
    AssociatedFn(TraitForTypeAssociatedFnNodeDecl),
    MethodFn(TraitForTypeMethodFnNodeDecl),
    AssociatedType(TraitForTypeAssociatedTypeNodeDecl),
    AssociatedVal(TraitForTypeAssociatedValNodeDecl),
}

impl From<TraitForTypeItemNodeDecl> for SynNodeDecl {
    fn from(decl: TraitForTypeItemNodeDecl) -> Self {
        SynNodeDecl::AssociatedItem(decl.into())
    }
}

impl TraitForTypeItemNodeDecl {
    pub fn syn_node_path(self, db: &dyn DeclDb) -> TraitForTypeItemSynNodePath {
        match self {
            TraitForTypeItemNodeDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemNodeDecl::MethodFn(decl) => decl.syn_node_path(db),
            TraitForTypeItemNodeDecl::AssociatedType(decl) => decl.syn_node_path(db),
            TraitForTypeItemNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn ast_idx(self, db: &dyn DeclDb) -> AstIdx {
        match self {
            TraitForTypeItemNodeDecl::AssociatedFn(decl) => decl.ast_idx(db),
            TraitForTypeItemNodeDecl::MethodFn(decl) => decl.ast_idx(db),
            TraitForTypeItemNodeDecl::AssociatedType(decl) => decl.ast_idx(db),
            TraitForTypeItemNodeDecl::AssociatedVal(decl) => decl.ast_idx(db),
        }
    }

    pub fn generic_parameters<'a>(self, _db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            TraitForTypeItemNodeDecl::AssociatedFn(_) => todo!(),
            TraitForTypeItemNodeDecl::MethodFn(_) => todo!(),
            TraitForTypeItemNodeDecl::AssociatedType(_) => todo!(),
            TraitForTypeItemNodeDecl::AssociatedVal(_) => todo!(),
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            TraitForTypeItemNodeDecl::AssociatedFn(decl) => decl.expr_region(db),
            TraitForTypeItemNodeDecl::MethodFn(decl) => decl.expr_region(db),
            TraitForTypeItemNodeDecl::AssociatedType(decl) => decl.expr_region(db),
            TraitForTypeItemNodeDecl::AssociatedVal(decl) => decl.expr_region(db),
        }
    }

    pub fn errors(self, db: &dyn DeclDb) -> NodeDeclErrorRefs {
        match self {
            TraitForTypeItemNodeDecl::AssociatedFn(node_decl) => node_decl.errors(db),
            TraitForTypeItemNodeDecl::MethodFn(node_decl) => node_decl.errors(db),
            TraitForTypeItemNodeDecl::AssociatedType(node_decl) => node_decl.errors(db),
            TraitForTypeItemNodeDecl::AssociatedVal(node_decl) => node_decl.errors(db),
        }
    }
}

impl HasNodeDecl for TraitForTypeItemSynNodePath {
    type NodeDecl = TraitForTypeItemNodeDecl;

    fn node_decl<'a>(self, db: &'a dyn DeclDb) -> Self::NodeDecl {
        trai_for_ty_item_node_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_node_decl(
    db: &dyn DeclDb,
    syn_node_path: TraitForTypeItemSynNodePath,
) -> TraitForTypeItemNodeDecl {
    let parser = DeclParser::new(db, syn_node_path.module_path(db));
    parser.parse_trai_for_ty_item_node_decl(syn_node_path)
}

impl<'a> DeclParser<'a> {
    fn parse_trai_for_ty_item_node_decl(
        &self,
        syn_node_path: TraitForTypeItemSynNodePath,
    ) -> TraitForTypeItemNodeDecl {
        let db = self.db();
        let node = syn_node_path.node(db);
        let ast_idx = node.ast_idx(db);
        match self.ast_sheet()[ast_idx] {
            Ast::Defn {
                token_group_idx,
                entity_kind:
                    EntityKind::AssociatedItem {
                        associated_item_kind: AssociatedItemKind::TraitForTypeItem(item_kind),
                    },
                saved_stream_state,
                ..
            } => self.parse_trai_for_ty_item_node_decl_aux(
                syn_node_path,
                node,
                ast_idx,
                token_group_idx,
                item_kind,
                saved_stream_state,
            ),
            _ => unreachable!(),
        }
    }

    pub(super) fn parse_trai_for_ty_item_node_decl_aux(
        &self,
        syn_node_path: TraitForTypeItemSynNodePath,
        node: TraitForTypeItemNode,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        trai_item_kind: TraitItemKind,
        saved_stream_state: TokenStreamState,
    ) -> TraitForTypeItemNodeDecl {
        match trai_item_kind {
            TraitItemKind::MethodFn => self
                .parse_trai_for_ty_method_fn_node_decl(
                    syn_node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
            TraitItemKind::AssociatedType => self
                .parse_trai_for_ty_associated_ty_node_decl(
                    syn_node_path,
                    node,
                    ast_idx,
                    token_group_idx,
                    saved_stream_state,
                )
                .into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::derive_debug_with_db(db = DeclDb)]
#[enum_class::from_variants]
pub enum TraitForTypeItemDecl {
    AssociatedFn(TraitForTypeAssociatedFnDecl),
    MethodFn(TraitForTypeMethodFnDecl),
    AssociatedType(TraitForTypeAssociatedTypeDecl),
    AssociatedVal(TraitForTypeAssociatedValDecl),
}

impl From<TraitForTypeItemDecl> for Decl {
    fn from(decl: TraitForTypeItemDecl) -> Self {
        Decl::AssociatedItem(decl.into())
    }
}

impl TraitForTypeItemDecl {
    fn from_node_decl(
        db: &dyn DeclDb,
        path: TraitForTypeItemPath,
        node_decl: TraitForTypeItemNodeDecl,
    ) -> DeclResult<Self> {
        Ok(match node_decl {
            TraitForTypeItemNodeDecl::AssociatedFn(node_decl) => {
                TraitForTypeAssociatedFnDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TraitForTypeItemNodeDecl::MethodFn(node_decl) => {
                TraitForTypeMethodFnDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TraitForTypeItemNodeDecl::AssociatedType(node_decl) => {
                TraitForTypeAssociatedTypeDecl::from_node_decl(db, path, node_decl)?.into()
            }
            TraitForTypeItemNodeDecl::AssociatedVal(_) => todo!(),
        })
    }

    pub fn path(self, db: &dyn DeclDb) -> TraitForTypeItemPath {
        match self {
            TraitForTypeItemDecl::AssociatedFn(decl) => decl.path(db),
            TraitForTypeItemDecl::MethodFn(decl) => decl.path(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.path(db),
            TraitForTypeItemDecl::AssociatedVal(decl) => decl.path(db),
        }
    }

    pub fn generic_parameters<'a>(self, db: &'a dyn DeclDb) -> &'a [GenericParameterDecl] {
        match self {
            TraitForTypeItemDecl::AssociatedFn(decl) => decl.generic_parameters(db),
            TraitForTypeItemDecl::MethodFn(decl) => decl.generic_parameters(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.generic_parameters(db),
            TraitForTypeItemDecl::AssociatedVal(_) => &[],
        }
    }

    pub fn expr_region(self, db: &dyn DeclDb) -> SynExprRegion {
        match self {
            TraitForTypeItemDecl::AssociatedFn(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::MethodFn(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::AssociatedType(decl) => decl.expr_region(db),
            TraitForTypeItemDecl::AssociatedVal(decl) => decl.expr_region(db),
        }
    }
}

impl HasDecl for TraitForTypeItemPath {
    type Decl = TraitForTypeItemDecl;

    fn decl(self, db: &dyn DeclDb) -> DeclResult<Self::Decl> {
        trai_for_ty_item_decl(db, self)
    }
}

#[salsa::tracked(jar = SynDeclJar)]
pub(crate) fn trai_for_ty_item_decl(
    db: &dyn DeclDb,
    path: TraitForTypeItemPath,
) -> DeclResult<TraitForTypeItemDecl> {
    let node_decl = path.syn_node_path(db).node_decl(db);
    TraitForTypeItemDecl::from_node_decl(db, path, node_decl)
}
