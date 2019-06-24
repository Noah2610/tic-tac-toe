use super::system_prelude::*;

#[derive(Default)]
pub struct ToggleCellSystem {
    is_mouse_down: bool,
}

impl<'a> System<'a> for ToggleCellSystem {
    type SystemData = (
        Read<'a, Settings>,
        Read<'a, InputHandler<String, String>>,
        WriteExpect<'a, ActivePlayer>,
        Entities<'a>,
        ReadStorage<'a, Transform>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Camera>,
        WriteStorage<'a, Cell>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            settings,
            input_handler,
            mut active_player,
            entities,
            transforms,
            sizes,
            cameras,
            mut cells,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        let mouse_offset = if let Some((camera_transform, _)) =
            (&transforms, &cameras).join().next()
        {
            let pos = camera_transform.translation();
            (pos.x, pos.y)
        } else {
            (0.0, 0.0)
        };

        let mouse_button_down =
            input_handler.mouse_button_is_down(MouseButton::Left);
        if mouse_button_down && !self.is_mouse_down {
            self.is_mouse_down = true;
            if let Some(mouse_pos) = input_handler.mouse_position() {
                let mouse_pos = (
                    mouse_pos.0 as f32 + mouse_offset.0,
                    settings.screen_dimensions.1 as f32 - mouse_pos.1 as f32
                        + mouse_offset.1,
                );

                let mouse_rect = CollisionRect::<(), ()> {
                    id:     None,
                    rect:   Rect {
                        top:    mouse_pos.1,
                        bottom: mouse_pos.1,
                        left:   mouse_pos.0,
                        right:  mouse_pos.0,
                    },
                    tag:    None,
                    custom: None,
                };

                for (
                    cell_entity,
                    cell_transform,
                    cell_size,
                    sprite_render,
                    cell,
                ) in (
                    &entities,
                    &transforms,
                    &sizes,
                    &mut sprite_renders,
                    &mut cells,
                )
                    .join()
                {
                    let cell_pos = cell_transform.translation();
                    let cell_rect = CollisionRect::<(), ()> {
                        id:     Some(cell_entity.id()),
                        rect:   Rect {
                            top:    cell_pos.y + cell_size.h * 0.5,
                            bottom: cell_pos.y - cell_size.h * 0.5,
                            left:   cell_pos.x - cell_size.w * 0.5,
                            right:  cell_pos.x + cell_size.w * 0.5,
                        },
                        tag:    None,
                        custom: None,
                    };

                    if CollisionGrid::<(), ()>::do_rects_collide(
                        &mouse_rect,
                        &cell_rect,
                    ) {
                        let prev_cell_type = cell.cell_type.clone();

                        cell.click(&active_player);
                        sprite_render.sprite_number = cell.sprite_number();

                        if cell.cell_type != prev_cell_type {
                            *active_player = active_player.other();
                        }
                    }
                }
            }
        } else if !mouse_button_down && self.is_mouse_down {
            self.is_mouse_down = false;
        }
    }
}
