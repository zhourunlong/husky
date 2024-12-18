use crate::{jar::*, *};

/// this is interned on purpose
#[salsa::interned(db = HirLazyExprDb, jar = HirLazyExprJar)]
pub struct HirLazyExprRegion {
    #[return_ref]
    pub hir_lazy_expr_arena: HirLazyExprArena,
    #[return_ref]
    pub hir_lazy_stmt_arena: HirLazyStmtArena,
    #[return_ref]
    pub hir_lazy_pattern_arena: HirLazyPatternArena,
    #[return_ref]
    pub hir_lazy_variable_region: HirLazyVariableRegion,
}

impl HirLazyExprRegion {
    pub fn data<'a>(self, db: &'a ::salsa::Db) -> HirLazyExprRegionData<'a> {
        HirLazyExprRegionData {
            hir_lazy_expr_arena: self.hir_lazy_expr_arena(db).as_arena_ref(),
            hir_lazy_stmt_arena: self.hir_lazy_stmt_arena(db).as_arena_ref(),
            hir_lazy_pattern_arena: self.hir_lazy_pattern_arena(db).as_arena_ref(),
            hir_lazy_variable_region: self.hir_lazy_variable_region(db),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct HirLazyExprRegionData<'a> {
    hir_lazy_expr_arena: HirLazyExprArenaRef<'a>,
    hir_lazy_stmt_arena: HirLazyStmtArenaRef<'a>,
    hir_lazy_pattern_arena: HirLazyPatternArenaRef<'a>,
    hir_lazy_variable_region: &'a HirLazyVariableRegion,
}

impl<'a> HirLazyExprRegionData<'a> {
    #[inline(always)]
    pub fn hir_lazy_expr_arena(self) -> HirLazyExprArenaRef<'a> {
        self.hir_lazy_expr_arena
    }

    #[inline(always)]
    pub fn hir_lazy_stmt_arena(self) -> HirLazyStmtArenaRef<'a> {
        self.hir_lazy_stmt_arena
    }

    #[inline(always)]
    pub fn hir_lazy_pattern_arena(self) -> HirLazyPatternArenaRef<'a> {
        self.hir_lazy_pattern_arena
    }

    pub fn hir_lazy_variable_region(self) -> &'a HirLazyVariableRegion {
        self.hir_lazy_variable_region
    }
}
