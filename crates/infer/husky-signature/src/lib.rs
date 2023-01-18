#![feature(trait_upcasting)]
mod db;
mod error;
mod parameter;
mod signature;
mod term;
#[cfg(test)]
mod tests;

pub use db::*;
pub use error::*;
pub use parameter::*;
pub use signature::*;
pub use term::*;

use husky_decl::*;
use husky_term::*;
use husky_word::*;
#[cfg(test)]
use tests::*;

#[salsa::jar(db = SignatureDb)]
pub struct SignatureJar(
    // type
    enum_ty_signature,
    EnumTypeSignature,
    unit_struct_ty_signature,
    UnitStructTypeSignature,
    tuple_struct_ty_signature,
    TupleStructTypeSignature,
    regular_struct_ty_signature,
    RegularStructTypeSignature,
    record_ty_signature,
    RecordTypeSignature,
    inductive_ty_signature,
    InductiveTypeSignature,
    structure_ty_signature,
    StructureTypeSignature,
    alien_ty_signature,
    AlienTypeSignature,
    union_ty_signature,
    UnionTypeSignature,
    // trait
    trai_signature,
    TraitSignature,
    // form
    // form_signature,
    value_signature,
    ValueSignature,
    feature_signature,
    FeatureSignature,
    function_signature,
    FunctionSignature,
    morphism_signature,
    MorphismSignature,
    type_alias_signature,
    TypeAliasSignature,
    // impl block
    // impl_block_signature,
    ty_impl_block_signature,
    TypeImplBlockSignature,
    ty_as_trai_impl_block_signature,
    TypeAsTraitImplBlockSignature,
    // variant
    UnitVariantSignature,
    PropsVariantSignature,
    TupleVariantSignature,
    // associated items
    // associated_item_signature,
    // type item
    TypeAssociatedFunctionSignature,
    TypeMethodSignature,
    TypeAssociatedTypeSignature,
    TypeAssociatedValueSignature,
    TypeMemoSignature,
    // trait item
    TraitAssociatedFunctionSignature,
    TraitMethodSignature,
    TraitAssociatedTypeSignature,
    TraitAssociatedValueSignature,
    // type as trait item
    TypeAsTraitAssociatedFunctionSignature,
    TypeAsTraitMethodSignature,
    TypeAsTraitAssociatedTypeSignature,
    TypeAsTraitAssociatedValueSignature,
);
