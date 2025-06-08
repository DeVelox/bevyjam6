//! The credits menu.

use bevy::{
    ecs::spawn::SpawnIter, input::common_conditions::input_just_pressed, prelude::*, ui::Val::*,
};

use crate::{asset_tracking::LoadResource, audio::music, menus::Menu, theme::prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Credits), spawn_credits_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Credits).and(input_just_pressed(KeyCode::Escape))),
    );

    app.register_type::<CreditsAssets>();
    app.load_resource::<CreditsAssets>();
    app.add_systems(OnEnter(Menu::Credits), start_credits_music);
}

fn spawn_credits_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        widget::ui_root("Credits Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Credits),
        children![
            (
                Name::new("Title image"),
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Px(512.0),
                    height: Val::Px(512.0),
                    bottom: Val::Px(-160.0),
                    right: Val::Px(-80.0),
                    ..default()
                },
                ImageNode::new(asset_server.load("images/orange.png")),
                Transform::from_rotation(Quat::from_rotation_z(25.0_f32.to_radians())),
                BorderRadius::all(Val::Px(10.0))
            ),
            widget::header("Creators"),
            created_by(),
            widget::header(" "), // just a gap
            widget::header("Assets"),
            assets(),
            widget::header(" "), // just a gap
            widget::button("Back", go_back_on_click),
        ],
    ));
}

fn created_by() -> impl Bundle {
    grid(vec![
        ["DeVelox", "Coded all the things"],
        ["MrRgon", "Designed all the things"],
        ["EjayFreeKay", "Painted all the things"],
    ])
}

fn assets() -> impl Bundle {
    grid(vec![
        ["Button SFX", "CC0 by D4XX - www.freesound.org"],
        ["Music", "Eric Matyas - www.soundimage.org"],
        ["JetBrains Mono font", "OFL v1.1 by Jetbrains"],
        [
            "Bevy logo",
            "All rights reserved by the Bevy Foundation, permission granted for splash screen use when unmodified",
        ],
        [
            "Special thanks to",
            "the Bevy and Rust community for the plethora of examples without which this game would not exist",
        ],
    ])
}

fn grid(content: Vec<[&'static str; 2]>) -> impl Bundle {
    (
        Name::new("Grid"),
        Node {
            display: Display::Grid,
            row_gap: Px(10.0),
            column_gap: Px(30.0),
            grid_template_columns: RepeatedGridTrack::px(2, 400.0),
            ..default()
        },
        Children::spawn(SpawnIter(content.into_iter().flatten().enumerate().map(
            |(i, text)| {
                (
                    widget::label(text),
                    Node {
                        justify_self: if i % 2 == 0 {
                            JustifySelf::End
                        } else {
                            JustifySelf::Start
                        },
                        ..default()
                    },
                )
            },
        ))),
    )
}

fn go_back_on_click(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Main);
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
struct CreditsAssets {
    #[dependency]
    music: Handle<AudioSource>,
}

impl FromWorld for CreditsAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            music: assets.load("audio/music/Winding-Down.ogg"),
        }
    }
}

fn start_credits_music(mut commands: Commands, credits_music: Res<CreditsAssets>) {
    commands.spawn((
        Name::new("Credits Music"),
        StateScoped(Menu::Credits),
        music(credits_music.music.clone()),
    ));
}
