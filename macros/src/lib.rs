pub mod lib {
    // 'foo' macro is similar to the one described in https://doc.rust-lang.org/book/ch19-06-macros.html
    // except that instead of pushing each expression $x into the vector, the result of calling
    // Foo::method on $x is pushed into the vector.
    #[macro_export]
    macro_rules! foo {
        ( $( $x:expr ),* ) => {
            {
                use structs::traits::foo::Foo;

                let mut results = Vec::new();
                $(
                    results.push(Foo::method($x));
                )*
                results
            }
        };
    }
}
