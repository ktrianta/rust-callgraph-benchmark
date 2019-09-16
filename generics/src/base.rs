pub struct Sub1(pub i32);
pub struct Sub2(pub i32);

pub trait BoundTrait {
    fn sub(&self) -> i32;
}

impl BoundTrait for Sub1 {
    fn sub(&self) -> i32 {
        self.0 - 1
    }
}

use traits::lib::bounds::BoundTrait as ForeignBoundTrait;

impl ForeignBoundTrait for Sub1 {
    fn sub(&self) -> i32 {
        self.0 - 10
    }
}

impl ForeignBoundTrait for Sub2 {
    fn sub(&self) -> i32 {
        self.0 - 20
    }
}

