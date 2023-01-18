use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowSelfValue {
    True,
    False,
}

impl AllowSelfValue {
    fn to_bool(self) -> bool {
        match self {
            AllowSelfValue::True => true,
            AllowSelfValue::False => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum AllowSelfType {
    True,
    False,
}

impl AllowSelfType {
    fn to_bool(self) -> bool {
        match self {
            AllowSelfType::True => true,
            AllowSelfType::False => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct SymbolSheet {
    inherited_symbol_arena: InheritedSymbolArena,
    current_symbol_arena: CurrentSymbolArena,
    allow_self_type: AllowSelfType,
    allow_self_value: AllowSelfValue,
}

impl SymbolSheet {
    pub(crate) fn new(
        parent_symbol_sheet: Option<&SymbolSheet>,
        allow_self_type: AllowSelfType,
        allow_self_value: AllowSelfValue,
    ) -> Self {
        #[cfg(test)]
        {
            if allow_self_value.to_bool() {
                assert!(allow_self_type.to_bool());
            } else {
                if let Some(parent_symbol_sheet) = parent_symbol_sheet {
                    assert!(!parent_symbol_sheet.allow_self_value.to_bool());
                }
            }
            if !allow_self_type.to_bool() {
                if let Some(parent_symbol_sheet) = parent_symbol_sheet {
                    assert!(!parent_symbol_sheet.allow_self_type.to_bool());
                }
            }
        }
        Self {
            // ad hoc
            inherited_symbol_arena: match parent_symbol_sheet {
                Some(parent_symbol_sheet) => parent_symbol_sheet.bequeath(),
                None => Default::default(),
            },
            current_symbol_arena: Default::default(),
            allow_self_type,
            allow_self_value,
        }
    }

    #[inline(always)]
    pub(crate) fn define_variables(
        &mut self,
        variables: Vec<CurrentSymbol>,
    ) -> ArenaIdxRange<CurrentSymbol> {
        self.current_symbol_arena.alloc_batch(variables)
    }

    pub(crate) fn resolve_ident(&self, token_idx: TokenIdx, ident: Identifier) -> Option<Symbol> {
        self.current_symbol_arena
            .find_rev_indexed(|symbol| {
                let accessible = match symbol.access_end {
                    Some(access_end) => access_end.token_idx() > token_idx,
                    None => true,
                };
                symbol.ident == ident && accessible
            })
            .map(|(current_symbol_idx, current_symbol)| {
                Symbol::Local(current_symbol_idx, current_symbol.kind)
            })
            .or_else(|| {
                self.inherited_symbol_arena
                    .find_rev_indexed(|symbol| symbol.ident == ident)
                    .map(|(inherited_symbol_idx, inherited_symbol)| {
                        Symbol::Inherited(inherited_symbol_idx, inherited_symbol.kind)
                    })
            })
    }

    pub fn indexed_inherited_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (InheritedSymbolIdx, &'a InheritedSymbol)> + 'a {
        self.inherited_symbol_arena.indexed_iter()
    }

    pub fn indexed_current_symbol_iter<'a>(
        &'a self,
    ) -> impl Iterator<Item = (CurrentSymbolIdx, &'a CurrentSymbol)> + 'a {
        self.current_symbol_arena.indexed_iter()
    }

    fn bequeath(&self) -> InheritedSymbolArena {
        let mut inherited_symbol_arena = InheritedSymbolArena::default();
        for _ in self.indexed_inherited_symbol_iter() {
            todo!()
        }
        for (original_current_symbol_idx, current_symbol) in self.indexed_current_symbol_iter() {
            inherited_symbol_arena.alloc_one(match current_symbol.kind {
                CurrentSymbolKind::Parameter { .. } => InheritedSymbol {
                    ident: current_symbol.ident,
                    kind: InheritedSymbolKind::Parameter {
                        original_current_symbol_idx,
                    },
                },
                CurrentSymbolKind::LetVariable { .. } => todo!(),
                CurrentSymbolKind::FrameVariable(_) => todo!(),
            });
        }
        inherited_symbol_arena
    }

    pub fn allow_self_type(&self) -> AllowSelfType {
        self.allow_self_type
    }

    pub fn allow_self_value(&self) -> AllowSelfValue {
        self.allow_self_value
    }
}

impl std::ops::Index<InheritedSymbolIdx> for SymbolSheet {
    type Output = InheritedSymbol;

    fn index(&self, index: InheritedSymbolIdx) -> &Self::Output {
        &self.inherited_symbol_arena[index]
    }
}

impl std::ops::Index<CurrentSymbolIdx> for SymbolSheet {
    type Output = CurrentSymbol;

    fn index(&self, index: CurrentSymbolIdx) -> &Self::Output {
        &self.current_symbol_arena[index]
    }
}

pub enum Prevariable {}
