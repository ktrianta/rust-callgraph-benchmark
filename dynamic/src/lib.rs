pub mod lib {
    use traits::lib::FooTrait;

    pub fn dynamic(x: &dyn FooTrait) -> String {
        x.method()  // dynamic dispatch
    }

    pub fn dynamic_ufcs(x: &dyn FooTrait) -> String {
        FooTrait::method(x)  // fully qualified syntax dynamic dispatch
    }
}
