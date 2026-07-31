#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
#![feature(iter_intersperse)]

use crate::prelude::*;
use tokio::runtime::Builder;

pub mod components;
pub mod dirs;
pub mod prelude;
pub mod queries;

fn main() {
    let rt = Builder::new_multi_thread().enable_all().build().unwrap();
    let _rt = rt.enter();

    env_logger::init();
    launch(LaunchConfig::new().with_window(WindowConfig::new(App::new)))
}

#[component]
#[derive(PartialEq, Debug)]
fn App() -> impl IntoElement {
    let mut theme = use_init_theme(|| Platform::get().preferred_theme.read().to_theme());

    use_side_effect(move || {
        let read = Platform::get().preferred_theme.read();
        let new_theme = read.to_theme();

        theme.set(new_theme)
    });

    let mut app_state = use_provide_root_context(|| State::create(AppState::default()));

    rect()
        .on_global_key_down(move |e: Event<KeyboardEventData>| {
            if e.modifiers.alt() {
                match e.key {
                    Key::Named(NamedKey::ArrowLeft) => app_state.write().page_back(),
                    Key::Named(NamedKey::ArrowRight) => app_state.write().page_forward(),
                    _ => {}
                }
            }
        })
        .width(percent(100))
        .height(percent(100))
        .theme_background()
        .child(
            ScrollView::new()
                .child(Nav::new())
                .child(match &*app_state.read().curr_page() {
                    PageSelected::Default => WelcomePage::new().into_element(),
                    PageSelected::Player { id } => PlayerPage::new(id.clone()).into_element(),
                    PageSelected::Leaderboard { id } => {
                        LeaderboardPage::new(id.clone()).into_element()
                    }
                }),
        )
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppState {
    history: Vec<PageSelected>,
    history_index: usize,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            history: vec![PageSelected::Default],
            history_index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum PageSelected {
    #[default]
    Default,
    Player {
        id: PlayerId,
    },
    Leaderboard {
        id: LeaderboardId,
    },
}

impl AppState {
    pub fn select_page(&mut self, page: PageSelected) {
        if &page == self.curr_page() {
            return;
        }

        self.history.insert(self.history_index + 1, page);
        self.history_index += 1;
        self.history.truncate(self.history_index + 1);
    }

    pub fn curr_page(&self) -> &PageSelected {
        &self.history[self.history_index]
    }

    pub fn page_back(&mut self) {
        if self.history_index > 0 {
            self.history_index -= 1;
        }
    }

    pub fn page_forward(&mut self) {
        if self.history_index < self.history.len() - 1 {
            self.history_index += 1;
        }
    }
}
