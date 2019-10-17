pub mod lib {
    // 'foo' macro is similar to the one described in https://doc.rust-lang.org/book/ch19-06-macros.html
    // except that instead of pushing each expression $x into the vector, the result of calling
    // Foo::method on $x is pushed into the vector.
    #[macro_export]
    macro_rules! foo {
        ( $( $x:expr ),* ) => {
            {
                use traits::lib::FooTrait;

                let mut results = Vec::new();
                $(
                    // instance method call (trait)
                    // structs::lib::fat::{impl FooTrait for type_of($x)}::method
                    // Fully qualified syntax call circumvents method lookup.
                    let result = FooTrait::method($x);

                    // instance method call (inherent)
                    // std::vec::Vec::push
                    results.push(result);
                )*
                results
            }
        };
    }

    use macros_derive::Macros;
    use traits::lib::MacroTrait;

    #[derive(Macros)]
    pub struct MacroStruct;
}

pub mod bench {
    pub fn run() {
        use crate::foo;
        use crate::lib::MacroStruct;
        use structs::lib::fat::Fat;
        use structs::lib::thin::Thin;
        use traits::lib::MacroTrait;

        // instance method call (trait)
        // macros::lib::{impl MacroTrait for MacroStruct}::method
        // Implementation of method is generated using a derive macro.
        let num1 = MacroStruct.method();

        let mut num2 = 0;
        let results = foo![&Fat(1000), &Thin];

        for result in results.iter() {
            num2 += result;
        }

        // This is here to ensure that the above calls are not optimized away as dead code.
        println!(
            "Just making sure no code is deemed dead by the compiler: {}",
            num1 + num2
        );
    }
}
