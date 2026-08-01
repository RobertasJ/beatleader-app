use crate::prelude::*;

#[component]
#[derive(PartialEq, Debug)]
pub fn PlayerPage(id: PlayerId) -> impl IntoElement {
    let player_query = use_query(Query::new(id, FetchPlayer));

    unquery(player_query)
        .map_ok(|player| {
            PageContainer::new()
                .child(PlayerCard::new(player.clone()))
                .child(PlayerScores::new(player.clone()))
                .build()
        })
        .unwrap_or_default()
}

#[component]
#[derive(PartialEq)]
pub fn PlayerScores(player: objects::player_api::Player) -> impl IntoElement {
    let theme = use_theme();
    let scores_query = use_query(Query::new(player.id.clone(), FetchPlayerScores));

    unquery(scores_query)
        .map_ok(move |scores| {
            Card::new().child(
                rect()
                    .spacing(20.0)
                    .width(90.percent())
                    .max_width(800.px())
                    .content(Content::Fit)
                    .children(
                        scores
                            .iter()
                            .map(|score| PlayerScore::new(score.clone()).into_element())
                            .intersperse(
                                rect()
                                    .width(Size::fill_minimum())
                                    .height(4.px())
                                    .background(theme.read().colors.background)
                                    .into_element(),
                            ),
                    ),
            )
        })
        .map_err(|err| Card::new().child(label().text(format!("{}", err))))
        .unwrap_or(Card::new().child(label().text("loading...")))
}

#[component]
#[derive(PartialEq)]
pub fn PlayerScore(score: objects::scores_api::Score) -> impl IntoElement {
    let theme = use_theme().read();
    let mut app_state = use_consume::<State<AppState>>();

    container()
        .rounded_lg()
        .cursor_icon(CursorIcon::Pointer)
        .hover_background(theme.colors.surface_secondary)
        .width(Fill)
        .padding(5.0)
        .child(
            rect()
                .content(Content::Flex)
                .main_align(Alignment::SpaceBetween)
                .direction(Horizontal)
                .width(100.percent())
                .spacing(50.0)
                .child(
                    rect()
                        .width(Size::flex(5.0))
                        .direction(Horizontal)
                        .spacing(20.0)
                        .child(
                            rect()
                                .child(
                                    Icon::new(score.leaderboard.song.cover)
                                        .width(100.px())
                                        .height(100.px())
                                        .rounded_lg(),
                                )
                                .child(
                                    rect()
                                        .background(
                                            Color::from_hex(
                                                score.leaderboard.difficulty.value.color_hex(),
                                            )
                                            .unwrap(),
                                        )
                                        .color(
                                            if Color::from_hex(
                                                score.leaderboard.difficulty.value.color_hex(),
                                            )
                                            .unwrap()
                                            .is_light()
                                            {
                                                theme.colors.text_primary
                                            } else {
                                                theme.colors.text_inverse
                                            },
                                        )
                                        .layer(Layer::Overlay)
                                        .rounded_sm()
                                        .padding(1.0)
                                        .position(Position::new_absolute().left(-15.0).top(10.0))
                                        .child(
                                            paragraph()
                                                .span(score.leaderboard.difficulty.stars.map_or(
                                                    "difficulty_placeholder".to_string(),
                                                    |v| format!("{:.2}★", v),
                                                ))
                                                .font_size(16.0 * 0.8),
                                        ),
                                ),
                        )
                        .child(
                            rect()
                                .child(
                                    container()
                                        .rounded_lg()
                                        .hover_color(theme.colors.text_highlight)
                                        .on_pointer_press(move |_e: Event<PointerEventData>| {
                                            app_state.write().select_page(
                                                PageSelected::Leaderboard {
                                                    id: score.leaderboard.id.clone(),
                                                },
                                            );
                                        })
                                        .child(
                                            paragraph()
                                                .span(score.leaderboard.song.name)
                                                .text_overflow(TextOverflow::Ellipsis),
                                        )
                                        .child(paragraph().span(score.leaderboard.song.author)),
                                )
                                .child(
                                    container()
                                        .color(
                                            theme.colors.text_primary.with_a((0.5 * 255.0) as u8),
                                        )
                                        .hover_color(
                                            theme.colors.text_highlight.with_a((0.5 * 255.0) as u8),
                                        )
                                        .child(
                                            paragraph()
                                                .font_size(0.8.em())
                                                .span(score.leaderboard.song.mapper),
                                        ),
                                ),
                        ),
                )
                .child(
                    rect()
                        .width(Size::flex(1.0))
                        .child(
                            rect().child(
                                paragraph()
                                    .span(format!("{:.2}", score.accuracy * 100.0))
                                    .span("%")
                                    .max_lines(Some(1)),
                            ),
                        )
                        .maybe_child(score.pp.pp().map(|pp| {
                            paragraph()
                                .span(format!("{:.2}", pp))
                                .span("pp")
                                .max_lines(Some(1))
                        })),
                ),
        )
}
