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
            Trait::Assign(name) => impl_assign(function, name),
            Trait::Index(name) => impl_index(function, name),
        }
    } else {
        match trait_ {
            Trait::Unary(name) => impl_unary_autoref(function, name),
            Trait::Binary(name) => impl_binary_autoref(function, name),
            Trait::Assign(name) => impl_assign_autoref(function, name),
            Trait::Index(name) => impl_index_autoref(function, name),
        }
    };
    output.into()
}

enum Trait {
    Unary(&'static str),
    Binary(&'static str),
    Assign(&'static str),
    Index(&'static str),
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
        "add_assign" => Trait::Assign("AddAssign"),
        "sub_assign" => Trait::Assign("SubAssign"),
        "mul_assign" => Trait::Assign("MulAssign"),
        "div_assign" => Trait::Assign("DivAssign"),
        "rem_assign" => Trait::Assign("RemAssign"),
        "bitand_assign" => Trait::Assign("BitAndAssign"),
        "bitor_assign" => Trait::Assign("BitOrAssign"),
        "bitxor_assign" => Trait::Assign("BitXorAssign"),
        "shl_assign" => Trait::Assign("ShlAssign"),
        "shr_assign" => Trait::Assign("ShrAssign"),
        "index" => Trait::Index("Index"),
        "index_mut" => Trait::Index("IndexMut"),
        _ => panic!("name must identify an operation in core::ops"),
    }
}

fn impl_unary(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let rhs_type = un_type(&function, trait_name);

    let fn_name = &function.sig.ident;
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
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
    let rhs_type = un_type(&function, trait_name);

    let fn_name = &function.sig.ident;
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
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
    let (lhs_type, rhs_type) = bin_types(&function, trait_name);

    let fn_name = &function.sig.ident;
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
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
    let (lhs_type, rhs_type) = bin_types(&function, trait_name);

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

fn impl_assign(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let (lhs_type, rhs_type) = assign_types(&function, trait_name);

    let fn_name = &function.sig.ident;
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;

    let output = quote! {
        impl #trait_path<#rhs_type> for #lhs_type {
            fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    output.into()
}

fn impl_assign_autoref(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let (lhs_type, rhs_type) = assign_types(&function, trait_name);

    let fn_name = &function.sig.ident;
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;
    let rhs_type_val = remove_reference(rhs_type);

    let ref_ = quote! {
        impl #trait_path<#rhs_type> for #lhs_type {
            fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    let val = if let Some(rhs_type) = rhs_type_val {
        quote! {
            impl #trait_path<#rhs_type> for #lhs_type {
                fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                    self.#fn_name(&rhs)
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

fn impl_index(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let (lhs_type, rhs_type) = index_types(&function, trait_name);

    let fn_name = &function.sig.ident;
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;
    let ret_type;
    let ret = match ret {
        syn::ReturnType::Type(arrow, typ) => match typ.as_ref() {
            syn::Type::Reference(typ) => {
                ret_type = typ.elem.as_ref();
                syn::ReturnType::Type(*arrow, Box::new(syn::Type::Reference(
                    syn::TypeReference {
                        lifetime: None,
                        .. typ.clone()
                    }
                )))
            }
            typ => panic!("index operation must return a reference type, found {:?}", typ),
        }
        syn::ReturnType::Default => panic!("index operation must return a reference type, found ()"),
    };
    let generics = &function.sig.generics;

    let output = if trait_name == "Index" {
        quote! {
            impl #generics #trait_path<#rhs_type> for #lhs_type {
                type Output = #ret_type;
                fn #fn_name(&self, rhs: #rhs_type) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
    } else {
        quote! {
            impl #generics #trait_path<#rhs_type> for #lhs_type {
                fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
    };
    output.into()
}

fn impl_index_autoref(function: syn::ItemFn, trait_name: &str) -> TokenStream {
    let (lhs_type, rhs_type) = index_types(&function, trait_name);

    let fn_name = &function.sig.ident;
    let trait_ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
    let trait_path = quote::quote_spanned!(fn_name.span()=> ::core::ops::#trait_ident);

    let ret = &function.sig.output;
    let ret_type = match ret {
        syn::ReturnType::Type(_, typ) => match typ.as_ref() {
            syn::Type::Reference(typ) => typ,
            typ => panic!("index operation must return a reference type, found {:?}", typ),
        }
        syn::ReturnType::Default => panic!("index operation must return a reference type, found ()"),
    };
    let rhs_type_val = remove_reference(rhs_type);

    let ref_ = if trait_name == "Index" {
        quote! {
            impl #trait_path<#rhs_type> for #lhs_type {
                type Output = #ret_type;
                fn #fn_name(&self, rhs: #rhs_type) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
    } else {
        quote! {
            impl #trait_path<#rhs_type> for #lhs_type {
                fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
    };
    let val = if let Some(rhs_type) = rhs_type_val {
        if trait_name == "Index" {
            quote! {
                impl #trait_path<#rhs_type> for #lhs_type {
                    type Output = #ret_type;
                    fn #fn_name(&self, rhs: #rhs_type) #ret {
                        self.#fn_name(&rhs)
                    }
                }
            }
        } else {
            quote! {
                impl #trait_path<#rhs_type> for #lhs_type {
                    fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                        self.#fn_name(&rhs)
                    }
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

fn un_type<'f>(function: &'f syn::ItemFn, trait_name: &str) -> &'f syn::Type {
    let params = &function.sig.inputs;
    if params.len() != 1 {
        panic!("operation `{}` takes exactly 1 argument, found {}", trait_name, params.len());
    }
    if let syn::FnArg::Typed(lhs) = &params[0] {
        lhs.ty.as_ref()
    } else {
        panic!("`self` receivers can only be used in associated methods");
    }
}

fn bin_types<'f>(function: &'f syn::ItemFn, trait_name: &str) -> (&'f syn::Type, &'f syn::Type) {
    let params = &function.sig.inputs;
    if params.len() != 2 {
        panic!("operation `{}` takes exactly 2 arguments, found {}", trait_name, params.len());
    }
    if let (syn::FnArg::Typed(lhs), syn::FnArg::Typed(rhs)) = (&params[0], &params[1]) {
        (lhs.ty.as_ref(), rhs.ty.as_ref())
    } else {
        panic!("`self` receivers can only be used in associated methods");
    }
}

fn assign_types<'f>(function: &'f syn::ItemFn, trait_name: &str) -> (&'f syn::Type, &'f syn::Type) {
    let params = &function.sig.inputs;
    if params.len() != 2 {
        panic!("operation `{}` takes exactly 2 arguments, found {}", trait_name, params.len());
    }
    if let (syn::FnArg::Typed(lhs), syn::FnArg::Typed(rhs)) = (&params[0], &params[1]) {
        let lhs = remove_reference(lhs.ty.as_ref()).expect("the first operand of an assignment must be a mutable reference");
        (lhs, rhs.ty.as_ref())
    } else {
        panic!("`self` receivers can only be used in associated methods");
    }
}

fn index_types<'f>(function: &'f syn::ItemFn, trait_name: &str) -> (&'f syn::Type, &'f syn::Type) {
    let params = &function.sig.inputs;
    if params.len() != 2 {
        panic!("operation `{}` takes exactly 2 arguments, found {}", trait_name, params.len());
    }
    if let (syn::FnArg::Typed(lhs), syn::FnArg::Typed(rhs)) = (&params[0], &params[1]) {
        let lhs = remove_reference(lhs.ty.as_ref()).expect("the first operand of `index` must be a reference");
        (lhs, rhs.ty.as_ref())
    } else {
        panic!("`self` receivers can only be used in associated methods");
    }
}

fn remove_reference(typ: &syn::Type) -> Option<&syn::Type> {
    // NOTE: This only works for types that look syntactically like references.
    // This means that it fails for type aliases like `type Ref<T> = &T;`. This
    // could potentially be fixed using `RemoveRef::WithoutRef` from
    // https://rust-lang.github.io/rfcs/2532-associated-type-defaults.html,
    // however that would require exporting a trait type, and thus a second
    // crate. Perhaps one day this will make it into `std` and we can use that.
    if let syn::Type::Reference(ref_type) = typ {
        Some(ref_type.elem.as_ref())
    } else {
        None
    }
}
