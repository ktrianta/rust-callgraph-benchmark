mod helpers {
    // Accepts a function pointer as its argument and calls the function it points to.
    pub fn run_benchmark(bench: &fn () -> ()) {
        // There is a dedicated function pointers benchmark. However, it does not hurt to test the
        // call-graph generator again at this point. This is part of the benchmark after all.

        // TODO: Define which functions should be included in a call-graph for the call to 'bench'.
        // All the functions with matching signatures that are available at this scope and are
        // declared in the current crate or one of its dependencies should be included.
        bench();

    }
}

fn main() {
    let benchmarks = [
        static_dispatch::bench::run,
        static_dispatch::bench_method_lookup::run,
        generics::bench::run,
        dynamic_dispatch::bench::run,
        function_pointers::bench::run,
        conditionally_compiled::bench::run,
        macros::bench::run,
    ];

    for bench in benchmarks.into_iter() {
        helpers::run_benchmark(bench);
    }
}
