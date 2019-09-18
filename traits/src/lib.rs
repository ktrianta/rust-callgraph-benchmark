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

    // TODO: fix
    pub trait GenericFooTrait<T> {
        fn method<K>(&self, _: T, _: K) -> u32;
    }

    pub mod bounds {
        pub trait BoundTrait {
            fn sub(&self) -> i32;
        }
    }
}
