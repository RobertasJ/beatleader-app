use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{ToTokens, quote};
use syn::{
    Ident,
    punctuated::Punctuated,
    spanned::Spanned,
    token::{Colon, Comma},
    *,
};

use crate::{
    attrs::Attributes,
    component::component_attrs,
};

mod attrs;
mod component;

#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let macro_attrs = parse_macro_input!(attr as Attributes);
    let marker_extensions = macro_attrs.marker_extensions();

    let ItemFn {
        mut attrs,
        vis,
        sig,
        block,
    } = input;

    attrs.retain(|v| !v.path().is_ident("doc"));

    let Signature {
        constness,
        asyncness,
        unsafety,
        abi,
        fn_token: _,
        ident,
        mut generics,
        paren_token: _,
        inputs,
        variadic,
        output,
    } = sig.clone();

    if let Some(constness) = constness {
        return syn::Error::new_spanned(constness, "const functions are not supported")
            .to_compile_error()
            .into();
    }

    if let Some(asyncness) = asyncness {
        return syn::Error::new_spanned(asyncness, "async functions are not supported")
            .to_compile_error()
            .into();
    }

    if let Some(unsafety) = unsafety {
        return syn::Error::new_spanned(unsafety, "unsafe functions are not supported")
            .to_compile_error()
            .into();
    }

    if let Some(abi) = abi {
        return syn::Error::new_spanned(abi, "extern functions are not supported")
            .to_compile_error()
            .into();
    }

    if let Some(variadic) = variadic {
        return syn::Error::new_spanned(variadic, "variadic inputs are not supported")
            .to_compile_error()
            .into();
    }

    // dont allow self parameter. change impl Trait into T with trait bounds
    let input = inputs
        .into_iter()
        .map(|i| match i {
            FnArg::Receiver(_) => Err(syn::Error::new_spanned(
                i,
                "self parameter is not supported",
            )
            .to_compile_error())
            .unwrap(),
            FnArg::Typed(t) => t,
        })
        .map(|mut input| {
            recursive_extract_impl_trait_from_fn_input(&mut generics, &mut input);
            input
        })
        .collect::<Punctuated<_, Comma>>();

    let component_attrs = match component_attrs(&input) {
        Ok(v) => v,
        Err(err) => {
            return err.to_compile_error().into();
        }
    };

    let extensions = component_attrs.extensions();

    let inputs = remove_component_attrs(input);

    let Generics {
        lt_token: _,
        mut params,
        gt_token: _,
        where_clause,
    } = generics.clone();

    let params_without_bounds = remove_bounds_from_generic_params(&params);

    let fields = map_inputs_to_fields(&inputs);

    let inputs_without_attrs = strip_input_attrs(&inputs);

    let field_idents = fields
        .iter()
        .map(|v| v.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let input_destructuring_patterns = inputs
        .iter()
        .map(|pat_type| *pat_type.pat.clone())
        .collect::<Vec<_>>();

    let struct_body = if fields.is_empty() && macro_attrs.bon.is_none() {
        quote! {;}
    } else {
        quote! {
            {
                #(#fields),*
            }
        }
    };

    let struct_constructor = if fields.is_empty() && macro_attrs.bon.is_none() {
        quote! {}
    } else {
        quote! {
            {
                #(#field_idents),*
            }
        }
    };

    let fn_sig = function_signature_string(&vis, &sig, &block);
    let doc = format!(
        "```rust\n{}\n```\n\nThis struct was generated using the `#[component]` macro.\n\n",
        fn_sig
    );

    let bon_derive = if macro_attrs.bon.is_some() {
        quote! {
            #[derive(bon::Builder)]
            #[builder(start_fn(name = new), state_mod(vis = "pub(self)"))]
        }
    } else {
        quote! {}
    };

    let builder_ident = Ident::new(&format!("{}Builder", ident), ident.span());
    let builder_module = Ident::new(
        &format!(
            "{}_builder",
            ident.to_string().chars().fold(String::new(), |mut acc, c| {
                if c.is_uppercase() {
                    if !acc.is_empty() {
                        acc.push('_');
                        acc.push(c.to_lowercase().next().unwrap());
                        acc
                    } else {
                        acc.push(c.to_lowercase().next().unwrap());
                        acc
                    }
                } else {
                    acc.push(c);
                    acc
                }
            })
        ),
        ident.span(),
    );

    let state_generic: GenericParam = parse_quote! { S: #builder_module::IsComplete };

    params.pop_punct();

    // i want to kill myself
    let comma_maybe = if params.is_empty() {
        quote! {}
    } else {
        quote! {,}
    };

    let new_definition = if macro_attrs.bon.is_some() {
        let params = params.iter();
        quote! {
            impl <#(#params),* #comma_maybe #state_generic> IntoElement
            for #builder_ident <#(#params_without_bounds),* #comma_maybe S>
            #where_clause {
                fn into_element(self) -> Element {
                    self.build().into_element()
                }
            }
        }
    } else {
        quote! {
            impl <#params> #ident <#(#params_without_bounds),*> #where_clause {
                pub fn new(#inputs) -> Self {
                    #(
                        let #field_idents = #input_destructuring_patterns;
                    )*
                    Self #struct_constructor
                }
            }
        }
    };

    let mut tokens = quote! {
        #bon_derive
        #[doc = #doc]
        #(#attrs)*
        #vis struct #ident #generics #struct_body

        #new_definition

        impl <#params> Component for #ident <#(#params_without_bounds),*> #where_clause {
            fn render(&self) #output {
                fn inner <#params> (#inputs_without_attrs) #output #where_clause {
                    #block
                }

                inner(#(self.#field_idents.clone()),*)
            }

        }
    };

    let state_generic: GenericParam = parse_quote! { S: #builder_module::State };

    for extension in extensions.into_iter().flatten() {
        let (field, trait_name, method_name, return_type) = extension;
        if macro_attrs.bon.is_none() {
            return syn::Error::new_spanned(
                field,
                "component extension traits can only be used with bon",
            )
            .to_compile_error()
            .into();
        }
        let params = params.iter();
        tokens.extend(quote! {
            impl <#(#params),* #comma_maybe #state_generic> #trait_name
            for #builder_ident <#(#params_without_bounds),* #comma_maybe S>
            #where_clause {
                fn #method_name(&mut self) -> #return_type {
                    &mut self.#field
                }
            }
        });
    }

    for (token, trait_name) in marker_extensions.into_iter().flatten() {
        if macro_attrs.bon.is_none() {
            return syn::Error::new_spanned(
                token,
                "component extension traits can only be used with bon",
            )
            .to_compile_error()
            .into();
        }
        let params = params.iter();
        tokens.extend(quote! {
            impl <#(#params),* #comma_maybe #state_generic> #trait_name
            for #builder_ident <#(#params_without_bounds),* #comma_maybe S>
            #where_clause {}
        });
    }

    // if let Ok(file) = syn::parse::<File>(tokens.clone().into()) {
    //     println!("{}", prettyplease::unparse(&file));
    // } else {
    //     println!("{}", tokens.to_string());
    // }

    tokens.into()
}

