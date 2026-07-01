# that
default:
	just --list

# Build library and integration tests
build: check doc
	cargo build
	cargo build --no-default-features
	cargo test --release --no-run 

# Run linter
check: cspell mdlint
	cargo clippy
	cargo clippy --no-default-features

# Spell check
cspell:
	cspell lint .

# Markdown linting
mdlint:
	markdownlint doc README.md

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
	cargo test --release --no-default-features

# Test doc examples
test-doc:
	cargo test --doc

# Generate wrapper.h and valgrind_version.rs
[working-directory: 'valgrind']
wrapper:
	python gen_wrapper.py wrapper.h valgrind_version.rs
