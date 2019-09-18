pub mod lib {
    pub mod fat;
    pub mod thin;

    pub struct One;

    impl One {
        pub fn method_1() -> i32 {
            1
        }

        pub fn method_2(&mut self) -> i32 {
            2
        }
    }

    pub struct Two(i32);

    impl Two {
        pub fn new(num: i32) -> Self {
            Two(num)
        }

        pub fn method_1(&self) -> i32 {
            self.0 + 1
        }

        pub fn method_2(&mut self) -> i32 {
            // instance method call (inherent)
            // structs::lib::Two::add_one
            // Call to inherent private method inside another method.
            self.add_one();

            // instance method call (inherent)
            // structs::lib::Two::method_1
            // Call to inherent public method inside another method.
            self.method_1()
        }

        fn add_one(&mut self) {
            self.0 = self.0 + 1;
        }
    }
}
