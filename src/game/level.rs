//! Spawn the main level.

use std::any::TypeId;

use bevy::ecs::component::HookContext;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use bevy::reflect::TypePath;

use crate::{asset_tracking::LoadResource, audio::music, screens::Screen, theme::palette::*};

use super::logic::{GridIterations, PlayerInput};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
    app.init_state::<Level>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    pub music: Handle<AudioSource>,
    #[dependency]
    pub puzzles: Handle<Levels>,
    #[dependency]
    pub solutions: Handle<Levels>,
    #[dependency]
    pub tilesheet: Handle<Image>,
    // #[dependency]
    pub atlas: Handle<TextureAtlasLayout>,
    pub tile_size: f32,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        let atlas = layouts.add(TextureAtlasLayout::from_grid(
            UVec2::splat(512),
            9,
            2,
            None,
            None,
        ));
        let assets = world.resource::<AssetServer>();
        Self {
            atlas,
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            puzzles: assets.load("levels/puzzles.ron"),
            solutions: assets.load("levels/solutions.ron"),
            tilesheet: assets.load("images/tilesheet.png"),
            tile_size: 128.,
        }
    }
}
#[derive(Component)]
pub struct Puzzle;
#[derive(Component)]
pub struct Solution;
pub type Grid = Vec<u8>;
#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Levels {
    levels: Vec<Grid>,
}
pub trait Utility {
    fn render_puzzle(&self, parent: Entity) -> (Vec<impl Bundle>, f32);
    fn render_solution(&self, parent: Entity) -> Vec<(Solution, ChildOf, Transform, Sprite)>;
    fn check_neighbours(&self, index: usize, input: &PlayerInput) -> Option<Tile>;
}
const TILE_SIZE: f32 = 128.;
const MINI_SCALE: f32 = 3.;
const PADDING: f32 = 8.;
impl Utility for Grid {
    fn render_puzzle(&self, parent: Entity) -> (Vec<impl Bundle>, f32) {
        let grid_size = self.len().isqrt();
        let tile_size = TILE_SIZE * (16 / grid_size) as f32;
        let offset = tile_size * grid_size as f32 / 2. - tile_size / 2.;
        let mut coords = Vec2::splat(-offset);
        let mut tiles = vec![];
        for (i, tile) in self.iter().enumerate() {
            if i > 0 && i % grid_size == 0 {
                coords.y += tile_size;
                coords.x = -offset;
            } else if i > 0 {
                coords.x += tile_size;
            }
            let tile = Tile::from_u8(*tile);
            tiles.push((
                tile,
                Puzzle,
                ChildOf(parent),
                Transform::from_translation(coords.extend(0.0)),
                children![Face::Thinking],
            ));
        }
        (tiles, tile_size)
    }

    fn render_solution(&self, parent: Entity) -> Vec<(Solution, ChildOf, Transform, Sprite)> {
        let grid_size = self.len().isqrt();
        let tile_size = TILE_SIZE / MINI_SCALE * (16 / grid_size) as f32;
        let offset = tile_size * grid_size as f32 / 2. - tile_size / 2.;
        let mut coords = Vec2::splat(-offset);
        let mut tiles = vec![];
        for (i, tile) in self.iter().enumerate() {
            if i > 0 && i % grid_size == 0 {
                coords.y += tile_size;
                coords.x = -offset;
            } else if i > 0 {
                coords.x += tile_size;
            }
            let tile = Tile::from_u8(*tile);
            tiles.push((
                Solution,
                ChildOf(parent),
                Transform::from_translation(
                    (TILE_SIZE * Vec2::new(35., 16.) / MINI_SCALE + coords).extend(0.0),
                ),
                Sprite::from_color(tile.color(), Vec2::splat(tile_size - PADDING / MINI_SCALE)),
            ));
        }
        tiles
    }

