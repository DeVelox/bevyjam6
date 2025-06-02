use Val::Px;
use bevy::prelude::*;

use crate::{
    screens::Screen,
    theme::widget::{self, BUTTON_COLORS_ALT, BUTTON_SIZE_ALT},
};

use super::logic::simulation_callback;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_ui);
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_left("Simulation Controls"),
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![
            widget::button_custom(
                "Simulate",
                simulation_callback,
                Some(BUTTON_COLORS_ALT),
                None
            ),
            (
                Node {
                    display: Display::Grid,
                    row_gap: Px(8.0),
                    column_gap: Px(8.0),
                    grid_template_columns: RepeatedGridTrack::px(2, BUTTON_SIZE_ALT.width),
                    ..default()
                },
                children![
                    widget::button_custom("Prev", simulation_callback, None, Some(BUTTON_SIZE_ALT)),
                    widget::button_custom("Next", simulation_callback, None, Some(BUTTON_SIZE_ALT)),
                ],
            ),
        ],
    ));
}
