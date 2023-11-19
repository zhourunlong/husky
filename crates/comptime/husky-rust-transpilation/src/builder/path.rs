use super::*;
use husky_entity_path::{AssociatedItemPath, TraitPath};

impl TranspileToRust<HirEagerExprRegion> for AssociatedItemPath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<HirEagerExprRegion>) {
        let db = builder.db;
        match self {
            AssociatedItemPath::TypeItem(path) => {
                path.impl_block(db).ty_path(db).transpile_to_rust(builder)
            }
            AssociatedItemPath::TraitItem(_) => todo!(),
            AssociatedItemPath::TraitForTypeItem(path) => {
                todo!()
            }
        }
        builder.opr(RustOpr::ColonColon);
        self.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRust<E> for PrincipalEntityPath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db;
        match self {
            PrincipalEntityPath::Module(path) => path.ident(db).transpile_to_rust(builder),
            PrincipalEntityPath::MajorItem(path) => path.ident(db).transpile_to_rust(builder),
            PrincipalEntityPath::TypeVariant(path) => {
                match path.parent_ty_path(db).prelude_ty_path(db) {
                    Some(PreludeTypePath::Option | PreludeTypePath::Result) => (),
                    _ => {
                        path.parent_ty_path(db).ident(db).transpile_to_rust(builder);
                        builder.opr(RustOpr::ColonColon);
                    }
                }
                path.ident(db).transpile_to_rust(builder)
            }
        }
    }
}

impl<E> TranspileToRust<E> for TypePath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        self.ident(db).transpile_to_rust(builder)
    }
}

impl<E> TranspileToRust<E> for TraitPath {
    fn transpile_to_rust(&self, builder: &mut RustTranspilationBuilder<E>) {
        let db = builder.db();
        self.ident(db).transpile_to_rust(builder)
    }
}
