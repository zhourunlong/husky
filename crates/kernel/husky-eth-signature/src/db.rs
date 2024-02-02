use crate::*;

#[salsa::jar]
pub struct EtherealSignatureJar(
    // associated_items
    // - type items
    ty_item_ethereal_signature_templates_map,
    TypeMethodFnEthTemplate,
    TypeMethodFunctionEthTemplate,
    TypeMemoizedFieldEthTemplate,
    TypeAssociatedFnEthTemplate,
    TypeAssociatedTypeEthTemplate,
    // ty_item_ethereal_signature_template,
    // - trait items
    TraitAssociatedFnEthTemplate,
    TraitMethodFnEthTemplate,
    // - trait for type items
    TraitForTypeAssociatedFnEthTemplate,
    TraitForTypeAssociatedValEthTemplate,
    TraitForTypeAssociatedTypeEthTemplate,
    TraitForTypeAssociatedTypeEtherealSignatureBuilder,
    // trai_for_ty_item_ethereal_signature_template,
    trai_for_ty_associated_ty_ethereal_signature_signature_builder_try_into_signature,
    TraitForTypeMethodFnEthTemplate,
    TraitForTypeMethodFnEtherealSignatureBuilder,
    trai_for_ty_method_fn_ethereal_signature_signature_builder_try_into_signature,
    // trai
    TraitEthTemplate,
    trai_ethereal_signature_template,
    // fugitive
    // fugitive_ethereal_signature_template,
    FunctionFnEthTemplate,
    GnFugitiveEthTemplate,
    TypeAliasEthTemplate,
    ValFugitiveEthTemplate,
    // ty
    EnumTypeEthTemplate,
    ExternTypeEthTemplate,
    InductiveTypeEthTemplate,
    PropsStructTypeEthTemplate,
    StructureTypeEthTemplate,
    TupleStructTypeEthTemplate,
    UnionTypeEthTemplate,
    UnitStructTypeEthTemplate,
    ty_ethereal_signature_template,
    // ty variant
    EnumTupleVariantEthTemplate,
    EnumPropsVariantEthTemplate,
    EnumUnitTypeVariantEthTemplate,
    ty_variant_ethereal_signature_template,
    // impl block
    // - type
    TypeImplBlockEthTemplate,
    // - trait for type
    TraitForTypeImplBlockEthTemplate,
    TraitForTypeImplBlockEtherealSignatureBuilder,
    trai_for_ty_impl_block_ethereal_signature_template,
    ty_impl_block_ethereal_signature_template,
    crate::helpers::trai_for_ty::ty_side_impl_block_signature_templates_map,
    crate::helpers::trai_for_ty::trai_side_path_leading_trai_for_ty_impl_block_ethereal_signature_templates_map,
    crate::helpers::trai_for_ty::trai_side_derive_any_ethereal_signature_templates,
    trai_for_ty_impl_block_with_ty_instantiated_associated_output_ethereal_signature_builder,
    trai_for_ty_impl_block_with_ty_instantiated_item_ethereal_signature_template,
    // attr
    signature::ty_path_derive_attr_ethereal_signature_templates_map,
    // attr_ethereal_signature_template,
    DeriveAttrEthTemplate,
    DeriveAttrShardEthTemplate
);