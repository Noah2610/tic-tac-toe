pub mod prelude {
    pub use super::ActivePlayer;
    pub use super::Player;
    pub use super::PlayerWon;
}

#[derive(Debug, PartialEq, Clone)]
pub enum Player {
    One,
    Two,
}

impl Player {
    pub fn other(&self) -> Self {
        match self {
            Player::One => Player::Two,
            Player::Two => Player::One,
        }
    }
}

pub struct ActivePlayer(pub Player);

impl ActivePlayer {
    pub fn other(&self) -> Self {
        Self(self.0.other())
    }
}

impl Default for ActivePlayer {
    fn default() -> Self {
        Self(Player::One)
    }
}

pub type PlayerWon = Option<Player>;
