// Generics and Monomorphization
//
// The Rust compiler monomorphizes generic functions, structs, etc. As explained in the Rust
// Programming Language book[1], monomorphization is the process of producing specific code from
// generic code by filling in the concrete types that are used when compiled. Rustc executes this
// process during the LLVM code generation phase. The code produced by the compiler contains only
// concrete types, thus avoiding dynamic dispatch of method calls.
//
// [1]: https://doc.rust-lang.org/book/ch10-01-syntax.html

pub mod base;

pub mod lib {
    // We define the traits to serve as trait bounds of our generic functions both within the
    // current package (BoundTrait) and inside other packages (traits::lib::bounds::BoundTrait).
    use crate::base::BoundTrait;
    use traits::lib::GenericFooTrait;
    use traits::lib::bounds::BoundTrait as ForeignBoundTrait;

    // Generic function bounded by BoundTrait (generics::base::BoundTrait).
    // BoundTrait is implemented by structs::lib::One.
    pub fn monomorphized<T: BoundTrait>(arg: T) -> i32 {
        // instance method call (trait)
        // <T as generics::base::BoundTrait>::method
        arg.method()
    }

    // Generic function bounded by ForeignBoundTrait (traits::lib::bounds::BoundTrait).
    // ForeignBoundTrait is implemented by generics::base::One and generics::base::Two.
    pub fn other_monomorphized<T: ForeignBoundTrait>(arg: T) -> i32 {
        // instance method call (trait)
        // <T as traits::lib::bounds::BoundTrait>::method
        // The call is similar to that of 'monomorphized' but the trait that bounds type parameter
        // T is defined in a different package than the function and the structs implementing it.
        arg.method()
    }

    pub fn monomorphized_i32<T: GenericFooTrait<i32>>(arg: T) -> i32 {
        // instance method call (trait)
        // <T as traits::lib::GenericFooTrait<i32>>::method
        arg.method()
    }

    pub fn monomorphized_p<T, P>(arg: T) -> P
        where T: GenericFooTrait<P>
    {
        // instance method call (trait)
        // <T as traits::lib::GenericFooTrait<P>>::method
        arg.method()
    }
}

pub mod bench {
    use crate::base::One;
    use crate::base::Two;
    use crate::base::Wrapper;
    use crate::lib::monomorphized;
    use crate::lib::monomorphized_i32;
    use crate::lib::monomorphized_p;
    use crate::lib::other_monomorphized;
    use structs::lib::One as ForeignOne;
    use traits::lib::GenericFooTrait;

    impl GenericFooTrait<i32> for Two {
        fn method(&self) -> i32 {
            42
        }
    }
    
    pub fn run() {

        // No dynamic dispatch should be needed to resolve calls inside 'monomorphized' and
        // 'other_monomorphized' as the compiler monomorphizes it during code generation.

        // static function call (monomorphized)
        // generics::lib::monomorphized::<structs::lib::One>
        let num1 = monomorphized(ForeignOne);

        // static function call (monomorphized)
        // generics::lib::other_monomorphized::<generics::base::One>
        let num2 = other_monomorphized(One);

        // static function call (monomorphized)
        // generics::lib::other_monomorphized::<generics::base::Two>
        let num3 = other_monomorphized(Two);

        // static function call (monomorphized)
        // generics::lib::monomorphized_i32::<generics::base::Two>
        let num4 = monomorphized_i32(Two);

        // static function call (monomorphized)
        // generics::lib::monomorphized_P::<generics::base::Two>
        let num5 = monomorphized_p(Two);

        // static method call (inherent monomorphized)
        // generics::base::Wrapper::new::<structs::lib::One>
        let wrapper = Wrapper::<ForeignOne>::new(ForeignOne);

        // instance method call (inherent)
        // generics::base::Wrapper::method_wrapper::<structs::lib::One>
        let num6 = wrapper.method_wrapper();

        // This is here to ensure that the above calls are not optimized away as dead code.
        println!(
            "Just making sure no code is deemed dead by the compiler: {}",
            num1 + num2 + num3 + num4 + num5 + num6
        );
    }
}
