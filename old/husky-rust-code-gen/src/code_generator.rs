// mod context;
// mod gen_item_route;
// mod impl_expr;
// mod impl_init;
// mod impl_lib;
// mod impl_mod;
// mod impl_registration;
// mod impl_routine_defn;
// mod impl_stmt;
// mod impl_ty_defn;
// mod impl_write;
// mod utils;

use crate::*;
// use context::*;

pub(crate) struct RustCodeGenerator<'a> {
    db: &'a dyn RustTranspileDb,
    result: String,
    target_entrance: DiffPath,
    // item_route_uses: LocalStack<EtherealTerm>,
    // context: RustCodeGenContext,
}

impl<'a> RustCodeGenerator<'a> {
    pub(crate) fn new(_db: &'a dyn RustTranspileDb, _module: EtherealTerm) -> Self {
        todo!()
        // let target_entrance = db
        //     .module_target_entrance(db.module_file(module).unwrap())
        //     .unwrap();
        // let item_defn = db.item_defn(module).unwrap();
        // let mut symbols = LocalStack::new();
        // for item_defn in item_defn.subentities.iter() {
        //     symbols.push(item_defn.base_route)
        // }
        // Self {
        //     db,
        //     target_entrance: target_entrance,
        //     result: Default::default(),
        //     item_route_uses: symbols,
        //     context: RustCodeGenContext::Normal,
        // }
    }

    pub(crate) fn new_lib(
        _db: &'a dyn RustTranspileDb,
        _target_entrance: DiffPath,
        _use_crate_all: bool,
    ) -> Self {
        // let mut symbols = LocalStack::new();
        // let package = db.package(target_entrance).unwrap();
        // if use_crate_all {
        //     for item_defn in package.subentities.iter() {
        //         symbols.push(item_defn.base_route)
        //     }
        // }
        todo!()
        // Self {
        //     db,
        //     target_entrance: target_entrance,
        //     result: Default::default(),
        //     item_route_uses: symbols,
        //     context: RustCodeGenContext::Normal,
        // }
    }

    // pub(crate) fn package(&self) -> Arc<Package> {
    //     self.db.package(self.target_entrance).unwrap()
    // }

    // fn exec_within_context(&mut self, new_context: RustCodeGenContext, f: impl FnOnce(&mut Self)) {
    //     let old_context = std::mem::replace(&mut self.context, new_context);
    //     f(self);
    //     self.context = old_context;
    // }
}