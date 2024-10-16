use super::{ty::TypeSynNodePath, *};
use husky_ast::DefnBlock;
use husky_entity_kind::TypeKind;
use husky_entity_path::path::{major_item::ty::TypePath, ty_variant::TypeVariantPath};

#[salsa::derive_debug_with_db]
#[salsa::as_id]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVariantSynNodePath(ItemSynNodePathId);

#[salsa::derive_debug_with_db]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TypeVariantSynNodePathData {
    pub parent_ty_node_path: TypeSynNodePath,
    disambiguated_item_path: DisambiguatedItemPath<TypeVariantPath>,
}

impl TypeVariantSynNodePath {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        parent_ty_node_path: TypeSynNodePath,
        ty_variant_path: TypeVariantPath,
    ) -> Self {
        Self(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::TypeVariant(TypeVariantSynNodePathData {
                parent_ty_node_path,
                disambiguated_item_path: registry.issue_maybe_ambiguous_path(ty_variant_path),
            }),
        ))
    }
}

/// # getters
impl TypeVariantSynNodePath {
    pub fn unambiguous_item_path(self, db: &::salsa::Db) -> Option<TypeVariantPath> {
        Some(match self.0.unambiguous_item_path(db)? {
            ItemPath::TypeVariant(_, path) => path,
            _ => unreachable!(),
        })
    }

    pub fn data(self, db: &::salsa::Db) -> TypeVariantSynNodePathData {
        match self.0.data(db) {
            ItemSynNodePathData::TypeVariant(data) => data,
            _ => unreachable!(),
        }
    }

    pub fn parent(self, db: &::salsa::Db) -> TypePath {
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .parent_ty_path(db)
    }

    pub fn ident(self, db: &::salsa::Db) -> Ident {
        self.data(db)
            .disambiguated_item_path
            .maybe_ambiguous_item_path
            .ident(db)
    }

    pub(crate) fn syn_node<'a>(self, db: &'a ::salsa::Db) -> &'a TypeVariantSynNode {
        self.data(db)
            .parent_ty_node_path
            .ty_variant_syn_nodes(db)
            .iter()
            .find_map(|&(_, syn_node_path, ref node)| (self == syn_node_path).then_some(node))
            .unwrap()
    }
}

impl TypeVariantSynNodePathData {
    #[inline(always)]
    pub fn syn_node_path(self, id: ItemSynNodePathId) -> TypeVariantSynNodePath {
        TypeVariantSynNodePath(id)
    }

    pub fn unambiguous_item_path(self) -> Option<TypeVariantPath> {
        self.disambiguated_item_path.unambiguous_item_path()
    }

    pub fn module_path(self, db: &::salsa::Db) -> ModulePath {
        self.disambiguated_item_path
            .maybe_ambiguous_item_path
            .module_path(db)
    }

    pub fn ast_idx(self, id: ItemSynNodePathId, db: &::salsa::Db) -> AstIdx {
        TypeVariantSynNodePath(id).syn_node(db).ast_idx
    }
}

impl TypeSynNodePath {
    pub fn ty_variant_syn_nodes<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> &'a [(Ident, TypeVariantSynNodePath, TypeVariantSynNode)] {
        ty_variant_syn_nodes(db, self)
    }

    pub fn ty_variant_syn_node_paths<'a>(
        self,
        db: &'a ::salsa::Db,
    ) -> impl Iterator<Item = TypeVariantSynNodePath> + 'a {
        self.ty_variant_syn_nodes(db)
            .iter()
            .map(|&(_, syn_node_path, _)| syn_node_path)
    }
}

impl HasSynNodePath for TypeVariantPath {
    type SynNodePath = TypeVariantSynNodePath;

    fn syn_node_path(self, db: &::salsa::Db) -> Self::SynNodePath {
        TypeVariantSynNodePath(ItemSynNodePathId::new(
            db,
            ItemSynNodePathData::TypeVariant(TypeVariantSynNodePathData {
                parent_ty_node_path: self.parent_ty_path(db).syn_node_path(db),
                disambiguated_item_path: DisambiguatedItemPath::from_path(self),
            }),
        ))
    }
}

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) struct TypeVariantSynNode {
    pub(crate) syn_node_path: TypeVariantSynNodePath,
    pub(crate) ast_idx: AstIdx,
    pub(crate) ident_token: IdentToken,
}

impl TypeVariantSynNode {
    fn new(
        db: &::salsa::Db,
        registry: &mut ItemSynNodePathRegistry,
        ty_node_path: TypeSynNodePath,
        ty_variant_path: TypeVariantPath,
        ast_idx: AstIdx,
        ident_token: IdentToken,
    ) -> (TypeVariantSynNodePath, Self) {
        let syn_node_path =
            TypeVariantSynNodePath::new(db, registry, ty_node_path, ty_variant_path);
        (
            syn_node_path,
            TypeVariantSynNode {
                syn_node_path,
                ast_idx,
                ident_token,
            },
        )
    }
}

#[salsa::tracked(return_ref)]
pub(crate) fn ty_variant_syn_nodes(
    db: &::salsa::Db,
    ty_node_path: TypeSynNodePath,
) -> Vec<(Ident, TypeVariantSynNodePath, TypeVariantSynNode)> {
    let module_path: ModulePath = ty_node_path.module_path(db);
    let ast_sheet = module_path.ast_sheet(db);
    match ty_node_path.ty_kind(db) {
        TypeKind::Enum | TypeKind::Inductive => (),
        _ => return vec![],
    }
    let mut registry = ItemSynNodePathRegistry::default();
    let AstData::Identifiable {
        block: DefnBlock::Type { variants, .. },
        ..
    } = ast_sheet[ty_node_path.syn_node(db).ast_idx]
    else {
        unreachable!()
    };
    let Some(variants) = variants else {
        return vec![];
    };
    ast_sheet
        .indexed_iter(variants.ast_idx_range())
        .filter_map(|(ast_idx, variant_ast)| match variant_ast {
            AstData::TypeVariant {
                variant_path,
                ident_token,
                ..
            } => {
                let ident = ident_token.ident();
                let (syn_node_path, node) = TypeVariantSynNode::new(
                    db,
                    &mut registry,
                    ty_node_path,
                    *variant_path,
                    ast_idx,
                    *ident_token,
                );
                Some((ident, syn_node_path, node))
            }
            AstData::Err { .. } => None,
            _ => unreachable!(),
        })
        .collect()
}

pub trait HasTypeVariantPaths: Copy {
    fn ty_variant_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [(Ident, TypeVariantPath)];
}

impl HasTypeVariantPaths for TypePath {
    fn ty_variant_paths<'a>(self, db: &'a ::salsa::Db) -> &'a [(Ident, TypeVariantPath)] {
        ty_variant_paths(db, self)
    }
}

/// guaranteed that each ident is unique
#[salsa::tracked(return_ref)]
pub(crate) fn ty_variant_paths(db: &::salsa::Db, path: TypePath) -> Vec<(Ident, TypeVariantPath)> {
    path.syn_node_path(db)
        .ty_variant_syn_nodes(db)
        .iter()
        .filter_map(|&(ident, variant_node_path, _)| {
            Some((ident, variant_node_path.unambiguous_item_path(db)?))
        })
        .collect()
}
