mod structs_bench {
    pub fn run() {
        use structs::lib::helloworld::Hello;
        use structs::lib::helloworld::HelloWorld;

        let h = Hello;
        let hi = Hello::hi();   // static method call
        let hey = Hello.hey();  // instance method call
        let hello = h.hello();  // instance method call

        let excitement = "!".to_string();
        let more_excitement = ToString::to_string("!!!");

        let mut hw = HelloWorld::new(excitement);         // static method call
        println!("{}", hw.greet(hi));                     // instance method call
        println!("{}", hw.greet_excited(hey));            // instance method call
        println!("{}", hw.greet_excited(hello.clone()));  // instance method call

        hw.update(more_excitement);                       // instance method call with &mut self
        println!("{}", hw.greet_excited(hello));          // instance method call
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

        // NOTE: The method calls that follow do not depend on any dynamic features and should be
        // fully resolved statically by the compiler. However, we include them for completeness of
        // our benchmark and to document several of the rules that govern method lookup.

        // Method 'method(&self) -> String' is provided by Fat but also from FooTrait and BarTrait,
        // which are implemented by Fat. However, method lookup considers Fat first according to
        // the rules at https://doc.rust-lang.org/reference/expressions/method-call-expr.html
        println!("{}", fat.method());  // equivalent to Fat::Method(&fat)

        // For the next 2 method calls we use the FooTrait to make explicit which method we want
        // to be invoked on fat and circumvent the method lookup.
        println!("{}", FooTrait::method(&fat));
        println!("{}", <Fat as FooTrait>::method(&fat));

        // Method 'another_method(&self) -> String' is provided in the current scope only by trait
        // BarTrait, as BazTrait is not in scope. Fat provides 'another_method(&mut self) -> String'.
        // As stated in https://doc.rust-lang.org/reference/expressions/method-call-expr.html,
        // &self methods are looked up first, thus the trait method is found before the struct's &mut
        // self method is found.
        println!("{}", fat.another_method());  // equivalent to BarTrait::another_method(&mut fat)

        // Method 'yet_another_method(&self) -> String' is provided by Fat and BarTrait. However,
        // this call is equivalent to 'BarTrait::yet_another_mathod(&fat)' as Fat has not made its
        // version publicly available by using the pub modifier, thus rendering it invisible here.
        println!("{}", fat.yet_another_method());

        {
            use traits::lib::BazTrait;

            // Traits BarTrait and BazTrait are both in scope here, meaning it is ambiguous whose
            // 'another_method' should be called if we write 'fat.another_method'. We use the traits
            // as, e.g., '<Fat as BarTrait>', to refer to the desired method so there is no ambiguity.
            println!("{}", <Fat as BarTrait>::another_method(&fat));
            println!("{}", <Fat as BazTrait>::another_method(&fat));
        }
    }
}

mod trait_objects_bench {
    pub fn run() {
        // struct::traits::foo::FooTrait is implemented for MyInt and MyString.
        use structs::lib::MyInt;
        use structs::lib::MyString;
        use traits::lib::FooTrait;

        // dynamic and dynamic_ufcs functions accept as argument a trait object
        // of type traits::lib::FooTrait and call a method with it as a receiver.
        // Dynamic dispatch is used to resolve these calls.
        // dynamic_ufcs uses fully quilified syntax for the method call.
        use dynamic::lib::dynamic;
        use dynamic::lib::dynamic_ufcs;


        let my_int = MyInt(10);
        let my_string = MyString("dummy".to_string());

        println!("{}", dynamic(&my_int));                        // &my_int is coerced to &FooTrait
        println!("{}", dynamic_ufcs(&my_int as &dyn FooTrait));  // &my_int is casted to &FooTrait

        println!("{}", dynamic(&my_string as &dyn FooTrait));    // &my_string is casted to &FooTrait
        println!("{}", dynamic_ufcs(&my_string));                // &my_string is coerced to &FooTrait

        // Casting to &FooTrait leads to slightly more MIR code to account for the casting.
        // The result should be the same but we include both for completeness.
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
        trait_objects_bench::run,
        function_pointers_bench::run,
        conditionally_compiled_bench::run,
        macros_bench::run,
    ];

    for bench in benchmarks.into_iter() {
        helpers::run_benchmark(bench);
    }
}
