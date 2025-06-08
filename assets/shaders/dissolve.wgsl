#import bevy_sprite::mesh2d_vertex_output::VertexOutput
#import bevy_shader_utils::voronoise::voronoise

@group(2) @binding(0) var sprite_texture: texture_2d<f32>;
@group(2) @binding(1) var sprite_sampler: sampler;
@group(2) @binding(2) var<uniform> params: vec4<f32>;
@group(2) @binding(3) var<uniform> burn_color: vec4<f32>;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let atlas_index = params.x;
    // only for the top row where all the sprites are
    let scale = 1.0 / 9.0;
    let index = atlas_index / 9.0;
    let atlas_uv = vec2<f32>(
        index + mesh.uv.x * scale,
        0.0 + mesh.uv.y * 0.5
    );


    let noise: f32 = voronoise(mesh.uv, 0.0, 1.0);
    let sprite = textureSample(sprite_texture, sprite_sampler, atlas_uv);
    let dissolve_value = params.y;
    let burn_size = params.z;

   	let burn_size_step = burn_size * step(0.001, dissolve_value) * step(dissolve_value, 0.999);
	let threshold = smoothstep(noise - burn_size_step, noise, dissolve_value);
	let border = smoothstep(noise, noise + burn_size_step, dissolve_value);

    return vec4(mix(burn_color.rgb, sprite.rgb, border), sprite.a * threshold);
}
