const e_taps = array<vec2<u32>, 18>(
    vec2<u32>(0u, 20u),
    vec2<u32>(1u, 20u),
    vec2<u32>(2u, 15u),
    vec2<u32>(3u, 16u),
    vec2<u32>(4u, 15u),
    vec2<u32>(5u, 8u),
    vec2<u32>(6u, 13u),
    vec2<u32>(7u, 16u),
    vec2<u32>(9u, 16u),
    vec2<u32>(10u, 8u),
    vec2<u32>(11u, 13u),
    vec2<u32>(12u, 15u),
    vec2<u32>(14u, 20u),
    vec2<u32>(17u, 8u),
    vec2<u32>(18u, 13u),
    vec2<u32>(19u, 20u),
    vec2<u32>(21u, 20u),
    vec2<u32>(22u, 20u)
);

fn e_taps_get(i: u32) -> vec2<u32> {
    switch (i) {
        case 0u: {
            return e_taps[0];
        }
        case 1u: {
            return e_taps[1];
        }
        case 2u: {
            return e_taps[2];
        }
        case 3u: {
            return e_taps[3];
        }
        case 4u: {
            return e_taps[4];
        }
        case 5u: {
            return e_taps[5];
        }
        case 6u: {
            return e_taps[6];
        }
        case 7u: {
            return e_taps[7];
        }
        case 8u: {
            return e_taps[8];
        }
        case 9u: {
            return e_taps[9];
        }
        case 10u: {
            return e_taps[10];
        }
        case 11u: {
            return e_taps[11];
        }
        case 12u: {
            return e_taps[12];
        }
        case 13u: {
            return e_taps[13];
        }
        case 14u: {
            return e_taps[14];
        }
        case 15u: {
            return e_taps[15];
        }
        case 16u: {
            return e_taps[16];
        }
        case 17u: {
            return e_taps[17];
        }
        default: {
            return vec2<u32>(23u, 24u);
        }
    }
}