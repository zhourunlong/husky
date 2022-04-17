use check_utils::should_eq;
use entity_kind::TypeKind;
use file::FilePtr;
use linkage_table::SearchLinkage;
use pack_semantics::PackQueryGroup;
use vm::{BoxedValue, EvalValue, Linkage, StackValue, VMResult};

use crate::*;

#[salsa::query_group(InstructionGenQueryGroupStorage)]
pub trait InstructionGenQueryGroup: EntityDefnQueryGroup + PackQueryGroup + SearchLinkage {
    fn entity_instruction_sheet(&self, route: EntityRoutePtr) -> Arc<InstructionSheet>;
    fn method_instruction_sheet(&self, member_route: EntityRoutePtr) -> Arc<InstructionSheet>;
    fn dataset_config_instruction_sheet(&self, pack_main: FilePtr) -> Arc<InstructionSheet>;
    fn virtual_vec_method_linkages(&self) -> Arc<IdentPairDict<Linkage>>;
}

fn entity_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    route: EntityRoutePtr,
) -> Arc<InstructionSheet> {
    let entity_defn = db.entity_defn(route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Module { .. } => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern { .. } => todo!(),
        EntityDefnVariant::Func {
            ref input_placeholders,
            ref stmts,
            ..
        } => InstructionSheetBuilder::new_decl(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityDefnVariant::Proc {
            ref input_placeholders,
            ref stmts,
            ..
        } => InstructionSheetBuilder::new_impr(
            db,
            input_placeholders
                .iter()
                .map(|input_placeholder| input_placeholder.ident)
                .collect(),
            stmts,
            false,
        ),
        EntityDefnVariant::Type { .. } => todo!(),
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Builtin => {
            p!(route.ident());
            todo!()
        }
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TypeField { .. } => todo!(),
        EntityDefnVariant::TypeMethod { .. } => todo!(),
        EntityDefnVariant::TraitMethod { .. } => todo!(),
        EntityDefnVariant::TraitMethodImpl { .. } => todo!(),
    }
}

fn method_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    member_route: EntityRoutePtr,
) -> Arc<InstructionSheet> {
    let ty = member_route.parent();
    let entity_defn = db.entity_defn(ty).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Main(_) => todo!(),
        EntityDefnVariant::Module {} => todo!(),
        EntityDefnVariant::Feature { .. } => todo!(),
        EntityDefnVariant::Pattern {} => todo!(),
        EntityDefnVariant::Func { .. } => todo!(),
        EntityDefnVariant::Proc { .. } => todo!(),
        EntityDefnVariant::Type {
            ref type_members,
            ref variants,
            kind,
            ref trait_impls,
            ref members,
        } => match kind {
            TypeKind::Enum => todo!(),
            TypeKind::Struct => {
                todo!()
                // let field_routine = ty_defn.methods.get(method_ident).unwrap();
                // let inputs = field_routine
                //     .input_placeholders
                //     .iter()
                //     .map(|input_placeholder| input_placeholder.ident)
                //     .collect();
                // match field_routine.kind {
                //     MethodKind::Func { ref stmts } => {
                //         InstructionSheetBuilder::new_decl(db, inputs, stmts, true)
                //     }
                //     MethodKind::Proc { ref stmts } => {
                //         InstructionSheetBuilder::new_impr(db, inputs, stmts, true)
                //     }
                // }
            }
            TypeKind::Record => todo!(),
            TypeKind::Primitive => todo!(),
            TypeKind::Vec => todo!(),
            TypeKind::Array => todo!(),
            TypeKind::Other => todo!(),
        },
        EntityDefnVariant::Builtin => todo!(),
        EntityDefnVariant::EnumVariant { .. } => todo!(),
        EntityDefnVariant::TypeField { .. } => todo!(),
        EntityDefnVariant::TypeMethod { .. } => todo!(),
        EntityDefnVariant::TraitMethod { .. } => todo!(),
        EntityDefnVariant::TraitMethodImpl { .. } => todo!(),
    }
}

fn dataset_config_instruction_sheet(
    db: &dyn InstructionGenQueryGroup,
    pack_main: FilePtr,
) -> Arc<InstructionSheet> {
    let pack = db.package(pack_main).unwrap();
    InstructionSheetBuilder::new_decl(db, vec![], &pack.config.dataset.stmts, false)
}

fn virtual_vec_method_linkages(db: &dyn InstructionGenQueryGroup) -> Arc<IdentPairDict<Linkage>> {
    let mut field_routine_linkages = IdentDict::default();
    field_routine_linkages.insert_new((
        db.intern_word("clone").opt_custom().unwrap(),
        Linkage {
            call: virtual_vec_clone,
            nargs: 1,
        },
    ));
    field_routine_linkages.insert_new((
        db.intern_word("len").opt_custom().unwrap(),
        Linkage {
            call: virtual_vec_len,
            nargs: 1,
        },
    ));
    field_routine_linkages.insert_new((
        db.intern_word("push").opt_custom().unwrap(),
        Linkage {
            call: virtual_vec_push,
            nargs: 2,
        },
    ));
    field_routine_linkages.insert_new((
        db.intern_word("pop").opt_custom().unwrap(),
        Linkage {
            call: virtual_vec_pop,
            nargs: 1,
        },
    ));
    field_routine_linkages.insert_new((
        db.intern_word("first").opt_custom().unwrap(),
        Linkage {
            call: virtual_vec_first,
            nargs: 1,
        },
    ));
    field_routine_linkages.insert_new((
        db.intern_word("last").opt_custom().unwrap(),
        Linkage {
            call: virtual_vec_last,
            nargs: 1,
        },
    ));
    Arc::new(field_routine_linkages)
}

fn virtual_vec_clone<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let virtual_vec: &Vec<EvalValue<'eval>> = values[0].downcast_ref();
    let virtual_vec_cloned: Vec<EvalValue<'eval>> = virtual_vec.clone();
    Ok(StackValue::Boxed(BoxedValue::new(virtual_vec_cloned)))
}

fn virtual_vec_len<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    let virtual_vec: &Vec<EvalValue<'eval>> = values[0].downcast_ref();
    let len: i32 = virtual_vec.len().try_into().unwrap();
    Ok(StackValue::Primitive(len.into()))
}

fn virtual_vec_push<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    should_eq!(values.len(), 2);
    let element = values[1].into_eval();
    let virtual_vec: &mut Vec<EvalValue<'eval>> = values[0].downcast_mut();
    virtual_vec.push(element);
    Ok(StackValue::Primitive(().into()))
}

fn virtual_vec_pop<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn virtual_vec_first<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}

fn virtual_vec_last<'stack, 'eval>(
    values: &mut [StackValue<'stack, 'eval>],
) -> VMResult<StackValue<'stack, 'eval>> {
    todo!()
}
