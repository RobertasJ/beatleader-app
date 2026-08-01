use crate::prelude::*;

#[component(bon)]
#[derive(PartialEq)]
pub fn Rank(#[builder(start_fn)] rank: u64, country_rank: Option<String>) -> impl IntoElement {
    let theme = use_theme().read();

    container()
        .background(theme.colors.surface_secondary)
        .rounded()
        .cursor_icon(CursorIcon::Pointer)
        .padding(Gaps::new_symmetric(2.0, 5.0))
        .child(
            paragraph()
                .span("#")
                .span(rank.to_string())
                .map(country_rank, |p, country_rank| {
                    p.span(" ").span(country_rank)
                }),
        )
}
