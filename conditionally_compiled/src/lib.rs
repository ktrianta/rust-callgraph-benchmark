// TODO: Define how conditionally compiled code should be handled by call-graph generators.

pub mod lib {
    // Function 'bar' calls the version of 'foo' function that is compiled depending on whether
    // feature 'foo' is defined or not during compilation.
    pub fn bar() -> u32 {
        // static function call
        // conditionally_compiled::lib::foo
        // Function 'foo' is conditionally compiled.
        foo()
    }

    #[cfg(feature = "foo")]
    pub fn foo() -> u32 {
        // static function call
        // conditionally_compiled::lib::base_one
        base_one()
    }

    #[cfg(not(feature = "foo"))]
    pub fn foo() -> u32 {
        // static function call
        // conditionally_compiled::lib::base_two
        base_two()
    }

    #[allow(dead_code)]
    fn base_one() -> u32 {
        1
    }

    #[allow(dead_code)]
    fn base_two() -> u32 {
        2
    }
}

pub mod bench {
    pub fn run() {
        use crate::lib::bar;
        use crate::lib::foo;

        // static function call
        // conditionally_compiled::lib::bar
        // Function 'bar' calls the conditionally compiled function 'foo'.
        let num1 = bar();

        // static function call
        // conditionally_compiled::lib::foo
        // Function 'foo' is conditionally compiled on feature 'foo'.
        let num2 = foo();

        // This is here to ensure that the above calls are not optimized away as dead code.
        println!(
            "Just making sure no code is deemed dead by the compiler: {}",
            num1 + num2
        );
    }
}
