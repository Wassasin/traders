use amethyst::{
    assets::Handle,
    ecs::{Builder, Component, Entity, NullStorage, VecStorage, World, WorldExt},
    ui::{Anchor, FontAsset, UiText, UiTransform},
};

use super::*;

pub struct UiRelative(pub Entity);

impl Component for UiRelative {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct UiSelectable;

impl Component for UiSelectable {
    type Storage = NullStorage<Self>;
}

pub fn create_ui_anchor(
    world: &mut World,
    target: &Entity,
    name: &str,
    selectable: bool,
) -> Entity {
    let mut res = world
        .create_entity()
        .with(UiRelative(*target))
        .with(UiTransform::new(
            compute_name(target, name),
            Anchor::BottomLeft,
            Anchor::Middle,
            0.0,
            0.0,
            0.0,
            0.,
            0.,
        ));

    if selectable {
        res = res.with(UiSelectable);
    }

    res.build()
}

pub fn create_ui_label(world: &mut World, anchor: &Entity, name: String) {
    let font_handle = (*world.fetch::<Handle<FontAsset>>()).clone();

    let mut ui_text = UiText::new(font_handle, name, [1., 1., 1., 1.], 10.);
    ui_text.align = Anchor::BottomLeft;

    world
        .create_entity()
        .with(Parent::new(*anchor))
        .with(UiTransform::new(
            "label".to_owned(),
            Anchor::TopRight,
            Anchor::TopLeft,
            0.,
            0.,
            0.,
            100.,
            10.,
        ))
        .with(ui_text)
        .build();
}
