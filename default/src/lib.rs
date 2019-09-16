pub mod lib {
    pub trait Default {
        fn default_method(&self) -> String {
            "Default".to_string()
        }

        fn default_method_no_self() -> String where Self: Sized {
            "Default".to_string()
        }
    }
}

