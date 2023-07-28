Ok(
    EntitySynTreeCrateBundle {
        sheets: [
            EntitySynTreeSheet {
                module_path: `math`,
                major_item_node_table: MajorEntityNodeTable {
                    entries: [
                        EntityNodeEntry {
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::algebra`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `algebra`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            ),
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `math::algebra`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `algebra`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::algebraic_geometry`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `algebraic_geometry`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            ),
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `math::algebraic_geometry`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `algebraic_geometry`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::algebraic_topology`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `algebraic_topology`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            ),
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `math::algebraic_topology`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `algebraic_topology`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::category_theory`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `category_theory`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            ),
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `math::category_theory`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `category_theory`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                        },
                        EntityNodeEntry {
                            node: EntitySynNode::Submodule(
                                SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::probability`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 4,
                                    ident_token: IdentToken {
                                        ident: `probability`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                },
                            ),
                            syn_node_path: EntitySynNodePath::Submodule(
                                SubmoduleSynNodePath {
                                    maybe_ambiguous_path: MaybeAmbiguousPath {
                                        path: SubmodulePath(
                                            `math::probability`,
                                        ),
                                        disambiguator: 0,
                                    },
                                },
                            ),
                            ident: `probability`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                        },
                    ],
                },
                item_symbol_table: EntitySymbolTable(
                    [
                        EntitySymbolEntry {
                            ident: `algebra`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `math::algebra`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::algebra`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `algebra`,
                                        token_idx: TokenIdx(
                                            1,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `algebraic_geometry`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `math::algebraic_geometry`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::algebraic_geometry`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 1,
                                    ident_token: IdentToken {
                                        ident: `algebraic_geometry`,
                                        token_idx: TokenIdx(
                                            3,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `algebraic_topology`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `math::algebraic_topology`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::algebraic_topology`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 2,
                                    ident_token: IdentToken {
                                        ident: `algebraic_topology`,
                                        token_idx: TokenIdx(
                                            5,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `category_theory`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `math::category_theory`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::category_theory`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 3,
                                    ident_token: IdentToken {
                                        ident: `category_theory`,
                                        token_idx: TokenIdx(
                                            7,
                                        ),
                                    },
                                },
                            },
                        },
                        EntitySymbolEntry {
                            ident: `probability`,
                            visibility: Scope::PubUnder(
                                `math`,
                            ),
                            symbol: EntitySymbol::Submodule {
                                submodule_path: SubmodulePath(
                                    `math::probability`,
                                ),
                                node: SubmoduleSynNode {
                                    syn_node_path: SubmoduleSynNodePath {
                                        maybe_ambiguous_path: MaybeAmbiguousPath {
                                            path: SubmodulePath(
                                                `math::probability`,
                                            ),
                                            disambiguator: 0,
                                        },
                                    },
                                    visibility: Scope::PubUnder(
                                        `math`,
                                    ),
                                    ast_idx: 4,
                                    ident_token: IdentToken {
                                        ident: `probability`,
                                        token_idx: TokenIdx(
                                            9,
                                        ),
                                    },
                                },
                            },
                        },
                    ],
                ),
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntitySynTreeSheet {
                module_path: `math::algebra`,
                major_item_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                item_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntitySynTreeSheet {
                module_path: `math::algebraic_geometry`,
                major_item_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                item_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntitySynTreeSheet {
                module_path: `math::algebraic_topology`,
                major_item_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                item_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntitySynTreeSheet {
                module_path: `math::category_theory`,
                major_item_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                item_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
            EntitySynTreeSheet {
                module_path: `math::probability`,
                major_item_node_table: MajorEntityNodeTable {
                    entries: [],
                },
                item_symbol_table: EntitySymbolTable(
                    [],
                ),
                impl_block_syn_node_table: [],
                once_use_rules: OnceUseRules(
                    [],
                ),
                use_all_rules: UseAllModuleSymbolsRules(
                    [],
                ),
                errors: [],
            },
        ],
        principal_item_path_expr_arena: Arena {
            data: [],
        },
    },
)