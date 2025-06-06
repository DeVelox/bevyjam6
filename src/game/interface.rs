use Val::Px;
use bevy::{ecs::spawn::SpawnIter, prelude::*};
// use bevy_egui::{EguiContextPass, EguiContextSettings, EguiContexts, EguiPlugin, egui};
#[cfg(feature = "dev")]
use crate::dev_tools::print_level;
use crate::{
    menus::Menu,
    screens::Screen,
    theme::{
        prelude::InteractionPalette,
        widget::{self, BUTTON_COLORS_ALT, BUTTON_SIZE_ALT, ButtonColors, ButtonSize},
    },
};

use super::{
    level::{Level, LevelAssets, Switch},
    logic::{
        AutomaticSimulation, DisableControls, Rule, Victory, reset_simulation, step_simulation,
        toggle_simulation,
    },
};
use super::{
    level::{Tile, spawn_level},
    logic::PlayerRules,
};

pub(super) fn plugin(app: &mut App) {
    // app.add_plugins(EguiPlugin::default());
    // app.add_systems(OnEnter(Screen::Gameplay), setup_egui);
    // app.add_systems(
    //     EguiContextPass,
    //     spawn_egui.run_if(in_state(Screen::Gameplay)),
    // );
    app.add_systems(
        Update,
        (
            spawn_rules_ui.run_if(resource_changed::<PlayerRules>),
            update_button_text.run_if(
                resource_added::<AutomaticSimulation>
                    .or(resource_added::<DisableControls>)
                    .or(resource_added::<Victory>)
                    .or(resource_removed::<AutomaticSimulation>)
                    .or(resource_removed::<DisableControls>)
                    .or(resource_removed::<Victory>),
            ),
            (
                handle_mask_buttons,
                handle_invert_buttons,
                handle_reset_buttons,
                handle_color_pickers,
            )
                .run_if(not(resource_exists::<DisableControls>)),
        )
            .run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
    );
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_simulation_ui, spawn_rules_ui.after(spawn_level)).chain(),
    );
    app.add_systems(Update, update_ui_scale);
    app.add_observer(change_font);
}

fn spawn_rules_ui(
    mut commands: Commands,
    sidebar: Single<(Entity, Option<&Children>), With<RulesWidget>>,
    level_assets: Res<LevelAssets>,
    player_input: Res<PlayerRules>,
) {
    let (entity, children) = sidebar.into_inner();
    if let Some(children) = children {
        for child in children.iter() {
            commands.entity(child).despawn();
        }
    }
    let rule_widgets: Vec<_> = player_input
        .rules
        .iter()
        .map(|(tile, rule)| widget::rule_ui(*tile, rule.clone(), level_assets.tilesheet.clone()))
        .collect();
    commands
        .entity(entity)
        .insert(Children::spawn(SpawnIter(rule_widgets.into_iter())));
}

fn spawn_simulation_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_row("Gameplay UI"),
        GlobalZIndex(1),
        StateScoped(Screen::Gameplay),
        children![
            (
                widget::ui_split("Left Sidebar", AlignItems::FlexEnd, JustifyContent::Center,),
                children![
                    (
                        Node {
                            height: Val::Percent(100.0),
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            row_gap: Px(15.0),
                            ..default()
                        },
                        RulesWidget
                    ),
                    (
                        Node {
                            flex_direction: FlexDirection::Column,
                            width: Val::Px(410.0),
                            row_gap: Px(8.0),
                            ..default()
                        },
                        children![
                            (
                                #[cfg(feature = "dev")]
                                widget::button_custom(
                                    "󰉉",
                                    print_level,
                                    None,
                                    Some(ButtonSize {
                                        width: 382.0,
                                        height: BUTTON_SIZE_ALT.height
                                    })
                                ),
                            ),
                            (
                                Node {
                                    display: Display::Flex,
                                    flex_direction: FlexDirection::Row,
                                    column_gap: Px(8.0),
                                    ..default()
                                },
                                children![
                                    widget::button_custom(
                                        "",
                                        toggle_simulation,
                                        Some(BUTTON_COLORS_ALT),
                                        Some(ButtonSize {
                                            width: 120.0,
                                            height: BUTTON_SIZE_ALT.height
                                        })
                                    ),
                                    (
                                        widget::button_custom(
                                            "󰑙",
                                            reset_simulation,
                                            None,
                                            Some(BUTTON_SIZE_ALT)
                                        ),
                                        LockReset
                                    ),
                                    widget::button_custom(
                                        "",
                                        step_simulation,
                                        None,
                                        Some(BUTTON_SIZE_ALT)
                                    ),
                                    (widget::button_custom(
                                        "",
                                        go_next_level,
                                        None,
                                        Some(BUTTON_SIZE_ALT)
                                    ),),
                                ],
                            ),
                        ],
                    ),
                ],
            ),
            widget::ui_split("Right Sidebar", AlignItems::Center, JustifyContent::Center,),
        ],
    ));
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

