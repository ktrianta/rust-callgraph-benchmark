use traits::lib::FooTrait;
use traits::lib::BarTrait;
use traits::lib::BazTrait;
use traits::lib::DefaultTrait;

pub struct Fat(pub u32);

impl Fat {
    pub fn method(&self) -> u32 {
        self.0
    }

    // Note the mut modifier on self. Method lookup is done for each type in order as described in
    // https://doc.rust-lang.org/reference/expressions/method-call-expr.html. &self and &mut self
    // are different types and &self methods are looked up first including the type's inherent
    // methods and any methods provided by a visible trait implemented by the type.
    pub fn another_method(&mut self) -> u32 {
        self.0 + 1
    }

    // Note the pub modifier missing from the method signature, rendering it invisible to the
    // outside world.
    #[allow(dead_code)]
    fn yet_another_method(&self) -> u32 {
        self.0 + 2
    }

    pub fn default_method_no_self() -> u32 {
       1
    }
}

impl FooTrait for Fat {
    fn method(&self) -> u32 {
        self.0 + 10
    }
}

impl BarTrait for Fat {
    fn method(&self) -> u32 {
        self.0 + 100
    }

    fn another_method(&self) -> u32 {
        self.0 + 101
    }

    fn yet_another_method(&self) -> u32 {
        self.0 + 102
    }
}

impl BazTrait for Fat {
    fn another_method(&self) -> u32 {
        self.0 + 1001
    }
}

impl DefaultTrait for Fat {
    fn default_method(&self) -> u32 {
        1
    }

    fn default_method_no_self() -> u32 {
        2
    }
}
