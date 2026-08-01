pub use crate::components::*;
pub use crate::queries::*;
pub use crate::{AppState, PageSelected};
pub use beatleader_api::{
    BlApi, objects,
    objects::common::{LeaderboardId, PlayerId, ScoreId, SongId},
};
use bon::builder;
pub use freya::icons::*;
pub use freya::prelude::*;
pub use freya::query::*;
use log::error;
pub use macros::*;
use num_traits::AsPrimitive;
pub use rustc_hash::FxHashMap;

pub fn container() -> ContainerBuilder {
    Container::new()
}

pub use Direction::*;
pub use Size::{Fill, FillMinimum, Inner};

pub trait SizeHelper {
    fn px(self) -> Size;
    fn percent(self) -> Size;
    fn flex(self) -> Size;
    fn window_percent(self) -> Size;
    fn em(self) -> f32;
}

impl<T: AsPrimitive<f32>> SizeHelper for T {
    fn px(self) -> Size {
        Size::px(self.as_())
    }

    fn percent(self) -> Size {
        Size::percent(self.as_())
    }

    fn flex(self) -> Size {
        Size::flex(self.as_())
    }

    fn window_percent(self) -> Size {
        Size::window_percent(self.as_())
    }

    fn em(self) -> f32 {
        16.0 * self.as_()
    }
}

pub struct Center;

impl From<Center> for TextAlign {
    fn from(value: Center) -> Self {
        TextAlign::Center
    }
}

impl From<Center> for Alignment {
    fn from(value: Center) -> Self {
        Alignment::Center
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

#[builder(finish_fn(name = call_inner, vis = "pub(self)"), state_mod(vis = "pub(self)"))]
pub fn unquery<'a, 'b, 'c, Q: QueryCapability, E: IntoElement>(
    #[builder(start_fn)] query: UseQuery<Q>,

    ///
    #[builder(setters(name = default_inner, vis = "pub(self)"))]
    default: Box<dyn FnOnce() -> E + 'a>,

    ///
    #[builder(with = |map_ok: impl FnOnce(&Q::Ok) -> E + 'b| Box::new(map_ok))]
    map_ok: Box<dyn FnOnce(&Q::Ok) -> E + 'b>,

    ///
    #[builder(with = |map_err: impl FnOnce(&Q::Err) -> E + 'c| Box::new(map_err))]
    map_err: Option<Box<dyn FnOnce(&Q::Err) -> E + 'c>>,
) -> Element
where
    Q::Err: std::error::Error,
{
    match &*query.read().state() {
        QueryStateData::Pending => default().into_element(),
        QueryStateData::Loading { res: None } => default().into_element(),
        QueryStateData::Loading { res: Some(Ok(res)) } => map_ok(res).into_element(),
        QueryStateData::Loading {
            res: Some(Err(err)),
        } => {
            error!("query error: {}", err);
            if let Some(map_err) = map_err {
                map_err(err).into_element()
            } else {
                label().text("an error has occured").into_element()
            }
        }
        QueryStateData::Settled {
            res: Ok(res),
            settlement_instant,
        } => map_ok(res).into_element(),
        QueryStateData::Settled {
            res: Err(err),
            settlement_instant,
        } => {
            error!("query error: {}", err);
            if let Some(map_err) = map_err {
                map_err(err).into_element()
            } else {
                label().text("an error has occured").into_element()
            }
        }
    }
}

impl<'a, 'b, 'c, Q: QueryCapability, E: IntoElement + Default, S: unquery_builder::State>
    UnqueryBuilder<'a, 'b, 'c, Q, E, S>
where
    Q::Err: std::error::Error,
{
    pub fn unwrap_or_default(self) -> Element
    where
        S::Default: unquery_builder::IsUnset,
        S::MapOk: unquery_builder::IsSet,
    {
        self.default_inner(Box::new(|| E::default())).call_inner()
    }
}

impl<'a, 'b, 'c, Q: QueryCapability, E: IntoElement, S: unquery_builder::State>
    UnqueryBuilder<'a, 'b, 'c, Q, E, S>
where
    Q::Err: std::error::Error,
{
    pub fn unwrap_or(self, v: E) -> Element
    where
        S::Default: unquery_builder::IsUnset,
        S::MapOk: unquery_builder::IsSet,
    {
        self.default_inner(Box::new(move || v)).call_inner()
    }

    pub fn unwrap_or_else(self, or_else: impl FnOnce() -> E + 'a) -> Element
    where
        S::Default: unquery_builder::IsUnset,
        S::MapOk: unquery_builder::IsSet,
    {
        self.default_inner(Box::new(or_else)).call_inner()
    }
}

impl<'a, 'b, 'c, Q: QueryCapability, E: IntoElement + Default, S: unquery_builder::State>
    IntoElement for UnqueryBuilder<'a, 'b, 'c, Q, E, S>
where
    S::Default: unquery_builder::IsUnset,
    S::MapOk: unquery_builder::IsSet,
    Q::Err: std::error::Error,
{
    fn into_element(self) -> Element {
        self.unwrap_or_default()
    }
}
