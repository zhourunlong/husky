use super::*;

#[salsa::interned(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeItemNodePath {
    maybe_ambiguous_path: MaybeAmbiguousPath<TypeItemPath>,
}

impl TypeItemNodePath {
    fn new(db: &dyn EntityTreeDb, registry: &mut EntityNodeRegistry, path: TypeItemPath) -> Self {
        Self::new_inner(db, registry.issue_maybe_ambiguous_path(path))
    }

    pub fn path(self, db: &dyn EntityTreeDb) -> Option<TypeItemPath> {
        self.maybe_ambiguous_path(db).unambiguous_path()
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.maybe_ambiguous_path(db).path.module_path(db)
    }

    pub fn impl_block(self, db: &dyn EntityTreeDb) -> TypeImplBlockNodePath {
        self.maybe_ambiguous_path(db)
            .path
            .impl_block(db)
            .node_path(db)
    }

    pub fn item_kind(self, db: &dyn EntityTreeDb) -> TypeItemKind {
        self.maybe_ambiguous_path(db).path.item_kind(db)
    }

    pub fn node(self, db: &dyn EntityTreeDb) -> TypeItemNode {
        todo!()
    }
}

impl From<TypeItemNodePath> for EntityNodePath {
    fn from(id: TypeItemNodePath) -> Self {
        EntityNodePath::AssociatedItem(id.into())
    }
}

#[salsa::tracked(db = EntityTreeDb, jar = EntityTreeJar, constructor = new_inner)]
pub struct TypeItemNode {
    #[id]
    pub node_path: TypeItemNodePath,
    pub impl_block_node_path: TypeImplBlockNodePath,
    pub ast_idx: AstIdx,
    pub ident: Ident,
    pub item_kind: TypeItemKind,
    pub visibility: Scope,
    pub is_generic: bool,
}

impl TypeItemNode {
    #[inline(always)]
    fn new(
        db: &dyn EntityTreeDb,
        registry: &mut EntityNodeRegistry,
        impl_block_node_path: TypeImplBlockNodePath,
        ast_idx: AstIdx,
        ident: Ident,
        item_kind: TypeItemKind,
        visibility: Scope,
        is_generic: bool,
    ) -> (TypeItemNodePath, TypeItemNode) {
        let path = TypeItemPath::new(db, impl_block_node_path.path(), ident, item_kind);
        let node_path = TypeItemNodePath::new(db, registry, path);
        (
            node_path,
            TypeItemNode::new_inner(
                db,
                node_path,
                impl_block_node_path,
                ast_idx,
                ident,
                item_kind,
                visibility,
                is_generic,
            ),
        )
    }

    pub fn module_path(self, db: &dyn EntityTreeDb) -> ModulePath {
        self.node_path(db).module_path(db)
    }
}

#[salsa::tracked(jar = EntityTreeJar, return_ref)]
pub(crate) fn ty_impl_block_items(
    db: &dyn EntityTreeDb,
    impl_block_node_path: TypeImplBlockNodePath,
) -> Vec<(Ident, TypeItemNodePath, TypeItemNode)> {
    let impl_block_node = impl_block_node_path.node(db);
    let module_path = impl_block_node_path.module_path(db);
    let ast_sheet = db.ast_sheet(module_path).unwrap();
    let items = impl_block_node.items(db);
    let mut registry = EntityNodeRegistry::default();
    items
        .ast_idx_range()
        .into_iter()
        .filter_map(|ast_idx| {
            let ast = &ast_sheet[ast_idx];
            match ast {
                Ast::Defn {
                    visibility_expr,
                    entity_kind,
                    ident_token,
                    is_generic,
                    ..
                } => {
                    let item_kind = match entity_kind {
                        EntityKind::AssociatedItem {
                            associated_item_kind: AssociatedItemKind::TypeItem(ty_item_kind),
                        } => *ty_item_kind,
                        _ => unreachable!(),
                    };
                    let (node_path, node) = TypeItemNode::new(
                        db,
                        &mut registry,
                        impl_block_node_path,
                        ast_idx,
                        ident_token.ident(),
                        item_kind,
                        visibility_expr.visibility(),
                        *is_generic,
                    );
                    Some((ident_token.ident(), node_path, node))
                }
                Ast::Err { .. } => None,
                _ => {
                    let ast_token_idx_range_sheet =
                        db.ast_token_idx_range_sheet(module_path).unwrap();
                    let token_sheet_data = db.token_sheet_data(module_path).unwrap();
                    let ast_range = ast_token_idx_range_sheet[ast_idx];
                    // p!(ast_range);
                    // assert!(token_sheet_data.len() >= ast_range.end().token_idx().raw());
                    // p!(token_sheet_data[ast_range.start().token_idx()].debug(db));
                    // p!(token_sheet_data[ast_range.start().token_idx() + 1].debug(db));
                    // p!(module_path.debug(db), impl_block.debug(db));
                    // p!(ast.debug(db));
                    // p!(token_sheet_data.debug(db));
                    todo!()
                }
            }
        })
        .collect()
}