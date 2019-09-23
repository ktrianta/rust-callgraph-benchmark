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
    pub fn monomorphized_foreign_bound<T: ForeignBoundTrait>(arg: T) -> i32 {
        // instance method call (trait)
        // <T as traits::lib::bounds::BoundTrait>::method
        // This call is similar to that of 'monomorphized' but the trait that bounds type parameter
        // T is defined in a different package than the function and the structs implementing it.
        arg.method()
    }

    pub fn impl_trait(arg: impl ForeignBoundTrait) -> i32 {
        // instance method call (trait)
        // <impl traits::lib::bound::BoundTrait as traits::lib::bounds::BoundTrait>::method
        // This call is equivalent to that of 'monomorphized_foreign_bound' as the compiler
        // produces the same ASM for both. However, produced MIR for the call differs slightly.
        arg.method()
    }

    // Generic function bounded by generic trait GenericFooTrait<T> (traits::lib::GenericFooTrait),
    // which is concretized by i32.
    pub fn monomorphized_i32<T: GenericFooTrait<i32>>(arg: T) -> i32 {
        // instance method call (trait)
        // <T as traits::lib::GenericFooTrait<i32>>::method
        arg.method()
    }

    // Generic function bounded by generic trait GenericFooTrait<T> (traits::lib::GenericFooTrait)
    // using also a where clause.
    pub fn monomorphized_where<T, P>(arg: T) -> P
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
    use crate::lib::impl_trait;
    use crate::lib::monomorphized;
    use crate::lib::monomorphized_i32;
    use crate::lib::monomorphized_where;
    use crate::lib::monomorphized_foreign_bound;
    use structs::lib::One as ForeignOne;
    use traits::lib::GenericFooTrait;

    impl GenericFooTrait<i32> for Two {
        fn method(&self) -> i32 {
            42
        }
    }
    
    pub fn run() {

        // No dynamic dispatch should be needed to resolve calls inside 'monomorphized' and
        // 'monomorphized_foreign_bound' as the compiler monomorphizes it during code generation.

        // static function call (monomorphized)
        // generics::lib::monomorphized::<structs::lib::One>
        let num1 = monomorphized(ForeignOne);

        // static function call (monomorphized)
        // generics::lib::monomorphized_foreign_bound::<generics::base::One>
        let num2 = monomorphized_foreign_bound(One);

        // static function call (monomorphized)
        // generics::lib::monomorphized_foreign_bound::<generics::base::Two>
        // Call with both types, generics::base::One and generics::base::Two, that implement
        // traits::lib::bounds::BoundTrait.
        let num3 = monomorphized_foreign_bound(Two);

        // static function call (monomorphized)
        // generics::lib::monomorphized_foreign_bound::<generics::base::Two>
        // Explicitly choose the conrete type as 'Two'.
        let num4 = monomorphized_foreign_bound::<Two>(Two);

        // static function call (monomorphized)
        // generics::lib::impl_trait::<generics::base::One>
        let num5 = impl_trait(One);

        // static function call (monomorphized)
        // generics::lib::monomorphized_i32::<generics::base::Two>
        let num6 = monomorphized_i32(Two);

        // static function call (monomorphized)
        // generics::lib::monomorphized_where::<generics::base::Two>
        // Call of function with generic parameter type T bounded by GenericFooTrait<P> and
        // concrete parameter type generics::base::Two which implements GenericFooTrait<i32>.
        let num7 = monomorphized_where(Two);

        // static method call (inherent monomorphized)
        // generics::base::Wrapper::new::<structs::lib::One>
        // Static method call implemented on generic struct Wrapper<T>
        let wrapper = Wrapper::<ForeignOne>::new(ForeignOne);

        // instance method call (inherent)
        // generics::base::Wrapper::method_wrapper::<structs::lib::One>
        // Instance method call implemented on generic struct Wrapper<T>
        let num8 = wrapper.method_wrapper();

        // This is here to ensure that the above calls are not optimized away as dead code.
        println!(
            "Just making sure no code is deemed dead by the compiler: {}",
            num1 + num2 + num3 + num4 + num5 + num6 + num7 + num8
        );
    }
}
