use crate::*;
use husky_path_utils::*;
use relative_path::RelativePathBuf;
use salsa::DebugWithDb;
use std::path::PathBuf;

pub trait VfsTestUnit: Sized {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self>;
    fn decide_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf;
}

impl VfsTestUnit for PackagePath {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        todo!()
    }

    fn decide_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf {
        todo!()
    }
}

impl VfsTestUnit for CratePath {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        db.collect_crates(package_path).unwrap_or_default()
    }

    fn decide_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf {
        package_expects_dir.join(format!(
            "{}/{}.{EXPECT_FILE_EXTENSION}",
            task_name,
            match self.crate_kind(db) {
                CrateKind::Library => format!("lib"),
                CrateKind::Main => format!("main"),
                CrateKind::Binary(_) => todo!(),
                CrateKind::StandaloneTest(_) => todo!(),
            }
        ))
    }
}

impl VfsTestUnit for ModulePath {
    fn collect_from_package_path(db: &dyn VfsDb, package_path: PackagePath) -> Vec<Self> {
        db.collect_probable_modules(package_path)
    }

    fn decide_expect_file_path(
        &self,
        db: &dyn VfsDb,
        task_name: &str,
        package_expects_dir: &Path,
    ) -> PathBuf {
        fn decide_expect_file_aux_path(
            db: &dyn VfsDb,
            module_path: ModulePath,
            task_name: &str,
            package_expects_dir: &Path,
        ) -> PathBuf {
            match module_path.data(db) {
                ModulePathData::Root(_) => package_expects_dir.join(task_name),
                ModulePathData::Child { parent, ident } => {
                    decide_expect_file_aux_path(db, parent, task_name, package_expects_dir)
                        .join(db.dt_ident(ident))
                }
            }
        }
        let aux_path = decide_expect_file_aux_path(db, *self, task_name, package_expects_dir);
        match self.data(db) {
            ModulePathData::Root(crate_path) => aux_path.join(format!(
                "{}.{EXPECT_FILE_EXTENSION}",
                match crate_path.crate_kind(db) {
                    CrateKind::Library => "lib",
                    CrateKind::Main => "main",
                    CrateKind::Binary(_) => todo!(),
                    CrateKind::StandaloneTest(_) => todo!(),
                }
            )),
            ModulePathData::Child { .. } => aux_path.with_extension(EXPECT_FILE_EXTENSION),
        }
    }
}

