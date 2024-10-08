```rust
[
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Type(
                    TypeEthTemplate::PropsStruct(
                        PropsStructEthTemplate {
                            path: TypePath(`mnist_classifier::fermi::FermiMatchResult`, `Struct`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            fields: [
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`FermiMatchResult`),
                                    ident: `matches`,
                                    ty: EthTerm(`Vec Option Leash ConcaveComponent`),
                                },
                                PropsFieldEthTemplate {
                                    self_ty: EthTerm(`FermiMatchResult`),
                                    ident: `others`,
                                    ty: EthTerm(`Vec Leash ConcaveComponent`),
                                },
                            ],
                            instance_constructor_ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`Vec Option Leash ConcaveComponent`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Move,
                                            ty: EthTerm(`Vec Leash ConcaveComponent`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`FermiMatchResult`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::fermi_match`),
        Ok(
            ItemEthTemplate::MajorItem(
                MajorItemEthTemplate::Form(
                    FormEthTemplate::Ritchie(
                        MajorFunctionRitchieEthTemplate {
                            path: MajorFormPath(`mnist_classifier::fermi::fermi_match`, `Ritchie(
                                Fn,
                            )`),
                            template_parameters: EthTemplateParameters {
                                data: [],
                            },
                            ritchie_ty: EthRitchie {
                                ritchie_kind: RitchieKind::Type(
                                    RitchieTypeKind::Item(
                                        RitchieItemKind::Fn,
                                    ),
                                ),
                                parameter_contracted_tys: [
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`Leash Vec ConcaveComponent`),
                                        },
                                    ),
                                    EtherealRitchieParameter::Simple(
                                        EthRitchieSimpleParameter {
                                            contract: Contract::Pure,
                                            ty: EthTerm(`Vec fn(( Leash ConcaveComponent) -> Option f32`),
                                        },
                                    ),
                                ],
                                return_ty: EthTerm(`FermiMatchResult`),
                            },
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)`),
        Ok(
            ItemEthTemplate::ImplBlock(
                ImplBlockEthTemplate::TypeImpl(
                    TypeImplBlockEthTemplate {
                        path: TypeImplBlockPath(`mnist_classifier::fermi::FermiMatchResult(0)`),
                        template_parameters: EthTemplateParameters {
                            data: [],
                        },
                        self_ty: EthTerm(`FermiMatchResult`),
                    },
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::fermi::FermiMatchResult(0)::norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::fermi::FermiMatchResult(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`FermiMatchResult`),
                            },
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::fermi::FermiMatchResult(0)::rel_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::fermi::FermiMatchResult(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`FermiMatchResult`),
                            },
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
    (
        ItemPath(`mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`),
        Ok(
            ItemEthTemplate::AssocItem(
                AssocItemEthTemplate::Type(
                    TypeItemEthTemplate::MemoizedField(
                        TypeMemoizedFieldEthTemplate {
                            path: TypeItemPath(
                                `mnist_classifier::fermi::FermiMatchResult(0)::angle_change_norm`,
                                TypeItemKind::MemoizedField,
                            ),
                            impl_block: TypeImplBlockEthTemplate {
                                path: TypeImplBlockPath(`mnist_classifier::fermi::FermiMatchResult(0)`),
                                template_parameters: EthTemplateParameters {
                                    data: [],
                                },
                                self_ty: EthTerm(`FermiMatchResult`),
                            },
                            return_ty: EthTerm(`f32`),
                            expr_ty: EthTerm(`f32`),
                        },
                    ),
                ),
            ),
        ),
    ),
]
```