pub fn update_button_text(
    mut commands: Commands,
    auto: Option<Res<AutomaticSimulation>>,
    locked: Option<Res<DisableControls>>,
    victory: Option<Res<Victory>>,
    mut text: Query<(Entity, &Name, &ChildOf, &mut Text)>,
) {
    for (entity, name, parent, mut text) in &mut text {
        if auto.is_some() && text.0 == "" {
            text.0 = "".to_string();
        } else if auto.is_none() && text.0 == "" {
            text.0 = "".to_string();
        }
        let icons = if name.as_str() == "Rule Reset" {
            ["󰑙", "󰌾"]
        } else {
            ["󰑙", "󰝳"]
        };
        if locked.is_some() && text.0 == icons[0] {
            text.0 = icons[1].to_string();
        } else if locked.is_none() && text.0 == icons[1] {
            text.0 = icons[0].to_string();
        }
        if victory.is_some() && text.0 == "" {
            commands
                .entity(entity)
                .insert(TextColor(BUTTON_COLORS_ALT.text));
            commands.entity(parent.0).insert((
                BackgroundColor(BUTTON_COLORS_ALT.background),
                InteractionPalette {
                    none: BUTTON_COLORS_ALT.background,
                    hovered: BUTTON_COLORS_ALT.hovered,
                    pressed: BUTTON_COLORS_ALT.pressed,
                },
            ));
        } else if victory.is_none() && text.0 == "" {
            let colors = ButtonColors::default();
            commands.entity(entity).insert(TextColor(colors.text));
            commands.entity(parent.0).insert((
                BackgroundColor(colors.background),
                InteractionPalette {
                    none: colors.background,
                    hovered: colors.hovered,
                    pressed: colors.pressed,
                },
            ));
        }
    }
}

fn update_ui_scale(
    mut ui_scale: ResMut<UiScale>,
    // mut egui_settings: Single<&mut EguiContextSettings>,
    window: Single<&Window>,
) {
    let scale_factor = calculate_scale(window.into_inner());
    ui_scale.0 = scale_factor;
    // egui_settings.scale_factor = scale_factor * 2.0;
}
pub(crate) fn calculate_scale(window: &Window) -> f32 {
    let base_width = 1920.0;
    let base_height = 1080.0;
    (window.width() / base_width).min(window.height() / base_height)
}

fn change_font(
    trigger: Trigger<OnInsert, TextFont>,
    mut text_font: Query<&mut TextFont>,
    level_assets: Res<LevelAssets>,
) {
    let mut text_font = text_font.get_mut(trigger.target()).unwrap();
    text_font.font = level_assets.font.clone();
}

#[derive(Component)]
pub struct RulesWidget;
#[derive(Component)]
pub struct LockReset;

#[derive(Component)]
pub struct MaskToggleButton {
    pub tile: Tile,
    pub index: usize,
}

#[derive(Component)]
pub struct InvertToggleButton {
    pub tile: Tile,
}

#[derive(Component)]
pub struct ResetRuleButton {
    pub tile: Tile,
}

#[derive(Component, Default)]
pub struct ColorPickerButton {
    pub tile: Tile,
    pub index: usize,
    pub color: Option<Tile>,
}

impl ColorPickerButton {
    pub fn change_color(&mut self, color_pool: &[Option<Tile>]) -> Option<Tile> {
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

fn handle_color_pickers(
    mut interaction_query: Query<(&Interaction, &mut ColorPickerButton), Changed<Interaction>>,
    mut rules: ResMut<PlayerRules>,
) {
    let color_pool = &rules.color_pool.clone();
    for (interaction, mut button) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(rule) = rules.rules.get_mut(&button.tile) {
                match button.index {
                    0 => rule.tiles[0] = button.change_color(color_pool),
                    1 => rule.tiles[1] = button.change_color(color_pool),
                    2 => rule.result = button.change_color(color_pool),
                    _ => {}
                };
            }
        }
    }
}

fn handle_mask_buttons(
    interaction_query: Query<(&Interaction, &MaskToggleButton), Changed<Interaction>>,
    mut rules: ResMut<PlayerRules>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(rule) = rules.rules.get_mut(&button.tile) {
                rule.mask[button.index] = !rule.mask[button.index];
            }
        }
    }
}

fn handle_invert_buttons(
    interaction_query: Query<(&Interaction, &InvertToggleButton), Changed<Interaction>>,
    mut rules: ResMut<PlayerRules>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(rule) = rules.rules.get_mut(&button.tile) {
                rule.invert = !rule.invert;
            }
        }
    }
}

fn handle_reset_buttons(
    interaction_query: Query<(&Interaction, &ResetRuleButton), Changed<Interaction>>,
    mut rules: ResMut<PlayerRules>,
) {
    for (interaction, button) in &interaction_query {
        if *interaction == Interaction::Pressed {
            if let Some(rule) = rules.rules.get_mut(&button.tile) {
                *rule = Rule::default();
            }
        }
    }
}

// fn setup_egui(
//     mut contexts: EguiContexts,
//     level_assets: Res<LevelAssets>,
//     font_assets: Res<Assets<Font>>,
// ) {
//     let ctxs = contexts.ctx_mut();
//     ctxs.set_visuals(egui::Visuals {
//         override_text_color: Some(egui::Color32::from_rgb(0xf3, 0xa8, 0x33)),
//         window_fill: egui::Color32::from_rgb(0x6b, 0x26, 0x43),
//         window_stroke: egui::Stroke::NONE,
//         window_shadow: egui::Shadow::NONE,
//         ..Default::default()
//     });
//     if let Some(font) = font_assets.get(level_assets.font.id()) {
//         let mut fonts = egui::FontDefinitions::default();
//         fonts.font_data.insert(
//             "SpaceMono".to_owned(),
//             egui::FontData::from_owned(font.data.to_vec()).into(),
//         );
//         fonts
//             .families
//             .entry(egui::FontFamily::Proportional)
//             .or_default()
//             .insert(0, "SpaceMono".to_owned());
//         ctxs.set_fonts(fonts);
//     }
// }

// fn spawn_egui(mut contexts: EguiContexts) {
//     egui::Window::new("Rules")
//         .anchor(egui::Align2::CENTER_CENTER, egui::Vec2::new(-362.0, 0.0))
//         .collapsible(false)
//         .resizable(false)
//         .movable(false)
//         .show(contexts.ctx_mut(), |ui| {
//             ui.set_min_size(egui::Vec2::new(200.0, 420.0));
//             ui.label("world");
//         });
// }
