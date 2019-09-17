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

pub mod bench {
    use structs::lib::fat::Fat;
    use structs::lib::thin::Thin;
    use traits::lib::FooTrait;
    use traits::lib::BarTrait;

    use crate::lib::FatMethod;
    use crate::lib::FooMethod;
    use crate::lib::GenMethod;

    fn indirection_one(foo: &Fat, fun: &dyn Fn(&Fat) -> String) {
        println!("{}", fun(foo));
    }

    fn indirection_two(foo: &Fat, fun: FatMethod) {
        println!("{}", fun(foo));
    }

//    fn indirection_three(foo: impl FooTrait, fun: GenMethod<FooTrait>) {
//        //println!("{}", fun(foo));
//    }

//    fn indirection_four(t: &Thin, fun: &dyn Fn(&FooTrait) -> String) {
//        println!("{}", fun(t));
//    }

    pub fn run() {
        let f = Fat(10);

        indirection_one(&f, &Fat::method);
        indirection_one(&f, &<Fat as FooTrait>::method);

        indirection_two(&f, Fat::method);
        indirection_two(&f, FooTrait::method);
        indirection_two(&f, <Fat as BarTrait>::method);

        //fun(&f);
        //indirection_three(f, FooTrait::method);
        //indirection_four(&Thin, &fun);
    }
}
