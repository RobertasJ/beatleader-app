use freya::query::{Query, use_query};

use crate::{prelude::*, queries::FetchCurrentPlayer};

#[component]
#[derive(PartialEq, Debug)]
pub fn WelcomePage() -> impl IntoElement {
    let curr_player_query = use_query(Query::new((), FetchCurrentPlayer));

    rect()
        .width(fill())
        .cross_align(Alignment::Center)
        .spacing(30.0)
        .map(curr_player_query.read().state().ok(), |el, player| {
            el.child(
                rect()
                    .cross_align(Alignment::Center)
                    .child(H1::new().span(format!("Welcome {}", player.name)))
                    .child(label().text("Today is a nice day to browse Beat Leader!")),
            )
            .child(
                PlayerCard::new(player.clone())
                    .width(percent(40))
                    .height(px(300)),
            )
        })
}
