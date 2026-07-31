use reqwest::IntoUrl;

use crate::prelude::*;

#[component]
#[derive(PartialEq)]
pub fn Nav() -> impl IntoElement {
    let theme = get_theme_or_default();

    rect()
        .direction(Direction::Horizontal)
        .spacing(10.0)
        .width(fill())
        .height(px(50))
        .padding(8.0)
        .background(theme.read().colors.surface_tertiary)
        .child(HomeButton)
        .child(UserButton)
}

#[component]
#[derive(PartialEq)]
fn HomeButton() -> impl IntoElement {
    let mut app_state = use_consume::<State<AppState>>();

    Button::new()
        .on_press(move |_e| {
            app_state.write().select_page(PageSelected::Default);
        })
        .child(rect().child("Home"))
        .padding(5.0)
        .cursor_icon(CursorIcon::Pointer)
}

#[component]
#[derive(PartialEq)]
fn UserButton() -> impl IntoElement {
    let mut app_state = use_consume::<State<AppState>>();
    let curr_player_query = use_query(Query::new((), FetchCurrentPlayer));

    Button::new().map(
        curr_player_query.read().state().ok().cloned(),
        |el, player| {
            el.on_press(move |_e| {
                app_state.write().select_page(PageSelected::Player {
                    id: player.id.clone(),
                });
            })
            .child(
                rect()
                    .height(fill())
                    .direction(Direction::Horizontal)
                    .rounded()
                    .spacing(5.0_f32)
                    // .background(theme.read().colors.surface_secondary)
                    .child(
                        rect()
                            .child(Icon::new(player.avatar.clone()))
                            .rounded()
                            .overflow(Overflow::Clip),
                    )
                    .child(player.name.clone()),
            )
            .padding(5.0)
            .cursor_icon(CursorIcon::Pointer)
        },
    )
}
