use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, token::Comma, *};

pub struct ComponentAttrs {
    pub layout: Option<PatIdent>,
    pub event_handlers: Option<PatIdent>,
    pub text_style: Option<PatIdent>,
    pub layer: Option<PatIdent>,
    pub style: Option<PatIdent>,
    pub children: Option<PatIdent>,
    pub key: Option<PatIdent>,
    pub image_data: Option<PatIdent>,
    pub accessibility: Option<PatIdent>,
    pub scrollable_effect: Option<PatIdent>,
    pub interactive_effect: Option<PatIdent>,
    pub effect: Option<PatIdent>,
}

impl ComponentAttrs {
    pub fn extensions(&self) -> [Option<(PatIdent, TokenStream, TokenStream, TokenStream)>; 12] {
        [
            self.layout.clone().map(|field| {
                (
                    field,
                    quote!(LayoutExt),
                    quote!(get_layout),
                    quote!(&mut LayoutData),
                )
            }),
            self.layer.clone().map(|field| {
                (
                    field,
                    quote!(LayerExt),
                    quote!(get_layer),
                    quote!(&mut Layer),
                )
            }),
            self.event_handlers.clone().map(|field| {
                (
                    field,
                    quote!(EventHandlersExt),
                    quote!(get_event_handlers),
                    quote!(&mut EventHandlers),
                )
            }),
            self.text_style.clone().map(|field| {
                (
                    field,
                    quote!(TextStyleExt),
                    quote!(get_text_style_data),
                    quote!(&mut TextStyleData),
                )
            }),
            self.style.clone().map(|field| {
                (
                    field,
                    quote!(StyleExt),
                    quote!(get_style),
                    quote!(&mut StyleState),
                )
            }),
            self.children.clone().map(|field| {
                (
                    field,
                    quote!(ChildrenExt),
                    quote!(get_children),
                    quote!(&mut Vec<Element>),
                )
            }),
            self.key.clone().map(|field| {
                (
                    field,
                    quote!(KeyExt),
                    quote!(write_key),
                    quote!(&mut DiffKey),
                )
            }),
            self.image_data.clone().map(|field| {
                (
                    field,
                    quote!(ImageExt),
                    quote!(get_image_data),
                    quote!(&mut ImageData),
                )
            }),
            self.accessibility.clone().map(|field| {
                (
                    field,
                    quote!(AccessibilityExt),
                    quote!(get_accessibility_data),
                    quote!(&mut AccessibilityData),
                )
            }),
            self.scrollable_effect.clone().map(|field| {
                (
                    field,
                    quote!(ScrollableExt),
                    quote!(get_effect),
                    quote!(&mut EffectData),
                )
            }),
            self.interactive_effect.clone().map(|field| {
                (
                    field,
                    quote!(InteractiveExt),
                    quote!(get_effect),
                    quote!(&mut EffectData),
                )
            }),
            self.effect.clone().map(|field| {
                (
                    field,
                    quote!(EffectExt),
                    quote!(get_effect),
                    quote!(&mut EffectData),
                )
            }),
        ]
    }
}

pub fn component_attrs(inputs: &Punctuated<PatType, Comma>) -> Result<ComponentAttrs> {
    let component_attributes = [
        "layout",
        "event_handlers",
        "text_style",
        "layer",
        "style",
        "children",
        "key",
        "image_data",
        "accessibility",
        "scrollable_effect",
        "interactive_effect",
        "effect",
    ];
    let mut component_fields = [
        None, None, None, None, None, None, None, None, None, None, None, None,
    ];

    for i in inputs.iter() {
        for v in i.attrs.iter().filter(|v| v.path().is_ident("component")) {
            let nested = v
                .parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated)
                .unwrap();

            let Pat::Ident(ident) = i.pat.as_ref().clone() else {
                return Err(syn::Error::new_spanned(
                    i,
                    "component arguments must be named",
                ));
            };

            for meta in &nested {
                if let Meta::Path(path) = meta {
                    let Some((index, name)) = component_attributes
                        .iter()
                        .enumerate()
                        .find(|(_, name)| path.is_ident(name))
                    else {
                        return Err(syn::Error::new_spanned(
                            meta,
                            "unexpected 'component' attribute",
                        ));
                    };

                    let field = &mut component_fields[index];
                    if field.is_some() {
                        return Err(syn::Error::new_spanned(
                            meta,
                            format!("duplicate '{name}' attribute"),
                        ));
                    }
                    *field = Some(ident.clone());
                } else {
                    return Err(syn::Error::new_spanned(
                        meta,
                        "unexpected 'component' attribute",
                    ));
                }
            }
        }
    }

    let [
        layout,
        event_handlers,
        text_style,
        layer,
        style,
        children,
        key,
        image_data,
        accessibility,
        scrollable_effect,
        interactive_effect,
        effect,
    ] = component_fields;

    Ok(ComponentAttrs {
        layout,
        event_handlers,
        text_style,
        layer,
        style,
        children,
        key,
        image_data,
        accessibility,
        scrollable_effect,
        interactive_effect,
        effect,
    })
}
