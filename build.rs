use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=native/glob.c");
    println!("cargo:rerun-if-env-changed=CC");
    println!("cargo:rerun-if-env-changed=AR");

    let output = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let object = output.join("glob.o");
    let library = output.join("libmsh_glob.a");
    let compiler = env::var_os("CC").unwrap_or_else(|| "cc".into());
    let archiver = env::var_os("AR").unwrap_or_else(|| "ar".into());

    let status = Command::new(compiler)
        .args(["-std=c99", "-fPIC", "-c", "native/glob.c", "-o"])
        .arg(&object)
        .status()
        .expect("could not run the C compiler");
    assert!(status.success(), "C compilation failed");

    let status = Command::new(archiver)
        .arg("crs")
        .arg(library)
        .arg(object)
        .status()
        .expect("could not run the archiver");
    assert!(status.success(), "C archive creation failed");

    println!("cargo:rustc-link-search=native={}", output.display());
    println!("cargo:rustc-link-lib=static=msh_glob");
}