fn function_signature_string(vis: &Visibility, sig: &Signature, block: &Box<Block>) -> String {
    let mut fn_sig = prettyplease::unparse(&File {
        items: vec![Item::Fn(ItemFn {
            attrs: vec![],
            vis: vis.clone(),
            sig: sig.clone(),
            block: Box::new(Block {
                brace_token: block.brace_token,
                stmts: vec![],
            }),
        })],
        attrs: vec![],
        shebang: None,
    });
    fn_sig.truncate(fn_sig.len() - 3);
    fn_sig
}

fn strip_input_attrs(inputs: &Punctuated<PatType, Comma>) -> Punctuated<PatType, Comma> {
    inputs
        .clone()
        .into_iter()
        .map(|mut i| {
            i.attrs.clear();
            i
        })
        .collect::<Punctuated<_, Comma>>()
}

fn map_inputs_to_fields(inputs: &Punctuated<PatType, Comma>) -> Vec<Field> {
    fn extract_ident_from_pat(pat: &Pat, default: Ident) -> Ident {
        match pat {
            Pat::Ident(PatIdent { ident, .. }) => ident.clone(),
            Pat::Reference(PatReference { pat, .. }) => extract_ident_from_pat(pat, default),
            _ => default,
        }
    }

    inputs
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, pat_type)| Field {
            attrs: pat_type.attrs,
            vis: syn::Visibility::Inherited,
            ident: Some(extract_ident_from_pat(
                &*pat_type.pat,
                Ident::new(&format!("arg{}", i), pat_type.pat.span()),
            )),
            colon_token: Some(pat_type.colon_token),
            ty: *pat_type.ty,
            mutability: syn::FieldMutability::None,
        })
        .collect::<Vec<_>>()
}

