[
    ItemPath::MajorItem(
        MajorItemPath::Trait(
            TraitPath(`core::clone::Clone`),
        ),
    ),
    ItemPath::ImplBlock(
        ImplBlockPath::TraitForTypeImplBlock(
            TraitForTypeImplBlock {
                data: TraitForTypeImplBlockPathData {
                    module_path: `core::clone`,
                    trai_path: TraitPath(`core::clone::Clone`),
                    ty_sketch: TypeSketch::DeriveAny,
                    disambiguator: 0,
                },
            },
        ),
    ),
    ItemPath::AssociatedItem(
        AssociatedItemPath::TraitForTypeItem(
            TraitForTypeItemPath(
                ItemPathId {
                    data: ItemPathData::AssociatedItem(
                        AssociatedItemPathData::TraitForTypeItem(
                            TraitForTypeItemPathData {
                                impl_block: TraitForTypeImplBlock {
                                    data: TraitForTypeImplBlockPathData {
                                        module_path: `core::clone`,
                                        trai_path: TraitPath(`core::clone::Clone`),
                                        ty_sketch: TypeSketch::DeriveAny,
                                        disambiguator: 0,
                                    },
                                },
                                ident: `clone`,
                                item_kind: MethodFn,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]