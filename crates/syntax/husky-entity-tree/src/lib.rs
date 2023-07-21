#![feature(let_chains)]
#![feature(stmt_expr_attributes)]
#![feature(trait_upcasting)]
mod bundle;
mod collector;
mod context;
mod db;
mod error;
mod expr;
mod helpers;
mod node;
mod prelude;
mod presheet;
mod region_path;
mod sheet;
mod subentity;
mod submodule;
mod symbol;
mod table;
#[cfg(test)]
mod tests;

pub use self::bundle::*;
pub use self::db::*;
pub use self::error::*;
pub use self::expr::*;
pub use self::helpers::*;
pub use self::node::*;
pub use self::prelude::*;
pub use self::presheet::*;
pub use self::region_path::*;
pub use self::sheet::*;
pub use self::subentity::*;
pub use self::symbol::*;
pub use self::table::*;

use self::collector::*;
use self::context::*;
use self::submodule::*;
#[cfg(test)]
use self::tests::*;
use husky_ast::*;
use husky_coword::{CowordDb, Ident};
use husky_entity_path::*;
use husky_entity_taxonomy::EntityKind;
use husky_scope::*;
use husky_scope_expr::*;
use husky_vfs::*;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};
use salsa::DebugWithDb;
use vec_like::AsVecMapEntry;

#[salsa::jar(db = EntitySynTreeDb)]
pub struct EntitySynTreeJar(
    SubmoduleSynNodePath,
    SubmoduleSynNode,
    submodule_node,
    ModuleItemSynNode,
    UseSymbol,
    // module items
    TraitSynNodePath,
    trai_node,
    trai_item_paths,
    TypeSynNodePath,
    ty_node,
    FugitiveSynNodePath,
    fugitive_node,
    // ty variant
    TypeVariantSynNodePath,
    // associated items
    TypeItemSynNodePath,
    TypeItemNode,
    ty_item_node,
    trai_item_nodes,
    TraitItemSynNodePath,
    TraitItemNode,
    TraitForTypeItemSynNodePath,
    TraitForTypeItemNode,
    trai_for_ty_item_node,
    IllFormedItemSynNodePath,
    IllFormedItemNode,
    TypeVariantSynNode,
    ty_variant_syn_node,
    // ty_impl_blocks,
    ty_item_node_paths,
    ty_item_paths_map,
    trai_for_ty_impl_block_items,
    trai_for_ty_impl_block_item_paths,
    // variants
    ty_variant_syn_nodes,
    ty_variant_paths,
    // impl blocks
    // - type impl block
    TypeImplBlockSynNode,
    ty_impl_block_node,
    ty_impl_block_items,
    // - trait for type impl block
    TraitForTypeImplBlockSynNode,
    trai_for_ty_impl_block_node,
    // - ill formed impl block
    IllFormedImplBlockSynNode,
    ill_formed_impl_block_node,
    // other
    entity_tree_presheet,
    entity_tree_crate_bundle,
    module_entity_paths,
    module_entity_node_paths,
    submodules,
    module_subentity_path,
    all_modules_within_crate,
    // prelude
    crate_specific_prelude,
    none_core_crate_universal_prelude,
    // helpers
    ty_side_trai_for_ty_impl_block_paths_map,
    trai_item_table,
    TraitOrderedSet,
    crate::helpers::non_core_crate_prelude_trait_items_table,
    crate::helpers::module_specific_trait_items_table,
    crate::helpers::trai_side_derive_any_trai_for_ty_impl_block_paths_map,
    crate::helpers::trai_side_path_leading_trai_for_ty_impl_block_paths_map,
);
