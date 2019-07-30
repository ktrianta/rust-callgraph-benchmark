pub use structs::traits::foo::Foo;

pub mod lib {
    use super::Foo;

    pub fn dynamic(x: &dyn Foo) -> String {
        x.method()  // dynamic dispatch
    }

    pub fn dynamic_ufcs(x: &dyn Foo) -> String {
        Foo::method(x)  // fully qualified syntax dynamic dispatch
    }
}
