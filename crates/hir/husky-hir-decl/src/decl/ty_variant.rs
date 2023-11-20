mod enum_props_ty_variant;
mod enum_tuple_ty_variant;
mod enum_unit_ty_variant;

use husky_entity_kind::TypeKind;
use husky_syn_decl::TypeVariantSynDecl;

pub use self::enum_props_ty_variant::*;
pub use self::enum_tuple_ty_variant::*;
pub use self::enum_unit_ty_variant::*;

use super::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
#[salsa::debug_with_db(db = HirDeclDb)]
#[enum_class::from_variants]
pub enum TypeVariantHirDecl {
    Props(EnumPropsTypeVariantHirDecl),
    Unit(EnumUnitTypeVariantHirDecl),
    Tuple(EnumTupleTypeVariantHirDecl),
}

impl TypeVariantHirDecl {
    pub fn path(self, _db: &dyn HirDeclDb) -> TypeVariantPath {
        match self {
            TypeVariantHirDecl::Props(_) => todo!(),
            TypeVariantHirDecl::Unit(_) => todo!(),
            TypeVariantHirDecl::Tuple(_) => todo!(),
        }
    }

    pub fn hir_expr_region(self, _db: &dyn HirDeclDb) -> HirExprRegion {
        match self {
            TypeVariantHirDecl::Props(_) => todo!(),
            TypeVariantHirDecl::Unit(_) => todo!(),
            TypeVariantHirDecl::Tuple(_) => todo!(),
        }
    }
}

impl HasHirDecl for TypeVariantPath {
    type HirDecl = TypeVariantHirDecl;

    fn hir_decl(self, db: &dyn HirDeclDb) -> Option<Self::HirDecl> {
        ty_variant_hir_decl(db, self)
    }
}

#[salsa::tracked(jar = HirDeclJar)]
fn ty_variant_hir_decl(db: &dyn HirDeclDb, path: TypeVariantPath) -> Option<TypeVariantHirDecl> {
    match path.parent_ty_path(db).ty_kind(db) {
        TypeKind::Enum => (),
        TypeKind::Inductive => return None,
        _ => unreachable!(),
    }
    Some(match path.syn_decl(db).expect("no errors for hir stage") {
        TypeVariantSynDecl::Props(_) => todo!(),
        TypeVariantSynDecl::Unit(syn_decl) => {
            EnumUnitTypeVariantHirDecl::from_syn(path, syn_decl, db).into()
        }
        TypeVariantSynDecl::Tuple(syn_decl) => {
            EnumTupleTypeVariantHirDecl::from_syn(path, syn_decl, db).into()
        }
    })
}
