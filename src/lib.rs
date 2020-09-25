use proc_macro::TokenStream;
use syn;
use quote::quote;

/*
    Traits to implement:

    Unary:
    - Neg,
    - Not,

    Binary:
    - Add, Sub, Mul, Div, Rem,
    - BitAnd, BitOr, BitXor,
    - Shl, Shr,

    Assignment:
    - AddAssign, SubAssign, MulAssign, DivAssign, RemAssign,
    - BitAndAssign, BitOrAssign, BitXorAssign,
    - ShlAssign, ShrAssign,

    Miscellaneous:
    - Fn, FnMut, FnOnce,
    - Deref, DerefMut,
    - Index, IndexMut,
    - Clone, Drop,
*/


#[proc_macro_attribute]
pub fn impl_op(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(item as syn::ItemFn);
    let trait_ = to_trait(function.sig.ident.to_string().as_str());
    let output = if attr.is_empty() {
        match trait_ {
            Trait::Unary(name) => impl_unary(function, name),
            Trait::Binary(name) => impl_binary(function, name),
            Trait::Assign(name) => todo!(),
        }
    } else {
        match trait_ {
            Trait::Unary(name) => impl_unary_autoref(function, name),
            Trait::Binary(name) => impl_binary_autoref(function, name),
            Trait::Assign(name) => todo!(),
        }
    };
    output.into()
}

enum Trait {
    Unary(&'static str),
    Binary(&'static str),
    Assign(&'static str),
}

fn to_trait(function: &str) -> Trait {
    match function {
        "neg" => Trait::Unary("Neg"),
        "not" => Trait::Unary("Not"),
        "add" => Trait::Binary("Add"),
        "sub" => Trait::Binary("Sub"),
        "mul" => Trait::Binary("Mul"),
        "div" => Trait::Binary("Div"),
        "rem" => Trait::Binary("Rem"),
        "bitand" => Trait::Binary("BitAnd"),
        "bitor" => Trait::Binary("BitOr"),
        "bitxor" => Trait::Binary("BitXor"),
        "shl" => Trait::Binary("Shl"),
        "shr" => Trait::Binary("Shr"),
        _ => panic!("name must identify an operation in core::ops"),
    }
}

fn impl_unary(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let rhs_type = un_type(&function);

    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let fn_name = &function.sig.ident;
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;
    let ret_type = match ret {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, typ) => quote!(#typ),
    };

    let output = quote! {
        impl #trait_path for #rhs_type {
            type Output = #ret_type;
            fn #fn_name(self) #ret {
                #function
                #fn_name(self)
            }
        }
    };
    output.into()
}

fn impl_unary_autoref(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let rhs_type = un_type(&function);

    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let fn_name = &function.sig.ident;
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;
    let ret_type = match ret {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, typ) => quote!(#typ),
    };
    let rhs_type_val = remove_reference(rhs_type);

    let ref_ = quote! {
        impl #trait_path for #rhs_type {
            type Output = #ret_type;
            fn #fn_name(self) #ret {
                #function
                #fn_name(self)
            }
        }
    };
    let val = if let Some(rhs_type) = rhs_type_val {
        quote! {
            impl #trait_path for #rhs_type {
                type Output = #ret_type;
                fn #fn_name(self) #ret {
                    (&self).#fn_name()
                }
            }
        }
    } else {
        quote!()
    };
    let output = quote! {
        #ref_
        #val
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
    output.into()
}

fn impl_binary_autoref(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let (lhs_type, rhs_type) = bin_types(&function);

    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let fn_name = &function.sig.ident;
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;
    let ret_type = match ret {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, typ) => quote!(#typ),
    };
    let lhs_type_val = remove_reference(lhs_type);
    let rhs_type_val = remove_reference(rhs_type);

    let refref = quote! {
        impl #trait_path<#rhs_type> for #lhs_type {
            type Output = #ret_type;
            fn #fn_name(self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    let valref = if let Some(lhs_type) = lhs_type_val {
        quote! {
            impl #trait_path<#rhs_type> for #lhs_type {
                type Output = #ret_type;
                fn #fn_name(self, rhs: #rhs_type) #ret {
                    (&self).#fn_name(rhs)
                }
            }
        }
    } else {
        quote!()
    };
    let refval = if let Some(rhs_type) = rhs_type_val {
        quote! {
            impl #trait_path<#rhs_type> for #lhs_type {
                type Output = #ret_type;
                fn #fn_name(self, rhs: #rhs_type) #ret {
                    self.#fn_name(&rhs)
                }
            }
        }
    } else {
        quote!()
    };
    let valval = if let (Some(lhs_type), Some(rhs_type)) = (lhs_type_val, rhs_type_val) {
        quote! {
            impl #trait_path<#rhs_type> for #lhs_type {
                type Output = #ret_type;
                fn #fn_name(self, rhs: #rhs_type) #ret {
                    (&self).#fn_name(&rhs)
                }
            }
        }
    } else {
        quote!()
    };
    let output = quote! {
        #refref
        #refval
        #valref
        #valval
    };
    output.into()
}

fn un_type(function: &syn::ItemFn) -> &syn::Type {
    let params = &function.sig.inputs;
    if params.len() != 1 {
        panic!("unary operation takes exactly one argument, found {}", params.len());
    }
    if let syn::FnArg::Typed(lhs) = &params[0] {
        lhs.ty.as_ref()
    } else {
        panic!("`self` receivers can only be used in associated methods");
    }
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

fn remove_reference(typ: &syn::Type) -> Option<&syn::Type> {
    if let syn::Type::Reference(ref_type) = typ {
        Some(ref_type.elem.as_ref())
    } else {
        None
    }
}
