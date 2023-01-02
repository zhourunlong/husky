use crate::*;
use husky_ast::{Ast, AstIdx, AstIdxRange, AstSheet};
use husky_entity_path::EntityPath;
use husky_entity_taxonomy::{EntityKind, FormKind, ModuleItemKind, TypeKind};
use husky_entity_tree::{CratePrelude, EntitySymbol, EntityTreeSheet};
use husky_expr::{parse_expr, ExprSheet};
use husky_opn_syntax::BinaryPunctuation;
use husky_print_utils::p;
use husky_symbol::{LocalSymbolSheet, SymbolContext};
use husky_token::{
    IdentifierToken, LeftAngleBracketToken, LeftBoxBracketToken, LeftCurlyBraceToken, Punctuation,
    RightCurlyBraceToken, TokenGroupIdx, TokenIdx, TokenSheet,
};
use parsec::{parse_separated_list, ParseContext, ParseFrom};
use salsa::DebugWithDb;
use vec_like::VecPairMap;

pub(crate) struct DeclCollector<'a> {
    db: &'a dyn DeclDb,
    crate_prelude: CratePrelude<'a>,
    token_sheet: &'a TokenSheet,
    ast_sheet: &'a AstSheet,
    entity_tree_sheet: &'a EntityTreeSheet,
}

impl<'a> DeclCollector<'a> {
    pub(crate) fn new(db: &'a dyn DeclDb, module_path: ModulePath) -> EntityTreeResult<Self> {
        let crate_prelude = db.crate_prelude(module_path.crate_path(db))?;
        Ok(Self {
            db,
            crate_prelude,
            token_sheet: db.token_sheet(module_path)?,
            ast_sheet: db.ast_sheet(module_path)?,
            entity_tree_sheet: db.entity_tree_sheet(module_path)?,
        })
    }

    pub(crate) fn collect_all(mut self) -> DeclSheet {
        let mut decls: VecPairMap<EntityPath, DeclResult<Decl>> = Default::default();
        for entity_symbol in self.entity_tree_sheet.module_symbols().iter() {
            match entity_symbol {
                EntitySymbol::CrateRoot { .. } => unreachable!(),
                EntitySymbol::Submodule { .. } | EntitySymbol::EntityUse { .. } => (),
                EntitySymbol::ModuleItem {
                    ident,
                    accessibility,
                    path,
                    ast_idx,
                } => decls.insert(((*path).into(), self.parse_decl(*ast_idx, (*path).into()))),
            }
        }
        for associated_item in self.entity_tree_sheet.associated_items().iter() {
            todo!()
        }
        DeclSheet::new(decls)
    }

    fn parse_decl(&mut self, ast_idx: AstIdx, entity_path: EntityPath) -> DeclResult<Decl> {
        match self.ast_sheet[ast_idx] {
            Ast::Defn {
                token_group_idx,
                ref body,
                accessibility,
                entity_kind,
                entity_path: _,
                is_generic,
                body_kind,
                saved_stream_state,
                ..
            } => match entity_path {
                EntityPath::Module(_) => todo!(),
                EntityPath::ModuleItem(path) => match path {
                    ModuleItemPath::Type(path) => self.parse_ty_decl(
                        ast_idx,
                        path.type_kind(self.db),
                        path,
                        entity_kind,
                        token_group_idx,
                        body,
                        saved_stream_state,
                    ),
                    ModuleItemPath::Trait(path) => self.parse_trai_decl(ast_idx, path),
                    ModuleItemPath::Form(path) => self.parse_form_decl(
                        ast_idx,
                        path,
                        entity_kind,
                        token_group_idx,
                        body,
                        saved_stream_state,
                    ),
                },
                EntityPath::GenericParameter(_) => todo!(),
                EntityPath::AssociatedItem(_) => todo!(),
                EntityPath::Variant(_) => todo!(),
            },
            Ast::Impl { .. }
            | Ast::Err { .. }
            | Ast::Use { .. }
            | Ast::Comment { .. }
            | Ast::Decor { .. }
            | Ast::Stmt { .. }
            | Ast::IfElseStmts { .. }
            | Ast::MatchStmts { .. }
            | Ast::ModuleItemVariant { .. }
            | Ast::Main { .. }
            | Ast::Config { .. } => unreachable!(),
        }
    }

