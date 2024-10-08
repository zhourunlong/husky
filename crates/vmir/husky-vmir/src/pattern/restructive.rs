use super::*;
use either::*;
use husky_hir_eager_expr::variable::runtime::HirEagerRuntimeVariableIdx;
use husky_place::place::idx::PlaceIdx;
use idx_arena::{Arena, ArenaIdx, ArenaIdxRange};

/// takes (mutable) reference of the match src, keep it
#[derive(Debug, PartialEq, Eq)]
pub enum VmirRestructivePatternData<LinketImpl: IsLinketImpl> {
    Literal,
    Some,
    Todo(LinketImpl),
    UnitPath,
}

pub type VmirRestructivePatternArena<LinketImpl> = Arena<VmirRestructivePatternData<LinketImpl>>;
pub type VmirRestructivePatternIdx<LinketImpl> = ArenaIdx<VmirRestructivePatternData<LinketImpl>>;
pub type VmirRestructivePatternIdxRange<LinketImpl> =
    ArenaIdxRange<VmirRestructivePatternData<LinketImpl>>;

#[salsa::derive_debug_with_db]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u8)]
pub enum VmirRestructivePattern<LinketImpl: IsLinketImpl> {
    Default(Option<HirEagerRuntimeVariableIdx>) = 1,
    Literal,
    UnitPath,
    OneOf(VmirRestructivePatternIdxRange<LinketImpl>),
    Other(VmirRestructivePatternIdx<LinketImpl>),
}

impl<'comptime, Linktime: IsLinktime> VmirBuilder<'comptime, Linktime> {
    pub(super) fn build_restructive_pattern(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> VmirRestructivePattern<Linktime::LinketImpl> {
        let pattern = self.build_restructive_pattern_aux(hir_eager_pattern);
        match pattern {
            Left(pattern) => pattern,
            Right(pattern) => {
                VmirRestructivePattern::Other(self.alloc_restructive_pattern(pattern))
            }
        }
    }

    pub(super) fn build_restructive_pattern_aux(
        &mut self,
        hir_eager_pattern: HirEagerPatternIdx,
    ) -> Either<
        VmirRestructivePattern<Linktime::LinketImpl>,
        VmirRestructivePatternData<Linktime::LinketImpl>,
    > {
        let hir_eager_pattern_entry = &self.hir_eager_pattern_arena()[hir_eager_pattern];
        match *hir_eager_pattern_entry.data() {
            HirEagerPatternData::Literal(_) => Left(VmirRestructivePattern::Literal),
            HirEagerPatternData::Ident {
                symbol_modifier,
                ident,
                variable_idx,
            } => Left(VmirRestructivePattern::Default(Some(variable_idx))),
            HirEagerPatternData::UnitPath(path) => Left(VmirRestructivePattern::UnitPath),
            HirEagerPatternData::Tuple { path, fields } => todo!(),
            HirEagerPatternData::Props { path, fields } => todo!(),
            HirEagerPatternData::OneOf { options } => {
                let options = options
                    .into_iter()
                    .map(|option| match self.build_restructive_pattern_aux(option) {
                        Left(pattern) => match pattern {
                            VmirRestructivePattern::Default(_) => todo!(),
                            VmirRestructivePattern::Literal => VmirRestructivePatternData::Literal,
                            VmirRestructivePattern::UnitPath => {
                                VmirRestructivePatternData::UnitPath
                            }
                            VmirRestructivePattern::OneOf(_) => todo!(),
                            VmirRestructivePattern::Other(_) => todo!(),
                        },
                        Right(pattern_data) => pattern_data,
                    })
                    .collect();
                let options = self.alloc_restructive_patterns(options);
                Left(VmirRestructivePattern::OneOf(options))
            }
            HirEagerPatternData::Binding { ident, src } => todo!(),
            HirEagerPatternData::Range { start, end } => todo!(),
            HirEagerPatternData::Some => Right(VmirRestructivePatternData::Some),
        }
    }
}

impl<LinketImpl: IsLinketImpl> VmirRestructivePattern<LinketImpl> {
    pub(crate) fn take_value<'comptime>(
        self,
        value: LinketImplThawedValue<LinketImpl>,
        ctx: &mut impl EvalVmir<'comptime, LinketImpl>,
    ) {
        match self {
            VmirRestructivePattern::Default(variable_idx) => match variable_idx {
                Some(variable_idx) => ctx.init_variable(variable_idx, value),
                None => (),
            },
            VmirRestructivePattern::OneOf(_) => todo!(),
            VmirRestructivePattern::Other(_) => todo!(),
            VmirRestructivePattern::Literal => todo!(),
            VmirRestructivePattern::UnitPath => todo!(),
        }
    }
}
