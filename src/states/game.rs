use super::state_prelude::*;

const CAMERA_Z: f32 = 10.0;

#[derive(Default)]
pub struct Game {
    cell_size: Option<(f32, f32)>,
}

impl Game {
    fn load_spritesheet(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let mut spritesheet_handles = SpriteSheetHandles::default();

        spritesheet_handles
            .load(resource("spritesheets/spritesheet.png"), &mut data.world);

        data.world.add_resource(spritesheet_handles);
    }

    fn initialize_camera(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let custom_data =
            data.data.custom.as_ref().expect("CustomData must be set");
        let dimensions = custom_data
            .display_config
            .dimensions
            .expect("Dimensions must be set in display_config.ron");

        let cell_size =
            self.cell_size.as_ref().expect("cell_size must be Some");

        let mut transform = Transform::default();
        transform.set_x(-cell_size.0 * 0.5);
        transform.set_y(-cell_size.1 * 0.5);
        transform.set_z(CAMERA_Z);
        let size = Size::new(dimensions.0 as f32, dimensions.1 as f32);

        data.world
            .create_entity()
            .with(transform)
            .with(size)
            .with(
                Camera::new()
                    .base_speed(Vector(250.0, 250.0))
                    .deadzone(Vector(32.0, 32.0))
                    .build(),
            )
            .with(AmethystCamera::from(Projection::orthographic(
                0.0,                 // Left
                dimensions.0 as f32, // Right
                0.0,                 // Bottom (!)
                dimensions.1 as f32, // Top    (!)
            )))
            .build();
    }

    fn initialize_grid(
        &mut self,
        data: &mut StateData<CustomGameData<CustomData>>,
    ) {
        let custom_data =
            data.data.custom.as_ref().expect("CustomData must be set");
        let world = &mut data.world;
        let settings = world.read_resource::<Settings>().clone();

        let dimensions = custom_data
            .display_config
            .dimensions
            .expect("Dimensions must be set in display_config.ron");
        let cell_size =
            self.cell_size.as_ref().expect("cell_size must be Some");
        let spritesheet = world
            .read_resource::<SpriteSheetHandles>()
            .get("spritesheet")
            .expect("Sprite sheet 'spritesheet.png' must be loaded");

        for col in 0 .. settings.grid_size.0 {
            for row in 0 .. settings.grid_size.1 {
                let mut transform = Transform::default();
                transform.set_x(row as f32 * cell_size.0);
                transform.set_y(col as f32 * cell_size.1);

                let size = Size::new(cell_size.0 as f32, cell_size.1 as f32);

                world
                    .create_entity()
                    .with(Cell::new(row, col))
                    .with(transform)
                    .with(size)
                    .with(SpriteRender {
                        sprite_sheet:  spritesheet.clone(),
                        sprite_number: 0,
                    })
                    .with(Transparent)
                    .with(ScaleOnce)
                    .build();
            }
        }
    }
}

impl<'a, 'b> State<CustomGameData<'a, 'b, CustomData>, StateEvent> for Game {
    fn on_start(&mut self, mut data: StateData<CustomGameData<CustomData>>) {
        // Register components (remove later, when used in systems)
        data.world.register::<Cell>();

        // Load resources
        data.world.add_resource(Settings::default());

        self.load_spritesheet(&mut data);

        // Set `cell_size`
        {
            let custom_data =
                data.data.custom.as_ref().expect("CustomData must be set");
            let settings = data.world.read_resource::<Settings>().clone();
            let dimensions = custom_data
                .display_config
                .dimensions
                .expect("Dimensions must be set in display_config.ron");
            self.cell_size = Some((
                (dimensions.0 / settings.grid_size.0) as f32,
                (dimensions.1 / settings.grid_size.1) as f32,
            ));
        }

        self.initialize_camera(&mut data);
        self.initialize_grid(&mut data);
    }

    fn update(
        &mut self,
        mut data: StateData<CustomGameData<CustomData>>,
    ) -> Trans<CustomGameData<'a, 'b, CustomData>, StateEvent> {
        data.data.update(&data.world, "game").unwrap();

        Trans::None
    }
}
