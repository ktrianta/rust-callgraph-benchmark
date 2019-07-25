use crate::sub::traits::SubTrait;

pub struct Sub1(pub i32);

impl SubTrait for Sub1 {
    fn sub(&self) -> i32 {
        self.0 - 1
    }
}
