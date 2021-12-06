//! Structural Search Replace
//!
//! Allows searching the AST for code that matches one or more patterns and then replacing that code
//! based on a template.

// Feature: Structural Search and Replace
//
// Search and replace with named wildcards that will match any expression, type, path, pattern or item.
// The syntax for a structural search replace command is `<search_pattern> ==>> <replace_pattern>`.
// A `$<name>` placeholder in the search pattern will match any AST node and `$<name>` will reference it in the replacement.
// Within a macro call, a placeholder will match up until whatever token follows the placeholder.
//
// All paths in both the search pattern and the replacement template must resolve in the context
// in which this command is invoked. Paths in the search pattern will then match the code if they
// resolve to the same item, even if they're written differently. For example if we invoke the
// command in the module `foo` with a pattern of `Bar`, then code in the parent module that refers
// to `foo::Bar` will match.
//
// Paths in the replacement template will be rendered appropriately for the context in which the
// replacement occurs. For example if our replacement template is `foo::Bar` and we match some
// code in the `foo` module, we'll insert just `Bar`.
//
// Inherent method calls should generally be written in UFCS form. e.g. `foo::Bar::baz($s, $a)` will
// match `$s.baz($a)`, provided the method call `baz` resolves to the method `foo::Bar::baz`. When a
// placeholder is the receiver of a method call in the search pattern (e.g. `$s.foo()`), but not in
// the replacement template (e.g. `bar($s)`), then *, & and &mut will be added as needed to mirror
// whatever autoderef and autoref was happening implicitly in the matched code.
//
// The scope of the search / replace will be restricted to the current selection if any, otherwise
// it will apply to the whole workspace.
//
// Placeholders may be given constraints by writing them as `${<name>:<constraint1>:<constraint2>...}`.
//
// Supported constraints:
//
// |===
// | Constraint    | Restricts placeholder
//
// | kind(literal) | Is a literal (e.g. `42` or `"forty two"`)
// | not(a)        | Negates the constraint `a`
// |===
//
// Available via the command `husky-lang-server.ssr`.
//
// ```rust
// // Using structural search replace command [foo($a, $b) ==>> ($a).foo($b)]
//
// // BEFORE
// String::from(foo(y + 5, z))
//
// // AFTER
// String::from((y + 5).foo(z))
// ```
//
// |===
// | Editor  | Action Name
//
// | VS Code | **Rust Analyzer: Structural Search Replace**
// |===
//
// Also available as an assist, by writing a comment containing the structural
// search and replace rule. You will only see the assist if the comment can
// be parsed as a valid structural search and replace rule.
//
// ```rust
// // Place the cursor on the line below to see the assist 💡.
// // foo($a, $b) ==>> ($a).foo($b)
// ```

#![allow(dead_code, unused)]
mod from_comment;
mod matching;
mod nester;
mod parsing;
mod replacing;
mod resolving;
mod search;
#[macro_use]
mod errors;

use common::*;

use crate::errors::bail;
pub use crate::errors::SsrError;
pub use crate::from_comment::ssr_from_comment;
pub use crate::matching::Match;
use crate::matching::MatchFailureReason;
use hir::Semantics;
use ide_db::base_db::{FileID, FilePosition, FileRange};
use resolving::ResolvedRule;
use rustc_hash::FxHashMap;
use syntax::{ast, SyntaxNode};
use text_edit::TextEdit;

// A structured search replace rule. Create by calling `parse` on a str.
#[derive(Debug)]
pub struct SsrRule {
    /// A structured pattern that we're searching for.
    pattern: parsing::RawPattern,
    /// What we'll replace it with.
    template: parsing::RawPattern,
    parsed_rules: Vec<parsing::ParsedRule>,
}

#[derive(Debug)]
pub struct SsrPattern {
    parsed_rules: Vec<parsing::ParsedRule>,
}

#[derive(Debug, Default)]
pub struct SsrMatches {
    pub matches: Vec<Match>,
}

/// Searches a crate for pattern matches and possibly replaces them with something else.
pub struct MatchFinder<'db> {
    /// Our source of information about the user's code.
    sema: Semantics<'db, ide_db::IdeDatabase>,
    rules: Vec<ResolvedRule>,
    resolution_scope: resolving::ResolutionScope<'db>,
    restrict_ranges: Vec<FileRange>,
}