pub trait VfsTestUtils: VfsDb {
    // toolchain
    fn dev_toolchain(&self) -> ToolchainResult<Toolchain>;
    fn dev_path_menu(&self) -> ToolchainResult<&VfsPathMenu> {
        let toolchain = self.dev_toolchain()?;
        self.vfs_path_menu(toolchain)
    }
    fn vfs_test<U>(&self, f: impl Fn(&Self, U))
    where
        U: VfsTestUnit,
    {
        for dir in test_dirs() {
            let vfs_db = <Self as salsa::DbWithJar<VfsJar>>::as_jar_db(self);
            let toolchain = self.dev_toolchain().unwrap();
            for (base, out) in expect_test_base_outs() {
                std::fs::create_dir_all(&out).expect("failed_to_create_dir_all");
                for path in collect_package_relative_dirs(&base).into_iter() {
                    let package_path =
                        PackagePath::new_local(vfs_db, toolchain, &path.to_logical_path(&base))
                            .unwrap();
                    for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path)
                    {
                        f(self, unit)
                    }
                }
            }
        }
    }

    fn expect_test_packages_debug_result<T, E>(
        name: &str,
        f: impl Fn(&Self, PackagePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug;

    fn vfs_expect_test_debug_with_db<'a, U, R>(&'a self, name: &str, f: impl Fn(&'a Self, U) -> R)
    where
        U: VfsTestUnit,
        R: salsa::DebugWithDb<Self>,
    {
        vfs_expect_test(self, name, &f, |_db, r| format!("{:#?}", &r.debug(self)))
    }

    fn vfs_expect_test_debug<'a, U, R>(&'a self, name: &str, f: impl Fn(&'a Self, U) -> R)
    where
        U: VfsTestUnit,
        R: std::fmt::Debug,
    {
        vfs_expect_test(self, name, &f, |_db, r| format!("{:#?}", r))
    }
}

struct TestPathResolver<'a> {
    db: &'a dyn VfsDb,
    name: &'a str,
    package_expects_dir: PathBuf,
}

const EXPECT_FILE_EXTENSION: &'static str = "md";

impl<Db> VfsTestUtils for Db
where
    Db: VfsDb + ?Sized,
{
    fn expect_test_packages_debug_result<T, E>(
        name: &str,
        f: impl Fn(&Self, PackagePath) -> Result<&T, E>,
    ) where
        Self: Default,
        T: std::fmt::Debug + ?Sized,
        E: std::fmt::Debug,
    {
        let db = Self::default();
        for (base, out) in expect_test_base_outs() {
            expect_test_packages(&db, name, &base, out, &f, |_db, r| format!("{:#?}", r));
        }
    }

    fn dev_toolchain(&self) -> ToolchainResult<Toolchain> {
        let library_path = derive_library_path_from_cargo_manifest_dir()?;
        let db = <Self as salsa::DbWithJar<VfsJar>>::as_jar_db(&self);
        Ok(Toolchain::new(
            db,
            ToolchainData::Local {
                library_path: DiffPath::try_new(db, &library_path).unwrap(),
            },
        ))
    }
}

fn test_dirs() -> Vec<PathBuf> {
    let env = HuskyDevPathEnv::new();
    vec![
        env.lang_dev_library_dir().to_owned(),
        env.lang_dev_examples_dir().to_owned(),
    ]
}

fn expect_test_base_outs() -> Vec<(PathBuf, PathBuf)> {
    let env = HuskyDevPathEnv::new();
    let dir = env
        .cargo_manifest_dir()
        .map(|p| p.to_owned())
        .unwrap_or("temp".into());
    vec![
        (
            env.lang_dev_library_dir().to_owned(),
            dir.join("expect-files/library"),
        ),
        (
            env.lang_dev_examples_dir().to_owned(),
            dir.join("expect-files/examples"),
        ),
    ]
}

fn expect_test_packages<Db, T, E>(
    db: &Db,
    name: &str,
    base: &Path,
    out: PathBuf,
    f: &impl Fn(&Db, PackagePath) -> Result<&T, E>,
    p: impl Fn(&Db, Result<&T, E>) -> String,
) where
    Db: VfsDb,
    T: ?Sized,
{
    let toolchain = db.dev_toolchain().unwrap();
    std::fs::create_dir_all(&out).unwrap();
    collect_package_relative_dirs(base)
        .into_iter()
        .for_each(|path| {
            let package_path =
                PackagePath::new_local(db, toolchain, &path.to_logical_path(base)).unwrap();
            let package_expect_file_path = path
                .to_logical_path(&out)
                .with_extension(EXPECT_FILE_EXTENSION);
            let dir = &package_expect_file_path.parent().unwrap();
            p!(dir);
            std::fs::create_dir_all(dir).unwrap();
            expect_test::expect_file![package_expect_file_path]
                .assert_eq(&p(&db, f(&db, package_path)));
        });
}

fn vfs_expect_test<'a, Db, U, R>(
    db: &'a Db,
    name: &str,
    f: &impl Fn(&'a Db, U) -> R,
    p: impl Fn(&'a Db, R) -> String,
) where
    Db: VfsDb + ?Sized,
    U: VfsTestUnit,
{
    let vfs_db = <Db as salsa::DbWithJar<VfsJar>>::as_jar_db(db);
    let toolchain = db.dev_toolchain().unwrap();
    for (base, out) in expect_test_base_outs() {
        std::fs::create_dir_all(&out).expect("failed_to_create_dir_all");
        for path in collect_package_relative_dirs(&base).into_iter() {
            let package_path =
                PackagePath::new_local(vfs_db, toolchain, &path.to_logical_path(&base)).unwrap();
            for unit in <U as VfsTestUnit>::collect_from_package_path(vfs_db, package_path) {
                let path = unit.decide_expect_file_path(vfs_db, name, &path.to_logical_path(&out));
                std::fs::create_dir_all(path.parent().unwrap()).unwrap();
                expect_test::expect_file![path].assert_eq(&p(db, f(db, unit)))
            }
        }
    }
}
