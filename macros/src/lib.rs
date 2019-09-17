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
                    results.push(FooTrait::method($x));
                )*
                results
            }
        };
    }
}

pub mod bench {
    pub fn run() {
        use crate::foo;
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
