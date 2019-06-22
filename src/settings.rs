#[derive(Debug, Clone)]
pub struct Settings {
    pub grid_size:  (u32, u32),
    pub win_length: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            grid_size:  (3, 3),
            win_length: 3,
        }
    }
}
