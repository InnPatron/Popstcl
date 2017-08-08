extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;

#[proc_macro_derive(IntoValue)]
pub fn into_value(input: TokenStream) -> TokenStream {
    let s = input.to_string();
    let ast = syn::parse_derive_input(&s).unwrap();
    let gen = gen_into_value(ast);
    gen.parse().unwrap()
}

fn gen_into_value(ast: syn::DeriveInput) -> quote::Tokens {
    use syn::VariantData;
    
    let name = &ast.ident;

    let lifetimes = ast.generics.lifetimes.iter().map(|lt| quote! { #lt });
    let ty_params = ast.generics.ty_params.iter().map(|ty| quote! { #ty });
    let generics = lifetimes.chain(ty_params).collect::<Vec<_>>();
    let generics = if generics.is_empty() {
        quote! {}
    } else {
        quote! { <#(#generics),*> }
    };

    let ty_constraints = ast.generics.ty_params.iter().map(|ty| quote! {#ty: IntoValue});
    let where_predicates = ast.generics.where_clause.predicates.iter().map(|pred| quote! {#pred});
    let where_clause = ty_constraints.chain(where_predicates).collect::<Vec<_>>();
    let where_clause = if where_clause.is_empty() {
        quote! { }
    } else {
        quote! { where #(#where_clause),* }
    };

    let body = if let syn::Body::Struct(variant_data) = ast.body {
        match variant_data {
            VariantData::Unit => handle_unit_struct(),
            VariantData::Tuple(fields) => handle_tuple(fields),
            VariantData::Struct(fields) => handle_struct(fields),
        }
    } else {
        panic!("PopstclValue derive currently does not support enums");
    };

    quote! {
        impl #generics IntoValue for #name #generics #where_clause {
            fn into_value(self) -> Value {
                #body
            }
        }
    }
}

fn handle_unit_struct() -> quote::Tokens {
    quote! {
        popstcl_core::StdObject::empty().into_value()
    }
}

fn handle_struct(fields: Vec<syn::Field>) -> quote::Tokens {
    let names = fields.into_iter().map(|f| f.ident.unwrap()).collect::<Vec<_>>();
    let str_names = names.iter().map(|n| n.as_ref().to_string()).collect::<Vec<String>>();
    quote! {
        let mut obj = StdObject::empty();
        #(obj.insert(&#str_names, self.#names.into_value().into());)*
        obj.into_value()
    }
}

fn handle_tuple(fields: Vec<syn::Field>) -> quote::Tokens {
    let names = fields.into_iter().map(|f| f.ident.unwrap()).collect::<Vec<_>>();
    let str_names = names.iter()
        .enumerate()
        .map(|(index, _)| { let index: usize = index; index.to_string() })
        .collect::<Vec<String>>();
    quote! {
        let mut obj = StdObject::empty();
        #(obj.insert(&#str_names, self.#names.into_value().into());)*
        obj.into_value()
    }
}
