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
            let check_cell_type = CellType::from(&active_player.0.other());

            let mut winning_cell_ids = Vec::new();

            for (root_cell_entity, root_cell) in (&entities, &cells).join() {
                if root_cell.cell_type == check_cell_type {
                    // CheckDirections
                    for check_direction in CheckDirection::iter() {
                        let mut direction_winning_cell_ids = Vec::new();
                        let mut next_pos_res = Ok(root_cell.pos);
                        let mut found_neighbor = true;

                        while found_neighbor
                            && direction_winning_cell_ids.len()
                                < settings.win_length
                            && next_pos_res.is_ok()
                        {
                            found_neighbor = false;
                            if let Ok(next_pos) = next_pos_res {
                                for (check_cell_entity, check_cell) in
                                    (&entities, &cells).join()
                                {
                                    if check_cell.cell_type == check_cell_type
                                        && check_cell.pos == next_pos
                                    {
                                        found_neighbor = true;
                                        next_pos_res = check_direction
                                            .next_pos_for(next_pos);
                                        direction_winning_cell_ids
                                            .push(check_cell_entity.id());
                                        break;
                                    }
                                }
                            }
                        }

                        if direction_winning_cell_ids.len()
                            >= settings.win_length
                        {
                            winning_cell_ids = direction_winning_cell_ids;
                            break;
                        }
                    }
                    if winning_cell_ids.len() >= settings.win_length {
                        break;
                    }
                }
            }

            if winning_cell_ids.len() >= settings.win_length {
                // WIN
                *player_won = Some(active_player.0.clone());
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
    RightUp,
}

impl CheckDirection {
    pub fn iter() -> CheckDirectionIter {
        CheckDirectionIter::default()
    }

    pub fn next_pos_for(
        &self,
        pos: (u32, u32),
    ) -> Result<(u32, u32), &'static str> {
        Ok(match self {
            CheckDirection::Right => (pos.0 + 1, pos.1),
            CheckDirection::Down => (pos.0, try_minus(pos.1, 1)?),
            CheckDirection::RightDown => (pos.0 + 1, try_minus(pos.1, 1)?),
            CheckDirection::RightUp => (pos.0 + 1, pos.1 + 1),
        })
    }
}

// x - y = z
// x: minuend
// y: subtrahend
fn try_minus(minuend: u32, subtrahend: u32) -> Result<u32, &'static str> {
    if minuend >= subtrahend {
        Ok(minuend - subtrahend)
    } else {
        Err("Minuend is larger than subtrahend")
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
            3 => Some(CheckDirection::RightUp),
            _ => None,
        };
        self.index += 1;
        ret
    }
}
