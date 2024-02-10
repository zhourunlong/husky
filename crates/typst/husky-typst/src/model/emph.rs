use crate::diag::SourceResult;
use crate::engine::Engine;
use crate::foundations::{elem, Packed, Show, StyleChain, TypstContent};
use crate::text::{ItalicToggle, TextElem};

/// Emphasizes content by toggling italics.
///
/// - If the current [text style]($text.style) is `{"normal"}`, this turns it
///   into `{"italic"}`.
/// - If it is already `{"italic"}` or `{"oblique"}`, it turns it back to
///   `{"normal"}`.
///
/// # Example
/// ```example
/// This is _emphasized._ \
/// This is #emph[too.]
///
/// #show emph: it => {
///   text(blue, it.body)
/// }
///
/// This is _emphasized_ differently.
/// ```
///
/// # Syntax
/// This function also has dedicated syntax: To emphasize content, simply
/// enclose it in underscores (`_`). Note that this only works at word
/// boundaries. To emphasize part of a word, you have to use the function.
#[elem(title = "Emphasis", Show)]
pub struct EmphElem {
    /// The content to emphasize.
    #[required]
    pub body: TypstContent,
}

impl Show for Packed<EmphElem> {
    #[husky_typst_macros::time(name = "emph", span = self.span())]
    fn show(&self, _: &mut Engine, _: StyleChain) -> SourceResult<TypstContent> {
        Ok(self
            .body()
            .clone()
            .styled(TextElem::set_emph(ItalicToggle(true))))
    }
}
