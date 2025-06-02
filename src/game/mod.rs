//! game gameplay. All of these modules are only intended for gamenstration
//! purposes and should be replaced with your own game logic.
//! Feel free to change the logic found here if you feel like tinkering around
//! to get a feeling for the template.

use bevy::prelude::*;

pub mod input;
pub mod level;
pub mod logic;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        level::plugin,
        logic::plugin,
        input::plugin,
        // animation::plugin,
    ));
}
