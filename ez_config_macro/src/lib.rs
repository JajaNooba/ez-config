extern crate proc_macro;
use dotenv::dotenv;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Config)]
pub fn config_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("Expected a struct with named fields"),
    };

    let field_name = fields.iter().map(|field| &field.ident);
    let field_name_cloned = field_name.clone();
    let field_type = fields.iter().map(|field| &field.ty);
    let var_name = fields
        .iter()
        .map(|field| format!("{}", &field.ident.clone().unwrap()).to_uppercase());

    let struct_name = &input.ident;

    dotenv().ok();

    TokenStream::from(quote! {
        impl Config for #struct_name {
            fn load() -> Self {
                #(
                    let #field_name_cloned: #field_type = env!(#var_name).parse::<#field_type>().unwrap();
                )*
                Self {
                    #(
                        #field_name
                    )*
                }
            }
        }
    })
}
