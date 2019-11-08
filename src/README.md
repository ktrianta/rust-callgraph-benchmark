# Benchmark Packages

The benchmark is organized into several packages, each one targeting a specific type of call, e.g.,
**static dispatch** or **dynamic dispatch** calls.

Next we present each package by providing a general description and descriptions for several "types"
of calls with links to the actual benchmark code. The benchmark includes several more calls, which
can be accessed by following the links of the example calls or by simply browsing the benchmark
code. For each "type" of call at least the following documentation is provided, as demonstrated in
the following example:

```Rust
// instance method call (trait)
// traits::lib::FooTrait::method
// Dynamic dispatch with fully qualified syntax.
FooTrait::method(x)
```

The first comment line gives a description of the "type" of the call. A call could be static or
instance call, function or method call, and it could be defined in a struct's implementation
(inherent) or in a trait's implementation (trait).

The second comment line specifies the target functions/methods. In dynamic scenarios such as
dynamic dispatch calls and function pointer calls the target method/function is the trait
defined method and the function employed by the function pointer respectively.

The third comment line gives a more detailed description of the call and the reasons behind
its inclusion in the benchmark.

## main
- Description:
    Main benchmark code. Calls the benchmarks defined in the supporting benchmark packages.
- Crates:
    1 binary
- Depends on:
    **traits**, **structs**, **static_dispatch**, **dynamic_dispatch**, **generics**,
    **function\_pointers**, **conditionally\_compiled**, **macros**

## traits
- Description:
    Trait definitions used by the individual benchmarks.
- Crates:
    1 library
- Depends on:
    nothing

## structs
- Description:
    Struct definitions and struct and trait implementations used by the individual benchmarks.
- Call examples:
    * [Call to inherent public method inside another method.][1]
    * [Call to inherent private method inside another method.][2]
- Crates:
    1 library
- Depends on:
    **traits**

[1]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/structs/src/lib.rs#L32
[2]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/structs/src/lib.rs#L37

## static\_dispatch
- Description:
    Static and instance method calls implemented directly on type `T` or provided by a trait
    implemented by `T`. Test cases that target different calling conventions but also touch upon
    static method resolution and lookup.
- Call examples:
    * [Static method call (inherent).][3]
    * [Instance method call (inherent).][4]
    * [Instance method call (inherent) on newly allocated object.][5]
    * [Instance method call (trait) using fully qualified syntax.][6]
- Crates:
    1 library
- Depends on:
    **traits**, **structs**

[3]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/static_dispatch/src/lib.rs#L8
[4]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/static_dispatch/src/lib.rs#L13
[5]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/static_dispatch/src/lib.rs#L18
[6]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/static_dispatch/src/lib.rs#L69

## dynamic\_dispatch
- Description:
    Trait objects and dynamic dispatch.
- Call examples:
    * [Dynamic dispatch.][7]
    * [Dynamic dispatch (qualified syntax).][8]
    * [Dynamic dispatch on generic trait object.][9]
    * [Dynamic dispatch on referenced vector elements.][10]
- Crates:
    1 library
- Depends on:
    **structs**, **traits**

[7]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/dynamic_dispatch/src/lib.rs#L21
[8]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/dynamic_dispatch/src/lib.rs#L28
[9]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/dynamic_dispatch/src/lib.rs#L45
[10]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/dynamic_dispatch/src/lib.rs#L108

## generics
- Description:
    Generic trait bounded functions and structs that are monomorphized during compilation.
- Call examples:
    * [Call on generic type receiver (trait bounded).][11]
    * [Call on generic type receiver (trait bounded by a generic trait).][12]
    * [Call on generic type receiver (trait bounded by a generic trait which is concretized).][13]
- Crates:
    1 library
- Depends on:
    **structs**, **traits**

[11]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/generics/src/lib.rs#L25
[12]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/generics/src/lib.rs#L61
[13]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/generics/src/lib.rs#L51

## function\_pointers
- Description:
    Function pointer and Fn trait instance calls.
- Call examples:
    * [Call via function pointer.][14]
    * [Call via generic function pointer.][15]
    * [Call of an Fn trait instance.][16]
- Crates:
    1 library
- Depends on:
    **structs**

[14]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/function_pointers/src/lib.rs#L13
[15]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/function_pointers/src/lib.rs#L20
[16]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/function_pointers/src/lib.rs#L44

## conditionally\_compiled
- Description:
    Conditionally compiled functions.
- Crates:
    1 library
- Depends on:
    **structs**

## macros
- Description:
    Function and method calls inside macros.
- Call examples:
    * [Call inside declarative macro.][17]
    * [Method definition and call generated by derive macro.][18]
- Crates:
    1 library
- Depends on:
    **structs**

[17]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/macros/src/lib.rs#L16
[18]: https://github.com/ktrianta/rust-callgraph-benchmark/blob/4b0c6d42e34106958a9d894cf40c51c7c3ac0201/src/macros/src/lib.rs#L45
