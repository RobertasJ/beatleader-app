use std::{borrow::Cow, clone::Clone, cmp::PartialEq};

use crate::prelude::*;

const H1_SIZE: f32 = 2.0 * 16.0;
const H2_SIZE: f32 = 1.5 * 16.0;
const H3_SIZE: f32 = 1.17 * 16.0;
const H4_SIZE: f32 = 1.0 * 16.0;
const H5_SIZE: f32 = 0.83 * 16.0;
const H6_SIZE: f32 = 0.67 * 16.0;

macro_rules! header_component {
    ($name:ident, $builder_mod:ident, $builder_ty:ident, $size:expr) => {
        #[component(bon, container_ext, maybe_ext)]
        #[derive(PartialEq)]
        pub fn $name(
            #[builder(field)]
            #[component(layout)]
            layout: LayoutData,
            #[builder(field)]
            #[component(text_style)]
            text_style_data: TextStyleData,
            // #[builder(field)]
            // #[component(layer)]
            // relative_layer: Layer,
            // #[builder(field)]
            // #[component(event_handlers)]
            // event_handlers: EventHandlers,
            #[builder(field)] spans: Vec<Span<'static>>,

            #[builder(default)] selectable: bool,
        ) -> impl IntoElement {
            if selectable {
                let mut selectable_text = freya::prelude::SelectableText::new();

                for span in spans {
                    selectable_text = selectable_text.span(span);
                }

                selectable_text
                    .layout(layout)
                    .text_style(text_style_data)
                    .font_size($size)
                    // .layer(relative_layer)
                    // .event_handlers(event_handlers)
                    .a11y_role(AccessibilityRole::Header)
                    .into_element()
            } else {
                paragraph()
                    .spans_iter(spans.into_iter())
                    .layout(layout)
                    .text_style(text_style_data)
                    .font_size($size)
                    // .layer(relative_layer)
                    // .event_handlers(event_handlers)
                    .a11y_role(AccessibilityRole::Header)
                    .into_element()
            }
        }

        impl<S: $builder_mod::State> $builder_ty<S> {
            pub fn span(mut self, span: impl Into<Span<'static>>) -> Self {
                self.spans.push(span.into());
                self
            }
        }
    };
}

header_component!(H1, h1_builder, H1Builder, H1_SIZE);
header_component!(H2, h2_builder, H2Builder, H2_SIZE);
header_component!(H3, h3_builder, H3Builder, H3_SIZE);
header_component!(H4, h4_builder, H4Builder, H4_SIZE);
header_component!(H5, h5_builder, H5Builder, H5_SIZE);
header_component!(H6, h6_builder, H6Builder, H6_SIZE);

#[component(bon, container_ext, maybe_ext)]
#[derive(PartialEq)]
pub fn Text(
    #[builder(field)]
    #[component(layout)]
    layout: LayoutData,
    #[builder(field)]
    #[component(text_style)]
    text_style_data: TextStyleData,
    #[builder(field)]
    #[component(layer)]
    relative_layer: Layer,
    #[builder(field)]
    #[component(event_handlers)]
    event_handlers: EventHandlers,

    #[builder(field)] spans: Vec<Span<'static>>,
    #[builder(default)] selectable: bool,
) -> impl IntoElement {
    if selectable {
        let mut selectable_text = freya::prelude::SelectableText::new();

        for span in spans {
            selectable_text = selectable_text.span(span);
        }

        selectable_text
            .layout(layout)
            .text_style(text_style_data)
            // .layer(relative_layer)
            // .event_handlers(event_handlers)
            .a11y_role(AccessibilityRole::Paragraph)
            .into_element()
    } else {
        paragraph()
            .spans_iter(spans.into_iter())
            .layout(layout)
            .text_style(text_style_data)
            // .layer(relative_layer)
            // .event_handlers(event_handlers)
            .a11y_role(AccessibilityRole::Paragraph)
            .into_element()
    }
}

impl<S: text_builder::State> TextBuilder<S> {
    pub fn span(mut self, span: impl Into<Span<'static>>) -> Self {
        self.spans.push(span.into());
        self
    }
}
