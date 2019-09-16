pub mod base;

pub mod lib {
    // We define the traits to serve as trait bounds of our generic functions both within the
    // current package (BoundTrait) and inside other packages (traits::lib::bounds::BoundTrait).
    pub use crate::base::BoundTrait;
    pub use traits::lib::bounds::BoundTrait as ForeignBoundTrait;

    // Generic function bounded by BoundTrait (crate::base::BoundTrait).
    pub fn monomorphized<T: BoundTrait>(arg: T) -> i32 {
        arg.sub()
    }

    // Generic function bounded by ForeignBoundTrait (traits::lib::bounds::BoundTrait).
    pub fn other_monomorphized<T: ForeignBoundTrait>(arg: T) -> i32 {
        arg.sub()
    }

    // These functions will be monomorphized by the compiler, i.e., during LLVM code generation the
    // compiler will produce specific code according to the available type information. This
    // optimization subsequentily enables static dispatch of the call 'arg.sub'.
    //
    // On generics and monomophization:
    // https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
}

pub mod bench {
    pub fn run() {
        use crate::base::Sub1;
        use crate::base::Sub2;
        use crate::lib::other_monomorphized;

        let sub1 = Sub1(100);
        let sub2 = Sub2(100);

        // No dynamic dispatch should be needed to resolve calls inside 'other_monomorphized' as
        // the compiler monomorphizes it during code generation.
        other_monomorphized(sub1);
        other_monomorphized(sub2);
    }
}
