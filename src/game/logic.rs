use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*, time::common_conditions::on_timer};

use crate::{menus::Menu, screens::Screen};

use super::{
    animation::AnimationConfig,
    input::show_next_level,
    level::{Face, Grid, LevelAssets, Puzzle, Tile, Utility},
};

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
    app.add_systems(OnEnter(IterationState::Victory), show_next_level);
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
            max: 50,
        }
    }
}
impl GridIterations {
    pub fn is_correct(&self, index: usize) -> bool {
        self.grid.last().unwrap()[index] == self.goal[index]
    }
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
pub fn toggle_simulation(
    _: Trigger<Pointer<Click>>,
    auto: Option<Res<AutomaticSimulation>>,
    mut commands: Commands,
) {
    if auto.is_some() {
        commands.remove_resource::<AutomaticSimulation>();
    } else {
        commands.init_resource::<AutomaticSimulation>();
    }
}
pub fn step_simulation(
    _: Trigger<Pointer<Click>>,
    mut state: ResMut<NextState<IterationState>>,
    mut commands: Commands,
) {
    commands.remove_resource::<AutomaticSimulation>();
    state.set(IterationState::Simulating);
}
pub fn reset_simulation(_: Trigger<Pointer<Click>>, mut state: ResMut<NextState<IterationState>>) {
    state.set(IterationState::Reset);
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
    level_assets: Res<LevelAssets>,
    grid: Res<GridIterations>,
    board: Query<Entity, With<Puzzle>>,
    faces: Query<Entity, With<Face>>,
    mut state: ResMut<NextState<IterationState>>,
) {
    let reset = if grid.grid.len() == 1 { true } else { false };
    let image = level_assets.tilesheet.clone();
    let atlas = level_assets.atlas.clone();
    for (i, entity) in board.iter().enumerate() {
        if grid.changed(i) || reset {
            commands
                .entity(entity)
                .insert(Tile::from_u8(grid.grid.last().unwrap()[i]));
            commands.spawn((
                ChildOf(entity),
                AnimationConfig::new(12, 16, 10),
                Sprite {
                    image: image.clone(),
                    custom_size: Some(Vec2::splat(level_assets.tile_size * 1.5)),
                    texture_atlas: Some(TextureAtlas {
                        layout: atlas.clone(),
                        index: 12,
                    }),
                    ..default()
                },
            ));
        }
    }
    for (i, entity) in faces.iter().enumerate() {
        commands.entity(entity).insert(if reset {
            Face::Thinking
        } else if grid.is_correct(i) {
            Face::Happy
        } else {
            Face::Sad
        });
    }
    if grid.grid.last().unwrap() == &grid.goal {
        commands.remove_resource::<AutomaticSimulation>();
        state.set(IterationState::Victory);
    }
}
fn reset_step(mut grid: ResMut<GridIterations>, mut state: ResMut<NextState<IterationState>>) {
    grid.grid.truncate(1);
    state.set(IterationState::Displaying);
}
