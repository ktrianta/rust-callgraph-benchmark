// Dynamic dispatch
//
// Dynamic dispatch in Rust is tightly coupled with trait objects. The set of target methods of a
// dynamic dispatch call might include all overriding methods of the trait's implementations as
// part of a sound call-graph. For example, this would be the output of a Class Hierarchy Analysis,
// whereas other analyses such as Pointers Analysis might be able to give more precise results in
// certain cases.

pub mod lib {
    use traits::lib::FooTrait;
    use traits::lib::DefaultTrait;
    use traits::lib::GenericFooTrait;

    // 'dynamic' and 'dynamic_ufcs' functions accept as argument a trait object of type
    // traits::lib::FooTrait and call 'method' on it. Dynamic dispatch is used to resolve these
    // method calls.
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

    // 'dynamic_default' function accepts as argument a trait object with default methods of type
    // traits::lib::GenericFooTrait<T> and calls 'default_method' on it. Dynamic dispatch is used to
    // resolve this method call.
    pub fn dynamic_default(x: &dyn DefaultTrait) -> u32 {
        // instance method call (trait)
        // traits::lib::DefaultTrait::default_method
        // Dynamic dispatch on trait object with default methods.
        x.default_method()
    }

    // 'dynamic_generic' function accepts as argument a generic trait object of type
    // traits::lib::GenericFooTrait<T> and calls 'method' on it. Dynamic dispatch is used to
    // resolve this method call.
    pub fn dynamic_generic<T>(x: &dyn GenericFooTrait<T>) -> T {
        // instance method call (trait)
        // traits::lib::GenericTrait<T>::method
        // Dynamic dispatch on generic trait object.
        GenericFooTrait::<T>::method(x)
    }
}

pub mod bench {
    pub fn run() {
        use crate::lib::dynamic;
        use crate::lib::dynamic_ufcs;
        use crate::lib::dynamic_default;
        use crate::lib::dynamic_generic;
        use structs::lib::fat::Fat;
        use structs::lib::thin::Thin;
        use traits::lib::FooTrait;
        use traits::lib::GenericFooTrait;

        let fat = Fat(10);
        let thin = Thin;

        // static function call
        // dynamic_dispatch::lib::dynamic
        // The dynamic dispatch call happens inside function 'dynamic'.
        let num1 = dynamic(&fat);  // &fat is coerced to &FooTrait

        // static function call
        // dynamic_dispatch::lib::dynamic_ufcs
        // Casting to &dyn FooTrait generates slightly more MIR code to account for the cast
        // operation. We include it along the coercion version for completeness.
        let num2 = dynamic_ufcs(&fat as &dyn FooTrait);  // &fat is cast to &FooTrait

        // static function call
        // dynamic_dispatch::lib::dynamic_default
        // Dynamic dispatch on DefaultTrait trait object with default method 'default_method'.
        // structs::lib::fat::Fat implements DefaultTrait overriding 'default_method' whereas
        // structs::lib::thin::Thin does not. Here a reference to &Fat is passed to
        // 'dynamic_default'. However, if an analysis does not consider references and pointers,
        // in order to be sound, it should take into account all possible implementations of
        // DefaultTrait and the fact that some might not implement its default methods.
        let num3 = dynamic_default(&fat);

        // static function call
        // dynamic_dispatch::lib::dynamic_generic
        // Casting to the concrete type of generic trait GenericFooTrait for disambiguation, as
        // Thin implements both GenericFooTrait<i32> and GenericFooTrait<u32>, which match generic
        // type parameter GenericFooTrait<T>.
        let num4 = dynamic_generic(&thin as &dyn GenericFooTrait<u32>);

        // NOTE: In the above calls a precise pointer analysis would be able to compute that only
        // objects of type Fat are passed to function 'dynamic' and 'dynamic_ufcs' under the
        // specific context, whereas an object of type Thin is passed to function 'dynamic_generic'.
        // In contrast a more imprecise analysis like Class Hierarchy Analysis should assume that
        // objects of all allowed types could be passed as arguments. The generated call-graph would
        // be different in these cases.

        let vec: Vec<&dyn FooTrait> = vec![&fat, &thin];
        let mut num5 = 0;

        for item in vec.iter() {
            // instance method call (trait)
            // traits::lib::FooTrait::method
            // Dynamic dispatch on referenced vector elements.
            num5 += item.method();
        }

        // This is here to ensure that the above calls are not optimized away as dead code.
        println!(
            "Just making sure no code is deemed dead by the compiler: {}",
            num1 + num2 + num3 + num4 + num5
        );
    }
}
