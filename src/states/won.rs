use super::state_prelude::*;

#[derive(Default)]
pub struct Won;

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Won {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
    }

    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "won");

        Trans::None
    }
}
