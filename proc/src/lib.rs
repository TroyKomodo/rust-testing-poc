use proc_macro::TokenStream;

struct MacroInput {
    exprs: Vec<syn::Expr>,
}

impl syn::parse::Parse for MacroInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let exprs = input
            .parse_terminated(syn::Expr::parse, syn::Token![,])?
            .into_iter()
            .collect();
        Ok(Self { exprs })
    }
}

#[proc_macro]
pub fn magic_macro(input: TokenStream) -> TokenStream {
    let input: MacroInput = syn::parse_macro_input!(input);
    
    let exprs = &input.exprs;

    let output = quote::quote! {
        #(#exprs)+*
    };

    output.into()
}

#[proc_macro]
pub fn magic_macro2(input: TokenStream) -> TokenStream {
    let input: MacroInput = syn::parse_macro_input!(input);

    let exprs = &input.exprs;

    let output = quote::quote! {
        #(#exprs)+*
    };

    output.into()
}