    fn parse_ty_decl(
        &mut self,
        ast_idx: AstIdx,
        type_kind: TypeKind,
        path: TypePath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> Result<Decl, DeclError> {
        match type_kind {
            TypeKind::Enum => self.parse_enum_type_decl(ast_idx, path),
            TypeKind::Inductive => self.parse_inductive_type_decl(ast_idx, path),
            TypeKind::Record => todo!(),
            TypeKind::Struct => self.parse_struct_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
            TypeKind::Structure => self.parse_structure_type_decl(ast_idx, path),
            TypeKind::Foreign => self.parse_foreign_type_decl(
                ast_idx,
                path,
                token_group_idx,
                body,
                saved_stream_state,
            ),
        }
    }

    fn parse_enum_type_decl(&self, ast_idx: AstIdx, path: TypePath) -> DeclResult<Decl> {
        Ok(Decl::Type(EnumTypeDecl::new(self.db, path, ast_idx).into()))
    }

    fn parse_trai_decl(&self, ast_idx: AstIdx, path: TraitPath) -> DeclResult<Decl> {
        Ok(Decl::Trait(TraitDecl::new(self.db, path, ast_idx)))
    }

    fn parse_inductive_type_decl(&self, ast_idx: AstIdx, path: TypePath) -> DeclResult<Decl> {
        Ok(Decl::Type(
            InductiveTypeDecl::new(self.db, path, ast_idx).into(),
        ))
    }

    fn parse_struct_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut sheet = ExprSheet::default();
        let mut local_symbol_sheet = LocalSymbolSheet::default();
        let mut parser = self.expr_parser(
            path.into(),
            token_group_idx,
            saved_stream_state,
            &mut sheet,
            &mut local_symbol_sheet,
        );
        let implicit_parameters = parser.parse()?;
        if let Some(lcurl) = parser.parse::<LeftCurlyBraceToken>()? {
            let (fields, separators) = parse_separated_list(&mut parser)?;
            let rcurl: RightCurlyBraceToken = parser.parse_expected()?;
            Ok(Decl::Type(
                PropsStructTypeDecl::new(
                    self.db,
                    path,
                    ast_idx,
                    implicit_parameters,
                    lcurl,
                    fields,
                    separators,
                    rcurl,
                )
                .into(),
            ))
        } else if let Some(lbox) = parser.parse::<LeftBoxBracketToken>()? {
            todo!()
        } else {
            Err(DeclError::ExpectLCurlOrLParOrSemicolon(parser.save_state()))
        }
    }

    fn expr_parser<'b, 'c>(
        &self,
        entity_path: EntityPath,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        sheet: &'b mut ExprSheet,
        local_symbol_sheet: &'c mut LocalSymbolSheet,
    ) -> ExprParser<'b, 'a, 'c>
    where
        'a: 'c,
    {
        let ctx = SymbolContext::new(self.db, self.crate_prelude, local_symbol_sheet);
        ExprParser::new(
            ctx,
            self.token_sheet
                .token_group_token_stream(token_group_idx, Some(saved_stream_state)),
            sheet,
        )
    }

    fn parse_structure_type_decl(&self, ast_idx: AstIdx, path: TypePath) -> DeclResult<Decl> {
        Ok(Decl::Type(
            StructureTypeDecl::new(self.db, path, ast_idx).into(),
        ))
    }

    // get declaration from tokens
    fn parse_foreign_type_decl(
        &self,
        ast_idx: AstIdx,
        path: TypePath,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> DeclResult<Decl> {
        let mut token_iter = self
            .token_sheet
            .token_group_token_stream(token_group_idx, Some(saved_stream_state));
        let mut expr_arena = ExprSheet::default();
        let local_symbol_sheet = LocalSymbolSheet::default();
        // if let Some(_) = token_iter.try_eat_special(BinaryOpr::Assign(None).into(), true) {
        //     todo!()
        // } else {
        //     match token_iter.try_eat_special(SpecialToken::Semicolon, true) {
        //         Some(_) => {
        //             if !token_iter.is_empty() {
        //                 todo!()
        //             }
        //             todo!()
        //         }
        //         None => todo!(),
        //     }
        // }
        Ok(Decl::Type(
            AlienTypeDecl::new(self.db, path, ast_idx).into(),
        ))
    }

    fn parse_form_decl(
        &mut self,
        ast_idx: AstIdx,
        path: FormPath,
        entity_kind: EntityKind,
        token_group_idx: TokenGroupIdx,
        body: &AstIdxRange,
        saved_stream_state: TokenIdx,
    ) -> Result<Decl, DeclError> {
        match path.form_kind(self.db) {
            FormKind::Feature => self.parse_feature_decl(ast_idx, path),
            FormKind::Function => {
                self.parse_function_decl(ast_idx, token_group_idx, saved_stream_state, path)
            }
            FormKind::Value => todo!(),
            FormKind::TypeAlias => todo!(),
        }
    }

    fn parse_feature_decl(&self, ast_idx: AstIdx, path: FormPath) -> Result<Decl, DeclError> {
        Ok(Decl::Form(FeatureDecl::new(self.db, path, ast_idx).into()))
    }

    fn parse_function_decl(
        &self,
        ast_idx: AstIdx,
        token_group_idx: TokenGroupIdx,
        saved_stream_state: TokenIdx,
        path: FormPath,
    ) -> Result<Decl, DeclError> {
        let mut sheet = ExprSheet::default();
        let mut local_symbol_sheet = LocalSymbolSheet::default();
        let mut parser = self.expr_parser(
            path.into(),
            token_group_idx,
            saved_stream_state,
            &mut sheet,
            &mut local_symbol_sheet,
        );
        let implicit_parameters = parser.parse()?;
        Ok(Decl::Form(
            FunctionDecl::new(self.db, path, ast_idx, implicit_parameters).into(),
        ))
    }
}
