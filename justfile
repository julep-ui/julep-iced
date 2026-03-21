# plushie-iced - Development Tasks
#
# Run `just` to see available recipes.
# Run `just preflight` before pushing to catch CI failures locally.

export RUSTFLAGS := "-D warnings"
export ICED_TEST_BACKEND := "tiny-skia"

default:
    @just --list

# === CI Preflight ===

preflight: check lint fmt doc test
    @echo ""
    @echo "All preflight checks passed!"

# === Individual Checks ===

check:
    cargo check --workspace --all-targets

lint:
    cargo lint

fmt:
    cargo fmt --all -- --check --verbose

# Checks docs build. CI uses nightly for docsrs cfg; locally we skip
# that flag and just verify the docs compile without broken links.
doc:
    cargo doc --no-deps --workspace

test:
    cargo test --verbose --workspace
    cargo test --verbose --workspace -- --ignored
    cargo test --verbose --workspace --all-features

# === Build Variants ===

build:
    cargo build --workspace

build-release:
    cargo build --release --workspace

# === Development Helpers ===

format:
    cargo fmt --all

lint-fix:
    cargo lint-fix

test-filter pattern:
    cargo test --workspace -- {{pattern}}

test-crate crate:
    cargo test -p {{crate}}

clean:
    cargo clean

docs:
    cargo doc --workspace --open

# === Watch Mode ===

watch-check:
    cargo watch -x 'check --workspace --all-targets'

watch-test:
    cargo watch -x 'test --workspace'

# === Dependency Health ===

audit:
    cargo audit

outdated:
    cargo outdated --workspace
