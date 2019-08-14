pub mod lib {
    use structs::traits::foo::Foo;
    use structs::traits::foo::Bar;

    // There exist two versions of function 'foo' and which one is compiled depends on whether
    // feature foo is defined during compilation. If it is, foo returns the result of calling
    // the 'method' method of trait object x. Otherwise, it returns the empty string.
    #[cfg(feature = "foo")]
    pub fn foo(x: &dyn Foo) -> String {
        x.method()
    }

    #[cfg(not(feature = "foo"))]
    pub fn foo(_x: &dyn Foo) -> String {
        String::new()
    }

    // There exist two versions of function 'bar' similar to those of function 'foo'.
    #[cfg(feature = "bar")]
    pub fn bar(x: &dyn Bar) -> String {
        x.another_method()
    }

    #[cfg(not(feature = "bar"))]
    pub fn bar(_x: &dyn Bar) -> String {
        String::new()
    }
}
