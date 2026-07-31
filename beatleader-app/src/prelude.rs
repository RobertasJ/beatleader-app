pub use crate::components::*;
pub use crate::queries::*;
pub use crate::{AppState, PageSelected};
pub use beatleader_api::{
    BlApi, objects, objects::LeaderboardId, objects::PlayerId, objects::ScoreId, objects::SongId,
};
pub use freya::icons::*;
pub use freya::prelude::*;
pub use freya::query::*;
pub use macros::*;
use num_traits::AsPrimitive;
pub use rustc_hash::FxHashMap;

pub fn px(px: impl AsPrimitive<f32>) -> Size {
    Size::px(px.as_())
}

pub fn percent(percent: impl AsPrimitive<f32>) -> Size {
    Size::percent(percent.as_())
}

pub fn fill() -> Size {
    Size::Fill
}

pub fn em(em: impl AsPrimitive<f32>) -> FontSize {
    (16.0 * em.as_()).into()
}
pub fn container() -> ContainerBuilder {
    Container::new()
}

pub struct Center;

impl From<Center> for TextAlign {
    fn from(value: Center) -> Self {
        TextAlign::Center
    }
}

pub trait LuminenceExt {
    fn luminence(&self) -> f32;

    fn is_light(&self) -> bool {
        self.luminence() > 0.5
    }

    fn is_dark(&self) -> bool {
        !self.is_light()
    }
}

impl LuminenceExt for Color {
    fn luminence(&self) -> f32 {
        let rgb = self.to_rgb();
        let r = rgb.r;
        let g = rgb.g;
        let b = rgb.b;

        0.2126 * (r as f32 / 255.0) + 0.7152 * (g as f32 / 255.0) + 0.0722 * (b as f32 / 255.0)
    }
}
