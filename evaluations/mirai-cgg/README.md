# MIRAI-CGG

## Prerequisites

This call graph generator uses MIRAI and a nightly version of Rust.

1. Please install [MIRAI](https://github.com/facebookexperimental/MIRAI) by following its
[installation instructions](https://github.com/facebookexperimental/MIRAI/blob/master/documentation/InstallationGuide.md).

2. Set an environment variable for the path to call graph configuration file (from this directory): 
`export CGG_PATH=$(pwd)/call_graph_config.json`.

3. Note that MIRAI uses a nightly version of Rust. Please set the following environment 
variable to  point to the 
[current](https://github.com/facebookexperimental/MIRAI/blob/master/rust-toolchain) 
nightly toolchain version.
Example using "nightly-2021-07-29": `export CGG_TOOLCHAIN="nightly-2021-07-29"`


## Running the call graph generator

The following instructions apply to running MIRAI-CGG over a particular crate
(e.g., [static_dispatch](../../src/static_dispatch)).

1. Run the following commands to build the crate with the nightly toolchain:
    - `cargo clean`
    - `rustup override set $CGG_TOOLCHAIN`
    - `RUSTFLAGS="-Z always_encode_mir" cargo build`

The next command uses the previously set environment variable `$CGG_PATH` which 
should point to [call_graph_config.json](./call_graph_config.json) in this directory.

2. Execute MIRAI with the `--call_graph_config` option:
    - `touch src/lib.rs && RUSTFLAGS="-Z always_encode_mir" RUSTC_WRAPPER=mirai MIRAI_FLAGS="--call_graph_config=$CGG_PATH" cargo build`

Running the above command should produce a `graph.dot` file in the current directory
(the directory of the crate being analyzed).

3. Create PDF or PNG version of call graph:
    - PDF: `dot -Tpdf graph.dot -o graph.pdf`
    - PNG: `dot -Tpng graph.dot -o graph.pdf`

Please note that when running MIRAI, additional diagnostic messages may be produced
(e.g. `"possible attempt to add with overflow"`). These are part of MIRAI's safety
checks.

## Using the provided script

The above steps have been packaged into `mirai_call_graph.sh` for convenience.
Please review the script and adjust it as needed for your setup.

To use the `mirai_call_graph.sh` script, e.g. from 
[src/static_dispatch](../../src/static_dispatch):
- `../../evaluations/mirai-cgg/mirai_call_graph.sh`

A dot file (`graph.dot`) and a PDF (`graph.pdf`) file will be produced in the 
current directory.
