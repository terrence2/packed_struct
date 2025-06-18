use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Generics, Ident, ItemFn, Stmt, Type, parse2};

pub(crate) type Ast = DeriveInput;

pub(crate) struct ResourceModel {
    ident: Ident,
    generics: Generics,
    fieldset: Vec<(Ident, Type)>,
    methods: Vec<ItemFn>,
    debug_fields: Vec<Stmt>,
    ast: DeriveInput,
}

pub(crate) fn parse(input: TokenStream) -> Ast {
    parse2(TokenStream2::from(input)).expect("parse result")
}

pub(crate) fn analyze(ast: Ast) -> ResourceModel {
    let mut fieldset = Vec::new();
    match &ast.data {
        Data::Struct(ds) => {
            for field in &ds.fields {
                if let Some(ident) = &field.ident {
                    fieldset.push((ident.to_owned(), field.ty.to_owned()));
                }
            }
        }
        _ => panic!("packed_struct must be used on a _struct_"),
    }
    ResourceModel {
        ident: ast.ident.clone(),
        generics: ast.generics.clone(),
        fieldset,
        methods: Vec::new(),
        debug_fields: Vec::new(),
        ast,
    }
}

pub(crate) fn lower(mut model: ResourceModel) -> ResourceModel {
    for (ident, ty) in &model.fieldset {
        let method: ItemFn = parse2(quote! {
            #[allow(unused)]
            pub fn #ident(&self) -> #ty {
                self.#ident
            }
        })
        .unwrap();
        model.methods.push(method);

        let debug_field: Stmt = parse2(quote! {
            ds.field(stringify!(#ident), &self.#ident());
        })
        .unwrap();
        model.debug_fields.push(debug_field);
    }
    model
}

pub(crate) fn codegen(model: ResourceModel) -> TokenStream {
    let ast = model.ast;
    let methods = model.methods;
    let debug_fields = model.debug_fields;
    let ident = model.ident;
    let (_impl_generics, ty_generics, where_clause) = model.generics.split_for_impl();
    proc_macro::TokenStream::from(quote! {
        #[repr(C, packed)]
        #[derive(::packed_struct::zerocopy::FromBytes, ::packed_struct::zerocopy::IntoBytes, ::packed_struct::zerocopy::KnownLayout, ::packed_struct::zerocopy::Immutable)]
        #ast

        impl #ident #ty_generics #where_clause {
            #(#methods)*

            #[allow(unused)]
            pub fn overlay(buf: &[u8]) -> ::packed_struct::anyhow::Result<&#ident> {
                ::packed_struct::zerocopy::Ref::<&[u8], #ident>::from_bytes(buf)
                    .map(|v| ::packed_struct::zerocopy::Ref::into_ref(v))
                    .map_err(|e| ::packed_struct::anyhow::anyhow!(format!("cannot overlay: {e}")))
            }

            #[allow(unused)]
            pub fn overlay_prefix(buf: &[u8]) -> ::packed_struct::anyhow::Result<&#ident> {
                ::packed_struct::zerocopy::Ref::<&[u8], #ident>::from_bytes(&buf[0..::std::mem::size_of::<#ident>()])
                    .map(|v| ::packed_struct::zerocopy::Ref::into_ref(v))
                    .map_err(|e| ::packed_struct::anyhow::anyhow!(format!("cannot overlay: {e}")))
            }

            #[allow(unused)]
            pub fn overlay_slice(buf: &[u8]) -> ::packed_struct::anyhow::Result<&[#ident]> {
                ::packed_struct::zerocopy::Ref::<&[u8], [#ident]>::from_bytes(buf)
                    .map(|v| ::packed_struct::zerocopy::Ref::into_ref(v))
                    .map_err(|e| ::packed_struct::anyhow::anyhow!(format!("cannot overlay: {e}")))
            }

            #[allow(unused)]
            pub fn size_of() -> usize {
                std::mem::size_of::<#ident>()
            }
        }

        impl ::std::fmt::Debug for #ident #ty_generics #where_clause {
            fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                let mut ds = fmt.debug_struct(stringify!(#ident));
                #(#debug_fields)*;
                ds.finish()
            }
        }
    })
}
