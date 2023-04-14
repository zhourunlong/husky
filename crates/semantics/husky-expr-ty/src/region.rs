use husky_print_utils::p;

use crate::*;

#[derive(Debug, PartialEq, Eq)]
#[salsa::derive_debug_with_db(db = ExprTypeDb)]
pub struct ExprTypeRegion {
    path: RegionPath,
    expr_ty_infos: ExprMap<ExprTypeInfo>,
    extra_expr_errors: Vec<(ExprIdx, ExprTypeError)>,
    expr_fluffy_terms: ExprMap<ExprTermResult<FluffyTerm>>,
    symbol_tys: SymbolMap<SymbolType>,
    symbol_terms: SymbolMap<FluffyTerm>,
    fluffy_term_region: FluffyTermRegion,
    return_ty: Option<Term>,
    self_ty: Option<Term>,
}

impl ExprTypeRegion {
    pub(crate) fn new(
        db: &dyn ExprTypeDb,
        path: RegionPath,
        expr_ty_infos: ExprMap<ExprTypeInfo>,
        extra_expr_errors: Vec<(ExprIdx, ExprTypeError)>,
        expr_fluffy_terms: ExprMap<ExprTermResult<FluffyTerm>>,
        symbol_terms: SymbolMap<FluffyTerm>,
        symbol_tys: SymbolMap<SymbolType>,
        fluffy_term_region: FluffyTermRegion,
        return_ty: Option<Term>,
        self_ty: Option<Term>,
    ) -> Self {
        Self {
            path,
            expr_ty_infos,
            extra_expr_errors,
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

    pub fn expr_ty_infos(&self) -> &ExprMap<ExprTypeInfo> {
        &self.expr_ty_infos
    }

    pub fn extra_expr_ty_errors(&self) -> &[(ExprIdx, ExprTypeError)] {
        &self.extra_expr_errors
    }

    pub fn expr_fluffy_terms(&self) -> &ExprMap<ExprTermResult<FluffyTerm>> {
        &self.expr_fluffy_terms
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

    pub fn return_ty(&self) -> Option<Term> {
        self.return_ty
    }

    pub fn self_ty(&self) -> Option<Term> {
        self.self_ty
    }

    pub fn expr_disambiguation(
        &self,
        expr_idx: ExprIdx,
    ) -> Option<ExprTypeResultRef<ExprDisambiguation>> {
        // ad hoc
        // todo: change this to always some
        self.expr_ty_infos
            .get(expr_idx)
            .map(|ty_info| ty_info.disambiguation())
    }
}

#[salsa::tracked(jar = ExprTypeJar, return_ref)]
pub(crate) fn expr_ty_region(db: &dyn ExprTypeDb, expr_region: ExprRegion) -> ExprTypeRegion {
    let mut engine = ExprTypeEngine::new(db, expr_region);
    engine.infer_all();
    engine.finish()
}

pub(crate) struct PatternExprTypeInfo {
    ty: PatternExprTypeResult<FluffyTerm>,
}

impl PatternExprTypeInfo {
    pub(crate) fn new(ty: PatternExprTypeResult<FluffyTerm>) -> Self {
        Self { ty }
    }

    pub(crate) fn ty(&self) -> Result<&FluffyTerm, &PatternExprTypeError> {
        self.ty.as_ref()
    }
}

pub(crate) struct PatternSymbolTypeInfo {
    ty: PatternSymbolTypeResult<FluffyTerm>,
}

impl PatternSymbolTypeInfo {
    pub(crate) fn new(ty: PatternSymbolTypeResult<FluffyTerm>) -> Self {
        Self { ty }
    }
}