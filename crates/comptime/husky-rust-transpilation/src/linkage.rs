use crate::*;
use either::*;
use husky_corgi_config::transpilation_setup::TranspilationSetup;
use husky_entity_path::{
    AssocItemPath, MajorFormPath, PreludeTypePath, TraitForTypeItemPath, TraitItemPath,
    TypeItemPath, TypeVariantPath,
};
use husky_eth_signature::signature::HasEthTemplate;
use husky_hir_decl::decl::{HasHirDecl, TypeHirDecl, TypeVariantHirDecl};
use husky_hir_ty::{ritchie::HirContract, trai::HirTrait, HirType};
use husky_javelin::template_argument::constant::JavelinConstant;
use husky_linkage::{
    instantiation::{LinInstantiation, LinTermSymbolResolution, LinkageInstantiate},
    linkage::LinkageField,
    template_argument::{
        constant::LinConstant,
        qual,
        ty::{LinType, LinkageRitchieParameter, LinkageRitchieType},
        LinTemplateArgument,
    },
    trai::LinkageTrait,
};
use husky_linkage::{
    linkage::{package_linkages, Linkage, LinkageData},
    template_argument::ty::LinTypePathLeading,
};

use self::helpers::TupleFieldVariable;

#[salsa::tracked(jar = RustTranspilationJar, return_ref)]
pub(crate) fn package_linkages_transpilation(
    db: &::salsa::Db,
    package_path: PackagePath,
    setup: TranspilationSetup,
) -> String {
    let mut builder_base = RustTranspilationBuilderBase::new(
        db,
        package_path.toolchain(db),
        setup,
        Some(format!(
            r#"#![feature(trait_upcasting)]
use husky_core::*;
use {}::{{*, ugly::*}};
use {}::*;
"#,
            setup.rust_data(db).unwrap().task_dependency_ident.data(db),
            package_path.ident(db).data(db)
        )),
        None,
    );
    let mut builder = RustTranspilationBuilder::new(&mut builder_base);
    builder.on_fresh_semicolon_paragraph(|builder| {
        builder.rustfmt_skip();
        builder.macro_name(RustMacroName::LinkageImpls);
        builder
            .delimited_multiline_comma_list(RustDelimiter::Box, package_linkages(db, package_path))
    });
    builder_base.finish()
}

