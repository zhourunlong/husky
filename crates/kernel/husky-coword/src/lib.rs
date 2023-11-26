mod db;
mod ident;
mod kebab;
mod label;
mod menu;
mod style;
#[cfg(test)]
mod tests;

use salsa::Database;

pub use self::db::*;
pub use self::ident::*;
pub use self::kebab::*;
pub use self::label::*;
pub use self::menu::*;
pub use self::style::*;

#[salsa::jar(db = CowordDb)]
pub struct CowordJar(
    Coword,
    ident_menu,
    kebab_to_ident,
    is_coword_valid_kebab,
    ident_to_name,
);

/// the underlying type for all word like types used in representing source code
#[derive(Copy, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Debug)]
pub struct Coword(salsa::Id);

#[doc = r" Internal struct used for interned item"]
#[derive(Eq, PartialEq, Hash, Clone)]
pub struct __CowordData {
    data: String,
}
impl salsa::storage::IngredientsFor for Coword {
    type Jar = CowordJar;
    type Ingredients = salsa::interned::InternedIngredient<Coword, __CowordData>;
    fn create_ingredients<DB>(routes: &mut salsa::routes::Routes) -> Self::Ingredients
    where
        DB: salsa::storage::JarFromJars<Self::Jar>,
    {
        let index = routes.push(
            |jars| {
                let jar = <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars(jars);
                <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient(jar)
            },
            |jars| {
                let jar = <DB as salsa::storage::JarFromJars<Self::Jar>>::jar_from_jars_mut(jars);
                <_ as salsa::storage::HasIngredientsFor<Self>>::ingredient_mut(jar)
            },
        );
        salsa::interned::InternedIngredient::new(index, "Word")
    }
}
impl salsa::AsId for Coword {
    fn as_id(self) -> salsa::Id {
        self.0
    }
    fn from_id(id: salsa::Id) -> Self {
        Coword(id)
    }
}

impl Coword {
    pub fn data<'db>(self, db: &'db <CowordJar as salsa::jar::Jar<'_>>::DynDb) -> &'db str {
        let (jar, runtime) = <_ as salsa::storage::HasJar<CowordJar>>::jar(db);
        let ingredients = <CowordJar as salsa::storage::HasIngredientsFor<Coword>>::ingredient(jar);
        &ingredients.data(runtime, self).data
    }
    pub fn new(db: &<CowordJar as salsa::jar::Jar<'_>>::DynDb, data: String) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<CowordJar>>::jar(db);
        let ingredients = <CowordJar as salsa::storage::HasIngredientsFor<Coword>>::ingredient(jar);
        ingredients.intern(runtime, __CowordData { data })
    }
}

impl<DB> salsa::salsa_struct::SalsaStructInDb<DB> for Coword
where
    DB: ?Sized + salsa::DbWithJar<CowordJar>,
{
    fn register_dependent_fn(_db: &DB, _index: salsa::routes::IngredientIndex) {}
}

impl ::salsa::DebugWithDb for Coword {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>, db: &::salsa::Db) -> ::std::fmt::Result {
        f.debug_tuple("Word")
            .field(&self.data(db.as_jar_db_dyn::<CowordJar>()))
            .finish()
    }
}

impl std::borrow::Borrow<str> for __CowordData {
    fn borrow(&self) -> &str {
        &self.data
    }
}

impl<'a> From<&'a str> for __CowordData {
    fn from(value: &'a str) -> Self {
        Self { data: value.into() }
    }
}

impl Coword {
    pub fn from_owned(db: &<CowordJar as salsa::jar::Jar<'_>>::DynDb, data: String) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<CowordJar>>::jar(db);
        let ingredients = <CowordJar as salsa::storage::HasIngredientsFor<Coword>>::ingredient(jar);
        ingredients.intern(runtime, __CowordData { data })
    }

    pub fn from_ref(db: &<CowordJar as salsa::jar::Jar<'_>>::DynDb, data: &str) -> Self {
        let (jar, runtime) = <_ as salsa::storage::HasJar<CowordJar>>::jar(db);
        let ingredients = <CowordJar as salsa::storage::HasIngredientsFor<Coword>>::ingredient(jar);
        ingredients.intern_borrowed(runtime, data)
    }
}
