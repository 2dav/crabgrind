# that
default:
	just --list

# Build library and integration tests
build: check
	cargo build --no-default-features
	cargo build --all-features
	cargo test --release --no-run 

# Run linter
check: cspell
	cargo clippy --no-default-features
	cargo clippy --all-features

# Spell check
cspell:
	cspell lint .

# Cleanup all build and tests artifacts
clean: clean-valgrind-out
	cargo clean

# Clean various valgrind tool's output and crush dumps
clean-valgrind-out:
	find . -type f -regex '.*\(callgrind\|cachegrind\|dhat\|vgcore\)\(\.out\)?\.[0-9]+.*' -exec rm -f {} +

# Generate doc
doc:
	cargo doc

# Test integration and documentation
test: check test-doc
	cargo test --release

# Test doc examples
test-doc:
	cargo test --doc

# Generate wrapper.h and valgrind_version.rs
[working-directory: 'valgrind']
wrapper:
	python gen_wrapper.py wrapper.h valgrind_version.rs
