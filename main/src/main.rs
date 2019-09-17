mod structs_bench {
    pub fn run() {
        use structs::lib::helloworld::Hello;
        use structs::lib::helloworld::HelloWorld;

        // Static method call: structs::lib::helloworld::Hello::hi
        let hi = Hello::hi();

        // Instance method call: structs::lib::helloworld::Hello::hello
        let hey = Hello.hello();

        let hello_instance = Hello;

        // Instance method call: structs::lib::helloworld::Hello::hello
        let hello = hello_instance.hello();

        // Instance method call with fully qualified syntax: std::string::ToString::to_string
        let excitement = ToString::to_string("!");

        // Instance method call: std::string::ToString::to_string
        let more_excitement = "!!".to_string();

        // Static method call: structs::lib::helloworld::HelloWorld::new
        let mut hw = HelloWorld::new(excitement);

        // Instance method call multiple arguments: structs::lib::helloworld::HelloWorld::greet
        hw.greet(&hi);

        // Instance method call with &mut self: structs::lib::helloworld::HelloWorld::update
        hw.update(hey + &more_excitement);

        // Instance method call with &mut self and multiple arguments:
        // structs::lib::helloworld::HelloWorld::greet_excited
        println!("{}", hw.greet_excited(&hello));
    }
}

mod traits_bench {
    pub fn run() {
        // Traits FooTrait, BarTrait and BazTrait are implemented by Fat.
        // BazTrait is not imported in the current scope.
        use structs::lib::fat::Fat;
        use traits::lib::FooTrait;
        use traits::lib::BarTrait;

        let fat = Fat(100);

        // NOTE: The method calls that follow do not depend on any dynamic features and are fully
        // resolved statically by the compiler. However, we include them for completeness of our
        // benchmark and to document several of the rules that govern method lookup.

        // Method 'method(&self) -> String' is provided by Fat but also from FooTrait and BarTrait,
        // which are implemented by Fat. Method lookup considers Fat first according to the rules
        // at https://doc.rust-lang.org/reference/expressions/method-call-expr.html.
        //
        // Instance method call: structs::lib::fat::Fat::method
        println!("{}", fat.method());

        // For the next 2 method calls we use fully qualified syntax to circumvent method lookup.
        //
        // Instance method call: structs::lib::fat::{impl FooTrait for Fat}::method
        println!("{}", FooTrait::method(&fat));

        // Instance method call: structs::lib::fat::{impl BarTrait for Fat}::method
        println!("{}", <Fat as BarTrait>::method(&fat));

        // Method 'another_method(&self) -> String' is provided in the current scope only by trait
        // BarTrait, as BazTrait is not in scope. Fat provides 'another_method(&mut self) -> String'
        // but as stated in https://doc.rust-lang.org/reference/expressions/method-call-expr.html,
        // &self methods are looked up first, thus the BarTrait method is selected.
        //
        // Instance method call: structs::lib::fat::{impl BarTrait for Fat}::another_method
        println!("{}", fat.another_method());

        // Method 'yet_another_method(&self) -> String' is provided by Fat and BarTrait. However,
        // Fat does not make it publicly available by using the pub modifier.
        //
        // Instance method call: structs::lib::fat::{impl BarTrait for Fat}::yet_another_method
        println!("{}", fat.yet_another_method());

        {
            use traits::lib::BazTrait;

            // Traits BarTrait and BazTrait are both in scope, meaning we should disambiguate a call
            // whose receiver is of type Fat. We are able to do this as, e.g., '<Fat as BarTrait>',
            // to refer to the desired method so there is no ambiguity.
            //
            // Instance method call: structs::lib::fat::{impl BarTrait for Fat}::another_method
            println!("{}", <Fat as BarTrait>::another_method(&fat));

            // Instance method call: structs::lib::fat::{impl BazTrait for Fat}::another_method
            println!("{}", <Fat as BazTrait>::another_method(&fat));
        }
    }
}


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
        structs_bench::run,
        traits_bench::run,
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
