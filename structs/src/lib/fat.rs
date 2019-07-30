use crate::traits::foo::Foo;
use crate::traits::foo::Bar;
use crate::traits::foo::Baz;

pub struct Fat(pub u32);

impl Fat {
    pub fn method(&self) -> String {
        format!("Fat: uint32 {}", self.0)
    }

    // Note the mut modifier on self.
    // As described in https://doc.rust-lang.org/reference/expressions/method-call-expr.html
    // method lookup is done for each type in order, which can sometimes lead to surprising
    // results. &self and &mut self are different types and &self methods are looked up first
    // including the type's inherent methods and any methods provided by a visible trait
    // implemented by the type.
    pub fn another_method(&mut self) -> String {
        self.0 += 1;
        format!("Fat: another_method")    
    }

    // Note the pub modifier that is missing.
    // This method cannot be called from the outside world.
    #[allow(dead_code)]
    fn yet_another_method(&self) -> String {
        format!("Fat: yet_another_method")
    }
}

impl Foo for Fat {
    fn method(&self) -> String {
        format!("Fat+Foo: uint32 {}", self.0)
    }

}

impl Bar for Fat {
    fn method(&self) -> String {
        format!("Fat+Bar: uint32 {}", self.0)
    }

    fn another_method(&self) -> String {
        format!("Fat+Bar: another_method")    
    }

    fn yet_another_method(&self) -> String {
        format!("Fat+Bar: yet_another_method")    
    }
}

impl Baz for Fat {
    fn another_method(&self) -> String {
        format!("Fat+Baz: another_method")    
    }
}