//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::{
    game::{
        interface::{ColorPickerButton, InvertToggleButton, MaskToggleButton, ResetRuleButton},
        level::Tile,
        logic::Rule,
    },
    theme::{interaction::InteractionPalette, palette::*},
};

/// A root UI node that fills the window and centers its content.
pub fn ui_root(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(16.0),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

/// A simple header label. Bigger than [`label`].
pub fn header(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Header"),
        Text(text.into()),
        TextFont::from_font_size(48.0),
        TextColor(HEADER_TEXT),
    )
}

/// A simple text label.
pub fn label(text: impl Into<String>) -> impl Bundle {
    (
        Name::new("Label"),
        Text(text.into()),
        TextFont::from_font_size(18.0),
        TextColor(LABEL_TEXT),
    )
}

/// A large rounded button with text and an action defined as an [`Observer`].
pub fn button<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        (
            Node {
                width: Px(300.0),
                height: Px(60.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
        ),
    )
}

/// A small square button with text and an action defined as an [`Observer`].
pub fn button_small<E, B, M, I>(text: impl Into<String>, action: I) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    button_base(
        text,
        action,
        Node {
            width: Px(30.0),
            height: Px(30.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
    )
}

/// A simple button with text and an action defined as an [`Observer`]. The button's layout is provided by `button_bundle`.
fn button_base<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BoxShadow::new(
                        BUTTON_PRESSED_BACKGROUND,
                        Val::Px(0.0),
                        Val::Px(8.0),
                        Val::Percent(0.0),
                        Val::Px(0.0),
                    ),
                    BackgroundColor(BUTTON_BACKGROUND),
                    InteractionPalette {
                        none: BUTTON_BACKGROUND,
                        hovered: BUTTON_HOVERED_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(32.0),
                        TextColor(BUTTON_TEXT),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}

pub struct ButtonColors {
    pub text: Color,
    pub hovered: Color,
    pub pressed: Color,
    pub background: Color,
}
impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            text: BUTTON_TEXT,
            hovered: BUTTON_HOVERED_BACKGROUND,
            pressed: BUTTON_PRESSED_BACKGROUND,
            background: BUTTON_BACKGROUND,
        }
    }
}

pub struct ButtonSize {
    pub width: f32,
    pub height: f32,
}
impl Default for ButtonSize {
    fn default() -> Self {
        ButtonSize {
            width: 300.0,
            height: 60.0,
        }
    }
}

pub fn button_custom<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    colors: Option<ButtonColors>,
    size: Option<ButtonSize>,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let colors = colors.unwrap_or_default();
    let size = size.unwrap_or_default();
    button_base_custom(
        text,
        action,
        (
            Node {
                width: Px(size.width),
                height: Px(size.height),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BorderRadius::all(Val::Px(10.0)),
        ),
        colors,
    )
}
fn button_base_custom<E, B, M, I>(
    text: impl Into<String>,
    action: I,
    button_bundle: impl Bundle,
    colors: ButtonColors,
) -> impl Bundle
where
    E: Event,
    B: Bundle,
    I: IntoObserverSystem<E, B, M>,
{
    let text = text.into();
    let action = IntoObserverSystem::into_system(action);
    (
        Name::new("Button"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Button Inner"),
                    Button,
                    BackgroundColor(colors.background),
                    BoxShadow::new(
                        colors.pressed,
                        Val::Px(0.0),
                        Val::Px(8.0),
                        Val::Percent(0.0),
                        Val::Px(0.0),
                    ),
                    InteractionPalette {
                        none: colors.background,
                        hovered: colors.hovered,
                        pressed: colors.pressed,
                    },
                    children![(
                        Name::new("Button Text"),
                        Text(text),
                        TextFont::from_font_size(32.0),
                        TextColor(colors.text),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert(button_bundle)
                .observe(action);
        })),
    )
}

pub const BUTTON_COLORS_ALT: ButtonColors = ButtonColors {
    text: BUTTON_TEXT_ALT,
    hovered: BUTTON_HOVERED_BACKGROUND_ALT,
    pressed: BUTTON_PRESSED_BACKGROUND_ALT,
    background: BUTTON_BACKGROUND_ALT,
};
pub const BUTTON_SIZE_ALT: ButtonSize = ButtonSize {
    width: 80.0,
    height: 60.0,
};

