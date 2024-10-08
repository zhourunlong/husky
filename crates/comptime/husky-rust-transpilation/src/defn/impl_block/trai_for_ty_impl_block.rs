use super::*;
use either::*;
use husky_entity_path::path::major_item::trai::PreludeTraitPath;
use husky_entity_tree::node::HasAssocItemPaths;
use husky_hir_decl::decl::TraitForTypeImplBlockHirDecl;
use husky_hir_defn::defn::impl_block::trai_for_ty_impl_block::TraitForTypeImplBlockHirDefn;
use husky_hir_ty::{HirComptermTemplateVariable, HirTemplateVariable, HirTemplateVariableClass};
use smallvec::SmallVec;

impl TranspileToRustWith for TraitForTypeImplBlockHirDefn {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        let hir_decl = self.hir_decl();
        let path = hir_decl.path(db);
        let refined_trai_path = path.trai_path(db).refine(db);
        match refined_trai_path {
            // skipping visualize, will be replaced by cfg to achieve the same skipping
            Left(PreludeTraitPath::INT_INDEX) => return,
            _ => (),
        }
        builder.rustfmt_skip();
        hir_decl.transpile_to_rust(builder);
        let runtime_const_symbols: SmallVec<[HirComptermTemplateVariable; 4]> = hir_decl
            .template_parameters(db)
            .iter()
            .filter_map(|param| match param.symbol() {
                HirTemplateVariable::Compterm(symbol) => {
                    (symbol.index(db).class() == HirTemplateVariableClass::Poly).then_some(symbol)
                }
                _ => None,
            })
            .collect();
        match refined_trai_path {
            Left(PreludeTraitPath::UNVEIL) => {
                builder.with_hir_eager_expr_region(hir_decl.hir_eager_expr_region(db), |builder| {
                    builder.keyword(RustKeyword::Where);
                    for symbol in &runtime_const_symbols {
                        symbol.ty(db).transpile_to_rust(builder);
                        builder.punctuation(RustPunctuation::Colon);
                        builder.copy_trait()
                    }
                })
            }
            _ => assert!(runtime_const_symbols.is_empty()),
        }
        builder.curly_block(|builder| {
            match refined_trai_path {
                Left(PreludeTraitPath::UNVEIL) => builder.on_fresh_semicolon_line(|builder| {
                    builder.type_runtime_const_symbols_is();
                    builder.with_hir_eager_expr_region(
                        hir_decl.hir_eager_expr_region(db),
                        |builder| {
                            builder.delimited_comma_list(
                                RustDelimiter::Par,
                                runtime_const_symbols.iter().map(|symbol| symbol.ty(db)),
                            )
                        },
                    )
                }),
                _ => (),
            }
            for &(_, trai_for_ty_item_path) in hir_decl.path(db).assoc_item_paths(db) {
                if let Some(hir_defn) = trai_for_ty_item_path.hir_defn(db) {
                    builder.on_fresh_paragraph(|builder| hir_defn.transpile_to_rust(builder));
                }
            }
        })
    }
}

impl TranspileToRustWith for TraitForTypeImplBlockHirDecl {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder) {
        let db = builder.db();
        builder.with_hir_eager_expr_region(self.hir_eager_expr_region(db), |builder| {
            builder.keyword(RustKeyword::Impl);
            self.template_parameters(db).transpile_to_rust(builder);
            self.trai(db).transpile_to_rust(builder);
            builder.keyword(RustKeyword::ConnectionFor);
            self.self_ty(db).transpile_to_rust(builder)
        })
    }
}
