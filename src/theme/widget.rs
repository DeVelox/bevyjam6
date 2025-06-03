//! Helper functions for creating common widgets.

use std::borrow::Cow;

use bevy::{
    ecs::{spawn::SpawnWith, system::IntoObserverSystem},
    prelude::*,
    ui::Val::*,
};

use crate::theme::{interaction::InteractionPalette, palette::*};

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
            row_gap: Px(8.0),
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
        TextFont::from_font_size(32.0),
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
    width: 146.0,
    height: 60.0,
};
pub fn ui_left(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            width: Percent(23.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(8.0),
            margin: UiRect {
                right: Val::Px(1450.),
                ..default()
            },
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}
pub fn ui_right(name: impl Into<Cow<'static, str>>) -> impl Bundle {
    (
        Name::new(name),
        Node {
            width: Percent(23.0),
            height: Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Px(8.0),
            margin: UiRect {
                left: Val::Px(1450.),
                ..default()
            },
            ..default()
        },
        // Don't block picking events for other UI roots.
        Pickable::IGNORE,
    )
}
