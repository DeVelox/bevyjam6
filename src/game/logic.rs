use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*, time::common_conditions::on_timer};

use crate::{menus::Menu, screens::Screen};

use super::level::{Grid, Tile};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (simulation_system.run_if(
            on_timer(Duration::from_secs(1))
                .and(resource_exists::<AutomaticSimulation>)
                .and(in_state(Screen::Gameplay))
                .and(in_state(Menu::None)),
        ),),
    );
}

#[derive(Resource)]
pub struct PlayerInput {
    rules: HashMap<Tile, Rule>,
}
pub struct Rule {
    tile: [Tile; 2],
    mask: [bool; 8],
}
#[derive(Resource, Default)]
pub struct GridIterations {
    pub grid: Vec<Grid>,
    pub max: usize,
}
#[derive(Resource, Default)]
pub struct AutomaticSimulation;
#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum IterationState {
    #[default]
    Simulating,
    Rendering,
}

fn simulation_system() {}
pub fn simulation_callback(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.init_resource::<AutomaticSimulation>()
}

fn simulation_step() {}
fn rendering_step() {}
