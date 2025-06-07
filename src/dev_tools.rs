//! Development tools for the game. This plugin is only enabled in dev builds.

use bevy::{
    dev_tools::states::log_transitions, ecs::spawn::SpawnWith,
    input::common_conditions::input_just_pressed, prelude::*, ui::UiDebugOptions,
};

use crate::{
    game::{
        level::{Level, Puzzle, Tile},
        logic::{GridIterations, IterationState, PlayerRules},
    },
    screens::Screen,
    theme::{palette::*, prelude::InteractionPalette},
};

pub(super) fn plugin(app: &mut App) {
    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);
    app.add_systems(Update, log_transitions::<Level>);
    app.add_systems(Update, log_transitions::<IterationState>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        (
            (reset_debug_picker, handle_debug_picker),
            toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
        ),
    );
    app.init_resource::<CurrentPaintColor>();
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}

#[derive(Component, Default, Debug)]
pub struct EditorTileColor {
    pub index: usize,
    pub color: Option<Tile>,
}

impl EditorTileColor {
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

#[derive(Component, Default, Debug)]
pub struct EditorColorPickerButton {
    pub color: Option<Tile>,
}
#[derive(Resource, Default)]
pub struct CurrentPaintColor(Option<Tile>);

pub fn handle_debug_picker(
    interaction_query: Query<(&Interaction, &EditorColorPickerButton), Changed<Interaction>>,
    mut color: ResMut<CurrentPaintColor>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            color.0 = button.color;
        }
    }
}
pub fn reset_debug_picker(
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut color: ResMut<CurrentPaintColor>,
) {
    if mouse_input.just_pressed(MouseButton::Right) {
        color.0 = None;
    }
}
pub fn handle_debug_painter(
    trigger: Trigger<Pointer<Move>>,
    mut commands: Commands,
    mut grid_iter: ResMut<GridIterations>,
    query: Query<(Entity, &EditorTileColor), With<Puzzle>>,
    mouse: Res<ButtonInput<MouseButton>>,
    color: Res<CurrentPaintColor>,
) {
    if let Some(color) = color.0 {
        if mouse.pressed(MouseButton::Left) {
            if let Ok((entity, button)) = query.get(trigger.target()) {
                grid_iter.grid.last_mut().unwrap()[button.index] = color as u8;
                commands.entity(entity).insert(color);
            }
        }
    }
}
pub fn handle_debug_editor(
    trigger: Trigger<Pointer<Click>>,
    mut commands: Commands,
    mut grid_iter: ResMut<GridIterations>,
    mut query: Query<(Entity, &mut EditorTileColor), With<Puzzle>>,
    color: Res<CurrentPaintColor>,
    rules: Res<PlayerRules>,
) {
    if trigger.event.button == PointerButton::Primary && color.0.is_none() {
        let mut color_pool = rules.color_pool.clone();
        color_pool.pop();
        if let Ok((entity, mut button)) = query.get_mut(trigger.target()) {
            let new_color = button.change_color(&color_pool).unwrap();
            grid_iter.grid.last_mut().unwrap()[button.index] = new_color as u8;
            commands.entity(entity).insert(new_color);
        }
    }
}

pub fn print_level(_: Trigger<Pointer<Click>>, grid: Res<GridIterations>) {
    warn!("{:?}", grid.grid.last().unwrap());
}

pub fn editor_color_picker(tile: Option<Tile>, action: EditorColorPickerButton) -> impl Bundle {
    let mut offset = Vec2::ZERO;
    if let Some(tile) = tile {
        offset.x = tile as u8 as f32;
    } else {
        offset.x = Tile::all().len() as f32 - 1.0;
        offset.y = 1.0;
    }
    let background_color = if let Some(tile) = tile {
        tile.color()
    } else {
        SOCKET
    };
    (
        Name::new("Color Picker"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Picker Inner"),
                    Button,
                    action,
                    BackgroundColor(BUTTON_PRESSED_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_PRESSED_BACKGROUND,
                        hovered: BUTTON_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Picker Color"),
                        Node {
                            width: Val::Px(30.0),
                            height: Val::Px(30.0),
                            ..default()
                        },
                        BackgroundColor(background_color),
                        Pickable::IGNORE,
                    )],
                ))
                .insert((
                    Node {
                        width: Val::Px(40.0),
                        height: Val::Px(40.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.0)),
                ));
        })),
    )
}
