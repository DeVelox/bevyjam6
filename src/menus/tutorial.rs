//! The tutorial menu.

use crate::game::interface::{Help, HelpSeen};
use crate::game::level::LevelAssets;
use crate::theme::interaction::InteractionPalette;
use crate::theme::palette::BUTTON_PRESSED_BACKGROUND;
use crate::theme::widget::{ButtonColors, ButtonSize};
use crate::{Pause, menus::Menu, theme::widget};
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
    level_assets: Res<LevelAssets>,
    mut state: ResMut<NextState<Menu>>,
    mut next_pause: ResMut<NextState<Pause>>,
    mut help_seen: ResMut<HelpSeen>,
    parent: Query<(&ChildOf, &Children, Entity), With<Button>>,
    help: Query<&Help>,
) {
    if let Ok((parent, text, button)) = parent.get(trigger.target()) {
        if let Ok(&help_type) = help.get(parent.0) {
            if !help_seen.0[help_type as usize] {
                let colors = ButtonColors::default();
                commands
                    .entity(*text.into_iter().next().unwrap())
                    .insert(TextColor(colors.text));
                commands.entity(button).insert((
                    BackgroundColor(colors.background),
                    InteractionPalette {
                        none: colors.background,
                        hovered: colors.hovered,
                        pressed: colors.pressed,
                    },
                    BoxShadow::new(
                        BUTTON_PRESSED_BACKGROUND,
                        Val::Px(0.0),
                        Val::Px(8.0),
                        Val::Percent(0.0),
                        Val::Px(0.0),
                    ),
                ));
                help_seen.0[help_type as usize] = true;
            }
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
                Help::General => level_assets.help_general.clone(),
                Help::Winning => level_assets.help_winning.clone(),
                Help::Search => level_assets.help_search.clone(),
                Help::Negate => level_assets.help_negate.clone(),
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
                        ImageNode::new(image),
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
