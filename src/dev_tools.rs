//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use crate::{
    game::{
        interface::ColorPickerButton,
        level::{Level, Puzzle, Tile},
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
    app.add_systems(Update, attach_observers);
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

fn handle_debug_editor(
    trigger: Trigger<Pointer<Click>>,
    mut query: Query<(Entity, &mut ColorPickerButton), With<Puzzle>>,
    rules: Res<PlayerRules>,
    mut commands: Commands,
) {
    let mut color_pool = rules.color_pool.clone();
    color_pool.pop();
    if let Ok((entity, mut button)) = query.get_mut(trigger.target()) {
        commands
            .entity(entity)
            .insert(button.change_color(&color_pool).unwrap());
    }
}

fn attach_observers(
    mut commands: Commands,
    query: Query<(Entity, &Tile), (With<Puzzle>, Without<ColorPickerButton>)>,
) {
    for (entity, &tile) in &query {
        commands
            .entity(entity)
            .insert((ColorPickerButton {
                index: 99, // disabled
                color: Some(tile),
                ..default()
            },))
            .observe(handle_debug_editor);
    }
}

fn toggle_picking(mut commands: Commands, query: Query<(Entity, Option<&Pickable>), With<Puzzle>>) {
    for (entity, pickable) in &query {
        if pickable.is_some() {
            commands.entity(entity).remove::<Pickable>();
        } else {
            commands.entity(entity).insert(Pickable::default());
        }
    }
}

pub fn print_level(_: Trigger<Pointer<Click>>, query: Query<(Entity, &Tile), With<Puzzle>>) {
    let mut tiles: Vec<(Entity, &Tile)> = query.iter().collect();
    tiles.sort_by(|a, b| {
        a.0.index()
            .partial_cmp(&b.0.index())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    let mut level = vec![];
    for (_, tile) in tiles {
        level.push(*tile as u8);
    }
    warn!("{:?}", level);
}
