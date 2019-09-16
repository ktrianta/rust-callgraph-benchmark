use traits::lib::FooTrait;

pub struct Thin;

impl FooTrait for Thin {
    fn method(&self) -> String {
        format!("Thin+Foo")
    }

}
