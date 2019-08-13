use crate::traits::foo::Foo;

pub struct Thin;

impl Foo for Thin {
    fn method(&self) -> String {
        format!("Thin+Foo")
    }

}
