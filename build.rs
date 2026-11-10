//! Build script for native library wrapping the Valgrind [Client Request API](https://valgrind.org/docs/manual/manual-core-adv.html#manual-core-adv.clientreq)

#[cfg(feature = "valgrind")]
fn env_path(var: &str) -> Result<std::path::PathBuf, std::env::VarError> {
    std::env::var(var).map(std::path::PathBuf::from)
}

#[cfg(feature = "valgrind")]
mod bindings {
    /*
    Why not `bindgen`?
    `crabgrind` needs numeric values of Valgrind client request identifiers
    as Rust compile-time constants.
    These identifiers are defined in Valgrind's C headers in the build environment,
    sometimes through multiple layers of macros and constant expressions.
    `bindgen` can extract these definitions, but it depends on `libclang`,
    which introduces an additional host-side dependency and configuration
    burden, particularly for cross-compilation.
    Instead, we let the C compiler do what it already does best: preprocess
    Valgrind's headers and evaluate their constant expressions.

    The process is:
    - Generate a small C translation unit referencing the request identifiers
    - Compile it using the target C compiler into an object file
    - Use the awesome `object` crate to extract initialized values of exported symbols
    - Generate Rust constants and configuration flags from those values

    This avoids parsing and evaluating C ourselves, avoids `libclang`, and does not
    require executing target binaries, so cross-compilation is supported.

    The object file serves only as a means of retrieving values already
    evaluated by the C compiler. Using the `object` crate(awesome one) lets us do
    this without relying on architecture-specific assembly or compiler-specific
    output parsing, agnostic to the supported target platform's object file format.

    And also, it's just cool to say: "By the way, my build.rs fishes for constants inside
    the object file" with a smug face.
    */

    use super::env_path;
    use object::{Object, ObjectSection, ObjectSymbol};
    use std::{
        collections::BTreeMap,
        fmt::Write,
        fs, io,
        mem::size_of,
        path::{Path, PathBuf},
    };

    const SYM_PREFIX: &str = "CG_";
    const C_UINT: &str = "uint32_t";
    const C_NDEF: &str = "0";

    type Uint = u32;

    #[derive(Debug)]
    #[allow(dead_code)]
    pub enum Error {
        InvalidDefinition { line: usize, reason: &'static str },
        DuplicateDefinition(String),
        MissingSymbol(String),
        InvalidSymbol(String),
        Io(io::Error),
        CC(cc::Error),
        Object(object::Error),
        UnexpectedObjectKind(object::ObjectKind),
    }

    impl From<io::Error> for Error {
        fn from(e: io::Error) -> Self {
            Self::Io(e)
        }
    }

    impl From<cc::Error> for Error {
        fn from(e: cc::Error) -> Self {
            Self::CC(e)
        }
    }

    impl From<object::Error> for Error {
        fn from(e: object::Error) -> Self {
            Self::Object(e)
        }
    }

    impl From<(usize, &'static str)> for Error {
        fn from((line, reason): (usize, &'static str)) -> Self {
            Self::InvalidDefinition { line, reason }
        }
    }

    type Result<T, E = Error> = std::result::Result<T, E>;

    struct Definition<'a> {
        request: &'a str,
        code: &'a str,
        version: u32,
    }

    struct Bindgen {
        variants: String,
        versions: String,
        codes: String,
    }

    impl Bindgen {
        fn new() -> Self {
            Self {
                variants: String::with_capacity(4 << 10),
                versions: String::with_capacity(4 << 10),
                codes: String::with_capacity(4 << 10),
            }
        }

        pub fn insert(&mut self, name: &str, value: u32, ver: u32) {
            writeln!(&mut self.variants, "{SYM_PREFIX}{name},").expect("infallible");
            writeln!(&mut self.codes, "Self::{SYM_PREFIX}{name} => {value},").expect("infallible");
            writeln!(&mut self.versions, "Self::{SYM_PREFIX}{name} => {ver},").expect("infallible");
        }

