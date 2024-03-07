#!/bin/sh

# Compile the crate with the nightly toolchain
# and MIR encoding always enabled
cargo clean
rustup override set $CGG_TOOLCHAIN
RUSTFLAGS="-Z always_encode_mir" cargo build

# Run MIRAI's call graph generator
touch src/lib.rs && RUSTFLAGS="-Z always_encode_mir" RUSTC_WRAPPER=mirai MIRAI_FLAGS="--call_graph_config=$CGG_PATH" cargo build

# Produce a PDF of the generated graph
dot -Tpdf graph.dot -o graph.pdf
