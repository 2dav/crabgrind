fn main() {
    println!("cargo:rerun-if-changed=export.c");

    let mut cc = cc::Build::new();
    if let Ok(lib) = pkg_config::Config::new().probe("valgrind") {
        for include in &lib.include_paths {
            cc.include(include);
        }
    }
    cc.file("export.c").compile("libcrabgrind.a");
}
