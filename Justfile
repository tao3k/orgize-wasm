set dotenv-load := false

default:
    @just --list

fmt:
    cargo fmt --manifest-path Cargo.toml --all -- --check

test:
    CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=../target/orgize-wasm cargo test --manifest-path Cargo.toml --all-targets --all-features

clippy:
    CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=../target/orgize-wasm cargo clippy --manifest-path Cargo.toml --all-targets --all-features -- -D warnings

build:
    rm -rf dist
    CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=../target/orgize-wasm wasm-pack build -t web -d dist --out-name orgize

pack: build
    mkdir -p package
    npm pack --pack-destination package

ci: fmt test clippy build
