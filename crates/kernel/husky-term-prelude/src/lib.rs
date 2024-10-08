mod contract;
mod curry;
mod error;
mod final_destination;
mod item_path;
pub mod jar;
pub mod literal;
mod modifier;
pub mod ritchie;
mod sort;
pub mod symbol;
pub mod template_var_class;
mod universe;

pub use self::contract::*;
pub use self::curry::*;
pub use self::error::*;
pub use self::final_destination::*;
pub use self::item_path::*;
pub use self::modifier::*;
pub use self::sort::*;
pub use self::universe::*;

use self::jar::TermPreludeJar as Jar;
use husky_entity_path::path::major_item::ty::TypePath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[salsa::derive_debug_with_db]
pub enum TypeFinalDestinationExpectation {
    EqsSort,
    EqsNonSortTypePath(TypePath),
    Any,
}

/// a type path can be interpreted in two different ways:
///
/// - \[explicit curries to\] a type
/// - \[explicit curries to\] a type constructor
///
/// the final curry destination of the two different interpretation are different
///
/// for example, the type of type path `List` can either be
///
/// - `∀ universe u, explicit covariant Sort u -> Sort u`,
///
///     the final curry destination is in universe `Sort u`
/// - `∀ universe u, explicit covariant (E: Sort u) -> () -> List E`,
///
///     the final curry destination is in universe `List E`
/// disambiguate between type itself (or template) and its instance or constructor
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum TypePathDisambiguation {
    OntologyConstructor,
    /// if type is a unit struct, this will become an instance,
    ///
    /// otherwise constructor
    InstanceConstructor,
}
