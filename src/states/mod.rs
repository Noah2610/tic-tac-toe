pub mod game;
pub mod won;

pub mod prelude {
    pub use super::game::Game;
    pub use super::won::Won;
}

mod state_prelude {
    pub use amethyst::prelude::*;
    pub use amethyst::renderer::{
        Camera as AmethystCamera,
        ElementState,
        MouseButton,
        Projection,
        SpriteRender,
    };
    pub use amethyst::winit::{Event, WindowEvent};
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::prelude::*;
    pub use deathframe::geo::Vector;
    pub use deathframe::handlers::SpriteSheetHandles;

    pub use super::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::player::prelude::*;
    pub use crate::resource_helpers::*;
    pub use crate::settings::Settings;
    pub use crate::CustomData;
}

pub use prelude::*;
