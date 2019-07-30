pub mod traits;

pub mod lib {
    pub mod fat;
    pub mod helloworld;

    use crate::traits::foo::Foo;

    pub struct MyInt(pub i32);
    
    impl Foo for MyInt {
        fn method(&self) -> String {
            format!("MyInt+Foo: int32 {}", self.0)
        }
    }

    pub struct MyString(pub String);

    impl Foo for MyString {
        fn method(&self) -> String {
            format!("MyString+Foo: string {}", self.0)
        }
    }
}
