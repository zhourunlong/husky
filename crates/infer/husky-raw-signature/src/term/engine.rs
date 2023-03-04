use super::*;
use husky_expr::*;
use husky_opn_syntax::{BinaryOpr, PrefixOpr};
use husky_print_utils::p;
use salsa::DebugWithDb;

pub(super) struct RawSignatureRawTermEngine<'a> {
    db: &'a dyn RawSignatureDb,
    expr_region_data: &'a ExprRegionData,
    raw_term_menu: &'a RawTermMenu,
    expr_terms: ExprMap<SignatureRawTermResult<RawTerm>>,
    term_symbol_region: RawTermSymbolRegion,
}

#[derive(Debug, PartialEq, Eq)]
pub struct RawTermSymbolRegion {
    registry: RawTermSymbolRegistry,
    inherited_symbol_terms: Vec<RawTermSymbol>,
    current_symbol_terms: Vec<RawTermSymbol>,
}

impl RawTermSymbolRegion {
    fn new(parent: Option<&RawTermSymbolRegion>, symbol_region: &SymbolRegion) -> Self {
        let registry = parent.map_or(Default::default(), |parent| parent.registry.clone());
        let inherited_symbol_terms = symbol_region
            .inherited_symbol_iter()
            .map(|symbol| {
                parent
                    .unwrap()
                    .parent_symbol_term(symbol.parent_symbol_idx())
            })
            .collect();
        Self {
            registry,
            inherited_symbol_terms,
            current_symbol_terms: Default::default(),
        }
    }

    fn parent_symbol_term(&self, parent_symbol_idx: ParentSymbolIdx) -> RawTermSymbol {
        match parent_symbol_idx {
            ParentSymbolIdx::Inherited(inherited_symbol_idx) => {
                self.inherited_symbol_term(inherited_symbol_idx)
            }
            ParentSymbolIdx::Current(current_symbol_idx) => {
                self.current_symbol_term(current_symbol_idx).unwrap()
            }
        }
    }

    pub fn inherited_symbol_term(&self, inherited_symbol_idx: InheritedSymbolIdx) -> RawTermSymbol {
        self.inherited_symbol_terms[inherited_symbol_idx.raw()]
    }

    pub fn current_symbol_term(
        &self,
        current_symbol_idx: CurrentSymbolIdx,
    ) -> Option<RawTermSymbol> {
        self.current_symbol_terms
            .get(current_symbol_idx.raw())
            .copied()
    }
}

impl<'a> RawSignatureRawTermEngine<'a> {
    pub(super) fn new(
        db: &'a dyn RawSignatureDb,
        expr_region: ExprRegion,
        parent_term_symbol_region: Option<&'a RawTermSymbolRegion>,
    ) -> Self {
        let toolchain = expr_region.toolchain(db);
        // ad hoc
        let raw_term_menu = db.raw_term_menu(toolchain).as_ref().unwrap();
        let expr_region_data = &expr_region.data(db);
        let mut this = Self {
            db,
            expr_region_data,
            raw_term_menu,
            expr_terms: ExprMap::new(expr_region_data.expr_arena()),
            term_symbol_region: RawTermSymbolRegion::new(
                parent_term_symbol_region,
                expr_region_data.symbol_region(),
            ),
        };
        this.init_current_symbol_term_symbols();
        this.init_expr_roots();
        this
    }

