use super::system_prelude::*;

#[derive(Default)]
pub struct CheckWinSystem {
    played_last: Option<Player>,
}

impl<'a> System<'a> for CheckWinSystem {
    type SystemData = (
        Entities<'a>,
        Read<'a, Settings>,
        Read<'a, ActivePlayer>,
        Write<'a, PlayerWon>,
        WriteStorage<'a, Cell>,
        WriteStorage<'a, SpriteRender>,
    );

    fn run(
        &mut self,
        (
            entities,
            settings,
            active_player,
            mut player_won,
            mut cells,
            mut sprite_renders,
        ): Self::SystemData,
    ) {
        if self
            .played_last
            .as_ref()
            .map(|player| player != &active_player.0)
            .unwrap_or(true)
        {
            self.played_last = Some(active_player.0.clone());
            let check_cell_type = CellType::from(&active_player.0);

            let mut winning_cell_ids = Vec::new();

            for (root_cell_entity, root_cell) in (&entities, &cells).join() {
                if root_cell.cell_type == check_cell_type {
                    // CheckDirections
                    for check_direction in CheckDirection::iter() {
                        winning_cell_ids = vec![root_cell_entity.id()];
                        let mut prev_pos = root_cell.pos;
                        let mut next_pos = (0, 0);
                        let mut found_neighbor = true;
                        while found_neighbor
                            && winning_cell_ids.len() < settings.win_length
                        {
                            found_neighbor = false;
                            next_pos = check_direction.next_pos_for(prev_pos);
                            for (check_cell_entity, check_cell) in
                                (&entities, &cells).join()
                            {
                                if check_cell.cell_type == check_cell_type
                                    && check_cell.pos == next_pos
                                {
                                    found_neighbor = true;
                                    prev_pos = check_cell.pos;
                                    winning_cell_ids
                                        .push(check_cell_entity.id());
                                    break;
                                }
                            }
                        }

                        if winning_cell_ids.len() >= settings.win_length {
                            break;
                        }
                    }
                    if winning_cell_ids.len() >= settings.win_length {
                        break;
                    }
                }

                if winning_cell_ids.len() >= settings.win_length {
                    // WIN
                    *player_won = Some(active_player.0.clone());
                }
            }

            // Only render winning cells.
            if player_won.is_some() {
                for (cell_entity, cell) in (&entities, &mut cells).join() {
                    if winning_cell_ids.contains(&cell_entity.id()) {
                        // TODO: `winning_cell` is not really used, currently.
                        cell.winning_cell = true;
                    } else {
                        cell.winning_cell = false;
                        sprite_renders.remove(cell_entity);
                    }
                }
            }
        }
    }
}

enum CheckDirection {
    Right,
    Down,
    RightDown,
}

impl CheckDirection {
    pub fn iter() -> CheckDirectionIter {
        CheckDirectionIter::default()
    }

    pub fn next_pos_for(&self, pos: (u32, u32)) -> (u32, u32) {
        match self {
            CheckDirection::Right => (pos.0 + 1, pos.1),
            CheckDirection::Down => (pos.0, pos.1 + 1),
            CheckDirection::RightDown => (pos.0 + 1, pos.1 + 1),
        }
    }
}

#[derive(Default)]
struct CheckDirectionIter {
    index: usize,
}

impl Iterator for CheckDirectionIter {
    type Item = CheckDirection;

    fn next(&mut self) -> Option<CheckDirection> {
        let ret = match self.index {
            0 => Some(CheckDirection::Right),
            1 => Some(CheckDirection::Down),
            2 => Some(CheckDirection::RightDown),
            _ => None,
        };

        self.index += 1;

        ret
    }
}
