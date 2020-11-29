use proc_macro::TokenStream;
use proc_macro2::{self, TokenStream as TokenStream2};
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
    let (imp, trait_group) = parse_fn(&function);
    let options = parse_options(attr);
    let output = match trait_group {
        TraitGroup::Unary => impl_unary(imp, &function, options),
        TraitGroup::Binary => impl_binary(imp, &function, options),
        TraitGroup::Assign => impl_assign(imp, &function, options),
        TraitGroup::Index => impl_index(imp, &function, options),
        TraitGroup::Deref => impl_deref(imp, &function, options),
    };
    output.into()
}

enum TraitGroup {
    Unary,
    Binary,
    Assign,
    Index,
    Deref,
}

fn to_trait(function: &str) -> (&'static str, TraitGroup) {
    use TraitGroup::*;
    match function {
        "neg" => ("Neg", Unary),
        "not" => ("Not", Unary),
        "add" => ("Add", Binary),
        "sub" => ("Sub", Binary),
        "mul" => ("Mul", Binary),
        "div" => ("Div", Binary),
        "rem" => ("Rem", Binary),
        "bitand" => ("BitAnd", Binary),
        "bitor" => ("BitOr", Binary),
        "bitxor" => ("BitXor", Binary),
        "shl" => ("Shl", Binary),
        "shr" => ("Shr", Binary),
        "add_assign" => ("AddAssign", Assign),
        "sub_assign" => ("SubAssign", Assign),
        "mul_assign" => ("MulAssign", Assign),
        "div_assign" => ("DivAssign", Assign),
        "rem_assign" => ("RemAssign", Assign),
        "bitand_assign" => ("BitAndAssign", Assign),
        "bitor_assign" => ("BitOrAssign", Assign),
        "bitxor_assign" => ("BitXorAssign", Assign),
        "shl_assign" => ("ShlAssign", Assign),
        "shr_assign" => ("ShrAssign", Assign),
        "index" => ("Index", Index),
        "index_mut" => ("IndexMut", Index),
        "deref" => ("Deref", Deref),
        "deref_mut" => ("DerefMut", Deref),
        _ => panic!("name must identify an operation in core::ops"),
    }
}

struct Impl<'f> {
    fn_name: &'f proc_macro2::Ident,
    trait_name: &'static str,
    trait_path: TokenStream2,
    ret: &'f syn::ReturnType,
    ret_type: TokenStream2,
    generic_params: syn::Generics,
    where_clause: Option<syn::WhereClause>,
}

fn parse_fn(function: &syn::ItemFn) -> (Impl, TraitGroup) {
    let fn_name = &function.sig.ident;
    let (trait_name, trait_group) = to_trait(fn_name.to_string().as_ref());
    let trait_path = {
        let ident = syn::Ident::new(trait_name, proc_macro2::Span::call_site());
        quote::quote_spanned!(fn_name.span()=> ::core::ops::#ident)
    };
    let ret = &function.sig.output;
    let ret_type = match ret {
        syn::ReturnType::Default => quote!(()),
        syn::ReturnType::Type(_, typ) => quote!(#typ),
    };
    let (generic_params, where_clause) = {
        let mut generics = function.sig.generics.clone();
        let where_clause = generics.where_clause.take();
        (generics, where_clause)
    };
 
    (Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type,
        generic_params,
        where_clause,
    }, trait_group)
}

struct Options {
    auto_ref: bool,
    commutative: bool,
}
impl Options {
    fn none() -> Self {
        Self {
            auto_ref: false,
            commutative: false,
        }
    }
}

fn parse_options(attr: TokenStream) -> Options {
    use syn::{Ident, parse::Parser, punctuated::Punctuated, Token};
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let opts = parser.parse(attr).unwrap();
    let mut options = Options::none();
    for opt in opts {
        let str = opt.to_string();
        match str.as_ref() {
            "autoref" => options.auto_ref = true,
            "commutative" => options.commutative = true,
            _ => panic!("invalid option: {}", str),
        }
    }
    options
}

fn impl_unary(imp: Impl, function: &syn::ItemFn, options: Options) -> TokenStream {
    let Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type,
        generic_params,
        where_clause,
    } = imp;

    let self_type = un_type(&function, trait_name);

    let mut output = quote! {
        impl #generic_params #trait_path for #self_type #where_clause {
            type Output = #ret_type;
            fn #fn_name(self) #ret {
                #function
                #fn_name(self)
            }
        }
    };
    if options.auto_ref {
        if let Some(self_type) = remove_reference(self_type) {
            output = quote! {
                #output
                impl #generic_params #trait_path for #self_type #where_clause {
                    type Output = #ret_type;
                    fn #fn_name(self) #ret {
                        <&#self_type as #trait_path>::#fn_name(&self)
                    }
                }
            }
        }
    }
    if options.commutative {
        panic!("operation `{}` cannot be made commutative", trait_name);
    }
    output.into()
}

