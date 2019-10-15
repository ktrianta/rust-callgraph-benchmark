pub mod bench {
    pub fn run() {
        use structs::lib::One;
        use structs::lib::Two;

        // static method call (inherent)
        // structs::lib::One::method_1
        let num1 = One::method_1();

        // instance method call (inherent)
        // structs::lib::One::method_2
        let mut one = One;
        let num2 = one.method_2();

        // instance method call (inherent)
        // structs::lib::One::method_2
        // Generates slightly different mir code than 'one.method_2()'.
        let num3 = One.method_2();

        // static method call (inherent)
        // structs::lib::Two::new
        // Returns the Self type.
        let mut two = Two::new(0);

        // instance method call (inherent)
        // structs::lib::Two::method_1
        // Same name as structs::lib::One::method_1 but different signature and definition path.
        let num4 = two.method_1();

        // instance method call (inherent)
        // structs::lib::Two::method_2
        // Same name and signature as structs::lib::One::method_2 but different definition path.
        let num5 = two.method_2();

        // This is here to ensure that the above calls are not optimized away as dead code.
        println!(
            "Just making sure no code is deemed dead by the compiler: {}",
            num1 + num2 + num3 + num4 + num5
        );
    }
}

pub mod bench_method_lookup {
    pub fn run() {
        // Traits FooTrait, BarTrait and BazTrait are implemented by Fat.
        // BazTrait is not imported in the current scope and thus is not visible.
        use structs::lib::fat::Fat;
        use structs::lib::thin::Thin;
        use traits::lib::FooTrait;
        use traits::lib::BarTrait;
        use traits::lib::DefaultTrait;

        let fat = Fat(100);

        // NOTE: The method calls that follow do not depend on any dynamic features and are fully
        // resolved statically by the compiler. However, we include them for completeness of our
        // benchmark and to document several of the rules that govern method lookup.

        // instance method call (inherent)
        // structs::lib::fat::Fat::method
        // Method lookup should resolve this call to structs::lib::fat::Fat::method and not to the
        // methods defined in the implementations of FooTrait or BarTrait by Fat as described in
        // https://doc.rust-lang.org/reference/expressions/method-call-expr.html.
        let num1 = fat.method();

        // instance method call (trait)
        // structs::lib::fat::{impl FooTrait for Fat}::method
        // Fully qualified syntax call circumvents method lookup.
        let num2 = FooTrait::method(&fat);

        // static method call (inherent)
        // structs::lib::fat::Fat::default_method_no_self
        let num3 = Fat::default_method_no_self();

        // static method call (trait)
        // structs::lib::fat::{impl DefaultTrait for Fat}::default_method_no_self
        // Called method overrides the default implementation defined in traits::lib::DefaultTrait
        let num4 = <Fat as DefaultTrait>::default_method_no_self();

        // static method call (trait default)
        // traits::lib::DefaultTrait::default_method_no_self
        // Only implementation of 'default_method_no_self' is the default one of DefaultTrait.
        // Equivalent to '<Thin as DefaultTrait>::default_method_no_self();'
        let num5 = Thin::default_method_no_self();

        // instance method call (trait)
        // structs::lib::fat::{impl BarTrait for Fat}::method
        // Fully qualified syntax `<T as TraitRef>::item` circumvents method lookup.
        let num6 = <Fat as BarTrait>::method(&fat);

        // instance method call (trait)
        // structs::lib::fat::{impl BarTrait for Fat}::another_method
        // BazTrait is not in scope and Fat provides 'another_method(&mut self) -> u32', but as
        // stated in https://doc.rust-lang.org/reference/expressions/method-call-expr.html, &self
        // methods are looked up first, thus the call is resolved to BarTrait's method.
        let num7 = fat.another_method();

        // instance method call (trait)
        // structs::lib::fat::{impl BarTrait for Fat}::yet_another_method
        // Method structs::lib::fat::Fat::yet_another_method is not public, thus the call to
        // yet_another_method is resolved to that of BarTrait's implementation by Fat.
        let num8 = fat.yet_another_method();

        {
            use traits::lib::BazTrait;

            // Fully qualified syntax is required to disambiguate a call to another_method as both
            // structs::lib::fat::{impl BarTrait for Fat}::another_method and
            // structs::lib::fat::{impl BazTrait for Fat}::another_method are in scope now.

            // instance method call (trait)
            // structs::lib::fat::{impl BarTrait for Fat}::another_method
            let num9 = <Fat as BarTrait>::another_method(&fat);

            // instance method call (trait)
            // structs::lib::fat::{impl BazTrait for Fat}::another_method
            let num10= <Fat as BazTrait>::another_method(&fat);

            // This is here to ensure that the above calls are not optimized away as dead code.
            println!(
                "Just making sure no code is deemed dead by the compiler: {}",
                num1 + num2 + num3 + num4 + num5 + num6 + num7 + num8 + num9 + num10
            );
        }
    }
}
