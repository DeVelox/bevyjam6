//! Spawn the main level.

use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::reflect::TypePath;
use bevy_common_assets::ron::RonAssetPlugin;

use crate::demo::player;
use crate::{
    asset_tracking::LoadResource,
    audio::music,
    demo::player::{PlayerAssets, player},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<LevelAssets>();
    app.load_resource::<LevelAssets>();
    app.init_state::<Level>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct LevelAssets {
    #[dependency]
    music: Handle<AudioSource>,
    #[dependency]
    puzzles: Handle<Levels>,
    #[dependency]
    solutions: Handle<Levels>,
}

impl FromWorld for LevelAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Fluffing A Duck.ogg"),
            puzzles: assets.load("levels/puzzles.ron"),
            solutions: assets.load("levels/solutions.ron"),
        }
    }
}
type Grid = Vec<u8>;
#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Levels {
    levels: Vec<Grid>,
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    mut levels: ResMut<Assets<Levels>>,
    current_level: Res<State<Level>>,
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
    let mut tiles = vec![];
    if let Some(level) = levels.remove(level_assets.puzzles.id()) {
        let level = &level.levels[*current_level.get() as usize];
        const TILE_SIZE: f32 = 30.;
        const PADDING: f32 = 5.;
        let grid_size = level.len().isqrt();
        let half_size = TILE_SIZE * grid_size as f32 / 2. - TILE_SIZE / 2.;
        let mut coords = Vec2::splat(-half_size);
        for (i, tile) in level.iter().enumerate() {
            if i > 0 && i % grid_size == 0 {
                coords.y += TILE_SIZE;
                coords.x = -half_size;
            } else if i > 0 {
                coords.x += TILE_SIZE;
            }
            let tile = Tile::from_u8(*tile);
            tiles.push((
                ChildOf(parent),
                tile.clone(),
                Sprite::from_color(tile.color(), Vec2::splat(TILE_SIZE - PADDING)),
                Transform::from_translation(coords.extend(0.0)),
            ));
        }
        info!("{}", tiles.len());
        commands.spawn_batch(tiles);
    }
}

#[derive(States, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[states(scoped_entities)]
pub enum Level {
    #[default]
    Intro,
}

#[derive(Component, Default, Copy, Clone)]
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

impl Tile {
    fn from_u8(value: u8) -> Tile {
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
    fn color(self: &Self) -> Color {
        match self {
            Tile::Red => Color::from(RED),
            Tile::Green => Color::from(GREEN),
            Tile::Blue => Color::from(BLUE),
            Tile::Yellow => Color::from(YELLOW),
            Tile::Orange => Color::from(ORANGE),
            Tile::Purple => Color::from(PURPLE),
            Tile::Brown => Color::from(BROWN),
            Tile::Pink => Color::from(PINK),
            Tile::Empty => Color::from(GRAY),
        }
    }
}
