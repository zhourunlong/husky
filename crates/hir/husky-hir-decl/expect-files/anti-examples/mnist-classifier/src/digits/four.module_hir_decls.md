```rust
[
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Val(
                MajorValHirDecl {
                    path: MajorFormPath(`mnist_classifier::digits::four::left_components`, `Val`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::four::left_components`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorFunctionRitchieHirDecl {
                    path: MajorFormPath(`mnist_classifier::digits::four::left_coordinate_max`, `Ritchie(
                        Fn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Fn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Eager(
                        HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::f32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: true,
                        },
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 26,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Val(
                MajorValHirDecl {
                    path: MajorFormPath(`mnist_classifier::digits::four::components_max_downwards`, `Val`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::four::components_max_downwards`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Val(
                MajorValHirDecl {
                    path: MajorFormPath(`mnist_classifier::digits::four::components_max_heights`, `Val`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_arguments: [],
                            always_copyable: false,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::four::components_max_heights`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Val(
                MajorValHirDecl {
                    path: MajorFormPath(`mnist_classifier::digits::four::is_four`, `Val`),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`malamute::OneVsAll`, `Enum`),
                            template_arguments: [],
                            always_copyable: true,
                        },
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`mnist_classifier::digits::four::is_four`),
                        ),
                        self_value_ty: None,
                        expr_arena: Arena {
                            data: [],
                        },
                        stmt_arena: Arena {
                            data: [],
                        },
                        pattern_arena: Arena {
                            data: [],
                        },
                        comptime_variable_region_data: HirEagerComptimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                        },
                        runtime_variable_region_data: HirEagerRuntimeVariableRegionData {
                            arena: Arena {
                                data: [],
                            },
                            self_value_variable: None,
                        },
                    },
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorFunctionRitchieHirDecl {
                    path: MajorFormPath(`mnist_classifier::digits::four::displacement_downwards`, `Ritchie(
                        Fn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Fn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Eager(
                        HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::f32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: true,
                        },
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 30,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
    HirDecl::MajorItem(
        MajorItemHirDecl::Form(
            MajorFormHirDecl::Ritchie(
                MajorFunctionRitchieHirDecl {
                    path: MajorFormPath(`mnist_classifier::digits::four::cc_box_heights`, `Ritchie(
                        Fn,
                    )`),
                    ritchie_item_kind: RitchieItemKind::Fn,
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    parenate_parameters: HirParenateParameters::Eager(
                        HirEagerParenateParameters(
                            [
                                HirEagerParenateParameter::Simple {
                                    pattern_idx: 0,
                                    contract: Pure,
                                    ty: HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::mem::Leash`, `Extern`),
                                            template_arguments: [
                                                HirTemplateArgument::Type(
                                                    HirType::PathLeading(
                                                        HirTypePathLeading {
                                                            ty_path: TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                                            template_arguments: [],
                                                            always_copyable: false,
                                                        },
                                                    ),
                                                ),
                                            ],
                                            always_copyable: true,
                                        },
                                    ),
                                },
                            ],
                        ),
                    ),
                    return_ty: HirType::PathLeading(
                        HirTypePathLeading {
                            ty_path: TypePath(`core::option::Option`, `Enum`),
                            template_arguments: [
                                HirTemplateArgument::Type(
                                    HirType::PathLeading(
                                        HirTypePathLeading {
                                            ty_path: TypePath(`core::num::f32`, `Extern`),
                                            template_arguments: [],
                                            always_copyable: true,
                                        },
                                    ),
                                ),
                            ],
                            always_copyable: true,
                        },
                    ),
                    hir_expr_region: Eager(
                        HirEagerExprRegion(
                            Id {
                                value: 31,
                            },
                        ),
                    ),
                },
            ),
        ),
    ),
]
```