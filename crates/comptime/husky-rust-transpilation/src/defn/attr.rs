use super::*;
use either::*;
use husky_entity_path::{
    menu::item_path_menu,
    path::{
        attr::AttrItemPath,
        major_item::trai::{PreludeTraitPath, TraitPath},
    },
};
use husky_hir_decl::decl::{AttrHirDecl, HasHirDecl};
use husky_hir_defn::defn::attr::AttrHirDefn;
use husky_hir_ty::trai::HirTrait;
use vec_like::VecSet;

impl TranspileToRustWith for AttrHirDefn {
    fn transpile_to_rust(self, _builder: &mut RustTranspilationBuilder) {
        todo!()
    }
}

pub(crate) struct Attrs {
    derive_attrs: DeriveAttrs,
    is_test: bool,
}

impl Attrs {
    pub(crate) fn new(attr_paths: &[AttrItemPath], builder: &RustTranspilationBuilder) -> Self {
        let db = builder.db();
        let mut derive_attrs = DeriveAttrs::base(builder);
        let mut is_test = false;
        for attr_path in attr_paths {
            match attr_path.hir_decl(db).unwrap() {
                AttrHirDecl::Affect(_) => (),
                AttrHirDecl::Backprop(_) => (),
                AttrHirDecl::Dep(_) => (),
                AttrHirDecl::Derive(decl) => derive_attrs.merge(decl.trais(db), db),
                AttrHirDecl::Task(_) => (),
                AttrHirDecl::Test(_) => is_test = true,
            }
        }
        Self {
            derive_attrs,
            is_test,
        }
    }
}

pub(crate) struct DeriveAttrs {
    trais: VecSet<TraitPath>,
}

impl DeriveAttrs {
    fn base(builder: &RustTranspilationBuilder) -> Self {
        let item_path_menu = item_path_menu(builder.db, builder.toolchain);
        Self {
            trais: VecSet::from_iter([
                item_path_menu.debug_trai_path(),
                item_path_menu.clone_trai_path(),
                item_path_menu.partial_eq_trai_path,
            ]),
        }
    }

    fn merge(&mut self, trais: &[HirTrait], db: &::salsa::Db) {
        for trai in trais {
            let trai_path = trai.trai_path(db);
            match trai_path.refine(db) {
                Left(PreludeTraitPath::VISUALIZE) => continue,
                _ => (),
            }
            self.trais.insert(trai_path)
        }
    }
}

impl<E> TranspileToRustWith<E> for Attrs {
    fn transpile_to_rust(self, builder: &mut RustTranspilationBuilder<E>) {
        builder.derive(&self.derive_attrs.trais)
    }
}
