// Dynamic dispatch
//
// Dynamic dispatch in Rust is tightly coupled with trait objects. The set of target methods of a
// dynamic dispatch call might include all overriding methods of the trait's implementations as
// part of a sound call-graph. For example, this would be the output of a Class Hierarchy Analysis,
// whereas other analyses such as Pointers Analysis might be able to give more precise results in
// certain cases.

pub mod lib {
    use traits::lib::FooTrait;

    // 'dynamic*' functions accept as argument a trait object of type traits::lib::FooTrait and
    // call 'method' on it. Dynamic dispatch is used to resolve these method calls.
    pub fn dynamic(x: &dyn FooTrait) -> u32 {
        // instance method call (trait)
        // traits::lib::FooTrait::method
        // Dynamic dispatch.
        x.method()
    }

    pub fn dynamic_ufcs(x: &dyn FooTrait) -> u32 {
        // instance method call (trait)
        // traits::lib::FooTrait::method
        // Dynamic dispatch with fully qualified syntax.
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

        // static function call
        // dynamic_dispatch::lib::dynamic
        // The dynamic dispatch call happens inside function 'dynamic'.
        dynamic(&fat);  // &my_int is coerced to &FooTrait

        // static function call
        // dynamic_dispatch::lib::dynamic_ufcs
        // Casting to &dyn FooTrait generates slightly more MIR code to account for the cast
        // operation. We include it along the coercion version for completeness.
        dynamic_ufcs(&fat as &dyn FooTrait);  // &my_int is casted to &FooTrait

        // NOTE: In the above two calls a precise Pointer Analysis would be able to compute that
        // only objects of type Fat are passed to function 'dynamic' and 'dynamic_ufcs' under the
        // specific context. In contrast a more imprecise analysis like Class Hierarchy Analysis
        // should assume that objects of all allowed types could be passed as arguments. The
        // generated call-graph would be different in these cases.

        let thin = Thin;
        let vec: Vec<&dyn FooTrait> = vec![&fat, &thin];

        for item in vec.iter() {
            // instance method call (trait)
            // traits::lib::FooTrait::method
            // Dynamic dispatch on referenced vector elements.
            item.method();
        }
    }
}
