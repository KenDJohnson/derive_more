use quote::{Tokens};
use syn::{Data, DeriveInput, Field, Ident, Type, Fields};
use utils::{unnamed_to_vec};


/// Provides the hook to expand `#[derive(FromStr)]` into an implementation of `From`
pub fn expand(input: &DeriveInput, trait_name: &str) -> Tokens {
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();
    let input_type = &input.ident;
    let (result, field_type) = match input.data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Unnamed(ref fields) => tuple_from_str(input_type, trait_name, unnamed_to_vec(fields)),
            // Fields::Named(ref fields) => struct_newtype(input, fields),
            Fields::Unit => panic_one_field(trait_name),
            _ => panic!("nooo not implemeted yet")
        },
        _ => panic_one_field(trait_name),
    };
    let trait_path = quote!(::std::str::FromStr);
    quote!{
        impl#impl_generics #trait_path for #input_type#ty_generics #where_clause
        {
            type Err = <#field_type as #trait_path>::Err;
            fn from_str(src: &str) -> Result<Self, Self::Err> {
                return Ok(#result)
            }
        }
    }
}

fn panic_one_field(trait_name : &str) -> ! {
    panic!(format!("Only structs with one field can derive({})", trait_name))
}

fn tuple_from_str<'a>(input_type: &Ident, trait_name: &str, fields: Vec<&'a Field>) -> (Tokens, &'a Type) {
    if fields.len() != 1 {
        panic_one_field(trait_name)
    };
    let field = &fields[0];
    let field_type = &field.ty;
    (quote!(#input_type(#field_type::from_str(src)?)), field_type)
}
