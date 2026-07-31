use crate::prelude::*;

#[component(bon, container_ext)]
#[derive(PartialEq)]
pub fn PlayerCard(
    #[builder(start_fn)] player: objects::Player,
    #[builder(field)]
    #[component(layout)]
    layout: LayoutData,
) -> impl IntoElement {
    let mut app_state = use_consume::<State<AppState>>();

    Card::new()
        .cursor_icon(CursorIcon::Pointer)
        .on_press(move |_e| {
            app_state.write().select_page(PageSelected::Player {
                id: player.id.clone(),
            });
        })
        .hoverable(true)
        .layout(layout)
        .child(
            rect()
                .spacing(20.0)
                .child(
                    rect()
                        .cross_align(Alignment::Center)
                        .spacing(20.0)
                        .direction(Direction::Horizontal)
                        .child(
                            Icon::new(player.avatar)
                                .width(px(50))
                                .height(px(50))
                                .rounded_lg(),
                        )
                        .child(paragraph().font_size(em(1.5)).span(player.name)),
                )
                .child(
                    rect()
                        .spacing(5.0)
                        .child(Rank::new(player.rank))
                        .child(Rank::new(player.country_rank).country_rank(player.country)),
                ),
        )
}
