mod prelude;

pub use self::prelude::*;

use crate::*;
use husky_manifest::PackageDependency;
use husky_print_utils::p;
use husky_token::IdentToken;

#[derive(Debug, Default, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct EntitySymbolTable(Vec<EntitySymbolEntry>);

impl EntitySymbolTable {
    pub(crate) fn as_ref<'a>(&'a self) -> EntitySymbolTableRef<'a> {
        EntitySymbolTableRef(&self.0)
    }

    pub(crate) fn data<'a>(&'a self) -> &'a [EntitySymbolEntry] {
        &self.0
    }

    pub(crate) fn insert(&mut self, new_entry: EntitySymbolEntry) -> EntitySynTreeResult<()> {
        // todo: should there be checks?
        self.0.push(new_entry);
        Ok(())
    }

    pub(crate) fn extend(
        &mut self,
        iter: impl IntoIterator<Item = EntitySymbolEntry>,
    ) -> EntitySynTreeResult<()> {
        for new_entry in iter {
            self.insert(new_entry)?
        }
        Ok(())
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct EntitySymbolTableRef<'a>(&'a [EntitySymbolEntry]);

impl<'a> EntitySymbolTableRef<'a> {
    // todo: add token_idx: TokenIdx
    pub fn resolve_ident(
        &self,
        db: &dyn EntitySynTreeDb,
        reference_module_path: ReferenceModulePath,
        ident: Ident,
    ) -> Option<EntitySymbol> {
        self.0.iter().find_map(|entry| {
            ((entry.ident == ident) && entry.is_visible_from(db, reference_module_path))
                .then_some(entry.symbol)
        })
    }

    pub(crate) fn data(&self) -> &'a [EntitySymbolEntry] {
        self.0
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct EntitySymbolEntry {
    ident: Ident,
    visibility: Scope,
    symbol: EntitySymbol,
}

impl EntitySymbolEntry {
    pub(crate) fn new_crate_root(db: &dyn EntitySynTreeDb, crate_path: CratePath) -> Self {
        let root_module_path = ModulePath::new_root(db, crate_path);
        Self {
            ident: db.coword_menu().crate_ident(),
            visibility: Scope::PubUnder(root_module_path),
            symbol: EntitySymbol::CrateRoot { root_module_path },
        }
    }

    pub(crate) fn new_package_dependency(
        db: &dyn EntitySynTreeDb,
        package_dependency: &PackageDependency,
    ) -> Self {
        let package_path = package_dependency.package_path();
        Self {
            ident: package_path.ident(db),
            visibility: Scope::Pub,
            symbol: EntitySymbol::PackageDependency {
                entity_path: package_path.lib_module(db).into(),
            },
        }
    }

    pub(crate) fn new_use_symbol_entry(
        db: &dyn EntitySynTreeDb,
        original_symbol: EntitySymbol,
        rule: &mut OnceUseRule,
    ) -> Self {
        rule.mark_as_resolved(original_symbol);
        let visibility = rule.visibility();
        Self {
            ident: rule.ident().unwrap(),
            visibility,
            symbol: UseSymbol::new(
                db,
                original_symbol,
                original_symbol.path(db),
                visibility,
                rule.ast_idx(),
                rule.use_expr_idx(),
            )
            .into(),
        }
    }

    pub(crate) fn new_use_ty_variant_entry(
        db: &dyn EntitySynTreeDb,
        parent_rule: &OnceUseRule,
        ident: Ident,
        ty_variant_path: TypeVariantPath,
    ) -> Self {
        let visibility = parent_rule.visibility();
        Self {
            ident,
            visibility,
            symbol: UseSymbol::new(
                db,
                EntitySymbol::TypeVariant { ty_variant_path },
                ty_variant_path.into(),
                visibility,
                parent_rule.ast_idx(),
                parent_rule.use_expr_idx(),
            )
            .into(),
        }
    }

