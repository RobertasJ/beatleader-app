use crate::prelude::*;

#[component]
#[derive(PartialEq)]
pub fn LeaderboardPage(id: LeaderboardId) -> impl IntoElement {
    let leaderboard_query = use_query(Query::new(id, FetchLeaderboard));

    unquery(leaderboard_query)
        .map_ok(|leaderboard| rect().child(SongCard::new(leaderboard.song.clone())))
        .unwrap_or(rect())
}

#[component]
#[derive(PartialEq)]
pub fn SongCard(song: objects::Song) -> impl IntoElement {
    Card::new().width(px(50)).height(px(50))
}
