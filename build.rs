//! Build script for native library wrapping the Valgrind [Client Request API](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)
use std::{
    env,
    path::{Path, PathBuf},
};

const ENV_VALGRIND_INCLUDE: &str = "VALGRIND_INCLUDE";

fn env_include() -> Option<PathBuf> {
    let Ok(path) = env::var(ENV_VALGRIND_INCLUDE).map(PathBuf::from) else { return None };
    assert!(path.exists(), "{ENV_VALGRIND_INCLUDE}={} Path does not exists", path.display());
    Some(path)
}

fn pkgconfig_include() -> Vec<PathBuf> {
    let mut cfg = pkg_config::Config::new();
    cfg.cargo_metadata(false)
        .env_metadata(false)
        .print_system_libs(false)
        .print_system_cflags(true);

    let lib = match cfg.probe("valgrind") {
        Ok(lib) => lib,
        Err(pkg_config::Error::Command { .. }) => return vec![],
        Err(e) => panic!(
            "\n\nCould not find valgrind via pkg-config:\n{e}\n\
            If valgrind is installed in a non-standard location or built from source without a corresponding .pc file, \
            run with `{ENV_VALGRIND_INCLUDE}=<valgrind dir>/include`."
        ),
    };

    lib.include_paths
        .into_iter()
        .map(|path| path.parent().map(Path::to_path_buf).unwrap_or(path))
        .collect()
}

fn valgrind_include_paths() -> Vec<PathBuf> {
    env_include().map_or_else(pkgconfig_include, |path| vec![path])
}

fn build_native(valgrind_include: &[PathBuf]) {
    let mut builder = cc::Build::new();

    for path in valgrind_include {
        builder.include(path);
    }

    builder.flag("-idiraftervalgrind/include").file("valgrind/native.c").compile("native");
}

fn gen_bindings(include: &[PathBuf]) {
    let bindings = include
        .iter()
        .fold(bindgen::builder(), |b, path| b.clang_arg(format!("-iquote{}", path.display())))
        .clang_arg("-idiraftervalgrind/include")
        .allowlist_var("__VALGRIND_MAJOR__")
        .allowlist_var("__VALGRIND_MINOR__")
        .allowlist_type("CG_.*ClientRequest")
        .rustified_enum("CG_.*ClientRequest")
        .raw_line(std::fs::read_to_string("valgrind/valgrind_version.rs").unwrap_or_else(|_| {
            panic!("valgrind/valgrind_version.rs should exists. 'Use: just --list'")
        }))
        .layout_tests(false)
        .use_core()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .header("valgrind/wrapper.h")
        .generate()
        .expect("`bindgen` bindings generation failed");

    let out_dir = env::var("OUT_DIR").map(PathBuf::from).unwrap();
    let path = out_dir.join("bindings.rs");

    bindings.write_to_file(path).unwrap();
}

fn main() {
    println!("cargo:rerun-if-changed=valgrind/wrapper.h");
    println!("cargo:rerun-if-changed=valgrind/valgrind_version.rs");
    println!("cargo:rerun-if-changed=valgrind/native.c");
    println!("cargo:rerun-if-env-changed={ENV_VALGRIND_INCLUDE}");
    println!("cargo:rerun-if-env-changed=TARGET");

    let include = valgrind_include_paths();

    build_native(&include);
    gen_bindings(&include);
}