    fn init_current_symbol_term_symbols(&mut self) {
        for (idx, symbol) in self
            .expr_region_data
            .symbol_region()
            .indexed_current_symbol_iter()
        {
            let ty = match symbol.variant() {
                CurrentSymbolVariant::ImplicitParameter {
                    implicit_parameter_variant,
                } => match implicit_parameter_variant {
                    CurrentImplicitParameterSymbol::Type { .. } => {
                        Ok(self.raw_term_menu.ty0().into())
                    }
                    CurrentImplicitParameterSymbol::Lifetime { .. } => {
                        Ok(self.raw_term_menu.lifetime_ty().into())
                    }
                    _ => todo!(),
                },
                CurrentSymbolVariant::RegularParameter {
                    pattern_symbol_idx, ..
                } => {
                    let pattern_symbol =
                        &self.expr_region_data.pattern_expr_region()[*pattern_symbol_idx];
                    match pattern_symbol {
                        PatternSymbol::Atom(pattern) => {
                            let ty = self
                                .expr_region_data
                                .symbol_region()
                                .regular_parameter_pattern_ty_constraint(*pattern)
                                .unwrap();
                            match self.infer_new(ty) {
                                Ok(ty) => Ok(ty),
                                Err(_) => Err(RawTermSymbolTypeErrorKind::SignatureRawTermError),
                            }
                        }
                    }
                }
                CurrentSymbolVariant::LetVariable { .. }
                | CurrentSymbolVariant::FrameVariable { .. } => return,
            };
            self.term_symbol_region
                .current_symbol_terms
                .push(self.term_symbol_region.registry.new_symbol(self.db, ty))
        }
    }

    fn init_expr_roots(&mut self) {
        for expr_root in self.expr_region_data.roots() {
            match expr_root.kind() {
                ExprRootKind::BlockExpr => return,
                ExprRootKind::SelfType
                | ExprRootKind::Trait
                | ExprRootKind::ReturnType
                | ExprRootKind::FieldType => (),
            }
            self.cache_new(expr_root.expr())
        }
    }

    // infer the term for expr, assuming it hasn't been computed before
    fn infer_new(&mut self, expr_idx: ExprIdx) -> SignatureRawTermResult<RawTerm> {
        let result = self.calc(expr_idx);
        let result_export = match result {
            Ok(term) => Ok(term),
            Err(_) => Err(DerivedSignatureRawTermError::RawTermAbortion.into()),
        };
        self.save(expr_idx, result);
        result_export
    }

    // cache the term for expr, assuming it hasn't been computed before
    fn cache_new(&mut self, expr_idx: ExprIdx) {
        let result = self.calc(expr_idx);
        self.save(expr_idx, result)
    }

    pub(crate) fn finish(self) -> RawSignatureRawTermRegion {
        RawSignatureRawTermRegion::new(
            self.expr_region_data.path(),
            self.term_symbol_region,
            self.expr_terms,
        )
    }

    fn save(&mut self, expr_idx: ExprIdx, outcome: SignatureRawTermResult<RawTerm>) {
        self.expr_terms.insert_new(expr_idx, outcome)
    }