fn remove_bounds_from_generic_params(
    params: &Punctuated<GenericParam, Comma>,
) -> Vec<GenericParam> {
    params
        .clone()
        .into_iter()
        .map(|v| match v {
            syn::GenericParam::Lifetime(lifetime_param) => {
                syn::GenericParam::Lifetime(LifetimeParam::new(lifetime_param.lifetime))
            }
            syn::GenericParam::Type(type_param) => syn::GenericParam::Type(type_param.ident.into()),
            syn::GenericParam::Const(const_param) => {
                syn::GenericParam::Type(const_param.ident.into())
            }
        })
        .collect::<Vec<_>>()
}

fn remove_component_attrs(inputs: Punctuated<PatType, Comma>) -> Punctuated<PatType, Comma> {
    inputs
        .into_iter()
        .map(|mut i| {
            i.attrs = i
                .attrs
                .clone()
                .into_iter()
                .filter(|attr| !attr.path().is_ident("component"))
                .collect();
            i
        })
        .collect::<Punctuated<_, Comma>>()
}

fn recursive_extract_impl_trait_from_fn_input(generics: &mut Generics, input: &mut PatType) {
    if let Type::ImplTrait(ty) = input.ty.as_ref().clone() {
        let ty = extract_impl_trait(generics, ty);
        *input.ty = ty;
    } else if let Type::Path(TypePath { path, .. }) = &mut *input.ty {
        extract_impl_trait_from_type_path(generics, path);
    }
}

fn extract_impl_trait_from_type_path(generics: &mut Generics, path: &mut Path) {
    let arguments = &mut path.segments.last_mut().unwrap().arguments;
    if let syn::PathArguments::AngleBracketed(angle_bracketed_generic_arguments) = arguments {
        for arg in &mut angle_bracketed_generic_arguments.args {
            if let GenericArgument::Type(type_generic) = arg {
                match type_generic {
                    Type::ImplTrait(implt) => {
                        let ty = extract_impl_trait(generics, implt.clone());
                        *arg = GenericArgument::Type(ty);
                    }
                    Type::Path(path) => extract_impl_trait_from_type_path(generics, &mut path.path),
                    _ => (),
                }
            }
        }
    }
}

fn extract_impl_trait(generics: &mut Generics, ty: TypeImplTrait) -> Type {
    let mut generic_name = ty.to_token_stream().to_string();
    generic_name.retain(|c| c.is_alphabetic() && !c.is_whitespace());
    generic_name.remove(0);
    generic_name.insert(0, 'I');
    generic_name.push_str(&generics.params.len().to_string());

    let ident = Ident::new(&generic_name, Span::mixed_site());
    generics.params.push(GenericParam::Type(TypeParam {
        attrs: vec![],
        ident: ident.clone(),
        colon_token: Some(Colon {
            spans: [Span::mixed_site()],
        }),
        bounds: ty.bounds,
        eq_token: None,
        default: None,
    }));

    Type::Path(TypePath {
        qself: None,
        path: Path {
            leading_colon: None,
            segments: [PathSegment {
                arguments: syn::PathArguments::None,
                ident,
            }]
            .into_iter()
            .collect(),
        },
    })
}
