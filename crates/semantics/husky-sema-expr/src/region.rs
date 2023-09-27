use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = SemaExprDb)]
pub struct SemaExprRegion {
    path: RegionPath,
    sema_expr_arena: SemaExprArena,
    pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
    pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
    expr_fluffy_terms: SynExprMap<SemaExprTermResult<FluffyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FluffyTerm>,
    fluffy_term_region: FluffyTermRegion,
    return_ty: Option<EtherealTerm>,
    self_ty: Option<EtherealTerm>,
}

impl SemaExprRegion {
    pub(crate) fn new(
        db: &dyn SemaExprDb,
        path: RegionPath,
        sema_expr_arena: SemaExprArena,
        pattern_expr_ty_infos: SynPatternExprMap<PatternExprTypeInfo>,
        pattern_symbol_ty_infos: SynPatternSymbolMap<PatternSymbolTypeInfo>,
        expr_fluffy_terms: SynExprMap<SemaExprTermResult<FluffyTerm>>,
        symbol_terms: SymbolMap<FluffyTerm>,
        symbol_tys: SymbolMap<SymbolType>,
        fluffy_term_region: FluffyTermRegion,
        return_ty: Option<EtherealTerm>,
        self_ty: Option<EtherealTerm>,
    ) -> Self {
        Self {
            path,
            sema_expr_arena,
            pattern_expr_ty_infos,
            pattern_symbol_ty_infos,
            expr_fluffy_terms,
            symbol_tys,
            symbol_terms,
            fluffy_term_region,
            return_ty,
            self_ty,
        }
    }

    pub fn path(&self) -> RegionPath {
        self.path
    }

    pub fn expr_fluffy_terms(&self) -> &SynExprMap<SemaExprTermResult<FluffyTerm>> {
        &self.expr_fluffy_terms
    }

    pub fn expr_fluffy_term(
        &self,
        syn_expr_idx: SynExprIdx,
    ) -> Option<SemaExprTermResultRef<FluffyTerm>> {
        Some(self.expr_fluffy_terms.get(syn_expr_idx)?.as_ref().copied())
    }

    pub fn symbol_terms(&self) -> &SymbolMap<FluffyTerm> {
        &self.symbol_terms
    }

    pub fn symbol_tys(&self) -> &SymbolMap<SymbolType> {
        &self.symbol_tys
    }

    pub fn fluffy_term_region(&self) -> &FluffyTermRegion {
        &self.fluffy_term_region
    }

    pub fn return_ty(&self) -> Option<EtherealTerm> {
        self.return_ty
    }

    pub fn self_ty(&self) -> Option<EtherealTerm> {
        self.self_ty
    }
}

#[salsa::tracked(jar = SemaExprJar, return_ref)]
pub(crate) fn expr_ty_region(
    db: &dyn SemaExprDb,
    syn_expr_region: SynExprRegion,
) -> SemaExprRegion {
    let mut engine = ExprTypeEngine::new(db, syn_expr_region);
    engine.infer_all();
    engine.finish()
}

#[derive(Debug, PartialEq, Eq)]
#[salsa::debug_with_db(db = FluffyTermDb)]
pub(crate) struct PatternExprTypeInfo {
    ty: PatternSemaExprResult<FluffyTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternSemaExprResult<FluffyTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&FluffyTerm, &PatternSemaExprError> {
        self.ty.as_ref()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<FluffyTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<FluffyTerm>) -> Self {
        Self { ty }
    }
}