mod associated_item;
mod decr;
mod impl_block;
mod module_item;
mod submodule;
mod ty_variant;

pub use self::associated_item::*;
pub use self::decr::*;
pub use self::impl_block::*;
pub use self::module_item::*;
pub use self::submodule::*;
pub use self::ty_variant::*;

use crate::*;
use husky_decl_ast::*;
use husky_regional_token::*;
use husky_token_data::{TokenData, TokenDataResult};
use parsec::{parse_separated_list, HasStreamState};

type SmallVecImpl<T> = smallvec::SmallVec<[T; 2]>;

/// A `NodeDecl` is a tolerant information-preserving declaration
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum ItemSynNodeDecl {
    Submodule(SubmoduleSynNodeDecl),
    MajorItem(MajorItemSynNodeDecl),
    ImplBlock(ImplBlockSynNodeDecl),
    AssociatedItem(AssociatedItemSynNodeDecl),
    TypeVariant(TypeVariantSynNodeDecl),
    Decr(DecrSynNodeDecl),
}

impl ItemSynNodeDecl {
    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> Option<SynExprRegion> {
        match self {
            ItemSynNodeDecl::Submodule(_) => None,
            ItemSynNodeDecl::MajorItem(syn_node_decl) => syn_node_decl.syn_expr_region(db).into(),
            ItemSynNodeDecl::ImplBlock(syn_node_decl) => Some(syn_node_decl.syn_expr_region(db)),
            ItemSynNodeDecl::AssociatedItem(syn_node_decl) => {
                syn_node_decl.syn_expr_region(db).into()
            }
            ItemSynNodeDecl::TypeVariant(_node_decl) => todo!(),
            ItemSynNodeDecl::Decr(syn_node_decl) => Some(syn_node_decl.syn_expr_region(db)),
        }
    }

    pub fn syn_node_path(self, db: &dyn SynDeclDb) -> ItemSynNodePath {
        match self {
            ItemSynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::MajorItem(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::AssociatedItem(syn_node_decl) => {
                syn_node_decl.syn_node_path(db).into()
            }
            ItemSynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.syn_node_path(db).into(),
            ItemSynNodeDecl::Decr(_) => todo!(),
        }
    }

    pub fn node_decl_errors(self, db: &dyn SynDeclDb) -> SynNodeDeclErrorRefs {
        match self {
            ItemSynNodeDecl::Submodule(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::MajorItem(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::ImplBlock(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::AssociatedItem(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::TypeVariant(syn_node_decl) => syn_node_decl.errors(db),
            ItemSynNodeDecl::Decr(_) => todo!(),
        }
    }
}

/// A `Decl` is a strict version, handy for subsequent processing
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = SynDeclDb)]
#[enum_class::from_variants]
pub enum SynDecl {
    Submodule(SubmoduleSynDecl),
    MajorItem(MajorItemSynDecl),
    ImplBlock(ImplBlockSynDecl),
    AssociatedItem(AssociatedItemSynDecl),
    TypeVariant(TypeVariantSynDecl),
    Decr(DecrSynDecl),
}

impl SynDecl {
    pub fn template_parameters<'a>(self, db: &'a dyn SynDeclDb) -> &'a [TemplateParameterObelisk] {
        match self {
            SynDecl::Submodule(_) => todo!(),
            SynDecl::MajorItem(decl) => decl.template_parameters(db),
            SynDecl::ImplBlock(decl) => decl.template_parameters(db),
            SynDecl::AssociatedItem(decl) => decl.template_parameters(db),
            SynDecl::TypeVariant(_decl) => &[],
            SynDecl::Decr(_) => todo!(),
        }
    }

    pub fn syn_expr_region(self, db: &dyn SynDeclDb) -> Option<SynExprRegion> {
        match self {
            SynDecl::Submodule(_) => None,
            SynDecl::MajorItem(decl) => decl.syn_expr_region(db).into(),
            SynDecl::ImplBlock(decl) => decl.syn_expr_region(db).into(),
            SynDecl::AssociatedItem(decl) => decl.syn_expr_region(db).into(),
            SynDecl::TypeVariant(_decl) => todo!(),
            SynDecl::Decr(_) => todo!(),
        }
    }

    pub fn path(self, db: &dyn SynDeclDb) -> ItemPath {
        match self {
            SynDecl::Submodule(_) => todo!(),
            SynDecl::MajorItem(decl) => decl.path(db).into(),
            SynDecl::ImplBlock(decl) => decl.path(db).into(),
            SynDecl::AssociatedItem(decl) => decl.path(db).into(),
            SynDecl::TypeVariant(decl) => decl.path(db).into(),
            SynDecl::Decr(_) => todo!(),
        }
    }
}

pub trait HasSynNodeDecl: Copy {
    type NodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl;
}

impl HasSynNodeDecl for ItemSynNodePath {
    type NodeDecl = ItemSynNodeDecl;

    fn syn_node_decl<'a>(self, db: &'a dyn SynDeclDb) -> Self::NodeDecl {
        match self {
            ItemSynNodePath::MajorItem(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::TypeVariant(_) => todo!(),
            ItemSynNodePath::ImplBlock(_) => todo!(),
            ItemSynNodePath::AssociatedItem(_) => todo!(),
            ItemSynNodePath::Submodule(syn_node_path) => syn_node_path.syn_node_decl(db).into(),
            ItemSynNodePath::Decr(_) => todo!(),
        }
    }
}

pub trait HasSynDecl: Copy {
    type Decl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl>;
}

impl HasSynDecl for ItemPath {
    type Decl = SynDecl;

    fn syn_decl(self, db: &dyn SynDeclDb) -> DeclResult<Self::Decl> {
        match self {
            ItemPath::Submodule(path) => path.syn_decl(db).map(Into::into),
            ItemPath::MajorItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::AssociatedItem(path) => path.syn_decl(db).map(Into::into),
            ItemPath::TypeVariant(path) => path.syn_decl(db).map(Into::into),
            ItemPath::ImplBlock(path) => path.syn_decl(db).map(Into::into),
            ItemPath::Decr(_) => todo!(),
        }
    }
}
