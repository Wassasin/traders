use amethyst::{
    assets::Handle,
    core::{math, transform::Transform},
    ecs::{Builder, Component, DenseVecStorage, Entity, NullStorage, VecStorage, World, WorldExt},
    renderer::{SpriteRender, SpriteSheet},
    ui::{Anchor, FontAsset, UiText, UiTransform},
};
use derive_more::{Add, Deref, DerefMut, Mul, Sub};

pub type Point2 = math::geometry::Point2<f32>;
pub type Translation2 = math::geometry::Translation2<f32>;

// Note: arithmatics directly on positions do not make sense. Hence first deref.
#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct Position(Point2);

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

impl Position {
    pub fn new(p: Point2) -> Self {
        Self(p)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct Velocity(Translation2);

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

impl Default for Velocity {
    fn default() -> Self {
        Self(Translation2::new(0., 0.))
    }
}

impl Velocity {
    pub fn new(vec: Translation2) -> Self {
        Self(vec)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug)]
pub struct Hitbox(Translation2);

impl Component for Hitbox {
    type Storage = VecStorage<Self>;
}

impl Default for Hitbox {
    fn default() -> Self {
        Self(Translation2::new(0., 0.))
    }
}

impl Hitbox {
    pub fn new(vec: Translation2) -> Self {
        Self(vec)
    }
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct Angle(f32);

impl Component for Angle {
    type Storage = VecStorage<Self>;
}

#[derive(Deref, DerefMut, Clone, Copy, Debug, Default, Sub, Mul, Add)]
pub struct AngularMomentum(f32);

impl Component for AngularMomentum {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
pub enum ShipBehaviour {
    Idle,
    FlyTo(Entity),
}

impl Component for ShipBehaviour {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct Station;

impl Component for Station {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
pub struct Trader;

impl Component for Trader {
    type Storage = NullStorage<Self>;
}

pub type Parent = amethyst::core::transform::Parent;
pub type ParentHierarchy = amethyst::core::transform::ParentHierarchy;

pub struct UiRelative(pub Entity);

impl Component for UiRelative {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct UiSelectable;

impl Component for UiSelectable {
    type Storage = NullStorage<Self>;
}

fn compute_name(target: &Entity, name: &str) -> String {
    format!("{}-{}-{}", name, target.gen().id(), target.id())
}

fn create_ui_anchor(world: &mut World, target: &Entity, name: &str, selectable: bool) -> Entity {
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

pub fn create_label(world: &mut World, anchor: &Entity, name: String) {
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

pub fn create_station(world: &mut World, pos: Position) -> Entity {
    let sprite_number = 1;
    let sprite_sheet = (*world.fetch::<Handle<SpriteSheet>>()).clone();

    let (width, height) = (51., 58.);
    let hitbox = Hitbox(Translation2::new(width, height));

    let res = world
        .create_entity()
        .with(Station)
        .with(pos)
        .with(SpriteRender {
            sprite_sheet,
            sprite_number,
        })
        .with(hitbox)
        .with(Transform::default())
        .with(Angle(f32::default()))
        .with(AngularMomentum(0.001))
        .build();

    let anchor = create_ui_anchor(world, &res, "station", true);
    create_label(world, &anchor, compute_name(&res, "station"));

    res
}

pub fn create_trader(world: &mut World, pos: Position, behaviour: ShipBehaviour) -> Entity {
    let sprite_number = 0;
    let sprite_sheet = (*world.fetch::<Handle<SpriteSheet>>()).clone();

    let (width, height) = (39., 57.);

    let hitbox = Hitbox(Translation2::new(width, height));

    let res = world
        .create_entity()
        .with(Trader)
        .with(pos)
        .with(Velocity::default())
        .with(SpriteRender {
            sprite_sheet,
            sprite_number,
        })
        .with(hitbox)
        .with(Transform::default())
        .with(behaviour)
        .build();

    let anchor = create_ui_anchor(world, &res, "trader", true);
    create_label(world, &anchor, compute_name(&res, "trader"));

    res
}