        fn finalize(self) -> String {
            let Self { variants, versions, codes } = self;
            format!(
                "
                #[derive(Debug, Copy, Clone, PartialEq, Eq)]
                pub enum CR {{ {variants} }}
                impl CR {{
                    pub const fn code(self) -> u32 {{
                        match self {{ {codes} }}
                    }}
                    pub const fn version(self) -> u32 {{
                        match self {{ {versions} }}
                    }}
                    pub const fn is_defined(self) -> bool {{
                        // Valgrind requests starts from 0x1001
                        self.code() > 0x1000
                    }}
                }}"
            )
        }

        pub fn write_to_file(self, dst: &Path) -> io::Result<()> {
            std::fs::write(dst, self.finalize())
        }
    }

    fn parse_definition((n, line): (usize, &str)) -> Result<Definition<'_>> {
        let err = |reason: &'static str| Error::from((n, reason));

        let (request, rest) = line.split_once('=').ok_or(err("missing '='"))?;
        let (code, ver) = rest.split_once(',').ok_or(err("missing ','"))?;
        let version = ver.trim().parse().map_err(|_| err("invalid version"))?;
        let request = request.trim();
        let code = code.trim();

        if request.is_empty() {
            return Err(err("empty request"));
        }

        if code.is_empty() {
            return Err(err("empty code"));
        }

        Ok(Definition { request, code, version })
    }

    fn parse_defs(defs: &str) -> Result<BTreeMap<&str, (&str, u32)>> {
        let defs_iter = defs
            .lines()
            .enumerate()
            .filter_map(|(n, line)| {
                let line = line.trim();
                (!line.is_empty() && !line.ends_with(':')).then_some((n + 1, line))
            })
            .map(parse_definition);

        let mut defs = BTreeMap::new();

        for def in defs_iter {
            let Definition { request, code, version } = def?;

            if defs.insert(request, (code, version)).is_some() {
                return Err(Error::DuplicateDefinition(request.into()));
            }
        }

        Ok(defs)
    }

    fn c_defs(defs: &BTreeMap<&str, (&str, u32)>) -> String {
        fn const_ifdef(defn: &str, val: &str, def: &str, pfx: &str, out: &mut String) {
            writeln!(
                out,
                "const {C_UINT} {pfx}{defn} =\n\
                #ifdef {defn}\n {val}\n#else\n {def}\n#endif\n;"
            )
            .expect("infallible");
        }

        let mut cdefs = String::with_capacity(2 << 10);
        cdefs.push_str(
            r#"#include <stddef.h>
#include <stdint.h>
#include <limits.h>
#include "valgrind/valgrind.h"
#include "valgrind/callgrind.h"
#include "valgrind/helgrind.h"
#include "valgrind/drd.h"
#include "valgrind/memcheck.h"
#include "valgrind/dhat.h"
#include "valgrind/cachegrind.h"

_Static_assert(CHAR_BIT == 8, "8-bit bytes required");
_Static_assert(sizeof(uint32_t) == 4, "uint32_t must occupy 4 bytes");
            "#,
        );

        for (req, (code, _)) in defs {
            const_ifdef(req, code, C_NDEF, SYM_PREFIX, &mut cdefs);
        }

        cdefs
    }