impl TranspileToRustWith<()> for Linkage {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<()>) {
        let db = builder.db;
        match *self.data(db) {
            LinkageData::MajorRitchieEager {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::MajorRitchieLazy {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::GnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::MajorVal {
                path,
                instantiation: _,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                path.transpile_to_rust(builder)
            }),
            LinkageData::MethodRitchie {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::EnumVariantConstructor {
                self_ty,
                path,
                ref instantiation,
            } => builder.macro_call(
                RustMacroName::EnumVariantConstructorLinkageImpl,
                |builder| {
                    self_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    path.transpile_to_rust(builder);
                    match path.hir_decl(db).unwrap() {
                        TypeVariantHirDecl::Props(hir_decl) => {
                            builder.punctuation(RustPunctuation::CommaSpaced);
                            builder.delimited_comma_list(
                                RustDelimiter::Par,
                                hir_decl.fields(db).iter().map(|field| field.ident()),
                            );
                        }
                        TypeVariantHirDecl::Unit(_) => (),
                        TypeVariantHirDecl::Tuple(hir_decl) => {
                            builder.punctuation(RustPunctuation::CommaSpaced);
                            builder.delimited_comma_list(
                                RustDelimiter::Par,
                                hir_decl
                                    .fields(db)
                                    .iter()
                                    .enumerate()
                                    .map(|(i, _)| TupleFieldVariable(i)),
                            );
                        }
                    }
                },
            ),
            LinkageData::EnumVariantDestructor {
                self_ty,
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::EnumVariantDestructorLinkageImpl, |builder| {
                self_ty.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                path.transpile_to_rust(builder);
                match path.hir_decl(db).unwrap() {
                    TypeVariantHirDecl::Props(hir_decl) => {
                        builder.punctuation(RustPunctuation::CommaSpaced);
                        builder.delimited_comma_list(
                            RustDelimiter::Curl,
                            hir_decl.fields(db).iter().map(|field| field.ident()),
                        );
                    }
                    TypeVariantHirDecl::Tuple(hir_decl) => {
                        builder.punctuation(RustPunctuation::CommaSpaced);
                        builder.delimited_comma_list(
                            RustDelimiter::Par,
                            hir_decl
                                .fields(db)
                                .iter()
                                .enumerate()
                                .map(|(i, _)| TupleFieldVariable(i)),
                        );
                    }
                    TypeVariantHirDecl::Unit(_) => unreachable!(),
                }
            }),
            LinkageData::EnumVariantDiscriminator {
                self_ty,
                path,
                ref instantiation,
            } => builder.macro_call(
                RustMacroName::EnumVariantDiscriminatorLinkageImpl,
                |builder| {
                    self_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    path.transpile_to_rust(builder);
                    match path.hir_decl(db).unwrap() {
                        TypeVariantHirDecl::Props(_) => {
                            builder.punctuation(RustPunctuation::CommaSpaced);
                            builder.curl_unit();
                        }
                        TypeVariantHirDecl::Tuple(_) => {
                            builder.punctuation(RustPunctuation::CommaSpaced);
                            builder.par_unit();
                        }
                        TypeVariantHirDecl::Unit(_) => (),
                    }
                },
            ),
            LinkageData::EnumVariantField {
                path,
                ref instantiation,
                field,
            } => builder.macro_call(RustMacroName::EnumVariantFieldLinkageImpl, |builder| {
                path.parent_ty_path(db).transpile_to_rust(builder);
                builder.delimited_comma_list(
                    RustDelimiter::Angle,
                    instantiation.iter().map(|(_, res)| match res {
                        LinTermSymbolResolution::Explicit(arg) => arg,
                        LinTermSymbolResolution::SelfLifetime
                        | LinTermSymbolResolution::SelfQual(_) => unreachable!(),
                    }),
                );
                builder.punctuation(RustPunctuation::CommaSpaced);
                path.transpile_to_rust(builder);
                builder.punctuation(RustPunctuation::CommaSpaced);
                match field {
                    LinkageField::Tuple { index } => builder
                        .delimited(RustDelimiter::Par, |builder| {
                            TupleFieldVariable(index.into()).transpile_to_rust(builder)
                        }),
                    LinkageField::Props { ident } => builder
                        .delimited(RustDelimiter::Curl, |builder| {
                            ident.transpile_to_rust(builder)
                        }),
                }
            }),
            LinkageData::EnumU8ToJsonValue { ty_path } => builder
                .macro_call(RustMacroName::EnumU8Presenter, |builder| {
                    ty_path.transpile_to_rust(builder)
                }),
            LinkageData::StructConstructor {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                builder.struct_ty_constructor_path(path);
                turbo_fish_instantiation(instantiation, builder);
            }),
            LinkageData::StructDestructor { self_ty } => {
                builder.macro_call(RustMacroName::StructDestructorLinkageImpl, |builder| {
                    self_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    self_ty.ty_path(db).transpile_to_rust(builder);
                    match self_ty.ty_path(db).hir_decl(db).unwrap() {
                        TypeHirDecl::PropsStruct(hir_decl) => {
                            for field in hir_decl.fields(db) {
                                builder.punctuation(RustPunctuation::CommaSpaced);
                                field.ident().transpile_to_rust(builder)
                            }
                        }
                        TypeHirDecl::TupleStruct(hir_decl) => todo!(),
                        TypeHirDecl::Enum(_)
                        | TypeHirDecl::UnitStruct(_)
                        | TypeHirDecl::Extern(_)
                        | TypeHirDecl::Union(_) => unreachable!(),
                    }
                })
            }
            LinkageData::StructField { self_ty, field } => {
                builder.macro_call(RustMacroName::StructFieldLinkageImpl, |builder| {
                    self_ty.transpile_to_rust(builder);
                    builder.punctuation(RustPunctuation::CommaSpaced);
                    match field {
                        LinkageField::Tuple { index } => todo!(),
                        LinkageField::Props { ident } => ident.transpile_to_rust(builder),
                    }
                })
            }
            LinkageData::AssocRitchie {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::UnveilAssocFn {
                path,
                ref instantiation,
            } => builder.macro_call(RustMacroName::UnveilLinkageImpl, |builder| {
                (path, instantiation).transpile_to_rust(builder)
            }),
            LinkageData::MemoizedField {
                path,
                instantiation: _,
            } => builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                path.transpile_to_rust(builder)
            }),
            LinkageData::Index => todo!(),
            LinkageData::TypeDefault { ty } => builder
                .macro_call(RustMacroName::TypeDefault, |builder| {
                    ty.transpile_to_rust(builder)
                }),
            LinkageData::VecConstructor { element_ty } => {
                builder.macro_call(RustMacroName::FnLinkageImpl, |builder| {
                    builder.delimited(RustDelimiter::Vert, |builder| {
                        builder.v();
                        builder.punctuation(RustPunctuation::Colon);
                        builder.vec_ty(element_ty)
                    });
                    builder.v()
                })
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for (AssocItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        match path {
            AssocItemPath::TypeItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssocItemPath::TraitItem(slf) => (slf, instantiation).transpile_to_rust(builder),
            AssocItemPath::TraitForTypeItem(slf) => (slf, instantiation).transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for (MajorFormPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        path.transpile_to_rust(builder);
        turbo_fish_instantiation(instantiation, builder);
    }
}

fn turbo_fish_instantiation<E>(
    instantiation: &LinInstantiation,
    builder: &mut RustTranspilationBuilder<'_, '_, E>,
) {
    if !instantiation.is_empty() {
        builder.delimited_comma_list(
            RustDelimiter::TurboFish,
            instantiation.iter().map(|&(_, res)| match res {
                LinTermSymbolResolution::Explicit(arg) => arg,
                LinTermSymbolResolution::SelfLifetime => todo!(),
                LinTermSymbolResolution::SelfQual(_) => todo!(),
            }),
        )
    }
}

impl<E> TranspileToRustWith<E> for (TypeVariantPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, instantiation) = self;
        path.transpile_to_rust(builder);
        turbo_fish_instantiation(instantiation, builder);
    }
}

impl<E> TranspileToRustWith<E> for (TypeItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, lin_instantiation) = self;
        let db = builder.db;
        let self_ty = HirType::from_eth(
            path.impl_block(db).eth_template(db).unwrap().self_ty(db),
            db,
        )
        .unwrap()
        .linkage_instantiate(lin_instantiation, db);
        let ident = path.ident(db).unwrap();
        builder.delimited(RustDelimiter::Angle, |builder| {
            match self_ty {
                LinType::PathLeading(self_ty) => match self_ty.ty_path(db).refine(db) {
                    Left(PreludeTypePath::VEC) => match ident.data(db) {
                        "first" | "last" => {
                            // `first` or `last` are methods from slice,
                            // so we write down Rust's slice type `[T]` instead
                            builder.delimited_comma_list(
                                RustDelimiter::Box,
                                self_ty.template_arguments(db),
                            );
                            return;
                        }
                        _ => (),
                    },
                    Left(PreludeTypePath::CYCLIC_SLICE) => {
                        builder.cyclic_slice_leashed_ty();
                        builder.delimited_comma_list(
                            RustDelimiter::Angle,
                            self_ty.template_arguments(db),
                        );
                        return;
                    }
                    _ => (),
                },
                _ => (),
            }
            self_ty.transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        let places = lin_instantiation.places();
        match places.len() {
            0 => ident.transpile_to_rust(builder),
            1 => {
                let (_symbol, place) = places[0];
                match place {
                    LinTermSymbolResolution::Explicit(LinTemplateArgument::Qual(_)) => {
                        todo!()
                    }
                    LinTermSymbolResolution::SelfQual(place) => match place {
                        qual::LinQual::Ref => ident.transpile_to_rust(builder),
                        qual::LinQual::RefMut => builder.method_ritchie_ident_mut(ident),
                        qual::LinQual::Transient => todo!(),
                    },
                    _ => unreachable!(),
                }
            }
            _ => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let _db = builder.db;
        match self {
            LinType::PathLeading(slf) => slf.transpile_to_rust(builder),
            LinType::Ritchie(slf) => slf.transpile_to_rust(builder),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageTrait {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        self.trai_path(db).transpile_to_rust(builder);
        let template_arguments = self.template_arguments(db);
        if !template_arguments.is_empty() {
            builder.delimited_comma_list(RustDelimiter::Angle, template_arguments)
        }
    }
}

impl<E> TranspileToRustWith<E> for LinTypePathLeading {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        let template_arguments = self.template_arguments(db);
        match self.ty_path(db).refine(db) {
            Left(PreludeTypePath::REF) => {
                debug_assert_eq!(template_arguments.len(), 2);
                builder.punctuation(RustPunctuation::Ambersand);
                template_arguments[0].transpile_to_rust(builder);
                template_arguments[1].transpile_to_rust(builder)
            }
            Left(PreludeTypePath::REF_MUT) => todo!(),
            Left(PreludeTypePath::SLICE) => todo!(),
            _ => {
                self.ty_path(db).transpile_to_rust(builder);
                let template_arguments = self.template_arguments(db);
                if !template_arguments.is_empty() {
                    builder.delimited_comma_list(RustDelimiter::Angle, template_arguments)
                }
            }
        }
    }
}

impl<E> TranspileToRustWith<E> for LinTemplateArgument {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self {
            LinTemplateArgument::Vacant => todo!(),
            LinTemplateArgument::Type(linkage_ty) => linkage_ty.transpile_to_rust(builder),
            LinTemplateArgument::Constant(constant) => constant.transpile_to_rust(builder),
            LinTemplateArgument::Lifetime => todo!(),
            LinTemplateArgument::Qual(_) => todo!(),
        }
    }
}

impl<E> TranspileToRustWith<E> for LinkageRitchieType {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        builder.keyword(RustKeyword::Fn);
        builder.delimited_comma_list(RustDelimiter::Par, self.parameters(db).iter());
        builder.punctuation(RustPunctuation::LightArrow);
        self.return_ty(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for LinkageRitchieParameter {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        match self.contract() {
            // ad hoc
            HirContract::Pure => (),
            // builder.punctuation(RustPunctuation::Ambersand),
            HirContract::Move => (),
            HirContract::Borrow => builder.punctuation(RustPunctuation::Ambersand),
            HirContract::BorrowMut => todo!(),
            HirContract::Const => todo!(),
            HirContract::Leash => todo!(),
            HirContract::At => todo!(),
        }
        self.parameter_ty().transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for (TraitItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, _instantiation) = self;
        let db = builder.db;
        path.trai_path(db).transpile_to_rust(builder);
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRustWith<E> for (TraitForTypeItemPath, &LinInstantiation) {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        let (path, lin_instantiation) = self;
        let db = builder.db;
        builder.delimited(RustDelimiter::Angle, |builder| {
            let trait_for_type_impl_block_eth_template =
                path.impl_block(db).eth_template(db).unwrap();
            let self_ty = HirType::from_eth(trait_for_type_impl_block_eth_template.self_ty(db), db)
                .unwrap()
                .linkage_instantiate(lin_instantiation, db);
            self_ty.transpile_to_rust(builder);
            builder.keyword(RustKeyword::As);
            let trai = HirTrait::from_eth(trait_for_type_impl_block_eth_template.trai(db), db);
            trai.linkage_instantiate(lin_instantiation, db)
                .transpile_to_rust(builder)
        });
        builder.punctuation(RustPunctuation::ColonColon);
        path.ident(db).transpile_to_rust(builder)
    }
}
