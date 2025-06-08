use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*, time::common_conditions::on_timer};

use super::{
    animation::AnimationConfig,
    level::{Face, Grid, LevelAssets, LevelEntity, PADDING, Puzzle, Tile, Utility},
};
use crate::{menus::Menu, screens::Screen, theme::shader::CustomMaterial};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<PlayerRules>();
    app.init_state::<IterationState>();
    app.init_resource::<GridIterations>();
    app.add_systems(OnEnter(IterationState::Reset), reset_step);
    app.add_systems(
        OnEnter(IterationState::Displaying),
        (clear_board, rendering_step).chain(),
    );
    app.add_systems(
        OnExit(IterationState::Displaying),
        (rendering_step, check_wincon).chain(),
    );
    app.add_systems(
        OnEnter(IterationState::Simulating),
        simulation_step.run_if(not(resource_exists::<Victory>)),
    );
    app.add_systems(
        Update,
        (
            calculate_color_pool.run_if(
                resource_exists_and_changed::<GridIterations>
                    .and(in_state(Screen::Gameplay))
                    .and(in_state(Menu::None)),
            ),
            simulation_system.run_if(
                resource_exists::<AutomaticSimulation>
                    .and(on_timer(Duration::from_secs_f32(ANIMATION_DURATION)))
                    .and(in_state(Screen::Gameplay))
                    .and(in_state(Menu::None)),
            ),
        ),
    );
}
pub const ANIMATION_DURATION: f32 = 0.6;
#[derive(Resource, Default, Debug)]
pub struct PlayerRules {
    pub rules: HashMap<Tile, Rule>,
    pub color_pool: Vec<Option<Tile>>,
}
#[derive(Clone, Debug)]
pub struct Rule {
    pub tiles: [Option<Tile>; 2],
    pub invert: bool,
    pub mask: [bool; 8],
    pub changed: [bool; 8],
    pub result: Option<Tile>,
}
impl Default for Rule {
    fn default() -> Self {
        Self {
            tiles: [None, None],
            invert: false,
            mask: [true; 8],
            changed: [false; 8],
            result: None,
        }
    }
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
pub fn step_through(
    _: Trigger<Pointer<Click>>,
    mut state: ResMut<NextState<IterationState>>,
    mut commands: Commands,
) {
    commands.remove_resource::<AutomaticSimulation>();
    commands.insert_resource(DisableControls);
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
    let current_grid = grid.grid.last().unwrap();
    let mut new_grid = current_grid.clone();
    for (i, new_tile) in new_grid.iter_mut().enumerate().take(current_grid.len()) {
        if let Some(tile) = current_grid.check_neighbours(i, &input) {
            *new_tile = tile as u8;
        }
    }
    grid.grid.push(new_grid);
    state.set(IterationState::Displaying);
}
fn clear_board(mut commands: Commands, board: Query<Entity, With<Puzzle>>) {
    for entity in &board {
        commands.entity(entity).despawn();
    }
}
fn rendering_step(
    mut commands: Commands,
    mut level_assets: ResMut<LevelAssets>,
    level_entity: Res<LevelEntity>,
    grid: Res<GridIterations>,
    state: Res<State<IterationState>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    time: Res<Time>,
) {
    let current = grid.grid.last().unwrap();
    let previous = grid.grid.get(grid.grid.len().saturating_sub(2)).unwrap();

    let (puzzle, tile_size) = current.render_puzzle(level_entity.0);
    level_assets.tile_size = tile_size;

    let mesh = meshes.add(Rectangle::default());

    for (i, bundle) in puzzle.into_iter().enumerate() {
        let tile = commands.spawn(bundle).id();

        #[cfg(feature = "dev")]
        commands
            .entity(tile)
            .insert(crate::dev_tools::EditorTileColor {
                index: i,
                color: Some(Tile::from_u8(current[i])),
            })
            .insert(Pickable::default())
            .observe(crate::dev_tools::handle_debug_editor)
            .observe(crate::dev_tools::handle_debug_painter);
        commands.spawn((
            ChildOf(tile),
            if grid.is_correct(i) {
                Face::Happy
            } else {
                Face::Sad
            },
            Transform::from_xyz(0.0, 0.0, 0.2),
        ));
        let material = materials.add(CustomMaterial {
            sprite_texture: Some(level_assets.tilesheet.clone()),
            params: Vec4::new(previous[i] as f32, 1.0, 0.04, time.elapsed_secs()),
            burn_color: LinearRgba::from(Tile::from_u8(previous[i]).color()),
        });
        if current[i] != previous[i] && *state.get() == IterationState::Displaying {
            commands.spawn((
                ChildOf(tile),
                StateScoped(IterationState::Displaying),
                Mesh2d(mesh.clone()),
                MeshMaterial2d(material.clone()),
                AnimationConfig::new(material.clone(), 60),
                Transform::default()
                    .with_scale(Vec3::splat(level_assets.tile_size - PADDING))
                    .with_translation(Vec3::new(0.0, 0.0, 0.1)),
            ));
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
fn check_wincon(mut commands: Commands, grid: Res<GridIterations>) {
    if grid.grid.last().unwrap_or(&Vec::new()) == &grid.goal {
        commands.insert_resource(Victory);
        commands.remove_resource::<AutomaticSimulation>();
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

fn calculate_color_pool(grid_iter: Res<GridIterations>, mut rules: ResMut<PlayerRules>) {
    let mut color_pool: Grid = default();
    color_pool.extend(grid_iter.grid.last().unwrap());
    color_pool.extend(grid_iter.goal.clone());
    color_pool.sort();
    color_pool.dedup();
    rules
        .rules
        .retain(|key, _| color_pool.contains(&(*key as u8)));
    rules.color_pool.clear();
    for tile in &color_pool {
        let tile = Tile::from_u8(*tile);
        rules.rules.entry(tile).or_default();
        rules.color_pool.push(Some(tile));
    }
    rules.color_pool.push(None);
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
