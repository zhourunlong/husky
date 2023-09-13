use husky_entity_syn_tree::OnceUseRuleState;
use husky_entity_taxonomy::{EntityKind, MajorItemConnectionKind, MajorItemKind, TypeKind};
use husky_syn_expr::{CurrentSynSymbolKind, InheritedSynSymbolKind};

use crate::*;

pub(crate) fn collect_semantic_tokens(
    db: &dyn SemanticTokenDb,
    module_path: ModulePath,
) -> EntitySynTreeResult<Vec<RangedSemanticToken>> {
    let ranged_token_sheet = db.ranged_token_sheet(module_path)?;
    let _token_sheet_data = db.token_sheet_data(module_path)?;
    let token_infer_sheet = db.token_info_sheet(module_path)?;
    let iter0 = token_infer_sheet
        .informative_ranged_token_iter(ranged_token_sheet, db)
        .filter_map(|(info, (range, token))| token_to_semantic_token(db, info, token, range));
    let iter1 = ranged_token_sheet
        .comments()
        .iter()
        .map(|comment| comment_to_semantic_token(comment));
    Ok(itertools::merge(iter0, iter1).collect())
}

fn token_to_semantic_token(
    db: &dyn SemanticTokenDb,
    info: &TokenInfo,
    token: &TokenData,
    range: &husky_text::TextRange,
) -> Option<RangedSemanticToken> {
    let semantic_token = match info {
        TokenInfo::None => match token {
            TokenData::Keyword(kw) => SemanticToken::Keyword(*kw),
            TokenData::Ident(_) | TokenData::Label(_) => return None,
            TokenData::Punctuation(_) => SemanticToken::Special,
            TokenData::WordOpr(_) => SemanticToken::WordOpr,
            TokenData::Literal(_) => SemanticToken::Literal,
            TokenData::Error(_) => return None,
        },
        TokenInfo::Entity(path) => SemanticToken::Entity(path.item_kind(db)),
        TokenInfo::EntityNode(path, item_kind) => SemanticToken::Entity(*item_kind),
        TokenInfo::CurrentSymbol {
            current_symbol_kind,
            ..
        } => match current_symbol_kind {
            CurrentSynSymbolKind::LetVariable { .. } => SemanticToken::Variable,
            CurrentSynSymbolKind::ExplicitRegularParameter { .. } => SemanticToken::Parameter,
            CurrentSynSymbolKind::FrameVariable(_) => SemanticToken::FrameVariable,
            CurrentSynSymbolKind::ImplicitParameter { .. } => SemanticToken::ImplicitParameter,
            CurrentSynSymbolKind::ExplicitVariadicParameter { .. } => SemanticToken::Parameter,
            CurrentSynSymbolKind::FieldVariable { .. } => SemanticToken::Variable,
        },
        // SemanticToken::Variable,
        TokenInfo::InheritedSymbol {
            inherited_symbol_kind,
            ..
        } => match inherited_symbol_kind {
            InheritedSynSymbolKind::ParenateParameter { .. } => SemanticToken::Parameter,
            InheritedSynSymbolKind::TemplateParameter { .. } => SemanticToken::ImplicitParameter,
            InheritedSynSymbolKind::FieldVariable { .. } => SemanticToken::Variable,
        },
        TokenInfo::SelfType => SemanticToken::SelfType,
        TokenInfo::SelfValue => SemanticToken::SelfValue,
        // SemanticToken::Variable,
        TokenInfo::Field => SemanticToken::Field,
        TokenInfo::Method => SemanticToken::Method,
        TokenInfo::BoxColon | TokenInfo::BoxPrefix => {
            SemanticToken::Entity(EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                connection: MajorItemConnectionKind::Connected,
            })
        }
        TokenInfo::UseExpr { state, .. } => match state {
            OnceUseRuleState::Resolved {
                original_symbol: Some(original_symbol),
            } => SemanticToken::Entity(original_symbol.path(db).item_kind(db)),
            _ => return None,
        },
        TokenInfo::UseExprStar => SemanticToken::Special,
        TokenInfo::HtmlFunctionIdent => SemanticToken::HtmlFunctionIdent,
        TokenInfo::HtmlPropertyIdent => SemanticToken::HtmlPropertyIdent,
        TokenInfo::SubmoduleIdent => SemanticToken::SubmoduleIdent,
        TokenInfo::UnitLeftParenthesis | TokenInfo::UnitRightParenthesis => {
            SemanticToken::Entity(EntityKind::MajorItem {
                module_item_kind: MajorItemKind::Type(TypeKind::Extern),
                connection: MajorItemConnectionKind::Connected,
            })
        }
        TokenInfo::Todo => SemanticToken::Todo,
    };
    Some(RangedSemanticToken {
        semantic_token,
        range: *range,
    })
}

fn comment_to_semantic_token(comment: &Comment) -> RangedSemanticToken {
    RangedSemanticToken {
        semantic_token: SemanticToken::Comment,
        range: comment.range(),
    }
}
