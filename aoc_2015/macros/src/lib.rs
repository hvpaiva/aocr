use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, ItemFn, Path};

#[proc_macro_attribute]
pub fn aoc(args: TokenStream, input: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(input as ItemFn);
    let args = parse_macro_input!(args as Path);

    let part_str = if args.segments.len() == 2
        && args.segments[0].ident == "Part"
        && args.segments[1].ident == "One"
    {
        "one"
    } else if args.segments.len() == 2
        && args.segments[0].ident == "Part"
        && args.segments[1].ident == "Two"
    {
        "two"
    } else {
        panic!("Expected `Part::One` or `Part::Two` as argument");
    };

    let fn_name = &input_fn.sig.ident;
    let register_fn_name = format_ident!("register_function_{}", fn_name);

    let expanded = quote! {
        #input_fn

        #[ctor::ctor]
        fn #register_fn_name() {
            use aoc_core::register_function;
            register_function(#part_str, #fn_name);
        }
    };

    TokenStream::from(expanded)
}
