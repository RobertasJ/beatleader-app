use crate::prelude::*;

#[component]
#[derive(PartialEq, Debug)]
pub fn PlayerPage(id: PlayerId) -> impl IntoElement {
    let player_query = use_query(Query::new(id, FetchPlayer));

    rect()
        .padding(50.0)
        .width(fill())
        .spacing(50.0)
        .cross_align(Alignment::Center)
        .map(player_query.read().state().ok(), |el, player| {
            el.child(PlayerCard::new(player.clone()))
                .child(PlayerScores::new(player.clone()))
        })
}

#[component]
#[derive(PartialEq)]
pub fn PlayerScores(player: objects::Player) -> impl IntoElement {
    let theme = use_theme();
    let scores_query = use_query(Query::new(player.id.clone(), FetchPlayerScores));

    Card::new().child(
        rect()
            .spacing(20.0)
            .width(percent(90))
            .max_width(px(800))
            .content(Content::Fit)
            .map(scores_query.read().state().ok(), |el, scores| {
                el.children(
                    scores
                        .iter()
                        .map(|score| PlayerScore::new(score.clone()).into_element())
                        .intersperse(
                            rect()
                                .width(Size::fill_minimum())
                                .height(px(4))
                                .background(theme.read().colors.background)
                                .into_element(),
                        ),
                )
            }),
    )
}

#[component]
#[derive(PartialEq)]
pub fn PlayerScore(score: objects::Score) -> impl IntoElement {
    let theme = use_theme().read();
    let mut app_state = use_consume::<State<AppState>>();

    container()
        .rounded_lg()
        .cursor_icon(CursorIcon::Pointer)
        .hover_background(theme.colors.surface_secondary)
        .width(fill())
        .padding(5.0)
        .child(
            rect()
                .content(Content::Flex)
                .main_align(Alignment::SpaceBetween)
                .direction(Direction::Horizontal)
                .width(percent(100))
                .spacing(50.0)
                .child(
                    rect()
                        .width(Size::flex(5.0))
                        .direction(Direction::Horizontal)
                        .spacing(20.0)
                        .child(
                            rect()
                                .child(
                                    Icon::new(score.leaderboard.song.cover)
                                        .width(px(100))
                                        .height(px(100))
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
                                    container().hover_color(theme.colors.text_highlight).child(
                                        paragraph()
                                            .font_size(em(0.8))
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
