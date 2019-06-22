use super::component_prelude::*;

#[derive(Debug, Clone, PartialEq)]
pub enum CellType {
    X,
    O,
    Empty,
}

impl Default for CellType {
    fn default() -> Self {
        CellType::Empty
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cell {
    pub x:    u32,
    pub y:    u32,
    pub team: CellType,
}

impl Cell {
    pub fn new(x: u32, y: u32) -> Self {
        Self {
            x,
            y,
            team: Default::default(),
        }
    }
}

impl Component for Cell {
    type Storage = DenseVecStorage<Self>;
}
