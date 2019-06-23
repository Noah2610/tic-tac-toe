use super::system_prelude::*;

#[derive(Default)]
pub struct CheckWinSystem {
    played_last: Option<Player>,
}

impl<'a> System<'a> for CheckWinSystem {
    type SystemData = (
        Read<'a, Settings>,
        Read<'a, ActivePlayer>,
        ReadStorage<'a, Cell>,
    );

    fn run(&mut self, (settings, active_player, cells): Self::SystemData) {
        if self
            .played_last
            .as_ref()
            .map(|player| player == &active_player.0)
            .unwrap_or(true)
        {
            self.played_last = Some(active_player.0.clone());

            for root_cell in cells.join() {
                if root_cell.cell_type != CellType::Empty {
                    // CheckDirections
                    for check_direction in CheckDirection::iter() {
                        for check_cell in cells.join() {}
                    }
                }
            }
        }
    }
}

enum CheckDirection {
    Right,
    RightDown,
    Down,
}

impl CheckDirection {
    fn iter() -> CheckDirectionIter {
        CheckDirectionIter::default()
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
