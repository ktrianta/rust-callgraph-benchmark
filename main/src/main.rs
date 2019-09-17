mod function_pointers_bench {
    use structs::lib::fat::Fat;
    use structs::lib::thin::Thin;
    use traits::lib::FooTrait;
    use traits::lib::BarTrait;

    use function_pointers::lib::FatMethod;
    use function_pointers::lib::FooMethod;
    use function_pointers::lib::GenMethod;

    fn indirection_one(foo: &Fat, fun: &dyn Fn(&Fat) -> String) {
        println!("{}", fun(foo));
    }

    fn indirection_two(foo: &Fat, fun: FatMethod) {
        println!("{}", fun(foo));
    }

//    fn indirection_three(foo: impl FooTrait, fun: GenMethod<FooTrait>) {
//        //println!("{}", fun(foo));
//    }

//    fn indirection_four(t: &Thin, fun: &dyn Fn(&FooTrait) -> String) {
//        println!("{}", fun(t));
//    }

    pub fn run() {
        let f = Fat(10);

        indirection_one(&f, &Fat::method);
        indirection_one(&f, &<Fat as FooTrait>::method);

        indirection_two(&f, Fat::method);
        indirection_two(&f, FooTrait::method);
        indirection_two(&f, <Fat as BarTrait>::method);

        //fun(&f);
        //indirection_three(f, FooTrait::method);
        //indirection_four(&Thin, &fun);
    }
}

mod conditionally_compiled_bench {
    pub fn run() {
        use structs::lib::fat::Fat;
        use structs::lib::thin::Thin;
        use traits::lib::FooTrait;
        use traits::lib::BarTrait;
        use conditionally_compiled::lib::foo;
        use conditionally_compiled::lib::bar;

        let fat = Fat(100);
        let thin = Thin;
        let foo_items: Vec<&dyn FooTrait> = vec![&fat, &thin];
        let bar_items: Vec<&dyn BarTrait> = vec![&fat];  // Does not contain 'thin'. Compiler is aware
                                                         // that BarTrait is not implemented for Thin.

        for item in foo_items.into_iter() {
            let result = foo(item);

            // Function foo is conditionally compiled on feature 'foo'.
            // If 'foo' is not defined the returned value is the empty String.
            if !result.is_empty() {
                println!("{}", result);
            }
        }

        for item in bar_items.into_iter() {
            let result = bar(item);

            // Function bar is conditionally compiled on feature 'bar'.
            // If 'bar' is not defined the returned value is the empty String.
            if !result.is_empty() {
                println!("{}", result);
            }
        }
    }
}

mod macros_bench {
    pub fn run() {
        use macros::foo;
        use structs::lib::fat::Fat;
        use structs::lib::thin::Thin;

        let fat = Fat(1000);
        let thin = Thin;
        let results = foo![&fat, &thin];

        for result in results.iter() {
            println!("{}", result);
        }
    }
}

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
        function_pointers_bench::run,
        conditionally_compiled_bench::run,
        macros_bench::run,
    ];

    for bench in benchmarks.into_iter() {
        helpers::run_benchmark(bench);
    }
}
