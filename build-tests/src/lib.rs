#![allow(unused)]
include!("../../build.rs");

use bindings::{Error, Objectifying};
use std::{path::PathBuf, sync::LazyLock};

pub static FX: LazyLock<Vec<PathBuf>> = LazyLock::new(|| {
    std::env::var("CARGO_MANIFEST_DIR")
        .map(PathBuf::from)
        .expect("cargo should set <CARGO_MANIFEST_DIR>")
        .join("fixtures/object")
        .read_dir()
        .expect("'object/' should exist inside '<CARGO_MANIFEST_DIR>/fixtures'")
        .map(|e| e.unwrap().path())
        .filter(|entry| entry.is_dir())
        // fixtures/object
        .flat_map(|dir| dir.read_dir().unwrap().map(|e| e.unwrap().path()).filter(|p| p.is_dir()))
        // fixtures/object/[gcc|clang]
        .flat_map(|dir| {
            dir.read_dir()
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|e| e == "o"))
        })
        .collect()
});

const EXPECTED_SYMBOLS: [(&str, u32); 12] = [
    ("CG_TEST_ZERO", 0x00000000),
    ("CG_TEST_MAX", 0xFFFFFFFF),
    ("CG_TEST_HI", 0x80000000),
    ("CG_TEST_LO", 0x000000FF),
    ("CG_TEST_MI", 0xBEDABEDA),
    // duplicate
    ("CG_TEST_DUP", 0xBEDABEDA),
    // padding between symbols within same section
    ("CG_TEST_A", 0x12345678),
    ("CG_TEST_B", 0xAABBCCDD),
    // same section, different alignment
    ("CG_TEST_SECTION_A", 0x11111111),
    ("CG_TEST_ALIGN_16", 0x33333333),
    // same section, different alignment
    ("CG_TEST_SECTION_B", 0x22222222),
    ("CG_TEST_ALIGN_32", 0x44444444),
];

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn test_valid_objects_read() {
        assert_eq!(FX.len(), 28);

        for fix in FX.iter() {
            let obj_bytes = std::fs::read(fix).unwrap();
            let obj = Objectifying::this(&obj_bytes);

            assert!(obj.is_ok(), "{fix:?}: {:?}", obj.unwrap_err());
        }
    }

    #[test]
    fn test_value_read() {
        for fix in FX.iter() {
            let mut symbols: HashMap<&str, u32> = EXPECTED_SYMBOLS.iter().cloned().collect();

            let obj_bytes = std::fs::read(fix).unwrap();
            let obj = Objectifying::this(&obj_bytes).unwrap();
            for symbol in obj.symbols() {
                let name = symbol.name();
                if let Some(expected) = symbols.remove(name) {
                    assert_eq!(expected, symbol.read_uint().unwrap(), "{name}");
                }
            }

            assert!(symbols.is_empty(), "{:?} {fix:?}", symbols.into_keys().collect::<Vec<&str>>());
        }
    }
}
