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

    let sprite = textureSample(sprite_texture, sprite_sampler, atlas_uv);
    let dissolve_value = params.y;
    let burn_size = params.z;
    let game_time = params.w;
    let noise_scale = 10.0;
    let offset_hash = hash12(vec2<f32>(atlas_index, game_time));
    let noise_uv = (mesh.uv + offset_hash) * noise_scale ;
    let noise = voronoise(noise_uv, 1.0, 1.0);

   	let burn_size_step = burn_size * step(0.001, dissolve_value) * step(dissolve_value, 0.999);
	let threshold = smoothstep(noise - burn_size_step, noise, dissolve_value);
	let border = smoothstep(noise, noise + burn_size_step, dissolve_value);

    return vec4(mix(burn_color.rgb, sprite.rgb, border), sprite.a * threshold);
}

fn hash12(p: vec2<f32>) -> f32
{
    var p3  = fract(vec3<f32>(p.xyx) * .1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}
fn hash22(p: vec2<f32>) -> vec2<f32>
{
    var p3 = fract(vec3<f32>(p.xyx) * vec3<f32>(.1031, .1030, .0973));
    p3 += dot(p3, p3.yzx+33.33);
    return fract((p3.xx+p3.yz)*p3.zy);
}
