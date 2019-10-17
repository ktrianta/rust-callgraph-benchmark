// 'Macros' derive macro is inspired by https://doc.rust-lang.org/book/ch19-06-macros.html

extern crate proc_macro;

use crate::proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(Macros)]
pub fn macros_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_macros(&ast)
}

fn impl_macros(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MacroTrait for #name {
            fn method(&self) -> u32 {
                // instance method call (inherent)
                self.another_method()
            }

            fn another_method(&self) -> u32 {
                100
            }
        }
    };
    gen.into()
}
