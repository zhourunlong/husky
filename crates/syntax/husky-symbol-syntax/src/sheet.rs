use crate::*;
use husky_identifier::Identifier;
use husky_recursive_symbol::RecursiveSymbolSheet;

#[derive(Default)]
pub struct SymbolSheet {
    recursive_symbols: RecursiveSymbolSheet,
}

impl SymbolSheet {
    pub fn define_symbol(&mut self, symbol: Symbol) {
        todo!()
        // self.symbols.push(symbol)
    }

    pub fn resolve_ident(&self, ident: Identifier) -> Symbol {
        todo!()
        // if let Some(symbol) = self.symbols.find_last(|symbol| symbol.ident == ident) {
        //     *symbol
        // } else {
        //     // ad hoc
        //     Symbol {
        //         ident,
        //         kind: SymbolKind::Unrecognized,
        //     }
        // }
    }

    fn try1() {
        let haha = Haha::default();

        #[derive(Default)]
        struct Haha;
    }
}