pub fn ui_split(
    name: impl Into<Cow<'static, str>>,
    align: AlignItems,
    justify: JustifyContent,
) -> impl Bundle {
    const MARGIN: Val = Val::Px(500.0);
    let mut padding = UiRect::default();
    match align {
        AlignItems::FlexStart => {
            padding.left = MARGIN;
        }
        AlignItems::FlexEnd => {
            padding.right = MARGIN;
        }
        _ => {}
    };
    (
        Name::new(name),
        Node {
            width: Percent(50.0),
            height: Px(920.0),
            align_items: align,
            justify_content: justify,
            flex_direction: FlexDirection::Column,
            row_gap: Px(16.0),
            padding,
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}
pub fn ui_row(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            position_type: PositionType::Absolute,
            width: Percent(100.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}

#[derive(Event)]
pub struct ColorPickerEvent {
    pub color: Color,
}
pub fn color_picker(
    tile: Option<Tile>,
    _image: Handle<Image>,
    is_key: bool,
    action: ColorPickerButton,
) -> impl Bundle {
    let mut offset = Vec2::ZERO;
    if let Some(tile) = tile {
        offset.x = tile as u8 as f32;
    } else {
        offset.x = Tile::all().len() as f32 - 1.0;
        offset.y = 1.0;
    }
    let color = if is_key {
        BUTTON_BACKGROUND
    } else {
        BUTTON_PRESSED_BACKGROUND
    };
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
                    BackgroundColor(color),
                    InteractionPalette {
                        none: color,
                        hovered: BUTTON_BACKGROUND,
                        pressed: BUTTON_PRESSED_BACKGROUND,
                    },
                    children![(
                        Name::new("Picker Color"),
                        Node {
                            width: Val::Px(50.0),
                            height: Val::Px(50.0),
                            ..default()
                        },
                        BackgroundColor(background_color),
                        Pickable::IGNORE,
                    )],
                ))
                .insert((
                    Node {
                        width: Px(60.0),
                        height: Px(60.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BorderRadius::all(Val::Px(10.0)),
                ));
        })),
    )
}
pub fn direction_picker(
    value: bool,
    invert: bool,
    is_invert_toggle: bool,
    action: impl Bundle,
) -> impl Bundle {
    let color = if invert {
        if value { INVERTED } else { DISABLED }
    } else if value {
        ENABLED
    } else {
        DISABLED
    };
    let color = if is_invert_toggle {
        if value { INVERTED } else { ENABLED }
    } else {
        color
    };
    (
        Name::new("Direction Picker"),
        Node::default(),
        Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
            parent
                .spawn((
                    Name::new("Picker Inner"),
                    Button,
                    action,
                    children![(
                        Name::new("Picker Direction"),
                        Node {
                            width: Val::Px(14.0),
                            height: Val::Px(14.0),
                            ..default()
                        },
                        BackgroundColor(color),
                        BorderRadius::all(Val::Px(match is_invert_toggle {
                            true => 10.0,
                            false => 0.0,
                        })),
                        // if is_invert_toggle {
                        //     if value { Text::new("") } else { Text::new("x") }
                        // } else {
                        //     Text::new("")
                        // },
                        // TextColor(BUTTON_TEXT_ALT),
                        // Don't bubble picking events from the text up to the button.
                        Pickable::IGNORE,
                    )],
                ))
                .insert((
                    Node {
                        width: Px(16.0),
                        height: Px(16.0),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    BackgroundColor(BUTTON_TEXT_ALT),
                    BorderRadius::all(Val::Px(match is_invert_toggle {
                        true => 10.0,
                        false => 0.0,
                    })),
                ));
        })),
    )
}
pub fn rule_ui(tile: Tile, rule: Rule, image: Handle<Image>) -> impl Bundle {
    (
        Name::new("Rule UI"),
        Node {
            position_type: PositionType::Relative,
            width: Percent(100.0),
            height: Percent(8.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Row,
            column_gap: Px(8.0),
            padding: UiRect::default().with_right(Val::Px(0.0)),
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
        children![
            color_picker(
                Some(tile),
                image.clone(),
                true,
                ColorPickerButton {
                    index: 99, // disabled
                    ..default()
                }
            ),
            (
                Text::new(""),
                TextColor(DISABLED),
                TextFont::from_font_size(24.0),
            ),
            color_picker(
                rule.result,
                image.clone(),
                false,
                ColorPickerButton {
                    tile,
                    index: 2,
                    color: rule.result
                }
            ),
            (
                Node {
                    display: Display::Grid,
                    margin: UiRect {
                        left: Val::Px(16.0),
                        right: Val::Px(16.0),
                        ..default()
                    },
                    row_gap: Px(5.0),
                    column_gap: Px(5.0),
                    grid_template_columns: RepeatedGridTrack::px(3, 16.0),
                    ..default()
                },
                Children::spawn(SpawnWith(move |parent: &mut ChildSpawner| {
                    for i in [5, 6, 7, 3, 4, 0, 1, 2] {
                        if i == 4 {
                            parent.spawn(direction_picker(
                                rule.invert,
                                false,
                                true,
                                InvertToggleButton { tile },
                            ));
                        }
                        parent.spawn(direction_picker(
                            rule.mask[i],
                            rule.invert,
                            false,
                            MaskToggleButton { tile, index: i },
                        ));
                    }
                }),),
            ),
            color_picker(
                rule.tiles[0],
                image.clone(),
                false,
                ColorPickerButton {
                    tile,
                    index: 0,
                    color: rule.tiles[0]
                }
            ),
            (
                Button,
                ResetRuleButton { tile },
                Node {
                    width: Val::Px(30.0),
                    height: Val::Px(30.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(
                    Name::new("Rule Reset"),
                    Text::new("󰑙"),
                    TextColor(DISABLED),
                    TextFont::from_font_size(32.0),
                )]
            ),
        ],
    )
}
