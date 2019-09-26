pub mod lib {
    use structs::lib::fat::Fat;
    use traits::lib::FooTrait;

    pub type FatMethod = fn(&Fat) -> u32;
    pub type FooMethod = fn(&dyn FooTrait) -> u32;
    pub type GenMethod<T> = fn(&T) -> u32;

    pub fn indirection(foo: &Fat, fun: FatMethod) -> u32 {
        // function pointer call
        // for<'r> fn(&'r structs::lib::fat::Fat) -> u32
        // Call via function pointer 'fun'.
        fun(foo)
    }

    pub fn indirection_generic<T: FooTrait>(foo: &T, fun: GenMethod<T>) -> u32 {
        // function pointer call
        // for<'r> fn(&'r T) -> u32
        // Call via generic function pointer 'fun'.
        fun(foo)
    }

    // Equivalent to indirection(foo: &Fat, fun: FatMethod) -> u32
    // fn indirection_concretized_generic(foo: &Fat, fun: GenMethod<Fat>) -> u32 {
    //     fun(foo)
    // }

    pub fn indirection_trait_object(foo: &dyn FooTrait, fun: FooMethod) -> u32 {
        // function pointer call
        // for<'r> fn(&'r (dyn traits::lib::FooTrait + 'r)) -> u32
        // Call via function pointer 'fun', which accepts a trait object as argument.
        fun(foo)
    }

    // Equivalent to indirection_trait_object(foo: &dyn FooTrait, fun: FooMethod) -> u32
    // fn indirection_sixth(foo: impl FooTrait + 'static, fun: GenMethod<dyn FooTrait>) -> u32 {
    //     fun(&foo)
    // }

    pub fn indirection_fn_trait(foo: &Fat, fun: &dyn Fn(&Fat) -> u32) -> u32 {
        // instance method call (trait - std::ops::Fn::call)
        // &dyn for<'r> std::ops::Fn(&'r structs::lib::fat::Fat) -> u32
        // Call of Fn trait instance 'fun'.
        fun(foo)
    }
}

pub mod bench {
    use crate::lib::indirection;
    use crate::lib::indirection_generic;
    use crate::lib::indirection_trait_object;
    use crate::lib::indirection_fn_trait;
    use structs::lib::fat::Fat;
    use traits::lib::BarTrait;
    use traits::lib::FooTrait;

    mod helpers {
        use super::FooTrait;

        pub fn m1(obj: &dyn FooTrait) -> u32 {
            obj.method()
        }

        pub fn m2(obj: &dyn FooTrait) -> u32 {
            obj.method()
        }
    }

    pub fn run() {
        let f = Fat(10);

        // static function call
        // function_pointers::lib::indirection
        // Pointed function is part of Fat's implementation (struct impl).
        indirection(&f, Fat::method);

        // static function call
        // function_pointers::lib::indirection
        // Pointed function is part of FooTrait's implementation by Fat (trait impl).
        indirection(&f, FooTrait::method);

        // static function call
        // function_pointers::lib::indirection
        // Pointed function is part of BarTrait's implementation by Fat (trait impl). The syntax
        // used to specify the method is slightly different than in the last testcase but normally
        // there should not be any significant difference. We include this case for completeness.
        indirection(&f, <Fat as BarTrait>::method);

        // static function call
        // function_pointers::lib::indirection_generic
        // Pointed function is generic.
        indirection_generic(&f, Fat::method);
        // The following two calls should be covered by the 'indirection' testcases.
        // indirection_generic(&f, FooTrait::method);
        // indirection_generic(&f, <Fat as BarTrait>::method);

        // static function call
        // function_pointers::lib::indirection_generic
        // Pointed function accepts a trait object as an argument.
        indirection_trait_object(&f, helpers::m2);

        // static function call
        // function_pointers::lib::indirection_fn_trait
        indirection_fn_trait(&f, &Fat::method);
        // The following two calls should be covered by the 'indirection' testcases.
        // indirection_fn_trait(&f, &BarTrait::method);
        // indirection_fn_trait(&f, &<Fat as FooTrait>::method);
    }
}
