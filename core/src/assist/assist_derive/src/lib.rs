extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

use sailfish::TemplateOnce;

#[proc_macro_derive(Assist)]
pub fn assist_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_assist(&ast)
}

fn impl_assist(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[derive(TemplateOnce)]
        #[template(path = "hello.stpl")]
        struct Locals {
            
        }
        
        impl Assist for #name {
            fn assist() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}