pub mod lib {
    pub trait FooTrait {
        fn method(&self) -> u32;
    }

    pub trait BarTrait {
        fn method(&self) -> u32;
        fn another_method(&self) -> u32;
        fn yet_another_method(&self) -> u32;
    }

    pub trait BazTrait {
        fn another_method(&self) -> u32;
    }

    pub trait GenericFooTrait<T> {
        fn method(&self) -> T;
    }

    pub mod bounds {
        pub trait BoundTrait {
            fn method(&self) -> i32;
        }
    }

    pub trait MacroTrait {
        fn method(&self) -> u32;
        fn another_method(&self) -> u32;
    }
}
