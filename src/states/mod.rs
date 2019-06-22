mod game;

pub mod prelude {
    pub use super::game::Game;
}

mod state_prelude {
    pub use amethyst::prelude::Builder;
    pub use amethyst::renderer::{
        Camera as AmethystCamera,
        Projection,
        SpriteRender,
    };
    pub use amethyst::{State, StateData, StateEvent, Trans};

    pub use deathframe::custom_game_data::prelude::*;
    pub use deathframe::geo::Vector;
    pub use deathframe::handlers::SpriteSheetHandles;

    pub use super::prelude::*;
    pub use crate::components::prelude::*;
    pub use crate::resource_helpers::*;
    pub use crate::settings::Settings;
    pub use crate::CustomData;
}

pub use prelude::*;
