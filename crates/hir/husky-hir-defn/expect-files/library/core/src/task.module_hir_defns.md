```rust
[
    HirDefn::MajorItem(
        MajorItemHirDefn::Trait(
            TraitHirDefn {
                path: TraitPath(`core::task::IsTask`),
                hir_decl: TraitHirDecl {
                    path: TraitPath(`core::task::IsTask`),
                    template_parameters: HirTemplateParameters(
                        [],
                    ),
                    hir_eager_expr_region: HirEagerExprRegion {
                        region_path: RegionPath::ItemDecl(
                            ItemPath(`core::task::IsTask`),
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
            },
        ),
    ),
    HirDefn::MajorItem(
        MajorItemHirDefn::Form(
            MajorFormHirDefn::StaticVar(
                MajorStaticVarHirDefn {
                    path: MajorFormPath(`core::task::TASK`, `StaticVar`),
                    hir_decl: MajorStaticVarHirDecl {
                        path: MajorFormPath(`core::task::TASK`, `StaticVar`),
                        return_ty: HirType::TypeVar(
                            MajorFormPath(`core::task::Task`, `TypeVar`),
                        ),
                        hir_eager_expr_region: HirEagerExprRegion {
                            region_path: RegionPath::ItemDecl(
                                ItemPath(`core::task::TASK`),
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
                    hir_expr_body_and_region: None,
                },
            ),
        ),
    ),
]
```