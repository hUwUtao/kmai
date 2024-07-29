#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/utils.wgsl"::{TAU,uv_atlas_maxrect,deg2vec}
#import "shaders/enums.wgsl"::{e_taps_get}

@group(2) @binding(0) var atlas: texture_2d<f32>;
@group(2) @binding(1) var atlas_c: sampler;
@group(2) @binding(2) var<uniform> offset: u32;
@group(2) @binding(3) var<uniform> ttr: u32;

const f_note_scale = vec2<f32>(180.0);
const f_half_note_scale = f_note_scale * 0.5;
const f_screen_size = vec2<f32>(1080.0);
const atlas_size = vec2<u32>(5, 5);
const f_atlas_size = vec2<f32>(atlas_size);

fn t_atl(i: u32) -> vec2<f32> {
    return uv_atlas_maxrect(i, atlas_size);
}

fn t_atl_s(i: u32, uv: vec2<f32>) -> vec2<f32> {
    return (t_atl(i) + v_clamp((uv * f_screen_size + f_half_note_scale) / f_note_scale)) / f_atlas_size;
}

fn v_clamp(v: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(clamp(v.x, 0.0, 1.0), clamp(v.y, 0.0, 1.0));
}

fn invert(c: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(1.0 - c.x, 1.0 - c.y, 1.0 - c.z, c.w);
}

fn t_draw(i: u32, uv: vec2<f32>) -> vec4<f32> {
    return textureSample(
        atlas, atlas_c, t_atl_s(i, uv)
    );
}

fn alpha_over(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    let na = mix(b.w, 1.0, a.w);
    return vec4<f32>(mix(b.w * b.xyz, a.xyz, a.w) / na, na);
}

fn t_draw_note(ni: u32, uv: vec2<f32>) -> vec4<f32> {
    let pair = e_taps_get(ni);
    return alpha_over(
        t_draw(pair.y, uv), t_draw(pair.x, uv),
    );
}

@fragment
fn fragment(
    mesh: VertexOutput,
) -> @location(0) vec4<f32> {
    let t = (f32(offset) / 1000);
    let cuv = mesh.uv - vec2<f32>(0.5) + deg2vec(22.5 + 45.0 * 2) * clamp(t, 0.0, 0.89) * 0.5;
    return t_draw_note(u32((3 * t) % 18), cuv);
}