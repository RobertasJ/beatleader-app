use std::ops::Deref;

use crate::prelude::*;

#[component]
#[derive(PartialEq)]
pub fn LeaderboardPage(id: LeaderboardId) -> impl IntoElement {
    let leaderboard_query = use_query(Query::new(id, FetchLeaderboard));

    unquery(leaderboard_query)
        .map_ok(|leaderboard| {
            PageContainer::new()
                .child(SongCard::new(leaderboard.song.clone()))
                .child(LeaderboardScores::new(leaderboard.clone()))
                .child(leaderboard.id.deref().clone())
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

#[component]
#[derive(PartialEq)]
pub fn LeaderboardScores(leaderboard: objects::leaderboard_api::Leaderboard) -> impl IntoElement {
    let mut app_state = use_consume::<State<AppState>>();

    Card::new()
        .child(rect().direction(Horizontal).spacing(10.0).children(
            leaderboard.song.difficulties.into_iter().map(|diff| {
                rect()
                    .child(diff.difficulty_name)
                    .on_press(move |_e: Event<PressEventData>| {
                        app_state.write().select_page(PageSelected::Leaderboard {
                            id: LeaderboardId::from_song_id_difficulty_value_and_mode(
                                &diff.song_id,
                                diff.value,
                                diff.mode.clone(),
                            ),
                        });
                    })
            }),
        ))
        .child(
            rect()
                .spacing(20.0)
                .width(90.percent())
                .max_width(800.px())
                .content(Content::Fit)
                .children(
                    leaderboard
                        .scores
                        .into_iter()
                        .map(|score| LeaderbardScore::new(score)),
                ),
        )
}

#[component]
#[derive(PartialEq)]
pub fn LeaderbardScore(score: objects::leaderboard_api::Score) -> impl IntoElement {
    Card::new()
        .direction(Horizontal)
        .width(Fill)
        .child(Text::new().span("#").span(score.rank.to_string()))
        .child(Icon::new(score.player.avatar).width(20.px()))
}
