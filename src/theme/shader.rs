use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};
use bevy_shader_utils::ShaderUtilsPlugin;

/// This example uses a shader source file from the assets subdirectory
const SHADER_ASSET_PATH: &str = "shaders/dissolve.wgsl";

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
    app.add_plugins(ShaderUtilsPlugin);
}

// This is the struct that will be passed to your shader
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[texture(0)]
    #[sampler(1)]
    pub sprite_texture: Option<Handle<Image>>,
    #[uniform(2)]
    pub atlas_index: u32,
    #[uniform(3)]
    pub dissolve_value: f32,
    #[uniform(4)]
    pub burn_size: f32,
    #[uniform(5)]
    pub burn_color: LinearRgba,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}
