//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use crate::{
    game::{
        interface::ColorPickerButton,
        level::{Level, Tile, spawn_level},
        logic::{IterationState, PlayerRules},
    },
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);
    app.add_systems(Update, log_transitions::<Level>);
    app.add_systems(Update, log_transitions::<IterationState>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        (toggle_debug_ui, toggle_picking).run_if(input_just_pressed(TOGGLE_KEY)),
    );
    app.add_systems(
        OnEnter(Screen::Gameplay),
        attach_observers.after(spawn_level),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn handle_debug_editor(
    trigger: Trigger<Pointer<Click>>,
    mut query: Query<(Entity, &mut ColorPickerButton), With<crate::game::level::Puzzle>>,
    rules: Res<PlayerRules>,
    mut commands: Commands,
) {
    let color_pool = &rules.color_pool.clone();
    if let Ok((entity, mut button)) = query.get_mut(trigger.target()) {
        if let Some(color) = button.change_color(color_pool) {
            commands.entity(entity).insert(color);
        } else {
            commands
                .entity(entity)
                .insert(button.change_color(color_pool).unwrap());
        }
    }
}

fn attach_observers(
    mut commands: Commands,
    query: Query<(Entity, &Tile), With<crate::game::level::Puzzle>>,
) {
    for (entity, &tile) in &query {
        commands
            .entity(entity)
            .insert((crate::game::interface::ColorPickerButton {
                index: 99, // disabled
                color: Some(tile),
                ..default()
            },))
            .observe(handle_debug_editor);
    }
}

fn toggle_picking(
    mut commands: Commands,
    query: Query<(Entity, Option<&Pickable>), With<crate::game::level::Puzzle>>,
) {
    for (entity, pickable) in &query {
        if pickable.is_some() {
            commands.entity(entity).remove::<Pickable>();
        } else {
            commands.entity(entity).insert(Pickable::default());
        }
    }
}

pub fn print_level(
    _: Trigger<Pointer<Click>>,
    query: Query<&Tile, With<crate::game::level::Puzzle>>,
) {
    let mut level = vec![];
    for tile in &query {
        level.push(*tile as u8);
    }
    warn!("{:?}", level);
}
