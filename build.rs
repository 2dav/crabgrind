fn main() {
    // 'DOCS_RS' is set by the docs.rs build env
    if std::env::var("DOCS_RS").is_ok() {
        // 'valgrind' is not required to render the documentation, more so it's not installed in the
        // `crates-build-env`, so rather than adding a new dependency to the docker image we just skip
        // the ffi library building part.
        // For this to work all the code examples in the doc comments should be marked with `no_run`.
        return;
    }

    println!("cargo:rerun-if-changed=export.c");

    let mut builder = cc::Build::new();

    if let Ok(dep_valgrind) = std::env::var("DEP_VALGRIND") {
        builder.include(dep_valgrind);
    }

    builder.file("export.c").compile("libcrabgrind.a");
}
