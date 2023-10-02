#[cfg(feature = "protocol_support")]
use husky_entity_protocol::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TypeKind {
    Enum,
    Inductive,
    Record,
    Struct,
    Structure,
    Extern,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum FugitiveKind {
    FunctionFn,
    FunctionGn,
    AliasType,
    Val,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EntityKind {
    Module,
    MajorItem {
        module_item_kind: MajorItemKind,
        connection: MajorItemConnectionKind,
    },
    AssociatedItem {
        associated_item_kind: AssociatedItemKind,
    },
    TypeVariant,
    ImplBlock,
    Attr,
}

#[cfg(feature = "protocol_support")]
impl EntityKind {
    pub fn protocol(self) -> EntityClass {
        match self {
            EntityKind::Module => EntityClass::Module,
            EntityKind::MajorItem {
                module_item_kind, ..
            } => match module_item_kind {
                MajorItemKind::Type(_) => EntityClass::Type,
                MajorItemKind::Fugitive(fugitive_kind) => match fugitive_kind {
                    FugitiveKind::FunctionFn => EntityClass::FunctionFn,
                    FugitiveKind::FunctionGn => EntityClass::FunctionGn,
                    FugitiveKind::AliasType => EntityClass::AliasType,
                    FugitiveKind::Val => EntityClass::Val,
                },
                MajorItemKind::Trait => EntityClass::Trait,
            },
            EntityKind::AssociatedItem {
                associated_item_kind,
            } => match associated_item_kind {
                AssociatedItemKind::TraitItem(trai_item_kind) => match trai_item_kind {
                    TraitItemKind::MethodFn => EntityClass::MethodFn,
                    TraitItemKind::AssociatedType => EntityClass::AssociatedType,
                },
                AssociatedItemKind::TypeItem(ty_item_kind) => match ty_item_kind {
                    TypeItemKind::MethodFn => EntityClass::MethodFn,
                    TypeItemKind::AssociatedFunctionFn => EntityClass::AssociatedFunctionFn,
                    TypeItemKind::AssociatedVal => EntityClass::AssociatedVal,
                    TypeItemKind::AssociatedType => EntityClass::AssociatedType,
                    TypeItemKind::MemoizedField => EntityClass::MemoizedField,
                },
                AssociatedItemKind::TraitForTypeItem(trai_for_ty_item_kind) => {
                    match trai_for_ty_item_kind {
                        TraitItemKind::MethodFn => EntityClass::MethodFn,
                        TraitItemKind::AssociatedType => EntityClass::AssociatedType,
                    }
                }
            },
            EntityKind::TypeVariant => EntityClass::TypeVariant,
            EntityKind::ImplBlock => EntityClass::ImplBlock,
            EntityKind::Attr => EntityClass::Attr,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[enum_class::from_variants]
pub enum MajorItemKind {
    Type(TypeKind),
    Fugitive(FugitiveKind),
    Trait,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AssociatedItemKind {
    TraitItem(TraitItemKind),
    TypeItem(TypeItemKind),
    TraitForTypeItem(TraitItemKind),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeItemKind {
    MethodFn,
    AssociatedFunctionFn,
    AssociatedVal,
    AssociatedType,
    MemoizedField,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TraitItemKind {
    MethodFn,
    AssociatedType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MajorItemConnectionKind {
    Connected,
    Disconnected,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum EnumVariantKind {
    Constant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoutineKind {
    Normal,
    TypeCall,
    TypeAssociated,
    TraitAssociated,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawMembRoutineKind {
    Proc,
    Func,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MemberKind {
    Field,
    Method { is_lazy: bool },
    Call,
    TraitAssociatedType,
    TraitAssociatedConstSize,
    TraitAssociatedAny,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum FieldKind {
    StructRegular,
    StructDefault,
    StructDerived,
    StructMemo, // property is not store along with struct
    RecordRegular,
    RecordProperty,
}
