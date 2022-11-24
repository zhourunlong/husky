use std::sync::Arc;

use crate::*;
use husky_entity_path::EntityPathDb;
use husky_identifier::IdentifierDb;
use salsa::{storage::HasJar, DbWithJar};

pub trait TermDb: DbWithJar<TermJar> + EntityPathDb {
    fn term_menu(&self) -> &TermMenu;
    fn it_term(&self, term_data: TermData) -> Term;
    fn it_entity_path_term(&self, entity_path: EntityPath) -> Term {
        self.it_term(TermData::Atom(TermAtom::Entity { entity_path }))
    }
}

#[derive(Default)]
pub struct TermMenuPlace(Arc<once_cell::sync::OnceCell<TermMenu>>);

impl<T> TermDb for T
where
    T: DbWithJar<TermJar> + EntityPathDb,
{
    fn term_menu(&self) -> &TermMenu {
        <Self as HasJar<TermJar>>::jar(self)
            .0
            .term_menu_place()
            .0
            .get_or_init(|| TermMenu::new(self))
    }

    fn it_term(&self, data: TermData) -> Term {
        Term::new(self, data)
    }
}

// fn namespace_decl(db: &dyn TermDb, namespace: TermNamespace) -> TermResultArc<NamespaceDecl> {
//     db.ask_namespace_decl(namespace)
// }

// fn ty_decl(db: &dyn TermDb, ty: Term) -> TermResultArc<TyDecl> {
//     db.ask_ty_decl(ty)
// }

// fn decl(db: &dyn TermDb, entity_path: EntityPath) -> TermResultArc<Decl> {
//     db.ask_decl(entity_path)
// }
