mod opn;
mod parser;

use crate::*;
use husky_ast::{AstIter, RawExprArena, RawExprIdx};
use husky_file::FilePtr;
use husky_infer_qualified_ty::{EagerValueQualifiedTy, EagerVariableQualifier};
use husky_primitive_literal_syntax::PrimitiveLiteralData;
use infer_contract::EagerContract;
use infer_total::InferQueryGroup;
pub use opn::*;
pub(crate) use parser::EagerExprParser;
use std::sync::Arc;

use husky_entity_route::EntityRoutePtr;
use husky_text::{RangedCustomIdentifier, TextRange};
use husky_word::CustomIdentifier;
use semantics_error::SemanticResultArc;
use vm::{Binding, InstructionId, InstructionSource, PrimitiveValueData, __LinkageFp};

#[derive(Clone, PartialEq, Eq)]
pub struct EagerExpr {
    pub file: FilePtr,
    pub range: TextRange,
    pub qualified_ty: EagerValueQualifiedTy,
    pub contract: EagerContract,
    pub variant: EagerExprVariant,
    pub instruction_id: InstructionId,
}

impl std::fmt::Debug for EagerExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EagerExpr")
            .field("file", &self.file)
            .field("range", &self.range)
            .field("qualified_ty", &self.qualified_ty)
            .field("instruction_id", &self.instruction_id)
            .finish()
    }
}

impl EagerExpr {
    pub fn ty(&self) -> EntityRoutePtr {
        self.qualified_ty.ty
    }
}

impl InstructionSource for EagerExpr {
    fn instruction_id(&self) -> InstructionId {
        self.instruction_id
    }

    fn file(&self) -> FilePtr {
        self.file
    }

    fn text_range(&self) -> TextRange {
        self.range
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerExprVariant {
    Variable {
        varname: CustomIdentifier,
        binding: Binding,
    },
    ThisValue {
        binding: Binding,
    },
    ThisField {
        field_ident: RangedCustomIdentifier,
        field_idx: usize,
        this_ty: EntityRoutePtr,
        this_binding: Binding,
        field_binding: Binding,
    },
    // EntityRoute {
    //     route: EntityRoutePtr,
    // },
    PrimitiveLiteral(PrimitiveLiteralData),
    EnumKindLiteral(EntityRoutePtr),
    Bracketed(Arc<EagerExpr>),
    Opn {
        opn_variant: EagerOpnVariant,
        opds: Vec<Arc<EagerExpr>>,
    },
    Lambda(
        Vec<(CustomIdentifier, Option<EntityRoutePtr>)>,
        Box<EagerExpr>,
    ),
    EntityFp {
        route: EntityRoutePtr,
    },
    EntityFeature {
        route: EntityRoutePtr,
    },
}

pub fn parse_eager_expr(
    db: &dyn InferQueryGroup,
    arena: &RawExprArena,
    file: FilePtr,
    raw_expr_idx: RawExprIdx,
) -> SemanticResultArc<EagerExpr> {
    EagerParser::new(db, arena, file).parse_eager_expr(raw_expr_idx)
}
