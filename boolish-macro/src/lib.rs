use proc_macro::TokenStream;
use boolish_code::make_boolish;

#[proc_macro_derive(Boolish, attributes(boolval))]
pub fn boolish(item: TokenStream) -> TokenStream{
    make_boolish(item.into()).into()
}
