use specs::{
    join::Join, Builder, Component, DispatcherBuilder, NullStorage, ReadStorage, RunNow, System,
    VecStorage, World, WorldExt, WriteStorage,
};

type Coordinate = fixed::types::I32F32;

#[derive(Debug)]
struct Position {
    x: Coordinate,
    y: Coordinate,
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

#[derive(Debug)]
struct Velocity {
    x: Coordinate,
    y: Coordinate,
}

impl Component for Velocity {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
struct Station;

impl Component for Station {
    type Storage = NullStorage<Self>;
}

#[derive(Default)]
struct Trader;

impl Component for Trader {
    type Storage = NullStorage<Self>;
}

struct Movement;

impl<'a> System<'a> for Movement {
    type SystemData = (ReadStorage<'a, Velocity>, WriteStorage<'a, Position>);

    fn run(&mut self, (vel, mut pos): Self::SystemData) {
        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x;
            pos.y += vel.y;
        }
    }
}

fn main() {
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.register::<Trader>();
    world.register::<Station>();

    world
        .create_entity()
        .with(Station)
        .with(Position {
            x: Coordinate::from_num(4),
            y: Coordinate::from_num(7),
        })
        .build();

    world
        .create_entity()
        .with(Trader)
        .with(Position {
            x: Coordinate::from_num(2),
            y: Coordinate::from_num(4),
        })
        .build();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Movement, "movement", &[])
        .build();

    dispatcher.dispatch(&mut world);
}
