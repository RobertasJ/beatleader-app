use reqwest::IntoUrl;

use crate::prelude::*;

#[component(bon, container_ext)]
#[derive(PartialEq)]
pub fn Icon(
    #[builder(start_fn)] url: impl IntoUrl + Clone + PartialEq + 'static,
    #[builder(field)]
    #[component(layout)]
    layout: LayoutData,
    #[builder(field)]
    #[component(style)]
    style: StyleState,
) -> impl IntoElement {
    rect()
        .corner_radius(style.corner_radius)
        .overflow(Overflow::Clip)
        .layout(layout)
        .child(ImageViewer::new(url.into_url().unwrap()).into_element())
}
