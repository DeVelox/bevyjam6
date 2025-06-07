//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, input::common_conditions::input_just_pressed, prelude::*,
    ui::UiDebugOptions,
};

use crate::{
    game::{
        level::{Level, Puzzle, Tile},
        logic::{GridIterations, IterationState, PlayerRules},
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
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

#[derive(Component)]
pub struct EditorColorPickerButton {
    pub index: usize,
    pub color: Option<Tile>,
}
impl EditorColorPickerButton {
    fn change_color(&mut self, color_pool: &[Option<Tile>]) -> Option<Tile> {
        let mut new_color = color_pool[0];
        for (index, &color) in color_pool.iter().enumerate() {
            if color == self.color {
                if index + 1 < color_pool.len() {
                    new_color = color_pool[index + 1];
                }
                break;
            }
        }
        self.color = new_color;
        new_color
    }
}
pub fn handle_debug_editor(
    trigger: Trigger<Pointer<Click>>,
    mut query: Query<(Entity, &mut EditorColorPickerButton), With<Puzzle>>,
    mut grid_iter: ResMut<GridIterations>,
    rules: Res<PlayerRules>,
    mut commands: Commands,
) {
    let mut color_pool = rules.color_pool.clone();
    color_pool.pop();
    if let Ok((entity, mut button)) = query.get_mut(trigger.target()) {
        let new_color = button.change_color(&color_pool).unwrap();
        grid_iter.grid.last_mut().unwrap()[button.index] = new_color as u8;
        commands.entity(entity).insert(new_color);
    }
}

fn toggle_picking(mut commands: Commands, query: Query<(Entity, &Pickable), With<Puzzle>>) {
    for (entity, pickable) in &query {
        if *pickable == Pickable::IGNORE {
            commands.entity(entity).insert(Pickable::default());
        } else {
            commands.entity(entity).insert(Pickable::IGNORE);
        }
    }
}

pub fn print_level(_: Trigger<Pointer<Click>>, grid: Res<GridIterations>) {
    warn!("{:?}", grid.grid.last().unwrap());
}
