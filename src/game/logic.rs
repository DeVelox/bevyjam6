use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*, time::common_conditions::on_timer};

use crate::{menus::Menu, screens::Screen};

use super::level::{Board, Grid, Level, Switch, Tile, Utility};

pub(super) fn plugin(app: &mut App) {
    // app.init_resource::<PlayerInput>();
    app.insert_resource(PlayerInput {
        rules: HashMap::from([(
            Tile::Red,
            Rule {
                tiles: vec![Tile::Orange],
                mask: [true, false, false, false, false, false, false, false],
                result: Tile::Orange,
            },
        )]),
    });
    app.init_resource::<GridIterations>();
    app.init_state::<IterationState>();
    app.add_systems(OnEnter(IterationState::Simulating), simulation_step);
    app.add_systems(OnEnter(IterationState::Displaying), rendering_step);
    app.add_systems(OnEnter(IterationState::Victory), next_level);
    app.add_systems(OnEnter(IterationState::Reset), reset_step);
    app.add_systems(
        Update,
        simulation_system.run_if(
            resource_exists::<AutomaticSimulation>
                .and(on_timer(Duration::from_secs_f32(0.5)))
                .and(in_state(Screen::Gameplay))
                .and(in_state(Menu::None)),
        ),
    );
}

#[derive(Resource, Default)]
pub struct PlayerInput {
    pub rules: HashMap<Tile, Rule>,
}
pub struct Rule {
    pub tiles: Vec<Tile>,
    pub mask: [bool; 8],
    pub result: Tile,
}
#[derive(Resource)]
pub struct GridIterations {
    pub grid: Vec<Grid>,
    pub goal: Grid,
    pub max: usize,
}
impl Default for GridIterations {
    fn default() -> Self {
        Self {
            grid: vec![],
            goal: vec![],
            max: 5,
        }
    }
}
impl GridIterations {
    pub fn changed(&self, index: usize) -> bool {
        let current = self.grid.last().unwrap();
        let previous = self.grid.get(self.grid.len().saturating_sub(2)).unwrap();
        current[index] != previous[index]
    }
}
#[derive(Resource, Default)]
pub struct AutomaticSimulation;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
pub enum IterationState {
    #[default]
    Paused,
    Simulating,
    Displaying,
    Victory,
    Reset,
}

fn simulation_system(
    mut commands: Commands,
    grid: Res<GridIterations>,
    mut state: ResMut<NextState<IterationState>>,
) {
    if grid.grid.len() >= grid.max {
        commands.remove_resource::<AutomaticSimulation>();
        return;
    }
    state.set(IterationState::Simulating);
}
pub fn run_simulation(_: Trigger<Pointer<Click>>, mut commands: Commands) {
    commands.init_resource::<AutomaticSimulation>()
}
pub fn step_simulation(_: Trigger<Pointer<Click>>, mut state: ResMut<NextState<IterationState>>) {
    state.set(IterationState::Simulating);
}
pub fn reset_simulation(_: Trigger<Pointer<Click>>, mut state: ResMut<NextState<IterationState>>) {
    state.set(IterationState::Reset);
}
pub fn next_level(
    current_level: Res<State<Level>>,
    mut level: ResMut<NextState<Level>>,
    mut screen: ResMut<NextState<Screen>>,
) {
    level.set(current_level.get().next());
    screen.set(Screen::Loading);
}

fn simulation_step(
    input: Res<PlayerInput>,
    mut grid: ResMut<GridIterations>,
    mut state: ResMut<NextState<IterationState>>,
) {
    debug!("{}", grid.grid.len());
    let current_grid = grid.grid.last().expect("Level not loaded.");
    let mut new_grid = current_grid.clone();
    for i in 0..current_grid.len() {
        if let Some(tile) = current_grid.check_neighbours(i, &input) {
            new_grid[i] = tile as u8;
        }
    }
    grid.grid.push(new_grid);
    state.set(IterationState::Displaying);
}
fn rendering_step(
    mut commands: Commands,
    grid: Res<GridIterations>,
    board: Query<Entity, With<Board>>,
    mut state: ResMut<NextState<IterationState>>,
) {
    let reset = if grid.grid.len() == 1 { true } else { false };
    for (i, entity) in board.iter().enumerate() {
        if grid.changed(i) || reset {
            commands
                .entity(entity)
                .insert(Tile::from_u8(grid.grid.last().unwrap()[i]));
        }
    }
    if grid.grid.last().unwrap() == &grid.goal {
        state.set(IterationState::Victory);
    }
}
fn reset_step(mut grid: ResMut<GridIterations>, mut state: ResMut<NextState<IterationState>>) {
    grid.grid.truncate(1);
    state.set(IterationState::Displaying);
}
