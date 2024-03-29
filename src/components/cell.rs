use super::component_prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    X,
    O,
    Empty,
}

impl CellType {
    pub fn sprite_number(&self) -> usize {
        match self {
            CellType::X => 1,
            CellType::O => 2,
            CellType::Empty => 0,
        }
    }
}

impl From<&Player> for CellType {
    fn from(player: &Player) -> Self {
        match player {
            Player::One => CellType::X,
            Player::Two => CellType::O,
        }
    }
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Empty
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub pos:       (u32, u32),
    pub cell_type: CellType,
    // TODO: `winning_cell` is not really used, currently.
    pub winning_cell: bool,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            pos:          (x, y),
            cell_type:    Default::default(),
            winning_cell: false,
        }
    }

    pub fn click(&mut self, active_player: &ActivePlayer) {
        if let CellType::Empty = self.cell_type {
            self.cell_type = match active_player.0 {
                Player::One => CellType::X,
                Player::Two => CellType::O,
            };
        }
    }

    pub fn sprite_number(&self) -> usize {
        self.cell_type.sprite_number()
    }
}

impl Component for Cell {
    type Storage = DenseVecStorage<Self>;
}
