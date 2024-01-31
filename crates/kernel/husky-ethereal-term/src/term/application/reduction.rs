use super::*;

impl ApplicationEtherealTerm {
    pub(in crate::term) fn reduce(self, db: &::salsa::Db) -> EtherealTerm {
        reduce_term_application(db, self)
    }
}

#[salsa::tracked(jar = EtherealTermJar)]
pub(crate) fn reduce_term_application(
    db: &::salsa::Db,
    term_application: ApplicationEtherealTerm,
) -> EtherealTerm {
    let function = term_application.function(db).reduce(db);
    let argument = term_application.argument(db).reduce(db);
    let shift = term_application.shift(db);
    match function {
        EtherealTerm::EntityPath(ItemPathTerm::Fugitive(_)) => todo!(),
        EtherealTerm::Ritchie(_) => todo!(),
        EtherealTerm::Abstraction(_) => todo!(),
        EtherealTerm::Application(function_term_application)
            if let function_shift = function_term_application.shift(db)
                && function_shift > 0 =>
        {
            ApplicationEtherealTerm::new_reduced(
                db,
                function_term_application.function(db),
                ApplicationEtherealTerm::new_reduced(
                    db,
                    function_term_application.argument(db),
                    argument,
                    shift,
                ),
                function_shift + shift - 1,
            )
        }
        _ => ApplicationEtherealTerm::new_inner(db, function, argument, shift).into(),
    }
}
