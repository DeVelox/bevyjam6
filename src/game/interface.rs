use Val::Px;
use bevy::{ecs::spawn::SpawnIter, prelude::*};
// use bevy_egui::{EguiContextPass, EguiContextSettings, EguiContexts, EguiPlugin, egui};

use crate::{
    menus::Menu,
    screens::Screen,
    theme::widget::{self, BUTTON_COLORS_ALT, BUTTON_SIZE_ALT, Sidebar},
};

use super::{level::spawn_level, logic::PlayerRules};
use super::{
    level::{Level, LevelAssets, Switch},
    logic::{AutomaticSimulation, reset_simulation, step_simulation, toggle_simulation},
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
            update_button_text,
        )
            .run_if(in_state(Screen::Gameplay).and(in_state(Menu::None))),
    );
    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_simulation_ui, spawn_rules_ui.after(spawn_level)),
    );
    app.add_systems(Update, update_ui_scale);
    app.add_observer(change_font);
}

fn spawn_rules_ui(
    mut commands: Commands,
    sidebars: Query<(Entity, &Name, Option<&Children>), With<Sidebar>>,
    level_assets: Res<LevelAssets>,
    player_input: Res<PlayerRules>,
) {
    for (entity, name, children) in &sidebars {
        if name.as_str() == "Left Sidebar" {
            if let Some(children) = children {
                for child in children.iter() {
                    commands.entity(child).despawn();
                }
            }
            let rule_widgets: Vec<_> = player_input
                .rules
                .iter()
                .map(|(tile, rule)| {
                    widget::rule_ui(*tile, rule.clone(), level_assets.tilesheet.clone())
                })
                .collect();
            commands
                .entity(entity)
                .insert(Children::spawn(SpawnIter(rule_widgets.into_iter())));
        }
    }
}

fn spawn_simulation_ui(mut commands: Commands) {
    commands.spawn((
        widget::ui_row("Gameplay UI"),
        GlobalZIndex(1),
        StateScoped(Screen::Gameplay),
        children![
            (widget::ui_split(
                "Left Sidebar",
                AlignItems::FlexEnd,
                JustifyContent::Center
            ),),
            (
                widget::ui_split(
                    "Right Sidebar",
                    AlignItems::FlexStart,
                    JustifyContent::FlexEnd
                ),
                children![
                    (
                        widget::button("Next Level", go_next_level),
                        Visibility::Hidden,
                        NextLevel,
                    ),
                    widget::button_custom(
                        "Simulate",
                        toggle_simulation,
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
                            widget::button_custom(
                                "Reset",
                                reset_simulation,
                                None,
                                Some(BUTTON_SIZE_ALT)
                            ),
                            widget::button_custom(
                                "Step",
                                step_simulation,
                                None,
                                Some(BUTTON_SIZE_ALT)
                            ),
                        ],
                    ),
                ],
            ),
        ],
    ));
}
#[derive(Component)]
pub struct NextLevel;
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
