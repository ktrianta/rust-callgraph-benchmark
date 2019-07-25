pub mod sub;

pub mod lib {
    pub use crate::sub::traits::SubTrait;

    // Generic function with trait bound on SubTrait.
    // This function will be monomorphized by the compiler, i.e., the generic code
    // will be turned into specific code leading to static dispatch of method sub.
    //
    // On generics and monomophization:
    // https://doc.rust-lang.org/book/ch10-01-syntax.html#performance-of-code-using-generics
    pub fn monomorphized<T: SubTrait>(arg: T) -> i32 {
        arg.sub()
    }
}
