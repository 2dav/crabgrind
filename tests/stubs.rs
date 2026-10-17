#![cfg(not(feature = "valgrind"))]

include!("native.rs");

#[test]
fn valgrind_available() {
    assert!(crabgrind::VALGRIND_AVAILABLE == false);
}
