use bevy::prelude::*;
use std::time::Duration;

use crate::theme::shader::CustomMaterial;

use super::logic::{ANIMATION_DURATION, IterationState};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, execute_animations);
}

#[derive(Component)]
pub struct AnimationConfig {
    fps: u8,
    material: Handle<CustomMaterial>,
    frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(material: Handle<CustomMaterial>, fps: u8) -> Self {
        Self {
            fps,
            material,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1.0 / (fps as f32)), TimerMode::Once)
    }
}

fn execute_animations(
    time: Res<Time>,
    mut query: Query<&mut AnimationConfig>,
    mut state: ResMut<NextState<IterationState>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for mut config in &mut query {
        config.frame_timer.tick(time.delta());
        if let Some(material) = materials.get_mut(config.material.id()) {
            if config.frame_timer.just_finished() {
                if material.params.y <= 0.0 {
                    state.set(IterationState::Ready);
                } else {
                    material.params.y -= (1.0 / config.fps as f32) / ANIMATION_DURATION;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}
