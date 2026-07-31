use crate::prelude::*;

#[component]
#[derive(PartialEq)]
pub fn LeaderboardPage(id: LeaderboardId) -> impl IntoElement {
    let leaderboard_query = use_query(Query::new(id, FetchLeaderboard));

    rect().map(leaderboard_query.read().state().ok(), |el, leaderboard| {
        el.child(SongCard::new(leaderboard.song.clone()))
    })
}

#[component]
#[derive(PartialEq)]
pub fn SongCard(song: objects::Song) -> impl IntoElement {
    Card::new().width(px(50)).height(px(50))
}
