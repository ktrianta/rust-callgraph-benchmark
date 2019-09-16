pub mod lib {
    use structs::lib::fat::Fat;
    use traits::lib::FooTrait;
    use traits::lib::BarTrait;

    pub type FatMethod = fn(&Fat) -> String;
    pub type FooMethod = fn(&FooTrait) -> String;
    pub type BarMethod = fn(&BarTrait) -> String;
    pub type GenMethod<T:FooTrait> = fn(T) -> String;
    
    fn test(b: impl BarTrait) {

    }
}