    fn calc(&mut self, expr_idx: ExprIdx) -> SignatureRawTermResult<RawTerm> {
        match self.expr_region_data.expr_arena()[expr_idx] {
            Expr::Literal(_) => todo!(),
            Expr::EntityPath {
                entity_path_expr,
                path: entity_path,
            } => match entity_path {
                Some(entity_path) => Ok(RawTerm::EntityPath(match entity_path {
                    EntityPath::Module(_) => todo!(),
                    EntityPath::ModuleItem(path) => match path {
                        ModuleItemPath::Type(path) => {
                            /* ad hoc */
                            RawTermEntityPath::TypeOntology(path)
                        }
                        ModuleItemPath::Trait(path) => path.into(),
                        ModuleItemPath::Form(path) => path.into(),
                    },
                    EntityPath::AssociatedItem(_) => todo!(),
                    EntityPath::Variant(_) => todo!(),
                })),
                None => Err(DerivedSignatureRawTermError::InvalidEntityPath.into()),
            },
            Expr::InheritedSymbol {
                ident,
                token_idx,
                inherited_symbol_idx,
                inherited_symbol_kind,
            } => todo!(),
            Expr::CurrentSymbol {
                current_symbol_idx, ..
            } => Ok(self
                .term_symbol_region
                .current_symbol_term(current_symbol_idx)
                .expect("not none")
                .into()),
            Expr::FrameVarDecl {
                token_idx,
                ident,
                frame_var_symbol_idx: current_symbol_idx,
                current_symbol_kind,
            } => todo!(),
            Expr::SelfType(_) => todo!(),
            Expr::SelfValue(_) => todo!(),
            Expr::Binary {
                lopd, opr, ropd, ..
            } => {
                let  Ok(lopd) = self.infer_new(lopd) else {
                    return  Err(DerivedSignatureRawTermError::CannotInferOperandRawTermInPrefix.into());
                };
                let  Ok(ropd) = self.infer_new(ropd) else {
                    return  Err(DerivedSignatureRawTermError::CannotInferOperandRawTermInPrefix.into());
                };
                match opr {
                    BinaryOpr::PureClosed(_) => todo!(),
                    BinaryOpr::Comparison(_) => todo!(),
                    BinaryOpr::ShortCircuitLogic(_) => todo!(),
                    BinaryOpr::Assign(_) => todo!(),
                    BinaryOpr::ScopeResolution => todo!(),
                    BinaryOpr::Curry => Ok(RawTermCurry::new(
                        self.db,
                        RawTermCurryKind::Explicit, // ad hoc
                        Variance::Invariant,        // ad hoc
                        None,                       // ad hoc
                        lopd,
                        ropd,
                    )
                    .into()),
                    BinaryOpr::As => todo!(),
                    BinaryOpr::Ins => todo!(),
                    BinaryOpr::In => todo!(),
                }
            }
            Expr::Be { .. } => todo!(),
            Expr::Prefix {
                opr,
                opr_token_idx,
                opd,
            } => {
                let  Ok(opd) = self.infer_new(opd) else {
                    return  Err(DerivedSignatureRawTermError::CannotInferOperandRawTermInPrefix.into());
                };
                let tmpl = match opr {
                    PrefixOpr::Minus => todo!(),
                    PrefixOpr::Not => todo!(),
                    PrefixOpr::BitNot => todo!(),
                    PrefixOpr::Ref => self.raw_term_menu.eval_ref_ty(),
                    PrefixOpr::Vector => todo!(),
                    PrefixOpr::Slice => todo!(),
                    PrefixOpr::CyclicSlice => todo!(),
                    PrefixOpr::Array(_) => todo!(),
                    PrefixOpr::Option => self.raw_term_menu.option_ty_path(),
                };
                Ok(RawTermApplication::new(self.db, tmpl, opd, /* ad hoc */ 0).into())
            }
            Expr::Suffix {
                opd,
                opr: punctuation,
                opr_token_idx: punctuation_token_idx,
            } => todo!(),
            Expr::Field {
                owner: self_expr,
                dot_token_idx,
                ident_token,
            } => todo!(),
            Expr::MethodCall { .. } => todo!(),
            Expr::TemplateInstantiation { .. } => todo!(),
            Expr::ExplicitApplicationOrRitchieCall { function, .. } => todo!(),
            Expr::ExplicitApplicationOrComposition { function, argument } => {
                let  Ok(argument) = self.infer_new(argument) else {
                        return  Err(DerivedSignatureRawTermError::CannotInferArgumentRawTermInApplication.into())
                    };
                match self.expr_region_data.expr_arena()[function] {
                    Expr::BoxColonList { items, .. } => match items.len() {
                        0 => Ok(RawTermApplication::new(
                            self.db,
                            self.raw_term_menu.slice_ty_path(),
                            argument,
                            /* ad hoc */ 0,
                        )
                        .into()),
                        _ => todo!(),
                    },
                    Expr::List { items, .. } => match items.len() {
                        0 => Ok(RawTermApplication::new(
                            self.db,
                            self.raw_term_menu.list(),
                            argument,
                            /* ad hoc */ 0,
                        )
                        .into()),
                        1 => match self.expr_region_data.expr_arena()[items.start()] {
                            Expr::Literal(_) => todo!(),
                            Expr::EntityPath {
                                entity_path_expr,
                                path: entity_path,
                            } => todo!(),
                            Expr::InheritedSymbol {
                                ident,
                                token_idx,
                                inherited_symbol_idx,
                                inherited_symbol_kind,
                            } => todo!(),
                            Expr::CurrentSymbol {
                                ident,
                                token_idx,
                                current_symbol_idx,
                                current_symbol_kind,
                            } => todo!(),
                            Expr::FrameVarDecl {
                                token_idx,
                                ident,
                                frame_var_symbol_idx: current_symbol_idx,
                                current_symbol_kind,
                            } => todo!(),
                            Expr::SelfType(_) => todo!(),
                            Expr::SelfValue(_) => todo!(),
                            Expr::Binary {
                                lopd,
                                opr,
                                opr_token_idx,
                                ropd,
                            } => todo!(),
                            Expr::Be {
                                src,
                                be_token_idx,
                                ref target,
                            } => todo!(),
                            Expr::Prefix {
                                opr,
                                opr_token_idx,
                                opd,
                            } => todo!(),
                            Expr::Suffix {
                                opd,
                                opr: punctuation,
                                opr_token_idx: punctuation_token_idx,
                            } => todo!(),
                            Expr::ExplicitApplicationOrRitchieCall { .. } => todo!(),
                            Expr::Field {
                                owner: self_expr,
                                dot_token_idx,
                                ident_token,
                            } => todo!(),
                            Expr::MethodCall {
                                self_argument: self_expr,
                                dot_token_idx,
                                ident_token,
                                ref implicit_arguments,
                                lpar_token_idx,
                                nonself_arguments: arguments,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::TemplateInstantiation {
                                template,
                                ref implicit_arguments,
                            } => todo!(),
                            Expr::ExplicitApplicationOrComposition { function, argument } => {
                                todo!()
                            }
                            Expr::Bracketed {
                                lpar_token_idx,
                                item,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::NewTuple {
                                lpar_token_idx,
                                items,
                                ref commas,
                                rpar_token_idx,
                            } => todo!(),
                            Expr::List { .. } => todo!(),
                            Expr::BoxColonList { .. } => todo!(),
                            Expr::Block { stmts } => todo!(),
                            Expr::Err(_) => Err(
                                DerivedSignatureRawTermError::CannotInferArgumentRawTermInBoxList
                                    .into(),
                            ),
                            Expr::IndexOrCompositionWithList { .. } => todo!(),
                        },
                        _ => todo!(),
                    },
                    _ => {
                        let  Ok(function) = self.infer_new(function) else {
                            return  Err(DerivedSignatureRawTermError::CannotInferFunctionRawTermInApplication.into())
                        };
                        todo!()
                    }
                }
            }
            Expr::NewTuple {
                lpar_token_idx,
                items,
                rpar_token_idx,
                ..
            } => {
                p!(self.expr_region_data.path().debug(self.db));
                p!(items.len());
                todo!()
            }
            Expr::List { .. } => todo!(),
            Expr::BoxColonList { .. } => todo!(),
            Expr::Bracketed { item, .. } => self.infer_new(item),
            Expr::Block { stmts } => todo!(),
            Expr::IndexOrCompositionWithList {
                owner,
                lbox_token_idx,
                items: indices,
                rbox_token_idx,
            } => todo!(),
            Expr::Err(_) => Err(DerivedSignatureRawTermError::ExprError.into()),
        }
    }

    pub(crate) fn current_symbol_term_symbol(&self, symbol: CurrentSymbolIdx) -> RawTermSymbol {
        self.term_symbol_region.current_symbol_terms[symbol.raw()]
    }

    pub(crate) fn raw_term_menu(&self) -> &RawTermMenu {
        self.raw_term_menu
    }
}