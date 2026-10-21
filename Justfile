set shell := ["bash", "-c"]

check:
    cargo check --workspace --all-features
    cargo fmt --all -- --check
    cargo clippy --workspace --all-targets --all-features -- -D warnings

test:
    cargo test --workspace --all-features

play *args:
    cargo run --package an-open-sky-bin -- {{ args }}

