use colored::Colorize;
use std::process::Command;

pub fn build_c(c_codegen_dir: &str, build_dir: &str, lib_dir: &str) {
    let cc_output = Command::new("cc")
        .arg("-c")
        .arg(format!("{c_codegen_dir}/husky_vm_interface.c"))
        .arg("-o")
        .arg(format!("{build_dir}/husky_vm_interface.o"))
        .output()
        .expect("cc failed");
    if cc_output.stderr.len() > 0 {
        panic!("{}", std::str::from_utf8(&cc_output.stderr).unwrap().red())
    }
    let ar_output = Command::new("ar")
        .arg("-rcs")
        .arg(format!("{lib_dir}/libhusky_vm_interface.a"))
        .arg(format!("{build_dir}/husky_vm_interface.o"))
        .output()
        .expect("ar failed");
    if ar_output.stderr.len() > 0 {
        panic!("{}", std::str::from_utf8(&ar_output.stderr).unwrap().red())
    }
}
