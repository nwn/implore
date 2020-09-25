use proc_macro::TokenStream;
use syn;
use quote::quote;

// #[proc_macro_attribute]
// pub fn impl_op(attr: TokenStream, item: TokenStream) -> TokenStream {
//     println!("attr: \"{}\"", attr.to_string());
//     println!("item: \"{}\"", item.to_string());
//     let input = syn::parse_macro_input!(item as OpImpl);
//     impl_add(input)
// }

// struct Param {
//     attrs: Vec<syn::Attribute>,
//     pat: Box<syn::Pat>,
//     colon_token: syn::token::Colon,
//     ref_token: Option<syn::token::And>,
//     lifetime: Option<syn::Lifetime>,
//     mutability: Option<syn::token::Mut>,
//     raw_type: Box<syn::Type>,
// }
// impl syn::parse::Parse for Param {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let attrs = input.call(syn::Attribute::parse_outer)?;
//         let pat = input.parse()?;
//         let colon_token = input.parse()?;
//         let ref_token = input.parse()?;
//         let lifetime = input.parse()?;
//         let mutability = input.parse()?;
//         let raw_type = input.parse()?;

//         Ok(Param {
//             attrs,
//             pat,
//             colon_token,
//             ref_token,
//             lifetime,
//             mutability,
//             raw_type,
//         })
//     }
// }

// enum Params {
//     Unary(Param, Option<syn::token::Comma>),
//     Binary(Param, syn::token::Comma, Param, Option<syn::token::Comma>),
// }
// impl syn::parse::Parse for Params {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let first = input.parse()?;
//         if input.is_empty() {
//             return Ok(Params::Unary(first, None));
//         }
//         let first_comma = input.parse()?;
//         if input.is_empty() {
//             return Ok(Params::Unary(first, Some(first_comma)));
//         }

//         let second = input.parse()?;
//         let second_comma = input.parse()?;
//         Ok(Params::Binary(
//             first,
//             first_comma,
//             second,
//             second_comma,
//         ))
//     }
// }

// struct OpImpl {
//     attrs: Vec<syn::Attribute>,
//     vis: Option<syn::Visibility>,
//     constness: Option<syn::token::Const>,
//     asyncness: Option<syn::token::Async>,
//     unsafety: Option<syn::token::Unsafe>,
//     abi: Option<syn::Abi>,
//     fn_token: syn::token::Fn,
//     op: syn::Ident,
//     generics: syn::Generics,
//     paren_token: syn::token::Paren,
//     params: Params,
//     retval: syn::ReturnType,
//     body: Box<syn::Block>,
// }
// impl syn::parse::Parse for OpImpl {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         let attrs = input.call(syn::Attribute::parse_outer)?;
//         //let vis = input.parse()?;
//         let constness = input.parse()?;
//         let asyncness = input.parse()?;
//         let unsafety = input.parse()?;
//         let abi = input.parse()?;

//         let fn_token = input.parse()?;
//         let op = input.parse()?;
//         let generics = input.parse()?;

//         let params;
//         let paren_token = syn::parenthesized!(params in input);
//         let params = params.parse()?;
//         let retval = input.parse()?;
//         let body = input.parse()?;

//         let result = Ok(OpImpl {
//             attrs,
//             vis: None,
//             constness,
//             asyncness,
//             unsafety,
//             abi,
//             fn_token,
//             op,
//             generics,
//             paren_token,
//             params,
//             retval,
//             body,
//         });
//         println!("attrs: {:?}", result.as_ref().unwrap().attrs.iter().map(|a| quote::ToTokens::to_token_stream(a)).collect::<Vec<_>>());
//         //println!("vis: {:?}", result.unwrap().vis);
//         println!("constness: {:?}", result.as_ref().unwrap().constness);
//         println!("asyncness: {:?}", result.as_ref().unwrap().asyncness);
//         println!("unsafety: {:?}", result.as_ref().unwrap().unsafety);
//         println!("abi: {:?}", result.as_ref().unwrap().abi);
//         result
//     }
// }

// fn impl_add(op_impl: OpImpl) -> TokenStream {
//     let output = quote! {
//         #(op_impl.attrs)
//         #(op_impl.vis) #(op_impl.constness)
//         #(op_impl.asyncness) #(op_impl.unsafety)
//         #(op_impl.abi)

//         #(op_impl.fn_token) add #(op_impl.generics)
//         (#(op_impl.params)) #(op_impl.retval)
//         #(op_impl.body)
//     };
//     let output = quote! { };
//     output.into()
// }

#[proc_macro_attribute]
pub fn impl_op2(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: \"{}\"", attr.to_string());
    println!("item: \"{}\"", item.to_string());
    let function = syn::parse_macro_input!(item as syn::ItemFn);
    let output = match function.sig.ident.to_string().as_str() {
        "add" => impl_binary(function, "Add"),
        "sub" => impl_binary(function, "Sub"),
        "mul" => impl_binary(function, "Mul"),
        "div" => impl_binary(function, "Div"),
        "rem" => impl_binary(function, "Rem"),
        "bitand" => impl_binary(function, "BitAnd"),
        "bitor" => impl_binary(function, "BitOr"),
        "bitxor" => impl_binary(function, "BitXor"),
        "shl" => impl_binary(function, "Shl"),
        "shr" => impl_binary(function, "Shr"),
        _ => panic!("name must identify an operation in core::ops"),
    };
    output.into()
}

fn impl_binary(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let (lhs_type, rhs_type) = bin_types(&function);

    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let fn_name = &function.sig.ident;
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;
    let ret_type = match ret {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, typ) => quote!(#typ),
    };

    let output = quote! {
        impl #trait_path<#rhs_type> for #lhs_type {
            type Output = #ret_type;
            fn #fn_name(self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    println!("impl: \"{}\"", output.to_string());
    output.into()
}

fn bin_types(function: &syn::ItemFn) -> (&syn::Type, &syn::Type) {
    let params = &function.sig.inputs;
    if params.len() != 2 {
        panic!("binary operation takes exactly two arguments, found {}", params.len());
    }
    if let (syn::FnArg::Typed(lhs), syn::FnArg::Typed(rhs)) = (&params[0], &params[1]) {
        (lhs.ty.as_ref(), rhs.ty.as_ref())
    } else {
        panic!("`self` receivers can only be used in associated methods");
    }
}
