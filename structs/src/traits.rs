pub mod foo {
    use default::lib::Default;

    pub trait Foo {
        fn method(&self) -> String;
    }

    pub trait Bar {
        fn method(&self) -> String;
        fn another_method(&self) -> String;
        fn yet_another_method(&self) -> String;
    }

    pub trait Baz : Default {
        fn another_method(&self) -> String;
    }
}
