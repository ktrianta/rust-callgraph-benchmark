use crate::sub::traits::SubTrait;

pub struct Sub2(pub i32);

impl SubTrait for Sub2 {
    fn sub(&self) -> i32 {
        self.0 - 2
    }
}
