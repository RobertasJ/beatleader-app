use crate::prelude::*;

#[derive(Clone, Copy)]
struct DefaultCursor(CursorIcon);

#[component(bon, maybe_ext, container_ext, container_with_content_ext)]
#[derive(PartialEq, Clone)]
#[builder(derive(Clone))]
pub fn Container(
    #[builder(field)]
    #[component(layout)]
    layout: LayoutData,
    #[builder(field)]
    #[component(text_style)]
    text_style_data: TextStyleData,
    #[builder(field)]
    #[component(layer)]
    layer: Layer,
    #[builder(field)]
    #[component(event_handlers)]
    event_handlers: EventHandlers,
    #[builder(field)]
    #[component(style)]
    style: StyleState,
    #[builder(field)]
    #[component(key)]
    key: DiffKey,
    #[builder(field)]
    #[component(children)]
    children: Vec<Element>,
    #[builder(field)]
    #[component(effect)]
    effect: EffectData,
    #[builder(field)]
    #[component(accessibility)]
    accessibility: AccessibilityData,
    cursor_icon: Option<CursorIcon>,
    #[builder(into)] hover_background: Option<Color>,
    #[builder(into)] hover_color: Option<Color>,
    #[builder(into)] hover_border: Option<Border>,
) -> impl IntoElement {
    let default_cursor = use_try_consume::<DefaultCursor>()
        .unwrap_or(DefaultCursor(CursorIcon::Default))
        .0;
    let cursor_icon = cursor_icon.unwrap_or(default_cursor);
    use_provide_context(|| DefaultCursor(cursor_icon));

    let mut is_hovered = use_state(|| false);

    use_drop(move || {
        let platform = Platform::get();
        if is_hovered() {
            platform.send(UserEvent::SetCursorIcon(default_cursor));
        }
    });

    use_side_effect(move || {
        let platform = Platform::get();
        if is_hovered() {
            platform.send(UserEvent::SetCursorIcon(cursor_icon));
        } else {
            platform.send(UserEvent::SetCursorIcon(default_cursor));
        }
    });

    let mut comp = freya::prelude::rect()
        .a11y_role(
            if event_handlers.contains_key(&name::EventName::PointerDown) {
                AccessibilityRole::Button
            } else {
                AccessibilityRole::Group
            },
        )
        .event_handlers(event_handlers)
        .layout(layout)
        .accessibility(accessibility)
        .text_style(text_style_data)
        .style(style)
        .layer(layer);

    if is_hovered() {
        comp = comp
            .map(hover_background, |comp, c| comp.background(c))
            .map(hover_border, |comp, b| comp.border(b))
            .map(hover_color, |comp, c| comp.color(c));
    }

    comp.key(key)
        .children(children)
        .effect(effect)
        .on_pointer_over(move |e: Event<PointerEventData>| {
            *is_hovered.write() = true;
        })
        .on_pointer_out(move |_e: Event<PointerEventData>| {
            *is_hovered.write() = false;
        })
}

impl<S: container_builder::State> RectThemeExt for ContainerBuilder<S> {
    fn theme_background(self) -> Self {
        let theme = get_theme_or_default();
        self.background(theme.read().colors.background)
    }

    fn theme_color(self) -> Self {
        let theme = get_theme_or_default();
        self.color(theme.read().colors.text_primary)
    }
}
