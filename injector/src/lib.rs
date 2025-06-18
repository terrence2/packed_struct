mod packed;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn packed_struct(_args: TokenStream, input: TokenStream) -> TokenStream {
    let ast = packed::parse(input);
    let model = packed::analyze(ast);
    let ir = packed::lower(model);
    packed::codegen(ir)
}
