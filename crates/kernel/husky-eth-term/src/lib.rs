#![feature(impl_trait_in_assoc_type)]
#![feature(result_flattening)]
#![doc = include_str ! ("../README.md")]
#![feature(let_chains)]
#![feature(if_let_guard)]
#![feature(anonymous_lifetime_in_impl_trait)]
pub mod context;
mod conversion;
mod db;
mod error;
pub mod fmt;
mod helpers;
pub mod instantiation;
pub mod jar;
mod menu;
mod rewrite;
mod template_parameter;
pub mod term;
#[cfg(test)]
mod tests;
mod ty;

pub use self::db::*;
pub use self::error::*;
pub use self::menu::*;
pub use self::rewrite::substitute::*;
pub use self::template_parameter::*;
pub use self::ty::*;

use self::jar::EthTermJar as Jar;
use self::term::EthTerm;
use either::*;
use husky_dec_term::{jar::DecTermDb, term::*, *};
use husky_print_utils::p;
use husky_term_prelude::*;
use husky_vfs::{jar::VfsDb, toolchain::Toolchain};
use salsa::DebugWithDb;
use smallvec::*;