impl<'db> MatchFinder<'db> {
    /// Constructs a new instance where names will be looked up as if they appeared at
    /// `lookup_context`.
    pub fn in_context(
        db: &'db ide_db::IdeDatabase,
        lookup_context: FilePosition,
        mut restrict_ranges: Vec<FileRange>,
    ) -> MatchFinder<'db> {
        todo!()
    }

    /// Constructs an instance using the start of the first file in `db` as the lookup context.
    pub fn at_first_file(db: &'db ide_db::IdeDatabase) -> Result<MatchFinder<'db>, SsrError> {
        use ide_db::base_db::SourceDatabaseExt;
        use ide_db::symbol_index::SymbolsDatabase;
        if let Some(first_file_id) = db
            .local_roots()
            .iter()
            .next()
            .and_then(|root| db.source_root(*root).iter().next())
        {
            Ok(MatchFinder::in_context(
                db,
                FilePosition {
                    file_id: first_file_id,
                    offset: 0.into(),
                },
                vec![],
            ))
        } else {
            bail!("No files to search");
        }
    }

    /// Adds a rule to be applied. The order in which rules are added matters. Earlier rules take
    /// precedence. If a node is matched by an earlier rule, then later rules won't be permitted to
    /// match to it.
    pub fn add_rule(&mut self, rule: SsrRule) -> Result<(), SsrError> {
        for parsed_rule in rule.parsed_rules {
            self.rules.push(ResolvedRule::new(
                parsed_rule,
                &self.resolution_scope,
                self.rules.len(),
            )?);
        }
        Ok(())
    }

    /// Finds matches for all added rules and returns edits for all found matches.
    pub fn edits(&self) -> FxHashMap<FileID, TextEdit> {
        use ide_db::base_db::SourceDatabaseExt;
        let mut matches_by_file = FxHashMap::default();
        for m in self.matches().matches {
            matches_by_file
                .entry(m.range.file_id)
                .or_insert_with(SsrMatches::default)
                .matches
                .push(m);
        }
        matches_by_file
            .into_iter()
            .map(|(file_id, matches)| {
                (
                    file_id,
                    replacing::matches_to_edit(
                        &matches,
                        &self.sema.db.file_text(file_id),
                        &self.rules,
                    ),
                )
            })
            .collect()
    }

    /// Adds a search pattern. For use if you intend to only call `find_matches_in_file`. If you
    /// intend to do replacement, use `add_rule` instead.
    pub fn add_search_pattern(&mut self, pattern: SsrPattern) -> Result<(), SsrError> {
        for parsed_rule in pattern.parsed_rules {
            self.rules.push(ResolvedRule::new(
                parsed_rule,
                &self.resolution_scope,
                self.rules.len(),
            )?);
        }
        Ok(())
    }

    /// Returns matches for all added rules.
    pub fn matches(&self) -> SsrMatches {
        let mut matches = Vec::new();
        let mut usage_cache = search::UsageCache::default();
        for rule in &self.rules {
            self.find_matches_for_rule(rule, &mut usage_cache, &mut matches);
        }
        nester::nest_and_remove_collisions(matches, &self.sema)
    }

    /// Finds all nodes in `file_id` whose text is exactly equal to `snippet` and attempts to match
    /// them, while recording reasons why they don't match. This API is useful for command
    /// line-based debugging where providing a range is difficult.
    pub fn debug_where_text_equal(&self, file_id: FileID, snippet: &str) -> Vec<MatchDebugInfo> {
        todo!()
    }

    fn output_debug_for_nodes_at_range(
        &self,
        node: &SyntaxNode,
        range: FileRange,
        restrict_range: &Option<FileRange>,
        out: &mut Vec<MatchDebugInfo>,
    ) {
        todo!()
    }
}

pub struct MatchDebugInfo {
    node: SyntaxNode,
    /// Our search pattern parsed as an expression or item, etc
    pattern: SyntaxNode,
    matched: Result<Match, MatchFailureReason>,
}

impl std::fmt::Debug for MatchDebugInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.matched {
            Ok(_) => writeln!(f, "Node matched")?,
            Err(reason) => writeln!(f, "Node failed to match because: {}", reason.reason)?,
        }
        writeln!(
            f,
            "============ AST ===========\n\
            {:#?}",
            self.node
        )?;
        writeln!(f, "========= PATTERN ==========")?;
        writeln!(f, "{:#?}", self.pattern)?;
        writeln!(f, "============================")?;
        Ok(())
    }
}

impl SsrMatches {
    /// Returns `self` with any nested matches removed and made into top-level matches.
    pub fn flattened(self) -> SsrMatches {
        let mut out = SsrMatches::default();
        self.flatten_into(&mut out);
        out
    }

    fn flatten_into(self, out: &mut SsrMatches) {
        for mut m in self.matches {
            for p in m.placeholder_values.values_mut() {
                std::mem::take(&mut p.inner_matches).flatten_into(out);
            }
            out.matches.push(m);
        }
    }
}

impl Match {
    pub fn matched_text(&self) -> String {
        todo!()
    }
}

impl std::error::Error for SsrError {}
