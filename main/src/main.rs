
mod helpers {
    // Accepts a function pointer as its argument and calls the function it points to.
    pub fn run_benchmark(bench: &fn () -> ()) {
        // There is a dedicated function pointers benchmark but it does not hurt to
        // check again here. This is a part of the benchmark after all.
        bench();

        // TODO: Define which functions should be considered by a call-graph generator
        // for the above call.
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