    fn object_compile(source: &Path, output: &Path, valgrind_include: &[PathBuf]) -> Result<()> {
        let mut builder = cc::Build::new();

        for path in valgrind_include {
            builder.include(path);
        }

        // We'd want `cc::Build::compile_intermediates()` here, but it doesn't let us
        // control the ordering of compiler flags. In particular, `-flto` can be
        // injected through environment variables (e.g. `CFLAGS`), producing an LTO object
        // instead of a regular object file.
        // So we're reusing `cc`'s compiler discovery and configuration, and append `-fno-lto`
        // last to ensure it takes precedence.
        let status = builder
            .inherit_rustflags(false)
            .flags(["-idirafter", "valgrind/include"])
            .try_get_compiler()?
            .to_command()
            .arg("-c")
            .arg(source)
            .arg("-o")
            .arg(output)
            .args(["-O0", "-fno-lto"])
            .status()?;

        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, format!("cc failed: {status}")).into());
        }

        Ok(())
    }

    fn compile_cdefs(src: String, valgrind_include: &[PathBuf]) -> Result<Vec<u8>> {
        let out_dir = env_path("OUT_DIR").expect("infallible");
        let source = out_dir.join("c_defs.c");
        let object = out_dir.join("c_defs.o");

        fs::write(&source, src)?;
        object_compile(&source, &object, valgrind_include)?;

        fs::read(object).map_err(Into::into)
    }

    fn decode_uint(
        bytes: &[u8],
        little_endian: bool,
    ) -> Result<Uint, std::array::TryFromSliceError> {
        <[u8; size_of::<Uint>()]>::try_from(bytes).map(|bytes| {
            if little_endian {
                Uint::from_le_bytes(bytes)
            } else {
                Uint::from_be_bytes(bytes)
            }
        })
    }

    fn read_sym(obj: &object::File, sym: &object::Symbol) -> Result<Uint> {
        let name = sym.name()?;
        let invalid_sym = || Error::InvalidSymbol(name.into());

        let sec_index = sym.section_index().ok_or_else(invalid_sym)?;
        let section = obj.section_by_index(sec_index)?;

        let offset = sym.address().checked_sub(section.address()).ok_or_else(invalid_sym)?;
        let offset = usize::try_from(offset).map_err(|_| invalid_sym())?;
        let end = offset.checked_add(size_of::<Uint>()).ok_or_else(invalid_sym)?;

        let bytes = section.data()?.get(offset..end).ok_or_else(invalid_sym)?;

        decode_uint(bytes, obj.is_little_endian())
            .map_err(|_| Error::InvalidSymbol(format!("{name}: Invalid integer width.")))
    }

    fn symbol_name<'a>(sym: &'a object::Symbol, fmt: object::BinaryFormat) -> Option<&'a str> {
        let name = sym.name().ok()?;
        match fmt {
            // Mach-O adds leading '_' to externally visible symbols
            object::BinaryFormat::MachO => name.strip_prefix('_').unwrap_or(name),
            _ => name,
        }
        .into()
    }

    fn extract_bindings(
        obj: &object::File,
        mut defs: BTreeMap<&str, (&str, u32)>,
    ) -> Result<(Bindgen, bool)> {
        let fmt = obj.format();
        let mut bindgen = Bindgen::new();
        let mut any_defined = false;

        for sym in obj
            .symbols()
            .filter(|sym| sym.is_definition() && sym.kind() == object::SymbolKind::Data)
        {
            let Some(name) = symbol_name(&sym, fmt).and_then(|n| n.strip_prefix(SYM_PREFIX)) else {
                continue;
            };
            let Some((_, ver)) = defs.remove(name) else {
                continue;
            };
            let value = read_sym(obj, &sym)?;

            // All fallback definitions are zero. A nonzero value therefore indicates
            // that request was defined by the real Valgrind headers.
            any_defined |= value != 0;

            bindgen.insert(name, value, ver);
        }

        if !defs.is_empty() {
            let symbols = defs.into_keys().collect::<Vec<_>>().join(", ");
            return Err(Error::MissingSymbol(symbols));
        }

        Ok((bindgen, any_defined))
    }

    pub fn generate(valgrind_include: &[PathBuf]) -> Result<bool> {
        let request_defs = fs::read_to_string("valgrind/client_request.defs")?;
        let defs = parse_defs(&request_defs)?;

        let obj_bytes = compile_cdefs(c_defs(&defs), valgrind_include)?;
        let obj = object::File::parse(obj_bytes.as_slice())?;

        if obj.kind() != object::ObjectKind::Relocatable {
            return Err(Error::UnexpectedObjectKind(obj.kind()));
        }

        let (bindings, any_defined) = extract_bindings(&obj, defs)?;

        bindings.write_to_file(&env_path("OUT_DIR").expect("infallible").join("bindings.rs"))?;

        Ok(any_defined)
    }
}

