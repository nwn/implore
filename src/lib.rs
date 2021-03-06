use proc_macro::TokenStream;
use proc_macro2::{self, TokenStream as TokenStream2};
use proc_macro_error::{abort, Diagnostic, emit_error, emit_warning, proc_macro_error};
use syn;
use quote::quote;

/*
    Names: implore, imply

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
#[proc_macro_error]
pub fn impl_op(attr: TokenStream, item: TokenStream) -> TokenStream {
    let function = syn::parse_macro_input!(item as syn::ItemFn);
    let (imp, trait_group) = parse_fn(&function);
    let options = parse_options(attr);
    let output = match trait_group {
        TraitGroup::Unary => impl_unary(imp, &function, options),
        TraitGroup::Binary => impl_binary(imp, &function, options),
        TraitGroup::Assign => impl_assign(imp, &function, options),
        TraitGroup::Deref => impl_deref(imp, &function, options),
        TraitGroup::DerefMut => impl_deref_mut(imp, &function, options),
        TraitGroup::Index => impl_index(imp, &function, options),
        TraitGroup::IndexMut => impl_index_mut(imp, &function, options),
    };
    output.into()
}

enum TraitGroup {
    Unary,
    Binary,
    Assign,
    Deref,
    DerefMut,
    Index,
    IndexMut,
}

fn to_trait(fn_name: &syn::Ident) -> (&'static str, TraitGroup) {
    use TraitGroup::*;
    match fn_name.to_string().as_ref() {
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
        "deref" => ("Deref", Deref),
        "deref_mut" => ("DerefMut", DerefMut),
        "index" => ("Index", Index),
        "index_mut" => ("IndexMut", IndexMut),
        name => abort!(fn_name, "unknown operation: `{}`", name),
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
    let (trait_name, trait_group) = to_trait(&fn_name);
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
    auto_ref: Option<syn::Ident>,
    commutative: Option<syn::Ident>,
}
impl Options {
    fn none() -> Self {
        Self {
            auto_ref: None,
            commutative: None,
        }
    }
}

fn try_add_option(opt: &mut Option<syn::Ident>, new: syn::Ident) {
    if let Some(previous) = opt {
        emit_error!(new, "repeated option: `{}`", new.to_string();
            note = previous.span() => "previously occurs here");
    } else {
        *opt = Some(new);
    }
}

fn parse_options(attr: TokenStream) -> Options {
    use syn::{Ident, parse::Parser, punctuated::Punctuated, Token};
    let parser = Punctuated::<Ident, Token![,]>::parse_terminated;
    let opts = parser.parse(attr).unwrap_or_else(|error|
        Diagnostic::from(error).abort()
    );
    let mut options = Options::none();
    for opt in opts {
        let str = opt.to_string();
        match str.as_ref() {
            "autoref" => try_add_option(&mut options.auto_ref, opt),
            "commutative" => try_add_option(&mut options.commutative, opt),
            _ => emit_error!(opt, "invalid option: `{}`", str),
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

    if let Some(token) = options.commutative {
        emit_error!(token, "operation `{}` cannot commute", trait_name);
    }

    let self_type = un_type(&function.sig, trait_name);

    let mut output = quote! {
        impl #generic_params #trait_path for #self_type #where_clause {
            type Output = #ret_type;
            fn #fn_name(self) #ret {
                #function
                #fn_name(self)
            }
        }
    };
    if let Some(auto_ref) = &options.auto_ref {
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
        } else {
            emit_warning!(auto_ref, "option `autoref` has no effect");
        }
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

    let (mut lhs_type, mut rhs_type) = bin_types(&function.sig, trait_name);

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
        if let Some(auto_ref) = &options.auto_ref {
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

            if lhs_type_val.is_none() && rhs_type_val.is_none() {
                emit_warning!(auto_ref, "option `autoref` has no effect");
            }
        }

        if options.commutative.is_some() != commuting {
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

    if let Some(token) = options.commutative {
        emit_error!(token, "operation `{}` cannot commute", trait_name);
    }

    let (lhs_type, rhs_type, self_lifetime) = assign_types(&function.sig, trait_name);
    let generic_params = remove_generic_param(generic_params, self_lifetime);

    let mut output = quote! {
        impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
            fn #fn_name <#self_lifetime> (& #self_lifetime mut self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    if options.auto_ref.is_some() {
        if let Some(rhs_type) = remove_reference(rhs_type) {
            output = quote! {
                #output
                impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
                    fn #fn_name <#self_lifetime> (& #self_lifetime mut self, rhs: #rhs_type) #ret {
                        <#lhs_type as #trait_path<&#rhs_type>>::#fn_name(self, &rhs)
                    }
                }
            };
        }
    }
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

    if let Some(token) = options.auto_ref {
        emit_error!(token, "operation `{}` does not support autoref", trait_name);
    }
    if let Some(token) = options.commutative {
        emit_error!(token, "operation `{}` cannot commute", trait_name);
    }

    let (self_type, self_lifetime) = deref_types(&function.sig, trait_name);
    let generic_params = remove_generic_param(generic_params, self_lifetime);
    let ret_type = unwrap_reference_return(&function.sig);

    let output = quote! {
        impl #generic_params #trait_path for #self_type #where_clause {
            type Target = #ret_type;
            fn #fn_name <#self_lifetime> (& #self_lifetime self) #ret {
                #function
                #fn_name(self)
            }
        }
    };
    output.into()
}

fn impl_deref_mut(imp: Impl, function: &syn::ItemFn, options: Options) -> TokenStream {
    let Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type: _,
        generic_params,
        where_clause,
    } = imp;

    if let Some(token) = options.auto_ref {
        emit_error!(token, "operation `{}` does not support autoref", trait_name);
    }
    if let Some(token) = options.commutative {
        emit_error!(token, "operation `{}` cannot commute", trait_name);
    }

    let (self_type, self_lifetime) = deref_mut_types(&function.sig, trait_name);
    let generic_params = remove_generic_param(generic_params, self_lifetime);
    let _ret_type = unwrap_mutable_reference_return(&function.sig);

    let output = quote! {
        impl #generic_params #trait_path for #self_type #where_clause {
            fn #fn_name <#self_lifetime> (& #self_lifetime mut self) #ret {
                #function
                #fn_name(self)
            }
        }
    };
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

    if let Some(token) = options.auto_ref {
        emit_error!(token, "operation `{}` does not support autoref", trait_name);
    }
    if let Some(token) = options.commutative {
        emit_error!(token, "operation `{}` cannot commute", trait_name);
    }

    let (lhs_type, rhs_type, self_lifetime) = index_types(&function.sig, trait_name);
    let generic_params = remove_generic_param(generic_params, self_lifetime);
    let ret_type = unwrap_reference_return(&function.sig);

    let output = quote! {
        impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
            type Output = #ret_type;
            fn #fn_name <#self_lifetime> (& #self_lifetime self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    output.into()
}

fn impl_index_mut(imp: Impl, function: &syn::ItemFn, options: Options) -> TokenStream {
    let Impl {
        fn_name,
        trait_name,
        trait_path,
        ret,
        ret_type: _,
        generic_params,
        where_clause,
    } = imp;

    if let Some(token) = options.auto_ref {
        emit_error!(token, "operation `{}` does not support autoref", trait_name);
    }
    if let Some(token) = options.commutative {
        emit_error!(token, "operation `{}` cannot commute", trait_name);
    }

    let (lhs_type, rhs_type, self_lifetime) = index_mut_types(&function.sig, trait_name);
    let generic_params = remove_generic_param(generic_params, self_lifetime);
    let _ret_type = unwrap_mutable_reference_return(&function.sig);

    let output = quote! {
        impl #generic_params #trait_path<#rhs_type> for #lhs_type #where_clause {
            fn #fn_name <#self_lifetime> (& #self_lifetime mut self, rhs: #rhs_type) #ret {
                #function
                #fn_name(self, rhs)
            }
        }
    };
    output.into()
}

fn expect_one_param<'s>(sig: &'s syn::Signature, trait_name: &str) -> &'s syn::Type {
    let params = &sig.inputs;
    if params.len() != 1 {
        let span = sig.paren_token.span;
        abort!(span, "operation `{}` takes exactly 1 argument, found {}", trait_name, params.len());
    }
    if let syn::FnArg::Typed(lhs) = &params[0] {
        lhs.ty.as_ref()
    } else {
        abort!(&params, "`self` receivers can only be used in associated methods");
    }
}

fn expect_two_params<'s>(sig: &'s syn::Signature, trait_name: &str) -> (&'s syn::Type, &'s syn::Type) {
    let params = &sig.inputs;
    if params.len() != 2 {
        let span = sig.paren_token.span;
        abort!(span, "operation `{}` takes exactly 2 arguments, found {}", trait_name, params.len());
    }
    if let (syn::FnArg::Typed(lhs), syn::FnArg::Typed(rhs)) = (&params[0], &params[1]) {
        (lhs.ty.as_ref(), rhs.ty.as_ref())
    } else {
        abort!(&params, "`self` receivers can only be used in associated methods");
    }
}

fn un_type<'s>(sig: &'s syn::Signature, trait_name: &str) -> &'s syn::Type {
    expect_one_param(sig, trait_name)
}

fn bin_types<'s>(sig: &'s syn::Signature, trait_name: &str) -> (&'s syn::Type, &'s syn::Type) {
    expect_two_params(sig, trait_name)
}

fn assign_types<'s>(sig: &'s syn::Signature, trait_name: &str) -> (&'s syn::Type, &'s syn::Type, Option<&'s syn::Lifetime>) {
    let (lhs, rhs) = expect_two_params(sig, trait_name);
    let lhs_ref_type = unwrap_reference(lhs).filter(|typ| typ.mutability.is_some()).unwrap_or_else(||
        abort!(lhs, "the first operand of `{}` must be a mutable reference", sig.ident.to_string())
    );
    (lhs_ref_type.elem.as_ref(), rhs, lhs_ref_type.lifetime.as_ref())
}

fn deref_types<'s>(sig: &'s syn::Signature, trait_name: &str) -> (&'s syn::Type, Option<&'s syn::Lifetime>) {
    let typ = expect_one_param(sig, trait_name);
    let ref_type = unwrap_reference(typ).unwrap_or_else(||
        abort!(typ, "`{}` must take a reference", sig.ident.to_string())
    );
    if ref_type.mutability.is_some() {
        abort!(typ, "`{}` must take an immutable reference", sig.ident.to_string())
    }
    (ref_type.elem.as_ref(), ref_type.lifetime.as_ref())
}

fn deref_mut_types<'s>(sig: &'s syn::Signature, trait_name: &str) -> (&'s syn::Type, Option<&'s syn::Lifetime>) {
    let typ = expect_one_param(sig, trait_name);
    let ref_type = unwrap_reference(typ).filter(|typ| typ.mutability.is_some()).unwrap_or_else(||
        abort!(typ, "`{}` must take a mutable reference", sig.ident.to_string())
    );
    (ref_type.elem.as_ref(), ref_type.lifetime.as_ref())
}

fn index_types<'s>(sig: &'s syn::Signature, trait_name: &str) -> (&'s syn::Type, &'s syn::Type, Option<&'s syn::Lifetime>) {
    let (lhs, rhs) = expect_two_params(sig, trait_name);
    let lhs_ref_type = unwrap_reference(lhs).unwrap_or_else(||
        abort!(lhs, "the first operand of `{}` must be a reference", sig.ident.to_string())
    );
    if lhs_ref_type.mutability.is_some() {
        abort!(lhs, "the first operand of `{}` must be an immutable reference", sig.ident.to_string())
    }
    (lhs_ref_type.elem.as_ref(), rhs, lhs_ref_type.lifetime.as_ref())
}

fn index_mut_types<'s>(sig: &'s syn::Signature, trait_name: &str) -> (&'s syn::Type, &'s syn::Type, Option<&'s syn::Lifetime>) {
    let (lhs, rhs) = expect_two_params(sig, trait_name);
    let lhs_ref_type = unwrap_reference(lhs).filter(|typ| typ.mutability.is_some()).unwrap_or_else(||
        abort!(lhs, "the first operand of `{}` must be a mutable reference", sig.ident.to_string())
    );
    (lhs_ref_type.elem.as_ref(), rhs, lhs_ref_type.lifetime.as_ref())
}

fn remove_reference(typ: &syn::Type) -> Option<&syn::Type> {
    unwrap_reference(typ).map(|ref_type| ref_type.elem.as_ref())
}

fn unwrap_reference(typ: &syn::Type) -> Option<&syn::TypeReference> {
    // NOTE: This only works for types that look syntactically like references.
    // This means that it fails for type aliases like `type Ref<T> = &T;`. This
    // could potentially be fixed using `RemoveRef::WithoutRef` from
    // https://rust-lang.github.io/rfcs/2532-associated-type-defaults.html,
    // however that would require exporting a trait type, and thus a second
    // crate. Perhaps one day this will make it into `std` and we can use that.
    if let syn::Type::Reference(ref_type) = typ {
        Some(ref_type)
    } else {
        None
    }
}

fn unwrap_reference_return(sig: &syn::Signature) -> &syn::Type {
    match &sig.output {
        syn::ReturnType::Type(_, typ) => {
            if let Some(typ) = unwrap_reference(typ.as_ref()) {
                if typ.mutability.is_none() {
                    typ.elem.as_ref()
                } else {
                    abort!(typ, "`{}` must return an immutable reference type", sig.ident.to_string());
                }
            } else {
                abort!(typ, "`{}` must return a reference type", sig.ident.to_string());
            }
        }
        syn::ReturnType::Default => {
            abort!(sig, "`{}` must return a reference type", sig.ident.to_string();
                note = "found `()`");
        }
    }
}

fn unwrap_mutable_reference_return(sig: &syn::Signature) -> &syn::Type {
    match &sig.output {
        syn::ReturnType::Type(_, typ) => {
            if let Some(typ) = unwrap_reference(typ.as_ref()) {
                if typ.mutability.is_some() {
                    typ.elem.as_ref()
                } else {
                    abort!(typ, "`{}` must return a mutable reference type", sig.ident.to_string());
                }
            } else {
                abort!(typ, "`{}` must return a mutable reference type", sig.ident.to_string());
            }
        }
        syn::ReturnType::Default => {
            abort!(sig, "`{}` must return a mutable reference type", sig.ident.to_string();
                note = "found `()`");
        }
    }
}

/// Remove the given lifetime from the list of generic parameters.
///
/// This should only by necessary to move the lifetime on a `&self` or
/// `&mut self` receiver from the `impl`'s generic parameters to the function's
/// generic parameters. More generally, we would need to find any occurrences
/// of the lifetime in any types or `where` clauses, but fortunately none of
/// the supported traits permit parameterizing the function beyond a `self`
/// lifetime.
fn remove_generic_param(mut generic_params: syn::Generics, remove: Option<&syn::Lifetime>) -> syn::Generics {
    if let Some(remove) = remove {
        generic_params.params = generic_params.params
            .into_pairs()
            .filter(|pair|
                if let syn::GenericParam::Lifetime(lifetime) = pair.value() {
                    &lifetime.lifetime != remove
                } else {
                    true
                }
            )
            .collect();
    }
    generic_params
}