    fn check_neighbours(&self, index: usize, input: &PlayerInput) -> Option<Tile> {
        // probably inefficient
        let gs = self.len().isqrt() as i32;
        let offsets = &[-(gs + 1), -gs, -(gs - 1), -1, 1, gs - 1, gs, gs + 1];
        let left_offset = &[-(gs + 1), -1, gs - 1];
        let right_offset = &[-(gs - 1), 1, gs + 1];
        let left_edge = index % gs as usize == 0;
        let right_edge = (index + 1) % gs as usize == 0;
        if let Some(rule) = &input.rules.get(&Tile::from_u8(self[index])) {
            let neighbours: Vec<Tile> = offsets
                .iter()
                .enumerate()
                .filter(|&(i, _)| rule.mask[i])
                .map(|(_, &offset)| {
                    let neighbor = index as i32 + offset;
                    if neighbor >= 0 && neighbor < self.len() as i32 {
                        if (left_edge && left_offset.contains(&offset))
                            || (right_edge && right_offset.contains(&offset))
                        {
                            Tile::Empty
                        } else {
                            Tile::from_u8(self[neighbor as usize])
                        }
                    } else {
                        Tile::Empty
                    }
                })
                .collect();
            if rule.tiles.iter().all(|tile| neighbours.contains(tile)) {
                Some(rule.result)
            } else {
                None
            }
        } else {
            None
        }
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    mut level_assets: ResMut<LevelAssets>,
    levels: Res<Assets<Levels>>,
    current_level: Res<State<Level>>,
    mut grid_iter: ResMut<GridIterations>,
) {
    let parent = commands
        .spawn((
            Name::new("Level"),
            Transform::default(),
            Visibility::default(),
            StateScoped(Screen::Gameplay),
            children![(
                Name::new("Gameplay Music"),
                music(level_assets.music.clone())
            )],
        ))
        .id();
    if let Some(level) = levels.get(level_assets.puzzles.id()) {
        let grid = &level.levels[*current_level.get() as usize];
        grid_iter.grid.clear();
        grid_iter.grid.push((*grid).to_vec());
        let (puzzle, tile_size) = grid.render_puzzle(parent);
        level_assets.tile_size = tile_size;
        for bundle in puzzle {
            commands.spawn(bundle);
        }
    }
    if let Some(solution) = levels.get(level_assets.solutions.id()) {
        let grid = &solution.levels[*current_level.get() as usize];
        grid_iter.goal = (*grid).to_vec();
        commands.spawn_batch(grid.render_solution(parent));
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Level {
    #[default]
    Intro,
    Beginner,
    Intermediate,
    Expert,
}

pub trait Switch {
    fn next(&self) -> Self;
    // fn prev(&self) -> Self;
}

impl Switch for Level {
    fn next(&self) -> Self {
        match self {
            Level::Intro => Level::Beginner,
            Level::Beginner => Level::Intermediate,
            Level::Intermediate => Level::Expert,
            Level::Expert => Level::Intro,
        }
    }
    // fn prev(&self) -> Self {
    //     match self {
    //         Level::Expert => Level::Intermediate,
    //         Level::Intermediate => Level::Beginner,
    //         Level::Beginner => Level::Intro,
    //         Level::Intro => Level::Expert,
    //     }
    // }
}

#[derive(Component, Default, Copy, Clone, Eq, Hash, PartialEq)]
#[component(on_insert = insert_sprite::<Tile>)]
pub enum Tile {
    Red,
    Green,
    Blue,
    Yellow,
    Orange,
    Purple,
    Brown,
    Pink,
    #[default]
    Empty,
}
#[derive(Component, Default, Copy, Clone, Eq, Hash, PartialEq)]
#[component(on_insert = insert_sprite::<Face>)]
pub enum Face {
    Happy,
    Sad,
    #[default]
    Thinking,
}
fn insert_sprite<T: Component>(mut world: DeferredWorld, context: HookContext) {
    let level_assets = world.resource::<LevelAssets>();
    let image = level_assets.tilesheet.clone();
    let atlas = level_assets.atlas.clone();
    let tile_size = level_assets.tile_size;
    let tile = if TypeId::of::<T>() == TypeId::of::<Tile>() {
        *world.get::<Tile>(context.entity).unwrap_or(&Tile::Empty) as usize
    } else if TypeId::of::<T>() == TypeId::of::<Face>() {
        9 + *world.get::<Face>(context.entity).unwrap_or(&Face::Thinking) as usize
    } else {
        17 // last tile
    };
    world.commands().entity(context.entity).insert(Sprite {
        image,
        custom_size: Some(Vec2::splat(tile_size - PADDING)),
        texture_atlas: Some(TextureAtlas {
            layout: atlas,
            index: tile,
        }),
        ..default()
    });
}
impl Tile {
    pub fn from_u8(value: u8) -> Tile {
        match value {
            0 => Tile::Red,
            1 => Tile::Green,
            2 => Tile::Blue,
            3 => Tile::Yellow,
            4 => Tile::Orange,
            5 => Tile::Purple,
            6 => Tile::Brown,
            7 => Tile::Pink,
            _ => Tile::Empty,
        }
    }
    pub fn color(&self) -> Color {
        match self {
            Tile::Red => RED,
            Tile::Green => GREEN,
            Tile::Blue => BLUE,
            Tile::Yellow => YELLOW,
            Tile::Orange => ORANGE,
            Tile::Purple => PURPLE,
            Tile::Brown => BROWN,
            Tile::Pink => PINK,
            Tile::Empty => EMPTY,
        }
    }
}
