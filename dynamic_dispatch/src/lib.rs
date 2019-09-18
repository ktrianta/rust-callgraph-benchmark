pub mod lib {
    use traits::lib::FooTrait;

    // dynamic* functions accept as argument a trait object of type traits::lib::FooTrait and
    // call 'method' on it. Dynamic dispatch is used to resolve these method calls.
    pub fn dynamic(x: &dyn FooTrait) -> u32 {
        // Dynamic dispatch: traits::lib::FooTrait::method
        x.method()
    }

    pub fn dynamic_ufcs(x: &dyn FooTrait) -> u32 {
        // Fully qualified syntax dynamic dispatch: traits::lib::FooTrait::method
        FooTrait::method(x)
    }
}

pub mod bench {
    pub fn run() {
        use crate::lib::dynamic;
        use crate::lib::dynamic_ufcs;
        use structs::lib::fat::Fat;
        use structs::lib::thin::Thin;
        use traits::lib::FooTrait;

        let fat = Fat(10);
        let thin = Thin;

        // Static function call: dynamic_dispatch::lib::dynamic
        println!("{}", dynamic(&fat));                          // &my_int is coerced to &FooTrait

        // Static function call: dynamic_dispatch::lib::dynamic_ufcs
        println!("{}", dynamic_ufcs(&fat as &dyn FooTrait));    // &my_int is casted to &FooTrait

        // Static function call: dynamic_dispatch::lib::dynamic
        println!("{}", dynamic(&thin as &dyn FooTrait));        // &my_string is casted to &FooTrait

        // Static function call: dynamic_dispatch::lib::dynamic_ufcs
        println!("{}", dynamic_ufcs(&thin));                    // &my_string is coerced to &FooTrait

        // Casting to &FooTrait leads to slightly more MIR code to account for the cast operation.
        // Otherwise the result is the same with that of the coerced version, but we include both
        // versions for completeness.
    }
}
