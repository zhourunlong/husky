Ok(
    EntityTreePresheet {
        module_path: `core::array`,
        native_symbol_entries: NativeEntitySymbolTable(
            [
                NativeEntitySymbolEntry {
                    ident: `Array`,
                    visibility: Scope::Pub,
                    symbol: NativeEntitySymbol::ModuleItem(
                        ModuleItemSymbol {
                            path: ModuleItemPath::Type(
                                TypePath(`core::array::Array`, `Extern`),
                            ),
                            visibility: Scope::Pub,
                            ast_idx: 0,
                            ident_token: IdentToken {
                                ident: `Array`,
                                token_idx: TokenIdx(
                                    2,
                                ),
                            },
                        },
                    ),
                },
            ],
        ),
        use_one_trackers: UseExprRules(
            [],
        ),
        use_all_trackers: UseAllRules(
            [],
        ),
        use_expr_arena: Arena {
            data: [],
        },
        errors: [],
    },
)