#[cfg(feature = "valgrind")]
mod imp {
    use std::path::{Path, PathBuf};

    const ENV_VALGRIND_INCLUDE: &str = "VALGRIND_INCLUDE";
    const WARN_HEADERS_NOT_FOUND: &str = "Valgrind headers were not found during the build.\n\
    This typically means Valgrind headers ain't found on the standard include paths:\n\
    \t<sysroot>/usr/include\n\
    \t<sysroot>/usr/local/include\n\
    \t...\n\
    \tnor via 'pkg-config' probe.\n\
    You might try 'VALGRIND_INCLUDE=<path to valgrind/include>' as a quick workaround.\n\
    Without the headers, executing Valgrind client requests will panic at runtime.\n\
    Build configuration doc: https://docs.rs/crabgrind#build-configuration";

    fn env_include() -> Option<PathBuf> {
        let Ok(path) = super::env_path(ENV_VALGRIND_INCLUDE) else { return None };
        assert!(path.exists(), "{ENV_VALGRIND_INCLUDE}={} Path does not exist", path.display());
        assert!(
            path.join("valgrind").join("valgrind.h").exists(),
            "{ENV_VALGRIND_INCLUDE}={} does not contain `valgrind/valgrind.h`",
            path.display()
        );
        Some(path)
    }

    fn pkgconfig_include() -> Option<Vec<PathBuf>> {
        pkg_config::Config::new()
            .cargo_metadata(false)
            .env_metadata(false)
            .print_system_libs(false)
            .print_system_cflags(true)
            .probe("valgrind")
            .ok()
            .map(|lib| {
                lib.include_paths
                    .into_iter()
                    .map(|path| path.parent().map(Path::to_path_buf).unwrap_or(path))
                    .collect()
            })
    }

    fn valgrind_include_paths() -> Vec<PathBuf> {
        if let Some(include) = env_include() {
            return vec![include];
        }

        pkgconfig_include().unwrap_or_default()
    }

    fn build_native(valgrind_include: &[PathBuf]) {
        let mut builder = cc::Build::new();

        for path in valgrind_include {
            builder.include(path);
        }

        builder
            .flags(["-idirafter", "valgrind/include"])
            .file("valgrind/native.c")
            .compile("native");
    }

    pub fn main() {
        println!("cargo:rerun-if-changed=valgrind/native.c");
        println!("cargo:rerun-if-changed=valgrind/client_request.defs");
        println!("cargo:rerun-if-env-changed={ENV_VALGRIND_INCLUDE}");
        println!("cargo:rerun-if-env-changed=TARGET");

        let include = valgrind_include_paths();

        build_native(&include);

        match super::bindings::generate(&include) {
            Ok(true) => println!("cargo:rustc-cfg=valgrind_available"),
            Ok(false) => {
                for line in WARN_HEADERS_NOT_FOUND.lines() {
                    println!("cargo:warning={line}");
                }
            }
            Err(err) => panic!("Failed to generate bindings: {err:?}"),
        }
    }
}

fn main() {
    // `valgrind_available` indicates that Valgrind headers were found during the build,
    // and client request bindings were successfully generated for that Valgrind installation.
    println!("cargo:rustc-check-cfg=cfg(valgrind_available)");

    // `has_core_error` indicates that `core::error::Error` is available
    // on the current compiler.
    println!("cargo:rustc-check-cfg=cfg(has_core_error)");

    // MSRV(1.71) compatibility workaround
    if rustversion::cfg!(since(1.81)) {
        println!("cargo:rustc-cfg=has_core_error");
    }

    #[cfg(feature = "valgrind")]
    imp::main();
}
