use traits::lib::FooTrait;
use traits::lib::GenericFooTrait;

pub struct Thin;

impl FooTrait for Thin {
    fn method(&self) -> u32 {
        0
    }
}

impl GenericFooTrait<i32> for Thin {
    fn method(&self) -> i32 {
        42
    }
}

impl GenericFooTrait<u32> for Thin {
    fn method(&self) -> u32 {
        42
    }
}
