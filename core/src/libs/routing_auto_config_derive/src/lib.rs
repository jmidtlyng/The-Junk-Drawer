extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;
use glob:glob;

#[proc_macro_derive(RoutingAutoConfig)]
pub fn routing_auto_config_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_routing_config(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let entry_path = &ast.ident;
    let entry_pattern = format!("{}/**", entry_path);
    
    for entry in glob(entry_pattern).expect("Failed to read entry path"){
        
    }
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}