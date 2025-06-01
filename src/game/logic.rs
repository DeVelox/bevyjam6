use bevy::{platform::collections::HashMap, prelude::*};

use super::level::Tile;

pub(super) fn plugin(app: &mut App) {}

#[derive(Resource)]
pub struct PlayerInput {
    rules: HashMap<Tile, Rule>,
}
pub struct Rule {
    tile: [Tile; 2],
    mask: [bool; 8],
}
