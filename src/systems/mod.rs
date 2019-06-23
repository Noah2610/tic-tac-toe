mod toggle_cell;

pub mod prelude {
    pub use deathframe::systems::prelude::*;

    pub use super::toggle_cell::ToggleCellSystem;
}

mod system_prelude {
    pub use amethyst::renderer::MouseButton;

    pub use deathframe::geo::prelude::*;
    pub use deathframe::systems::system_prelude::*;

    pub use crate::components::prelude::*;
    pub use crate::settings::Settings;
    pub use crate::states::game::ActivePlayer;
}

pub use prelude::*;
