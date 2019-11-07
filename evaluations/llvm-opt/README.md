# LLVM opt

## Determine LLVM opt's version

The version of LLVM opt should be compatible with rustc's LLVM version. To determine which version
of LLVM rustc uses run:

```bash
rustc -v -V
```

## Generate LLVM bitcode

By disabling incremental compilation we get all the generated code in one file.

```bash
CARGO_INCREMENTAL=0 cargo rustc -- --emit=llvm-bc -C codegen-units=1 -o output-name
```

## Generate call-graph

```bash
# opt output is written to callgraph.dot
opt -dot-callgraph output-name-<some-id>.bc -o /dev/null

# Create PostScript or PNG file from callgraph.dot
dot -Tps callgraph.dot > callgraph.eps
dot -Tpng callgraph.dot > callgraph.png
```


