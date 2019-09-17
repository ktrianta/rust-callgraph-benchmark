pub mod bench {
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

pub mod bench_method_lookup {
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
