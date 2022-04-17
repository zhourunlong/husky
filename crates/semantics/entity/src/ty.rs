mod member;

use infer_decl::MemberIdx;
pub use member::*;
use print_utils::msg_once;

use std::{iter::Peekable, sync::Arc};

use super::*;
use ast::*;
use entity_route::{EntityRoute, EntityRouteKind, EntityRoutePtr, RangedEntityRoute};
use file::FilePtr;
use infer_total::InferQueryGroup;
use semantics_eager::{FuncStmt, ProcStmt};
use semantics_error::SemanticResult;
use semantics_lazy::LazyStmt;
use vec_dict::{HasKey, VecDict};
use vm::{FieldContract, InputContract};
use word::{CustomIdentifier, IdentDict};

impl EntityDefnVariant {
    pub(crate) fn ty_from_ast(
        db: &dyn InferQueryGroup,
        entity_route: EntityRoutePtr,
        head: &Ast,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        let (ident, kind, generic_placeholders) = match head.kind {
            AstKind::TypeDefnHead {
                ident,
                kind,
                ref generic_placeholders,
            } => (ident, kind, generic_placeholders.clone()),
            _ => panic!(),
        };
        let mut children = children.peekable();
        let mut type_members = IdentDict::default();
        let mut trait_impls = Vec::new();
        msg_once!("todo");
        Self::collect_fields(db, &mut children, &mut type_members, entity_route, file)?;
        Self::collect_member_calls(
            db,
            arena,
            file,
            entity_route,
            &mut children,
            &mut type_members,
        )?;
        let variants = Self::collect_variants(db, file, children)?;
        Ok(EntityDefnVariant::new_ty(
            type_members,
            variants,
            kind,
            trait_impls,
        ))
    }

    fn new_ty(
        type_members: IdentDict<Arc<EntityDefn>>,
        variants: IdentDict<Arc<EntityDefn>>,
        kind: TypeKind,
        trait_impls: Vec<Arc<EntityDefn>>,
    ) -> Self {
        let members = collect_all_members(&type_members, &trait_impls);
        EntityDefnVariant::Type {
            type_members,
            variants,
            kind,
            trait_impls,
            members,
        }
    }

    fn collect_variants(
        db: &dyn InferQueryGroup,
        file: FilePtr,
        mut children: Peekable<AstIter>,
    ) -> SemanticResult<IdentDict<Arc<EntityDefn>>> {
        let mut variants = VecDict::default();
        for child in children {
            let ast = child.value.as_ref()?;
            match ast.kind {
                AstKind::EnumVariantDefnHead {
                    ident,
                    variant_class: raw_variant_kind,
                } => {
                    variants.insert_new(EntityDefn::new(
                        ident.into(),
                        EntityDefnVariant::EnumVariant {
                            ident,
                            variant: match raw_variant_kind {
                                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
                            },
                        },
                        db.intern_entity_route(EntityRoute {
                            kind: todo!(),
                            generic_arguments: todo!(),
                        }),
                        file,
                        ast.range,
                    ));
                }
                _ => panic!(),
            }
        }
        Ok(variants)
    }

    fn record_from_ast(
        db: &dyn InferQueryGroup,
        children: AstIter,
        arena: &RawExprArena,
        file: FilePtr,
    ) -> SemanticResult<EntityDefnVariant> {
        todo!()
        // let mut fields = VecDict::default();
        // for subitem in children {
        //     match subitem.value.as_ref()?.kind {
        //         AstKind::Use { ident, scope } => (),
        //         AstKind::RoutineDefnHead(_) => todo!(),
        //         AstKind::FieldDefn(ref field_var_defn) => fields.insert_new(field_var_defn.clone()),
        //         AstKind::MembFeatureDefnHead { ident, ty } => {
        //             let stmts = semantics_lazy::parse_lazy_stmts(
        //                 &[],
        //                 db,
        //                 arena,
        //                 subitem.children.unwrap(),
        //                 file,
        //             )?;
        //             fields.insert_new(FieldDefn {
        //                 ident,
        //                 output_ty: ty,
        //                 stmts,
        //             });
        //         }
        //         _ => panic!(),
        //     }
        // }
        // Ok(TyKind::Record { fields })
    }

    pub fn method(&self, member_idx: usize) -> &Arc<EntityDefn> {
        todo!()
        // match self.members[member_idx] {
        //     MemberDefn::TypeField(_) => todo!(),
        //     MemberDefn::TypeMethod(_) => todo!(),
        // }
    }
}

impl EntityDefn {
    pub fn method(&self, member_idx: MemberIdx) -> &Arc<EntityDefn> {
        match self.variant {
            EntityDefnVariant::Type { ref members, .. } => &members[member_idx.0 as usize],
            EntityDefnVariant::EnumVariant { ident, ref variant } => todo!(),
            EntityDefnVariant::Builtin => todo!(),
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum MethodKind {
    Func { stmts: Arc<Vec<Arc<FuncStmt>>> },
    Proc { stmts: Arc<Vec<Arc<ProcStmt>>> },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantDefnVariant {
    Constant,
}

impl EntityDefnVariant {
    pub fn enum_variant(
        db: &dyn EntityDefnQueryGroup,
        ident: CustomIdentifier,
        enum_variant_kind: EnumVariantKind,
        children: Option<AstIter>,
    ) -> EntityDefnVariant {
        EntityDefnVariant::EnumVariant {
            ident,
            variant: match enum_variant_kind {
                EnumVariantKind::Constant => EnumVariantDefnVariant::Constant,
            },
        }
    }
}
