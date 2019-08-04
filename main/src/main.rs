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
        // Traits Foo, Bar and Baz are implemented by Fat.
        // Baz is not imported in the current scope.
        use structs::lib::fat::Fat;
        use structs::traits::foo::Foo;
        use structs::traits::foo::Bar;

        let fat = Fat(100);

        // NOTE: The method calls that follow do not depend on any dynamic features and should be
        // fully resolved statically by the compiler. However, we include them for completeness of
        // our benchmark and to document several of the rules that govern method lookup.

        // Method 'method(&self) -> String' is provided by Fat but also from Foo and Bar traits,
        // which are implemented by Fat. However, method lookup considers Fat first according to
        // the rules at https://doc.rust-lang.org/reference/expressions/method-call-expr.html
        println!("{}", fat.method());  // equivalent to Fat::Method(&fat)

        // For the next 2 method calls we use the Foo trait to make explicit which method we want
        // to be invoked on fat and circumvent the method lookup.
        println!("{}", Foo::method(&fat));
        println!("{}", <Fat as Foo>::method(&fat));

        // Method 'another_method(&self) -> String' is provided in the current scope only by trait
        // Bar, as trait Baz is not in scope. Fat provides 'another_method(&mut self) -> String'.
        // As stated in https://doc.rust-lang.org/reference/expressions/method-call-expr.html,
        // &self methods are looked up first, thus the trait method is found before the struct's &mut
        // self method is found.
        println!("{}", fat.another_method());  // equivalent to Bar::another_method(&mut fat)

        // Method 'yet_another_method(&self) -> String' is provided by Fat and trait Bar. However,
        // the call here is equivalent to 'Bar::yet_another_mathod(&fat)' as Fat has not made its
        // version publicly available by using the pub modifier, thus rendering it invisible here.
        println!("{}", fat.yet_another_method());

        {
            use structs::traits::foo::Baz;

            // Traits Bar and Baz are both in scope here, meaning it is ambiguous whose
            // 'another_method' should be called if we write 'fat.another_method'. We use the traits
            // as, e.g., '<Fat as Bar>', to refer to the desired method so there is no ambiguity.
            println!("{}", <Fat as Bar>::another_method(&fat));
            println!("{}", <Fat as Baz>::another_method(&fat));
        }
    }
}

mod generics_bench {
    pub fn run() {
        // generics::sub::traits::SubTrait is implemented for Sub, Sub1 and Sub2.
        use generics::sub::sub::Sub;
        use generics::sub::sub1::Sub1;
        use generics::sub::sub2::Sub2;

        // monomorphized is a generic function with a trait bound on SubTrait.
        use generics::lib::monomorphized;

        let sub = Sub(100);
        let sub1 = Sub1(100);
        let sub2 = Sub2(100);

        // The calls to function monomorphized should be monomorphized.
        // No dynamic dispatch should be needed to resolve these calls.
        println!("{}", monomorphized(sub));
        println!("{}", monomorphized(sub1));
        println!("{}", monomorphized(sub2));
    }
}

mod trait_objects_bench {
    pub fn run() {
        // struct::traits::foo::Foo is implemented for MyInt and MyString.
        use structs::lib::MyInt;
        use structs::lib::MyString;
        use structs::traits::foo::Foo;

        // dynamic and dynamic_ufcs functions accept as argument a trait object
        // of type structs::traits::foo::Foo and call a method with it as a receiver.
        // Dynamic dispatch is used to resolve these calls.
        // dynamic_ufcs uses fully quilified syntax for the method call.
        use dynamic::lib::dynamic;
        use dynamic::lib::dynamic_ufcs;


        let my_int = MyInt(10);
        let my_string = MyString("dummy".to_string());

        println!("{}", dynamic(&my_int));                    // &my_int is coerced to &Foo
        println!("{}", dynamic_ufcs(&my_int as &dyn Foo));   // &my_int is casted to &Foo

        println!("{}", dynamic(&my_string as &dyn Foo));     // &my_string is casted to &Foo
        println!("{}", dynamic_ufcs(&my_string));            // &my_string is coerced to &Foo

        // Casting to &Foo leads to slightly more MIR code to account for the casting.
        // The result should be the same but we include both for completeness.
    }
}

mod function_pointers_bench {
    use structs::lib::fat::Fat;
    use structs::lib::fat::Thin;
    use structs::traits::foo::Foo;
    use structs::traits::foo::Bar;

    use function_pointers::lib::FatMethod;
    use function_pointers::lib::FooMethod;
    use function_pointers::lib::GenMethod;

    fn indirection_one(foo: &Fat, fun: &dyn Fn(&Fat) -> String) {
        println!("{}", fun(foo));
    }

    fn indirection_two(foo: &Fat, fun: FatMethod) {
        println!("{}", fun(foo));
    }

//    fn indirection_three(foo: impl Foo, fun: GenMethod<Foo>) {
//        //println!("{}", fun(foo));
//    }

//    fn indirection_four(t: &Thin, fun: &dyn Fn(&Foo) -> String) {
//        println!("{}", fun(t));
//    }

    pub fn run() {
        let f = Fat(10);

        indirection_one(&f, &Fat::method);
        indirection_one(&f, &<Fat as Foo>::method);

        indirection_two(&f, Fat::method);
        indirection_two(&f, Foo::method);
        indirection_two(&f, <Fat as Bar>::method);

        //fun(&f);
        //indirection_three(f, Foo::method);
        //indirection_four(&Thin, &fun);
    }
}

mod helpers {
    // Function accepts a function pointer as its argument.
    pub fn run_benchmark(bench: fn () -> ()) {
        // Which function calls should be included here for a call-graph to be sound?
        bench();
    }
}

fn main() {
    helpers::run_benchmark(structs_bench::run);
    helpers::run_benchmark(traits_bench::run);
    helpers::run_benchmark(generics_bench::run);
    helpers::run_benchmark(trait_objects_bench::run);
    helpers::run_benchmark(function_pointers_bench::run);
}
