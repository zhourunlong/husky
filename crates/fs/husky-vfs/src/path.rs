mod crate_path;
mod diff_path;
mod menu;
mod module_path;
mod package_path;

pub use crate_path::*;
pub use diff_path::*;
pub use menu::*;
pub use module_path::*;
pub use package_path::*;

use crate::*;

pub(crate) fn package_manifest_path(db: &dyn VfsDb, package: PackagePath) -> VfsResult<DiffPath> {
    DiffPath::try_new(
        db,
        &package_dir(db, package)
            .as_ref()?
            .path(db)
            .join("Corgi.toml"),
    )
}

#[salsa::tracked(jar = VfsJar, return_ref)]
pub(crate) fn package_dir(db: &dyn VfsDb, package: PackagePath) -> VfsResult<DiffPath> {
    match package.data(db) {
        PackagePathData::Toolchain { toolchain, ident } => {
            let toolchain_library_path = db.toolchain_library_path(*toolchain);
            DiffPath::try_new(db, &toolchain_library_path.join(ident.data(db)))
        }
        PackagePathData::Global {
            name: _,
            version: _,
        } => todo!(),
        PackagePathData::Local { path } => Ok(path.clone()),
        PackagePathData::Git { .. } => todo!(),
    }
}

#[salsa::tracked(jar = VfsJar)]
pub(crate) fn module_diff_path(db: &dyn VfsDb, module_path: ModulePath) -> VfsResult<DiffPath> {
    match module_path.data(db) {
        ModulePathData::Root(crate_path) => DiffPath::try_new(
            db,
            &package_dir(db, crate_path.package_path(db))
                .as_ref()?
                .path(db)
                .join(crate_path.relative_path(db).as_ref()),
        )
        .map_err(|e| e.into()),
        ModulePathData::Child { parent, ident } => {
            let parent_module_path = module_diff_path(db, parent)?;
            let dir = match parent.data(db) {
                ModulePathData::Root(_) => parent_module_path.path(db).parent().unwrap().to_owned(),
                ModulePathData::Child {
                    parent: _,
                    ident: _,
                } => parent_module_path.path(db).with_extension(""),
            };
            DiffPath::try_new(db, &dir.join(db.dt_ident(ident)).with_extension("hsy"))
        }
    }
}

// this shouldn't be tracked
pub(crate) fn resolve_module_path(
    db: &dyn VfsDb,
    toolchain: Toolchain,
    path: impl AsRef<Path>,
) -> VfsResult<ModulePath> {
    let path = path.as_ref();
    if path.extension().and_then(|s| s.to_str()) != Some("hsy") {
        todo!()
    }
    let parent = path.parent().ok_or(VfsError::ModulePathResolveFailure)?;
    let file_stem = path
        .file_stem()
        .and_then(|s| s.to_str())
        .ok_or(VfsError::ModulePathResolveFailure)?;
    Ok(
        if parent.ends_with("src")
            && parent
                .parent()
                .ok_or(VfsError::ModulePathResolveFailure)?
                .join("Corgi.toml")
                .exists()
        {
            match file_stem {
                "lib" =>  ModulePath::new_root(
                    db,
                    CratePath::new(db,   PackagePath::new_local(db, toolchain,parent.parent().ok_or(VfsError::ModulePathResolveFailure)?)?,
                    CrateKind::Library,)
                ) ,
                "main" => ModulePath::new_root(
                    db,
                    CratePath::new(db, PackagePath::new_local(db, toolchain,
                        parent.parent().ok_or(VfsError::ModulePathResolveFailure)?
                    )?  ,  CrateKind::Main,)
                 ),
                _ => {
                    if let lib_path = parent.join("lib.hsy") && lib_path.exists() {
                        ModulePath::new_child(
                            db,
                            resolve_module_path(db,toolchain, lib_path)?,
                            db.it_ident_borrowed(file_stem)
                                .ok_or(VfsError::ModulePathResolveFailure)?,
                        )
                    } else if let main_path = parent.join("main.hsy") && main_path.exists() {
                        ModulePath::new_child(
                            db,
                            resolve_module_path(db, toolchain,main_path)?,
                            db.it_ident_borrowed(file_stem)
                                .ok_or(VfsError::ModulePathResolveFailure)?,
                         )
                    } else {
                        todo!()
                    }
                }
            }
        } else {
            let parent_module_path = parent.with_extension("hsy");
            if !parent_module_path.exists() {
                todo!()
            }
            ModulePath::new_child(
                db,
                resolve_module_path(db, toolchain, parent_module_path)?,
                db.it_ident_borrowed(file_stem)
                    .ok_or(VfsError::ModulePathResolveFailure)?,
            )
        },
    )
}

#[test]
fn resolve_module_path_works() {
    DB::default().vfs_plain_test("resolve-module-path", |db, module_path| {
        let abs_path = module_diff_path(db, module_path).unwrap();
        let toolchain = module_path.toolchain(db);
        let entity_path_resolved = db
            .resolve_module_path(toolchain, abs_path.path(db))
            .unwrap();
        assert_eq!(module_path, entity_path_resolved)
    })
}
