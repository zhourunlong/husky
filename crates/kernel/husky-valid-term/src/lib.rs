// #![doc = include_str!("../README.md")]
// #![feature(trait_upcasting)]
// #![feature(let_chains)]
// // #![deny(unsafe_code, missing_docs, clippy::unwrap_used)]

// mod context;
// mod db;
// mod error;
// mod rewrite;
// mod term;
// #[cfg(test)]
// mod tests;

// pub use self::context::*;
// pub use self::db::*;
// pub use self::error::*;
// pub use self::rewrite::*;
// pub use self::term::*;

// use either::*;
// use husky_entity_path::*;
// use husky_precise_term::*;
// use husky_print_utils::p;
// use husky_term_prelude::*;
// use husky_vfs::*;
// use husky_word::Identifier;

// #[salsa::jar(db = RawTermDb)]
// pub struct RawTermJar(
//     RawTermSymbol,
//     RawTermCurry,
//     valid_term_curry_from_precise,
//     RawTermRitchie,
//     RawTermAbstraction,
//     RawTermApplication,
//     valid_term_application_from_precise,
//     valid_term_application_ty,
//     RawTermSubentity,
//     RawTermAsTraitSubentity,
//     RawTermTraitConstraint,
//     // only use this inside crate::context::entry
//     is_ty_path_lifetime_ty,
// );