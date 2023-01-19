use crate::*;
use husky_vfs::{ModulePath, Toolchain, VfsTestUtils};

#[salsa::tracked(jar = ExprJar)]
pub struct ExprPage {
    pub path: ExprPath,
    #[return_ref]
    pub expr_arena: ExprArena,
    #[return_ref]
    pub entity_path_expr_arena: EntityPathExprArena,
    #[return_ref]
    pub stmt_arena: StmtArena,
    #[return_ref]
    pub pattern_expr_page: PatternExprPage,
    #[return_ref]
    pub symbol_page: SymbolPage,
}

impl ExprPage {
    pub fn toolchain(self, db: &dyn ExprDb) -> Toolchain {
        // ad hoc
        match self.path(db) {
            ExprPath::Snippet(toolchain) => toolchain,
            ExprPath::Decl(path) => match path {
                DeclExprPath::Entity(path) => path.toolchain(db),
                DeclExprPath::ImplBlock(impl_block) => impl_block.module_path(db).toolchain(db),
                DeclExprPath::AssociatedItem(item) => item.module_path(db).toolchain(db),
            },
            ExprPath::Defn(path) => match path {
                DefnExprPath::Entity(path) => path.toolchain(db),
                DefnExprPath::AssociatedItem(item) => item.module_path(db).toolchain(db),
            },
        }
    }
}