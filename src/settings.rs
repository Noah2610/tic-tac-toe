#[derive(Debug, Clone)]
pub struct Settings {
    pub screen_dimensions: (u32, u32),
    pub grid_size:         (u32, u32),
    pub win_length:        u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            screen_dimensions: (400, 400),
            grid_size:         (3, 3),
            win_length:        3,
        }
    }
}
