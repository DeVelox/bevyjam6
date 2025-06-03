use Val::Px;
use bevy::prelude::*;

use crate::{
    menus::Menu,
    screens::Screen,
    theme::widget::{self, BUTTON_COLORS_ALT, BUTTON_SIZE_ALT},
};

use super::{
    level::{Level, Switch},
    logic::{AutomaticSimulation, reset_simulation, step_simulation, toggle_simulation},
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_ui);
    app.add_systems(
        Update,
        update_button_text.run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
    );
}

// TODO: Toggle reset/step button visibility based on simulation state
// TODO: Lock the rule editor based on simulation state

fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Gameplay UI"),
        GlobalZIndex(2),
        StateScoped(Screen::Gameplay),
        children![(
            widget::ui_left("Simulation Controls"),
            children![
                widget::button_custom("Simulate", toggle_simulation, Some(BUTTON_COLORS_ALT), None),
                (
                    Node {
                        display: Display::Grid,
                        row_gap: Px(8.0),
                        column_gap: Px(8.0),
                        grid_template_columns: RepeatedGridTrack::px(2, BUTTON_SIZE_ALT.width),
                        ..default()
                    },
                    children![
                        widget::button_custom(
                            "Reset",
                            reset_simulation,
                            None,
                            Some(BUTTON_SIZE_ALT)
                        ),
                        widget::button_custom("Step", step_simulation, None, Some(BUTTON_SIZE_ALT)),
                    ],
                ),
                (
                    Visibility::Hidden,
                    NextLevel,
                    widget::button("Next Level", go_next_level)
                ),
            ],
        )],
    ));
}
#[derive(Component)]
pub struct NextLevel;
pub fn show_next_level(mut commands: Commands, button: Single<Entity, With<NextLevel>>) {
    commands
        .entity(button.into_inner())
        .insert(Visibility::Visible);
}
pub fn go_next_level(
    _: Trigger<Pointer<Click>>,
    current_level: Res<State<Level>>,
    mut level: ResMut<NextState<Level>>,
    mut screen: ResMut<NextState<Screen>>,
) {
    level.set(current_level.get().next());
    screen.set(Screen::Loading);
}

pub fn update_button_text(auto: Option<Res<AutomaticSimulation>>, mut text: Query<&mut Text>) {
    if auto.is_some() {
        for mut text in &mut text {
            if text.0 == "Simulate" {
                text.0 = "Pause".to_string();
            }
        }
    } else {
        for mut text in &mut text {
            if text.0 == "Pause" {
                text.0 = "Simulate".to_string();
            }
        }
    }
}
