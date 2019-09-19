use structs::lib::One as ForeignOne;
use traits::lib::bounds::BoundTrait as ForeignBoundTrait;

pub struct One;
pub struct Two;

pub trait BoundTrait {
    fn method(&self) -> i32;
}

impl BoundTrait for ForeignOne {
    fn method(&self) -> i32 {
        1
    }
}

impl ForeignBoundTrait for One {
    fn method(&self) -> i32 {
        1
    }
}

impl ForeignBoundTrait for Two {
    fn method(&self) -> i32 {
        2
    }
}

pub struct Wrapper<T>(T);

impl <T: BoundTrait> Wrapper<T> {
    pub fn new(object: T) -> Self {
        Wrapper(object)
    }

    pub fn method_wrapper(&self) -> i32 {
        // instance method call (trait)
        // <T as generics::base::BoundTrait>::method
        // Generic receiver method call inside generic struct.
        self.0.method()
    }
}
