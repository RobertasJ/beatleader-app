use crate::prelude::*;

#[component]
#[derive(PartialEq)]
pub fn LeaderboardPage(id: LeaderboardId) -> impl IntoElement {
    let leaderboard_query = use_query(Query::new(id, FetchLeaderboard));

    unquery(leaderboard_query)
        .map_ok(|leaderboard| {
            PageContainer::new()
                .child(SongCard::new(leaderboard.song.clone()))
                .build()
        })
        .unwrap_or_default()
}

#[component]
#[derive(PartialEq)]
pub fn SongCard(song: objects::leaderboard_api::Song) -> impl IntoElement {
    Card::new().child(
        ///
        rect()
            .direction(Horizontal)
            .child(
                ///
                Icon::new(song.cover).width(150.px()),
            )
            .child(
                rect()
                    .child(H1::new().span(song.name).selectable(true))
                    .child(H2::new().span(song.author).selectable(true)),
            ),
    )
}
