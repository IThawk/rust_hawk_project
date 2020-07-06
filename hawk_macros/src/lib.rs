// extern crate proc_macro;
// use proc_macro::TokenStream;
// use proc_macro_hack::proc_macro_hack;
// use quote::quote;
// use syn::{parse_macro_input, Expr};
#[macro_export]
macro_rules! format1 {
    () => {
        1 + 3
    };
}

// #[allow(unused_extern_crates)]
// extern crate proc_macro;
//
// use proc_macro::TokenStream;
// #[proc_macro_attribute]
// pub fn test11(args: TokenStream, item: TokenStream) -> TokenStream {
//     println!("121133");
//     item
// }
