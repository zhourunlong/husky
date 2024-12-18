```rust
Ok(
    TokenInfoSheet {
        token_infos_list: [
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::UseExpr(
                        1,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 1,
                        rule_idx: OnceUseRuleIdx(
                            0,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::CrateRoot {
                                    root_module_path: ModulePath(`core`),
                                },
                            ),
                        },
                    },
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::UseExpr(
                        3,
                    ),
                    data: TokenInfoData::UseExpr {
                        use_expr_idx: 3,
                        rule_idx: OnceUseRuleIdx(
                            1,
                        ),
                        state: UseOneRuleState::Resolved {
                            original_symbol: Some(
                                EntitySymbol::MajorItem {
                                    major_item_path: MajorItemPath::Type(
                                        TypePath(`core::result::Result`, `Enum`),
                                    ),
                                },
                            ),
                        },
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::UseExpr(
                        2,
                    ),
                    data: TokenInfoData::UseExprStar,
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::MajorItem(
                            MajorItemSynNodePath::Type(
                                TypeSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::MajorItem(
                                            MajorItemSynNodePathData::Type(
                                                TypeSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TypePath(`core::result::Result`, `Enum`),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        EntityKind::MajorItem {
                            module_item_kind: MajorItemKind::Type(
                                TypeKind::Enum,
                            ),
                            connection: MajorItemConnectionKind::Connected,
                        },
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result::Ok`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `T`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result::Err`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `E`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            3,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        1,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        2,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::TemplateParameter(
                        3,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            11,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::Module(
                            ModulePath(`core`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core`),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::ops`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::ops`),
                        ),
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            15,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Trait(
                                TraitPath(`core::ops::Unveil`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            16,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        3,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            17,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 1,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        5,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            18,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 3,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E2`,
                                    regional_token_idx: RegionalTokenIdx(
                                        9,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            7,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            20,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        4,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            21,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `T1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        3,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            22,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`core::result::Result as core::ops::Unveil(0)`),
                        ),
                        SemExprIdx(
                            10,
                        ),
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 2,
                        current_variable_kind: CurrentVariableKind::TemplateParameter {
                            template_parameter_kind: CurrentTemplateParameterVariableKind::Type {
                                ident_token: IdentRegionalToken {
                                    ident: `E1`,
                                    regional_token_idx: RegionalTokenIdx(
                                        7,
                                    ),
                                },
                            },
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssocItem(
                            AssocItemSynNodePath::TraitForTypeItem(
                                TraitForTypeItemSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::AssocItem(
                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                TraitForTypeItemSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                            `<core::result::Result as core::ops::Unveil(0)>::Continue`,
                                                            TraitItemKind::AssocType,
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        EntityKind::AssocItem {
                            assoc_item_kind: AssocItemKind::TraitForTypeItem(
                                TraitItemKind::AssocType,
                            ),
                        },
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            4,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::Continue`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 3,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `E2`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: None,
                    source: TokenInfoSource::AstIdentifiable,
                    data: TokenInfoData::EntityNode(
                        ItemSynNodePath::AssocItem(
                            AssocItemSynNodePath::TraitForTypeItem(
                                TraitForTypeItemSynNodePath(
                                    ItemSynNodePathId {
                                        data: ItemSynNodePathData::AssocItem(
                                            AssocItemSynNodePathData::TraitForTypeItem(
                                                TraitForTypeItemSynNodePathData {
                                                    disambiguated_item_path: DisambiguatedItemPath {
                                                        maybe_ambiguous_item_path: TraitForTypeItemPath(
                                                            `<core::result::Result as core::ops::Unveil(0)>::unveil`,
                                                            TraitItemKind::AssocRitchie(
                                                                RitchieItemKind::Fn,
                                                            ),
                                                        ),
                                                        disambiguator: 0,
                                                    },
                                                },
                                            ),
                                        ),
                                    },
                                ),
                            ),
                        ),
                        EntityKind::AssocItem {
                            assoc_item_kind: AssocItemKind::TraitForTypeItem(
                                TraitItemKind::AssocRitchie(
                                    RitchieItemKind::Fn,
                                ),
                            ),
                        },
                    ),
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        0,
                        PrincipalEntityPath::Module(
                            ModulePath(`core::result`),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::Module(
                            Room32,
                            ModulePath(`core::result`),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            5,
                        ),
                    ),
                    source: TokenInfoSource::Pattern(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        0,
                    ),
                    data: TokenInfoData::CurrentVariable {
                        current_variable_idx: 0,
                        current_variable_kind: CurrentVariableKind::SimpleParenateParameter {
                            pattern_variable_idx: 0,
                        },
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            0,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            7,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        1,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            8,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            1,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 1,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `T2`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            9,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            3,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 3,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `E2`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
            [],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            5,
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            12,
                        ),
                    ),
                    source: TokenInfoSource::SynPrincipalEntityPathExpr(
                        2,
                        PrincipalEntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                    data: TokenInfoData::Entity(
                        EntityPath::MajorItem(
                            MajorItemPath::Type(
                                TypePath(`core::result::Result`, `Enum`),
                            ),
                        ),
                    ),
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            13,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            6,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 0,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `T1`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [
                TokenInfo {
                    regional_token_idx: Some(
                        RegionalTokenIdx(
                            14,
                        ),
                    ),
                    source: TokenInfoSource::SemExpr(
                        RegionPath::ItemDecl(
                            ItemPath(`<core::result::Result as core::ops::Unveil(0)>::unveil`),
                        ),
                        SemExprIdx(
                            8,
                        ),
                    ),
                    data: TokenInfoData::InheritedVariable {
                        inherited_variable_idx: 2,
                        inherited_variable_kind: InheritedVariableKind::Template(
                            InheritedTemplateVariable::Type {
                                ident: `E1`,
                            },
                        ),
                        syn_expr_region: ExprRegionLeash(_),
                    },
                },
            ],
            [],
        ],
    },
)
```