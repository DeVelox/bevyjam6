//! Spawn the main level.

use bevy::prelude::*;
use bevy::reflect::TypePath;

use crate::{asset_tracking::LoadResource, audio::music, screens::Screen, theme::palette::*};

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

pub type Grid = Vec<u8>;
#[derive(serde::Deserialize, Asset, TypePath)]
pub struct Levels {
    levels: Vec<Grid>,
}
trait Render {
    fn render(&self, parent: Entity) -> Vec<(Tile, ChildOf, Sprite, Transform)>;
}
impl Render for Grid {
    fn render(&self, parent: Entity) -> Vec<(Tile, ChildOf, Sprite, Transform)> {
        const TILE_SIZE: f32 = 128.;
        const PADDING: f32 = 8.;
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
                ChildOf(parent),
                Sprite::from_color(tile.color(), Vec2::splat(tile_size - PADDING)),
                Transform::from_translation(coords.extend(0.0)),
            ));
        }
        tiles
    }
}

/// A system that spawns the main level.
pub fn spawn_level(
    mut commands: Commands,
    level_assets: Res<LevelAssets>,
    levels: Res<Assets<Levels>>,
    current_level: Res<State<Level>>,
    mut state: ResMut<NextState<Level>>,
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
        commands.spawn_batch(grid.render(parent));
        // Only on win
        state.set(current_level.get().next());
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

trait Switch {
    fn next(&self) -> Self;
    fn prev(&self) -> Self;
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
    fn prev(&self) -> Self {
        match self {
            Level::Expert => Level::Intermediate,
            Level::Intermediate => Level::Beginner,
            Level::Beginner => Level::Intro,
            Level::Intro => Level::Expert,
        }
    }
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
    fn color(&self) -> Color {
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