    pub(crate) fn export_via_use_all(
        &self,
        db: &dyn EntitySynTreeDb,
        reference_module_path: ModulePath,
        rule: &UseAllModuleSymbolsRule,
    ) -> Option<Self> {
        self.is_visible_from(db, reference_module_path.into())
            .then_some(EntitySymbolEntry {
                ident: self.ident,
                visibility: rule.visibility(),
                symbol: UseSymbol::new(
                    db,
                    self.symbol,
                    self.symbol.path(db),
                    rule.visibility(),
                    rule.ast_idx(),
                    rule.use_expr_idx(),
                )
                .into(),
            })
    }

    pub(crate) fn is_visible_from(
        &self,
        db: &dyn EntitySynTreeDb,
        module_path: ReferenceModulePath,
    ) -> bool {
        self.visibility.is_visible_from(db, module_path)
    }

    pub fn symbol(&self) -> EntitySymbol {
        self.symbol
    }

    pub fn ident(&self) -> Ident {
        self.ident
    }

    pub fn visibility(&self) -> Scope {
        self.visibility
    }
}

// module items and submodules
#[derive(Debug, Default, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct MajorEntityNodeTable {
    entries: Vec<EntityNodeEntry>,
}

impl MajorEntityNodeTable {
    pub(crate) fn entity_symbol_table(&self, db: &dyn EntitySynTreeDb) -> EntitySymbolTable {
        EntitySymbolTable(
            self.entries
                .iter()
                .filter_map(|entry| EntitySymbolEntry::from_node(db, entry))
                .collect(),
        )
    }

    pub(crate) fn try_add_new_node(
        &mut self,
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        entity_path: EntityPath,
        block: DefnBlock,
    ) {
        if let Some(entry) = EntityNodeEntry::new(
            db,
            registry,
            visibility,
            ast_idx,
            ident_token,
            entity_path,
            block,
        ) {
            self.entries.push(entry)
        }
    }

    pub(crate) fn node(&self, syn_node_path: EntitySynNodePath) -> Option<EntitySynNode> {
        self.entries
            .iter()
            .find_map(|entry| (entry.syn_node_path == syn_node_path).then_some(entry.node))
    }

    pub(crate) fn node_paths<'a>(&'a self) -> impl Iterator<Item = EntitySynNodePath> + 'a {
        self.entries.iter().map(|entry| entry.syn_node_path)
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
#[salsa::derive_debug_with_db(db = EntitySynTreeDb)]
pub struct EntityNodeEntry {
    node: EntitySynNode,
    /// cached for performance, always equal to node.syn_node_path(db)
    syn_node_path: EntitySynNodePath,
    /// cached for performance, always equal to node.ident(db)
    ident: Ident,
    /// cached for performance, always equal to node.visibility(db)
    visibility: Scope,
}

impl EntitySymbolEntry {
    fn from_node(db: &dyn EntitySynTreeDb, node_entry: &EntityNodeEntry) -> Option<Self> {
        Some(EntitySymbolEntry {
            ident: node_entry.ident,
            visibility: node_entry.visibility,
            symbol: EntitySymbol::from_node(db, node_entry.node)?,
        })
    }
}

impl EntityNodeEntry {
    fn new(
        db: &dyn EntitySynTreeDb,
        registry: &mut EntityNodeRegistry,
        visibility: Scope,
        ast_idx: AstIdx,
        ident_token: IdentToken,
        entity_path: EntityPath,
        block: DefnBlock,
    ) -> Option<Self> {
        let node = EntitySynNode::try_new(
            db,
            registry,
            visibility,
            ast_idx,
            ident_token,
            entity_path,
            block,
        )?;
        Some(Self {
            syn_node_path: node.syn_node_path(db),
            ident: ident_token.ident(),
            visibility,
            node,
        })
    }

    pub fn node(&self) -> EntitySynNode {
        self.node
    }
}
