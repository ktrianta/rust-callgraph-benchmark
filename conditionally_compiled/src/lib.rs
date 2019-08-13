pub mod lib {
    use structs::traits::foo::Foo;
    use structs::traits::foo::Bar;

    #[cfg(feature = "foo")]
    pub fn foo(x: &dyn Foo) -> String {
        x.method()
    }

    #[cfg(not(feature = "foo"))]
    pub fn foo(x: &dyn Foo) -> String {
        String::new()
    }

    #[cfg(feature = "bar")]
    pub fn bar(x: &dyn Bar) -> String {
        x.another_method()
    }

    #[cfg(not(feature = "bar"))]
    pub fn bar(x: &dyn Bar) -> String {
        String::new()
    }
}
