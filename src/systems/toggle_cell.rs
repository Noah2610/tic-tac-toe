use super::system_prelude::*;

pub struct ToggleCellSystem;

impl<'a> System<'a> for ToggleCellSystem {
    type SystemData = (ReadStorage<'a, Transform>, WriteStorage<'a, Cell>);

    fn run(&mut self, (transforms, cells): Self::SystemData) {
        for (transform, cell) in (&transforms, &cells).join() {}
    }
}
