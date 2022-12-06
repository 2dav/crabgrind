fn main() {
    println!("cargo:rerun-if-changed=export.c");

    cc::Build::new().file("export.c").compile("libcrabgrind.a");
}
