use specs::{ReadStorage, System, Join};

use crate::cmp_position::Position;

pub struct PrintPositionSystem;

impl<'a> System<'a> for PrintPositionSystem {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        for position in position.join() {
            println!("Position: {:?}", *position);
        }
    }
}

