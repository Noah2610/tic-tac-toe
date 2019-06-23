pub mod cell;

pub mod prelude {
    pub use deathframe::components::prelude::*;

    pub use super::cell::Cell;
    pub use super::cell::CellType;
}

mod component_prelude {
    pub use deathframe::components::component_prelude::*;

    pub use crate::states::game::ActivePlayer;
}

pub use prelude::*;
