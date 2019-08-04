pub mod lib {
    use structs::lib::fat::Fat;
    use structs::traits::foo::Foo;
    use structs::traits::foo::Bar;

    pub type FatMethod = fn(&Fat) -> String;
    pub type FooMethod = fn(&Foo) -> String;
    pub type BarMethod = fn(&Bar) -> String;
    pub type GenMethod<T:Foo> = fn(T) -> String; 
    
    fn test(b: impl Bar) {

    }
}
