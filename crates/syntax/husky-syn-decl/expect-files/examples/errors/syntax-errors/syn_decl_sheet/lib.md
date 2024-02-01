SynDeclSheet {
    decls: [
        (
            ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `syntax_errors::ast`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `syntax_errors::ast`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        ),
        (
            ItemPath::Submodule(
                Room32,
                SubmoduleItemPath(
                    ItemPathId {
                        data: ItemPathData::SubmoduleItem(
                            SubmoduleItemPathData {
                                submodule_path: SubmodulePath(
                                    `syntax_errors::uses`,
                                ),
                            },
                        ),
                    },
                ),
            ),
            SynDecl::Submodule(
                SubmoduleSynDecl {
                    path: SubmoduleItemPath(
                        ItemPathId {
                            data: ItemPathData::SubmoduleItem(
                                SubmoduleItemPathData {
                                    submodule_path: SubmodulePath(
                                        `syntax_errors::uses`,
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        ),
    ],
}