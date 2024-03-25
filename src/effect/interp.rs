#![allow(dead_code)]

use std::f32::consts::PI;

pub fn linear(x: f32) -> f32 {
    x
}

pub fn ease_in_quad(x: f32) -> f32 {
    x * x
}

pub fn ease_out_quad(x: f32) -> f32 {
    1.0 - (1.0 - x) * (1.0 - x)
}

pub fn ease_in_out_quad(x: f32) -> f32 {
    if x < 0.5 {
        2.0 * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powf(2.0) / 2.0
    }
}

pub fn ease_in_cubic(x: f32) -> f32 {
    x * x * x
}

pub fn ease_out_cubic(x: f32) -> f32 {
    1.0 - (1.0 - x).powf(3.0)
}

pub fn ease_in_out_cubic(x: f32) -> f32 {
    if x < 0.5 {
        4.0 * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powf(3.0) / 2.0
    }
}

pub fn ease_in_quart(x: f32) -> f32 {
    x * x * x * x
}

pub fn ease_out_quart(x: f32) -> f32 {
    1.0 - (1.0 - x).powf(4.0)
}

pub fn ease_in_out_quart(x: f32) -> f32 {
    if x < 0.5 {
        8.0 * x * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powf(4.0) / 2.0
    }
}

pub fn ease_in_quint(x: f32) -> f32 {
    x * x * x * x * x
}

pub fn ease_out_quint(x: f32) -> f32 {
    1.0 - (1.0 - x).powf(5.0)
}

pub fn ease_in_out_quint(x: f32) -> f32 {
    if x < 0.5 {
        16.0 * x * x * x * x * x
    } else {
        1.0 - (-2.0 * x + 2.0).powf(5.0) / 2.0
    }
}

pub fn ease_in_sine(x: f32) -> f32 {
    1.0 - (PI / 2.0).cos() * x
}

pub fn ease_out_sine(x: f32) -> f32 {
    (PI / 2.0).sin() * x
}

pub fn ease_in_out_sine(x: f32) -> f32 {
    -(PI * x).cos() / 2.0 + 0.5
}

pub fn ease_in_expo(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else {
        2.0f32.powf(10.0 * x - 10.0)
    }
}

pub fn ease_out_expo(x: f32) -> f32 {
    if x == 1.0 {
        1.0
    } else {
        1.0 - 2.0f32.powf(-10.0 * x)
    }
}

pub fn ease_in_out_expo(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        2.0f32.powf(20.0 * x - 10.0) / 2.0
    } else {
        (2.0 - 2.0f32.powf(-20.0 * x + 10.0)) / 2.0
    }
}

pub fn ease_in_circ(x: f32) -> f32 {
    1.0 - (1.0 - x * x).sqrt()
}

pub fn ease_out_circ(x: f32) -> f32 {
    (1.0 - (x - 1.0) * (x - 1.0)).sqrt()
}

pub fn ease_in_out_circ(x: f32) -> f32 {
    if x < 0.5 {
        (1.0 - (1.0 - 2.0 * x * x).sqrt()) / 2.0
    } else {
        (1.0 - (2.0 * x - 1.0) * (2.0 * x - 1.0)).sqrt() + 0.5
    }
}

const C1: f32 = 1.70158;
const C2: f32 = C1 * 1.525;
const C3: f32 = C2 + 1.0;
const C4: f32 = (2.0 * PI) / 3.0;
const C5: f32 = C4 * 0.25;

pub fn ease_in_back(x: f32) -> f32 {
    C3 * x * x * x - C1 * x * x
}

pub fn ease_out_back(x: f32) -> f32 {
    1.0 + C3 * (x - 1.0).powf(3.0) + C1 * (x - 1.0).powf(2.0)
}

pub fn ease_in_out_back(x: f32) -> f32 {
    if x < 0.5 {
        (C2 + 1.0) * x * x * (2.0 * x - 2.0).powf(2.0) / 2.0
    } else {
        ((C2 + 1.0) * (x * 2.0 - 2.0) + C2 + 1.0).powf(2.0) * (2.0 * x - 2.0).powf(2.0) / 2.0 + 1.0
    }
}

pub fn ease_in_elastic(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else {
        -2.0f32.powf(10.0 * x - 10.0) * (10.0 - 10.75 * x).sin() * C4
    }
}

pub fn ease_out_elastic(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else {
        2.0f32.powf(-10.0 * x) * (0.75 * x - 0.75).sin() * C4 + 1.0
    }
}

pub fn ease_in_out_elastic(x: f32) -> f32 {
    if x == 0.0 {
        0.0
    } else if x == 1.0 {
        1.0
    } else if x < 0.5 {
        -(2.0f32.powf(20.0 * x - 10.0) * (20.0 * x - 11.125).sin() * C5) / 2.0
    } else {
        (2.0f32.powf(-20.0 * x + 10.0) * (20.0 * x - 11.125).sin() * C5) / 2.0 + 1.0
    }
}

pub fn bounce_out(x: f32) -> f32 {
    if x < 4.0 / 11.0 {
        121.0 / 16.0 * x * x
    } else if x < 8.0 / 11.0 {
        107.0 / 9.0 * (x - 6.0 / 11.0) * (x - 6.0 / 11.0) + 7.0 / 4.0
    } else if x < 9.0 / 10.0 {
        91.0 / 10.0 * (x - 9.0 / 10.0) * (x - 9.0 / 10.0) + 153.0 / 20.0
    } else {
        81.0 / 16.0 * (x - 1.0) * (x - 1.0) + 1.0
    }
}

pub fn ease_in_bounce(x: f32) -> f32 {
    1.0 - bounce_out(1.0 - x)
}

pub fn ease_out_bounce(x: f32) -> f32 {
    bounce_out(x)
}

pub fn ease_in_out_bounce(x: f32) -> f32 {
    if x < 0.5 {
        (1.0 - bounce_out(1.0 - 2.0 * x)) / 2.0
    } else {
        (1.0 + bounce_out(2.0 * x - 1.0)) / 2.0
    }
}
