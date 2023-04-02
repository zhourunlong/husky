Ok(
    EntityTreeSheet {
        module_path: `mnist_classifier`,
        symbols: EntitySymbolTable(
            [
                EntitySymbolEntry {
                    ident: `connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::connected_component`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 11,
                            ident_token: IdentToken {
                                ident: `connected_component`,
                                token_idx: TokenIdx(
                                    1,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::raw_contour`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 12,
                            ident_token: IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    3,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `geom2d`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::geom2d`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 13,
                            ident_token: IdentToken {
                                ident: `geom2d`,
                                token_idx: TokenIdx(
                                    5,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::line_segment_sketch`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 14,
                            ident_token: IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    7,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::fermi`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 15,
                            ident_token: IdentToken {
                                ident: `fermi`,
                                token_idx: TokenIdx(
                                    9,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `digits`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::digits`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 16,
                            ident_token: IdentToken {
                                ident: `digits`,
                                token_idx: TokenIdx(
                                    11,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Submodule(
                        SubmoduleSymbol {
                            path: `mnist_classifier::major`,
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 17,
                            ident_token: IdentToken {
                                ident: `major`,
                                token_idx: TokenIdx(
                                    13,
                                ),
                            },
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `MnistLabel`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist::MnistLabel`, `Enum`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 0,
                                    ident_token: IdentToken {
                                        ident: `MnistLabel`,
                                        token_idx: TokenIdx(
                                            2,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist::MnistLabel`, `Enum`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 24,
                            use_expr_idx: 18,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `connected_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 19,
                                    ident_token: IdentToken {
                                        ident: `connected_components`,
                                        token_idx: TokenIdx(
                                            6,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::connected_components`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_connected_component`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 20,
                                    ident_token: IdentToken {
                                        ident: `major_connected_component`,
                                        token_idx: TokenIdx(
                                            20,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_connected_component`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ignored_connected_components_row_span_sum_sum`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 21,
                                    ident_token: IdentToken {
                                        ident: `ignored_connected_components_row_span_sum_sum`,
                                        token_idx: TokenIdx(
                                            72,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::ignored_connected_components_row_span_sum_sum`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contours`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 22,
                                    ident_token: IdentToken {
                                        ident: `major_raw_contours`,
                                        token_idx: TokenIdx(
                                            107,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_raw_contours`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_raw_contour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 23,
                                    ident_token: IdentToken {
                                        ident: `major_raw_contour`,
                                        token_idx: TokenIdx(
                                            120,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_raw_contour`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_line_segment_sketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 24,
                                    ident_token: IdentToken {
                                        ident: `major_line_segment_sketch`,
                                        token_idx: TokenIdx(
                                            134,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_line_segment_sketch`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `major_concave_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 25,
                                    ident_token: IdentToken {
                                        ident: `major_concave_components`,
                                        token_idx: TokenIdx(
                                            145,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::major::major_concave_components`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 18,
                            use_expr_idx: 0,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `is_one`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 63,
                                            ident_token: IdentToken {
                                                ident: `is_one`,
                                                token_idx: TokenIdx(
                                                    24,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                        ),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 10,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::digits::one::is_one`, `Val`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 19,
                            use_expr_idx: 3,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `FermiMatchResult`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 22,
                                    ident_token: IdentToken {
                                        ident: `FermiMatchResult`,
                                        token_idx: TokenIdx(
                                            6,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 20,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `fermi_match`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 24,
                                    ident_token: IdentToken {
                                        ident: `fermi_match`,
                                        token_idx: TokenIdx(
                                            150,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::fermi::fermi_match`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 20,
                            use_expr_idx: 6,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `RawContour`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 199,
                                    ident_token: IdentToken {
                                        ident: `RawContour`,
                                        token_idx: TokenIdx(
                                            24,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::raw_contour::RawContour`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_raw_contours`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 214,
                                    ident_token: IdentToken {
                                        ident: `find_raw_contours`,
                                        token_idx: TokenIdx(
                                            990,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::raw_contour::find_raw_contours`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 21,
                            use_expr_idx: 9,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentStroke`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 169,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentStroke`,
                                        token_idx: TokenIdx(
                                            41,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentStroke`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 22,
                            use_expr_idx: 12,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegmentSketch`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 171,
                                    ident_token: IdentToken {
                                        ident: `LineSegmentSketch`,
                                        token_idx: TokenIdx(
                                            161,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::LineSegmentSketch`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 22,
                            use_expr_idx: 12,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConcaveComponent`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 74,
                                            ident_token: IdentToken {
                                                ident: `ConcaveComponent`,
                                                token_idx: TokenIdx(
                                                    34,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                        ),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::concave_component::ConcaveComponent`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 22,
                            use_expr_idx: 12,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_concave_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Form(
                                                FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 76,
                                            ident_token: IdentToken {
                                                ident: `find_concave_components`,
                                                token_idx: TokenIdx(
                                                    522,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Form(
                                            FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                        ),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 163,
                                    use_expr_idx: 0,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::line_segment_sketch::concave_component::find_concave_components`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 22,
                            use_expr_idx: 12,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `LineSegment`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::Use(
                                UseSymbol {
                                    original_symbol: EntitySymbol::ModuleItem(
                                        ModuleItemSymbol {
                                            path: ModuleItemPath::Type(
                                                TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                            ),
                                            visibility: Visibility::Pub,
                                            ast_idx: 16,
                                            ident_token: IdentToken {
                                                ident: `LineSegment`,
                                                token_idx: TokenIdx(
                                                    8,
                                                ),
                                            },
                                        },
                                    ),
                                    path: EntityPath::ModuleItem(
                                        ModuleItemPath::Type(
                                            TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                        ),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 165,
                                    use_expr_idx: 4,
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::line_segment_sketch::line_segment::LineSegment`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 22,
                            use_expr_idx: 12,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponentDistribution`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 120,
                                    ident_token: IdentToken {
                                        ident: `ConnectedComponentDistribution`,
                                        token_idx: TokenIdx(
                                            12,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponentDistribution`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 23,
                            use_expr_idx: 15,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `EffHoles`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 121,
                                    ident_token: IdentToken {
                                        ident: `EffHoles`,
                                        token_idx: TokenIdx(
                                            33,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::EffHoles`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 23,
                            use_expr_idx: 15,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `ConnectedComponent`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Type(
                                        TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 123,
                                    ident_token: IdentToken {
                                        ident: `ConnectedComponent`,
                                        token_idx: TokenIdx(
                                            71,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Type(
                                    TypePath(`mnist_classifier::connected_component::ConnectedComponent`, `Struct`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 23,
                            use_expr_idx: 15,
                        },
                    ),
                },
                EntitySymbolEntry {
                    ident: `find_connected_components`,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    symbol: EntitySymbol::Use(
                        UseSymbol {
                            original_symbol: EntitySymbol::ModuleItem(
                                ModuleItemSymbol {
                                    path: ModuleItemPath::Form(
                                        FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                    ),
                                    visibility: Visibility::Pub,
                                    ast_idx: 126,
                                    ident_token: IdentToken {
                                        ident: `find_connected_components`,
                                        token_idx: TokenIdx(
                                            646,
                                        ),
                                    },
                                },
                            ),
                            path: EntityPath::ModuleItem(
                                ModuleItemPath::Form(
                                    FormPath(`mnist_classifier::connected_component::find_connected_components`, `Fn`),
                                ),
                            ),
                            visibility: Visibility::PubUnder(
                                `mnist_classifier`,
                            ),
                            ast_idx: 23,
                            use_expr_idx: 15,
                        },
                    ),
                },
            ],
        ),
        impl_blocks: [],
        use_expr_rules: UseExprRules(
            [
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 2,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    15,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            1..2,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 5,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    21,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            4..5,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 20,
                    use_expr_idx: 8,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    27,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            7..8,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 21,
                    use_expr_idx: 11,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    34,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            10..11,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 22,
                    use_expr_idx: 14,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    40,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            13..14,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 23,
                    use_expr_idx: 17,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::SelfValue(
                            SelfValueToken {
                                token_idx: TokenIdx(
                                    46,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            16..17,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::SelfModule {
                            module_path: `mnist_classifier`,
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 24,
                    use_expr_idx: 19,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `mnist`,
                                token_idx: TokenIdx(
                                    52,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            18..19,
                        ),
                    },
                    parent: None,
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::PackageDependency {
                            entity_path: EntityPath::Module(
                                `mnist`,
                            ),
                        },
                    },
                },
                UseExprRule {
                    ast_idx: 18,
                    use_expr_idx: 1,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `major`,
                                token_idx: TokenIdx(
                                    17,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            0..1,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::major`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 17,
                                ident_token: IdentToken {
                                    ident: `major`,
                                    token_idx: TokenIdx(
                                        13,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 19,
                    use_expr_idx: 4,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `digits`,
                                token_idx: TokenIdx(
                                    23,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            3..4,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::digits`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 16,
                                ident_token: IdentToken {
                                    ident: `digits`,
                                    token_idx: TokenIdx(
                                        11,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 20,
                    use_expr_idx: 7,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `fermi`,
                                token_idx: TokenIdx(
                                    29,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            6..7,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::fermi`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 15,
                                ident_token: IdentToken {
                                    ident: `fermi`,
                                    token_idx: TokenIdx(
                                        9,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 21,
                    use_expr_idx: 10,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `raw_contour`,
                                token_idx: TokenIdx(
                                    36,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            9..10,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::raw_contour`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 12,
                                ident_token: IdentToken {
                                    ident: `raw_contour`,
                                    token_idx: TokenIdx(
                                        3,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 22,
                    use_expr_idx: 13,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `line_segment_sketch`,
                                token_idx: TokenIdx(
                                    42,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            12..13,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::line_segment_sketch`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 14,
                                ident_token: IdentToken {
                                    ident: `line_segment_sketch`,
                                    token_idx: TokenIdx(
                                        7,
                                    ),
                                },
                            },
                        ),
                    },
                },
                UseExprRule {
                    ast_idx: 23,
                    use_expr_idx: 16,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    variant: UseExprRuleVariant::Parent {
                        parent_name_token: NameToken::Ident(
                            IdentToken {
                                ident: `connected_component`,
                                token_idx: TokenIdx(
                                    48,
                                ),
                            },
                        ),
                        children: ArenaIdxRange(
                            15..16,
                        ),
                    },
                    parent: Some(
                        EntityPath::Module(
                            `mnist_classifier`,
                        ),
                    ),
                    state: UseExprRuleState::Resolved {
                        original_symbol: EntitySymbol::Submodule(
                            SubmoduleSymbol {
                                path: `mnist_classifier::connected_component`,
                                visibility: Visibility::PubUnder(
                                    `mnist_classifier`,
                                ),
                                ast_idx: 11,
                                ident_token: IdentToken {
                                    ident: `connected_component`,
                                    token_idx: TokenIdx(
                                        1,
                                    ),
                                },
                            },
                        ),
                    },
                },
            ],
        ),
        use_all_rules: UseAllRules(
            [
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Outside,
                        path: `mnist`,
                    },
                    ast_idx: 24,
                    use_expr_idx: 18,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    progress: 1,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::major`,
                    },
                    ast_idx: 18,
                    use_expr_idx: 0,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    progress: 29,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::digits`,
                    },
                    ast_idx: 19,
                    use_expr_idx: 3,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    progress: 47,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::fermi`,
                    },
                    ast_idx: 20,
                    use_expr_idx: 6,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    progress: 29,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::raw_contour`,
                    },
                    ast_idx: 21,
                    use_expr_idx: 9,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    progress: 44,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::line_segment_sketch`,
                    },
                    ast_idx: 22,
                    use_expr_idx: 12,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    progress: 21,
                },
                UseAllRule {
                    parent: KinshipedModulePath {
                        kinship: Inside,
                        path: `mnist_classifier::connected_component`,
                    },
                    ast_idx: 23,
                    use_expr_idx: 15,
                    visibility: Visibility::PubUnder(
                        `mnist_classifier`,
                    ),
                    progress: 31,
                },
            ],
        ),
        errors: [],
    },
)