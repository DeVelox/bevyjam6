//! The tutorial menu.

use crate::game::interface::Help;
use crate::theme::widget::ButtonSize;
use crate::{menus::Menu, theme::widget, Pause};
use bevy::{input::common_conditions::input_just_pressed, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Tutorial).and(input_just_pressed(KeyCode::Escape))),
    );
}

pub fn spawn_tutorial_menu(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut state: ResMut<NextState<Menu>>,
    mut next_pause: ResMut<NextState<Pause>>,
    parent: Query<&ChildOf, With<Button>>,
    help: Query<&Help>,
) {
    if let Ok(parent) = parent.get(trigger.target()) {
        if let Ok(help_type) = help.get(parent.0) {
            commands.spawn((
                Name::new("Pause Overlay"),
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                GlobalZIndex(1),
                BackgroundColor(Color::srgba(0.063, 0.071, 0.110, 0.95)),
                StateScoped(Pause(true)),
            ));
            next_pause.set(Pause(true));
            state.set(Menu::Tutorial);
            let image = match help_type {
                Help::General => "images/tutorial1.png",
                Help::Winning => "images/tutorial1a.png",
                Help::Search => "images/tutorial2.png",
                Help::Negate => "images/tutorial3.png",
            };
            commands.spawn((
                widget::ui_root("Tutorial Menu"),
                GlobalZIndex(2),
                StateScoped(Menu::Tutorial),
                children![
                    (
                        Name::new("Tutorial image"),
                        Node {
                            position_type: PositionType::Absolute,
                            width: Val::Percent(100.0),
                            height: Val::Percent(100.0),
                            top: Val::Px(0.0),
                            left: Val::Px(0.0),
                            ..default()
                        },
                        ImageNode::new(asset_server.load(image)),
                    ),
                    (
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(20.0),
                            right: Val::Px(60.0),
                            ..default()
                        },
                        children![widget::button_custom(
                            "",
                            close_menu,
                            None,
                            Some(ButtonSize {
                                width: 60.0,
                                height: 60.0
                            })
                        ),]
                    )
                ],
            ));
        }
    }
}
fn close_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
