#import bevy_sprite::mesh2d_vertex_output::VertexOutput
// we can import items from shader modules in the assets folder with a quoted path
#import "shaders/utils.wgsl"::{TAU,uv_atlas_maxrect,deg2vec}
// #import "shaders/enums.wgsl"::{e_taps_get}

@group(2) @binding(0) var atlas: texture_2d<f32>;
@group(2) @binding(1) var atlas_c: sampler;
@group(2) @binding(2) var<uniform> offset: u32;
@group(2) @binding(3) var<uniform> ttr: u32;

const NOTE_SCALE = vec2<f32>(180.0);
const HALF_NOTE_SCALE = NOTE_SCALE * 0.5;
const SCREEN_SIZE = vec2<f32>(1080.0);
const ATLAS_SIZE = vec2<u32>(5, 5);
const F_ATLAS_SIZE = vec2<f32>(ATLAS_SIZE);

fn t_atl(i: u32) -> vec2<f32> {
    return uv_atlas_maxrect(i, ATLAS_SIZE);
}

const SCREEN_SCALE: vec2<f32> = SCREEN_SIZE / NOTE_SCALE;
const HALF_NOTE_SCALE_NORMALIZED: vec2<f32> = HALF_NOTE_SCALE / NOTE_SCALE;
const ATLAS_SIZE_RECIPROCAL: vec2<f32> = vec2<f32>(1.0) / F_ATLAS_SIZE;

fn t_atl_s(i: u32, v: vec2<f32>, rot: f32) -> vec2<f32> {
    let s = sin(rot);
    let c = cos(rot);

    // Combined rotation and scaling
    let rotated_scaled = vec2<f32>(
        v.x * c - v.y * s,
        v.x * s + v.y * c
    ) * SCREEN_SCALE;

    return (t_atl(i) + v_clamp(rotated_scaled + HALF_NOTE_SCALE_NORMALIZED)) * ATLAS_SIZE_RECIPROCAL;
}

fn v_clamp(v: vec2<f32>) -> vec2<f32> {
    return vec2<f32>(clamp(v.x, 0.0, 1.0), clamp(v.y, 0.0, 1.0));
}

fn invert(c: vec4<f32>) -> vec4<f32> {
    return vec4<f32>(1.0 - c.x, 1.0 - c.y, 1.0 - c.z, c.w);
}

fn t_draw(i: u32, uv: vec2<f32>, rot: f32) -> vec4<f32> {
    return textureSample(atlas, atlas_c, t_atl_s(i, uv, rot));
}

fn alpha_over(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    let na = mix(b.w, 1.0, a.w);
    return vec4<f32>(mix(b.w * b.xyz, a.xyz, a.w) / na, na);
}

fn t_draw_note(ni: u32, uv: vec2<f32>, rot: f32) -> vec4<f32> {
    // Extract note properties from bit flags
    let isLegacySkin = u32(bool(ni & 1u));
    let isChord = u32(bool(ni & 2u));
    let isSlide = u32(bool(ni & 4u));
    let isBreak = u32(bool(ni & 8u));
    let isDual = u32(bool(ni & 16u));

    // Calculate x/y coordinates in atlas
    // x: 0 for normal, 1 for chord, 2 for chord+slide
    let x = isSlide * 2u + isChord;
    // y: Based on skin type, break status and dual status
    let y = (isLegacySkin * 2u) + isBreak + isDual;

    return t_draw(y * ATLAS_SIZE.x + x, uv, rot);
}

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let t = f32(offset) / 1000.0;
    let l = t * 2 % 8;
    let angle = TAU/4 + TAU/16 + TAU * f32(l) / 8;
    let offset_vec = vec2<f32>(cos(angle), sin(angle));
    let cuv = mesh.uv - vec2<f32>(0.5) + offset_vec * clamp(t, 0.0, 0.89) * 0.5;

    let normalized_angle = (angle % TAU + TAU) % TAU;
    let should_invert = (normalized_angle > TAU/4 && normalized_angle <= TAU/2) ||
                       (normalized_angle > TAU*0.75 && normalized_angle <= TAU);

    let base_angle = select(angle, angle + TAU/2, !should_invert);
    let result = t_draw_note(1u | 2u | 4u, cuv, base_angle);
    return result;
}
