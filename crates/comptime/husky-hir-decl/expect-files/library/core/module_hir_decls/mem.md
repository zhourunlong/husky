[
    HirDecl::ModuleItem(
        ModuleItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 9,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ModuleItem(
        ModuleItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 10,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ModuleItem(
        ModuleItemHirDecl::Type(
            Extern(
                ExternTypeHirDecl(
                    Id {
                        value: 11,
                    },
                ),
            ),
        ),
    ),
    HirDecl::ImplBlock(
        ImplBlockHirDecl::TraitForType(
            TraitForTypeImplBlockHirDecl {
                path: TraitForTypeImplBlockPath {
                    module_path: `core::mem`,
                    trai_path: TraitPath(`core::marker::Copy`),
                    ty_sketch: Path(
                        TypePath(
                            Id {
                                value: 13,
                            },
                        ),
                    ),
                    disambiguator: 0,
                },
                template_parameters: HirTemplateParameters {
                    data: [
                        HirTemplateParameter {
                            symbol: Type(
                                Type {
                                    attrs: HirSymbolAttrs,
                                    variance: None,
                                    disambiguator: 0,
                                },
                            ),
                            traits: [],
                        },
                    ],
                },
                trai: HirTrait {
                    trai_path: TraitPath(`core::marker::Copy`),
                    template_arguments: [],
                },
                self_ty: PathLeading(
                    HirTypePathLeading(
                        Id {
                            value: 8,
                        },
                    ),
                ),
            },
        ),
    ),
]