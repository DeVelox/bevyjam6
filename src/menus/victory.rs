//! The victory menu.

use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::{game::interface::go_next_level, menus::Menu, theme::widget};
use bevy_shuffle_bag::ShuffleBag;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Menu::Victory), spawn_victory_menu);
    app.add_systems(
        Update,
        go_back.run_if(in_state(Menu::Victory).and(input_just_pressed(KeyCode::Escape))),
    );
}

fn spawn_victory_menu(mut commands: Commands) {
    let mut rng = rand::thread_rng();
    let mut victory_messages = ShuffleBag::try_new(
        [
            "Lightning fast! Nice work!",
            "I Lava what you did there!",
            "Wood solve again, 10/10!",
            "You're on Fire!",
            "Mother Earth would be proud!",
            "Water those?",
            "No time for Bed, you Rock!",
            "Nice solution, no Salt!",
        ],
        &mut rng,
    )
    .unwrap();
    commands.spawn((
        widget::ui_root("Victory Menu"),
        GlobalZIndex(2),
        StateScoped(Menu::Victory),
        children![
            widget::header(*victory_messages.pick(&mut rng)),
            widget::header(" "), // just a gap
            widget::button("Go Back", close_menu),
            widget::button("Next Level", go_next_level),
        ],
    ));
}

fn close_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}

fn go_back(mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::None);
}
