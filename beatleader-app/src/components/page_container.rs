use crate::prelude::*;

#[component(bon)]
#[derive(PartialEq, Default)]
pub fn PageContainer(
    #[builder(field)]
    #[component(children)]
    children: Vec<Element>,
) -> impl IntoElement {
    rect()
        .padding(50.0)
        .width(Fill)
        .spacing(50.0)
        .cross_align(Center)
        .children(children)
}
