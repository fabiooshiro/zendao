use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn route(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // item.
    dbg!(item)
}
