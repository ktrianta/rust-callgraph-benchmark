use crate::sub::traits::SubTrait;

pub struct Sub(pub i32);

impl SubTrait for Sub {
    fn sub(&self) -> i32 {
        self.0
    }
}
