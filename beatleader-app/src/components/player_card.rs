use crate::prelude::*;

#[component(bon, container_ext)]
#[derive(PartialEq)]
pub fn PlayerCard(
    #[builder(start_fn)] player: objects::player_api::Player,
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
                        .cross_align(Center)
                        .spacing(20.0)
                        .direction(Horizontal)
                        .child(
                            Icon::new(player.avatar)
                                .width(50.px())
                                .height(50.px())
                                .rounded_lg(),
                        )
                        .child(paragraph().font_size(1.5.em()).span(player.name)),
                )
                .child(
                    rect()
                        .spacing(5.0)
                        .child(Rank::new(player.rank))
                        .child(Rank::new(player.country_rank).country_rank(player.country)),
                ),
        )
}
