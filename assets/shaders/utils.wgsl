const TAU = 2.0 * radians(180.0);

// Only ok with uniform size atlas
// for packing: https://gammafp.com/tool/atlas-packer
fn uv_atlas_maxrect(i: u32, c: vec2<u32>) -> vec2<f32> {
    return vec2<f32>(f32(i % c.x), f32(i / c.x));
}

fn uv_atlas(i: u32, c: vec2<u32>, i_scl: vec2<u32>) -> vec2<u32> {
    return uv_atlas_maxrect(i, c) * i_scl;
}

fn pack_note(a: u32, b: u32, c: u32) -> u32 {
    return (a << 24) | (b << 19) | c;
}

fn unpack_note(value: u32) -> vec3<u32> {
    let a = value >> 24;
    let b = (value >> 19) & 0x1F;
    let c = value & 0xFFFFFF;
    return vec3<u32>(a, b, c);
}
