pub mod foo {
    pub trait Foo {
        fn method(&self) -> String;
    }

    pub trait Bar {
        fn method(&self) -> String;
        fn another_method(&self) -> String;
        fn yet_another_method(&self) -> String;
    }

    pub trait Baz {
        fn another_method(&self) -> String;
    }
}
