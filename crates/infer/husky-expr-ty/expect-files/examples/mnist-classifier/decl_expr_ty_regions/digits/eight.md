[
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::eight::upper_mouth_match`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::eight::is_eight`, `Feature`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Err(
                    Derived(
                        ExprError,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
            TypeInfo {
                ty_result: Err(
                    Derived(
                        PrefixOperandTypeNotInferred,
                    ),
                ),
                opt_expectation: None,
                resolved_ty: None,
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
    ExprTypeRegion {
        path: RegionPath::Decl(
            DeclExprPath::Entity(
                FormPath(`mnist_classifier::digits::eight::big_mouth`, `Function`),
            ),
        ),
        expr_ty_infos: [
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
            TypeInfo {
                ty_result: Ok(
                    Resolved(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
                opt_expectation: None,
                resolved_ty: Some(
                    Ok(
                        Category(
                            TermCategory {
                                universe: TermUniverse(
                                    1,
                                ),
                            },
                        ),
                    ),
                ),
            },
        ],
        unresolved_term_table: UnresolvedTermTable {
            unresolved_terms: [],
            first_unresolved_term: 0,
            expectation_rules: Arena {
                data: [],
            },
            first_unresolved_expectation: 0,
        },
    },
]