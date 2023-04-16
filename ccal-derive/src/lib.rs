use proc_macro::{self, TokenStream};
use quote::quote;
use syn::DeriveInput;
use darling::FromField;

#[derive(FromField, Default)]
#[darling(default, forward_attrs)] //attributes(short)
struct Flag {
    ident: Option<syn::Ident>,
    attrs: Vec<syn::Attribute>,
}


#[proc_macro_derive(CommandCall,attributes(arg))]
pub fn grepi_derive(input: TokenStream) -> TokenStream {
    //cf.: https://doc.rust-lang.org/book/ch19-06-macros.html
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let input:DeriveInput = syn::parse(input).unwrap();
    let DeriveInput { ident, .. } = input;
    let fields = match input.data{
        syn::Data::Struct(d) => d,
        syn::Data::Enum(_) => panic!("Cannot use c-cal on Enums"),
        syn::Data::Union(_) => panic!("Cannot use c-cal on Unions"),
    }.fields;
    let mut opts: Vec<Flag> = Vec::new();
    for f in fields {
        opts.push(Flag::from_field(&f).expect("Ouchie owie just did a fucky wucky :3"));
    }
    let q = opts.get(0).unwrap().ident.as_ref().unwrap();
    let c = &opts.get(0).unwrap().attrs;
    let mut res = String::new();
    for attr in c {
        res.push_str(format!("{:?}", attr.path).as_str());
        res.push_str(format!("{:?}", attr.tokens).as_str());
    }
    // Build the trait implementation
    let gen = quote! {
        impl CommandCall for #ident {
            fn command_call() {
                println!("Hello, Macro! My name is {} {}!", stringify!(#q), #res);
            }
        }
    };
    gen.into()
}