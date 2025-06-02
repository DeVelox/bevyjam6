use Val::Px;
use bevy::prelude::*;

use crate::{
    screens::Screen,
    theme::widget::{self, BUTTON_COLORS_ALT, BUTTON_SIZE_ALT},
};

use super::logic::{reset_simulation, run_simulation, step_simulation};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_ui);
}

// TODO: Toggle reset/step button visibility based on simulation state
// TODO: Lock the rule editor based on simulation state

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_left("Simulation Controls"),
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![
            widget::button_custom("Simulate", run_simulation, Some(BUTTON_COLORS_ALT), None),
            (
                Node {
                    display: Display::Grid,
                    row_gap: Px(8.0),
                    column_gap: Px(8.0),
                    grid_template_columns: RepeatedGridTrack::px(2, BUTTON_SIZE_ALT.width),
                    ..default()
                },
                children![
                    widget::button_custom("Reset", reset_simulation, None, Some(BUTTON_SIZE_ALT)),
                    widget::button_custom("Step", step_simulation, None, Some(BUTTON_SIZE_ALT)),
                ],
            ),
        ],
    ));
}
