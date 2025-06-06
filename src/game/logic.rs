use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*, time::common_conditions::on_timer};

use super::{
    animation::AnimationConfig,
    interface::NextLevel,
    level::{Face, Grid, LevelAssets, PADDING, Puzzle, Tile, Utility},
};
use crate::{menus::Menu, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerRules>();
    app.init_state::<IterationState>();
    app.init_resource::<GridIterations>();
    app.add_systems(
        OnEnter(IterationState::Simulating),
        simulation_step.run_if(not(resource_exists::<Victory>)),
    );
    app.add_systems(OnEnter(IterationState::Displaying), rendering_step);
    app.add_systems(
        OnExit(IterationState::Displaying),
        (rendering_step, check_faces, check_wincon).chain(),
    );
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
pub struct PlayerRules {
    pub rules: HashMap<Tile, Rule>,
    pub color_pool: Vec<Option<Tile>>,
}
#[derive(Clone, Default)]
pub struct Rule {
    pub tiles: [Option<Tile>; 2],
    pub invert: bool,
    pub mask: [bool; 8],
    pub result: Option<Tile>,
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
#[derive(Resource)]
pub struct Victory;
#[derive(Resource)]
pub struct DisableControls;

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
    commands.insert_resource(DisableControls);
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
    input: Res<PlayerRules>,
    mut grid: ResMut<GridIterations>,
    mut state: ResMut<NextState<IterationState>>,
) {
    debug!("{}", grid.grid.len());
    let current_grid = grid.grid.last().expect("Level not loaded.");
    let mut new_grid = current_grid.clone();
    for (i, new_tile) in new_grid.iter_mut().enumerate().take(current_grid.len()) {
        if let Some(tile) = current_grid.check_neighbours(i, &input) {
            *new_tile = tile as u8;
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
    state: Res<State<IterationState>>,
) {
    let reset = grid.grid.len() == 1;
    let image = level_assets.tilesheet.clone();
    let atlas = level_assets.atlas.clone();
    for (i, entity) in board.iter().enumerate() {
        let tile = Tile::from_u8(grid.grid.last().unwrap()[i]);
        if grid.changed(i) || reset {
            if *state.get() == IterationState::Displaying {
                commands.spawn((
                    ChildOf(entity),
                    AnimationConfig::new(12, 16, 15),
                    StateScoped(IterationState::Displaying),
                    Transform::from_xyz(0.0, 0.0, 0.2),
                    Sprite {
                        image: image.clone(),
                        color: tile.color(),
                        custom_size: Some(Vec2::splat(level_assets.tile_size - PADDING)),
                        texture_atlas: Some(TextureAtlas {
                            layout: atlas.clone(),
                            index: 12,
                        }),
                        ..default()
                    },
                ));
            } else {
                commands.entity(entity).insert(tile);
            }
        }
    }
}
fn reset_step(
    mut commands: Commands,
    mut grid: ResMut<GridIterations>,
    mut state: ResMut<NextState<IterationState>>,
) {
    grid.grid.truncate(1);
    commands.remove_resource::<AutomaticSimulation>();
    commands.remove_resource::<DisableControls>();
    commands.remove_resource::<Victory>();
    state.set(IterationState::Displaying);
}
fn check_faces(
    mut commands: Commands,
    grid: Res<GridIterations>,
    faces: Query<Entity, With<Face>>,
) {
    for (i, entity) in faces.iter().enumerate() {
        commands.entity(entity).insert(if grid.is_correct(i) {
            Face::Happy
        } else {
            Face::Sad
        });
    }
}
fn check_wincon(
    mut commands: Commands,
    grid: Res<GridIterations>,
    button: Single<Entity, With<NextLevel>>,
) {
    if grid.grid.last().unwrap_or(&Vec::new()) == &grid.goal {
        commands.insert_resource(Victory);
        commands.remove_resource::<AutomaticSimulation>();
        commands
            .entity(button.into_inner())
            .insert(Name::new("Next level"));
        // .insert(Visibility::Visible);
    }
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

#[derive(Resource, Default)]
pub struct AutomaticSimulation;

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum IterationState {
    #[default]
    Ready,
    Simulating,
    Displaying,
    Reset,
}
