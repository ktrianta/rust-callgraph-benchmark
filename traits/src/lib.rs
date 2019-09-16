pub mod lib {
    pub trait FooTrait {
        fn method(&self) -> String;
    }

    pub trait BarTrait {
        fn method(&self) -> String;
        fn another_method(&self) -> String;
        fn yet_another_method(&self) -> String;
    }

    pub trait BazTrait {
        fn another_method(&self) -> String;
    }

    pub trait GenericFooTrait<T> {
        fn method<K>(&self, _: T, _: K) -> String;
    }

    pub mod bounds {
        pub trait BoundTrait {
            fn sub(&self) -> i32;
        }
    }
}
