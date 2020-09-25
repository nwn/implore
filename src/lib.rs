use proc_macro::TokenStream;
use syn;
use quote::quote;

#[proc_macro_attribute]
pub fn impl_op(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(item as syn::ItemFn);
    let output = if attr.is_empty() {
        match function.sig.ident.to_string().as_str() {
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
        }
    } else {
        match function.sig.ident.to_string().as_str() {
            "add" => impl_binary_autoref(function, "Add"),
            "sub" => impl_binary_autoref(function, "Sub"),
            "mul" => impl_binary_autoref(function, "Mul"),
            "div" => impl_binary_autoref(function, "Div"),
            "rem" => impl_binary_autoref(function, "Rem"),
            "bitand" => impl_binary_autoref(function, "BitAnd"),
            "bitor" => impl_binary_autoref(function, "BitOr"),
            "bitxor" => impl_binary_autoref(function, "BitXor"),
            "shl" => impl_binary_autoref(function, "Shl"),
            "shr" => impl_binary_autoref(function, "Shr"),
            _ => panic!("name must identify an operation in core::ops"),
        }
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