fn impl_binary(imp: Impl, function: &syn::ItemFn, options: Options) -> TokenStream {
    let Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type,
        generic_params,
        where_clause,
    } = imp;

    let (mut lhs_type, mut rhs_type) = bin_types(&function, trait_name);

    let mut commuting = false;
    let mut output = quote!();
    loop {
        let body = if !commuting {
            quote! {
                #function
                #fn_name(self, rhs)
            }
        } else {
            quote! {
                <#rhs_type as #trait_path<#lhs_type>>::#fn_name(rhs, self)
            }
        };
        output = quote! {
            #output
            impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                type Output = #ret_type;
                fn #fn_name(self, rhs: #rhs_type) #ret {
                    #body
                }
            }
        };
        if options.auto_ref {
            let lhs_type_val = remove_reference(lhs_type);
            let rhs_type_val = remove_reference(rhs_type);

            if let Some(lhs_type) = lhs_type_val {
                output = quote! {
                    #output
                    impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                        type Output = #ret_type;
                        fn #fn_name(self, rhs: #rhs_type) #ret {
                            <&#lhs_type as #trait_path<#rhs_type>>::#fn_name(&self, rhs)
                        }
                    }
                };
            }

            if let Some(rhs_type) = rhs_type_val {
                output = quote! {
                    #output
                    impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                        type Output = #ret_type;
                        fn #fn_name(self, rhs: #rhs_type) #ret {
                            <#lhs_type as #trait_path<&#rhs_type>>::#fn_name(self, &rhs)
                        }
                    }
                };
            }

            if let (Some(lhs_type), Some(rhs_type)) = (lhs_type_val, rhs_type_val) {
                output = quote! {
                    #output
                    impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                        type Output = #ret_type;
                        fn #fn_name(self, rhs: #rhs_type) #ret {
                            <&#lhs_type as #trait_path<&#rhs_type>>::#fn_name(&self, &rhs)
                        }
                    }
                };
            }
        }

        if options.commutative != commuting {
            core::mem::swap(&mut lhs_type, &mut rhs_type);
            commuting = true;
        } else {
            break;
        }
    }
    output.into()
}

fn impl_assign(imp: Impl, function: &syn::ItemFn, options: Options) -> TokenStream {
    let Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type: _,
        generic_params,
        where_clause,
    } = imp;

    let (lhs_type, rhs_type) = assign_types(&function, trait_name);

    let mut output = quote! {
        impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
            fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    if options.auto_ref {
        if let Some(rhs_type) = remove_reference(rhs_type) {
            output = quote! {
                #output
                impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                    fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                        <#lhs_type as #trait_path<&#rhs_type>>::#fn_name(self, &rhs)
                    }
                }
            };
        }
    }
    if options.commutative {
        panic!("operation `{}` cannot be made commutative", trait_name);
    }
    output.into()
}

fn impl_index(imp: Impl, function: &syn::ItemFn, options: Options) -> TokenStream {
    let Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type: _,
        generic_params,
        where_clause,
    } = imp;

    let ret_type = match ret {
        syn::ReturnType::Type(_, typ) => match typ.as_ref() {
            syn::Type::Reference(typ) => {
                typ.elem.as_ref()
            }
            typ => panic!("index operation must return a reference type, found {:?}", typ),
        }
        syn::ReturnType::Default => panic!("index operation must return a reference type, found ()"),
    };

    let (lhs_type, rhs_type) = index_types(&function, trait_name);

    let output = if trait_name == "Index" {
        quote! {
            impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                type Output = #ret_type;
                fn #fn_name(&self, rhs: #rhs_type) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
    } else {
        quote! {
            impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                fn #fn_name(&mut self, rhs: #rhs_type) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
    };
    output.into()
}

fn impl_deref(imp: Impl, function: &syn::ItemFn, options: Options) -> TokenStream {
    let Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type: _,
        generic_params,
        where_clause,
    } = imp;

    let ret_type = match ret {
        syn::ReturnType::Type(_, typ) => match typ.as_ref() {
            syn::Type::Reference(typ) => {
                typ.elem.as_ref()
            }
            typ => panic!("deref operation must return a reference type, found {:?}", typ),
        }
        syn::ReturnType::Default => panic!("deref operation must return a reference type, found ()"),
    };

    let self_type = deref_types(&function, trait_name);

    let output = if trait_name == "Deref" {
        quote! {
            impl #generic_params #trait_path for #self_type #where_clause {
                type Target = #ret_type;
                fn #fn_name(&self) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
    } else {
        quote! {
            impl #generic_params #trait_path for #self_type #where_clause {
                fn #fn_name(&mut self) #ret {
                    #function
                    #fn_name(self, rhs)
                }
            }
        }
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

fn deref_types<'f>(function: &'f syn::ItemFn, trait_name: &str) -> &'f syn::Type {
    let params = &function.sig.inputs;
    if params.len() != 1 {
        panic!("operation `{}` takes exactly 1 argument, found {}", trait_name, params.len());
    }
    if let syn::FnArg::Typed(lhs) = &params[0] {
        let lhs = remove_reference(lhs.ty.as_ref()).expect("the operand of `deref` must be a reference");
        lhs
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
