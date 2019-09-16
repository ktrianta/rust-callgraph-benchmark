
pub mod lib {
    pub mod fat;
    pub mod thin;
    pub mod helloworld;

    use traits::lib::FooTrait;

    pub struct MyInt(pub i32);

    impl FooTrait for MyInt {
        fn method(&self) -> String {
            format!("MyInt+Foo: int32 {}", self.0)
        }
    }

    pub struct MyString(pub String);

    impl FooTrait for MyString {
        fn method(&self) -> String {
            format!("MyString+Foo: string {}", self.0)
        }
    }
}
