use bevy::{prelude::default, window::Window};

pub fn config_window() -> Window {
    Window {
        title: "KaniMai!client".to_string(),
        present_mode: bevy::window::PresentMode::AutoNoVsync,
        ..default()
    }
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
pub enum Sensor {
    E1,
    E2,
    E3,
    E4,
    E5,
    E6,
    E7,
    E8,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    C1,
    C2,
    B1,
    B2,
    B3,
    B4,
    B5,
    B6,
    B7,
    B8,
    A1,
    A2,
    A3,
    A4,
    A5,
    A6,
    A7,
    A8,
}

impl Sensor {
    pub const ENUM_STORE: [Sensor; 34] = [
        Self::E1,
        Self::E2,
        Self::E3,
        Self::E4,
        Self::E5,
        Self::E6,
        Self::E7,
        Self::E8,
        Self::D1,
        Self::D2,
        Self::D3,
        Self::D4,
        Self::D5,
        Self::D6,
        Self::D7,
        Self::D8,
        Self::C1,
        Self::C2,
        Self::B1,
        Self::B2,
        Self::B3,
        Self::B4,
        Self::B5,
        Self::B6,
        Self::B7,
        Self::B8,
        Self::A1,
        Self::A2,
        Self::A3,
        Self::A4,
        Self::A5,
        Self::A6,
        Self::A7,
        Self::A8,
    ];
    pub fn iter() -> std::slice::Iter<'static, Self> {
        Self::ENUM_STORE.iter()
    }
    pub fn map_in(index: usize) -> Option<Self> {
        Self::ENUM_STORE.get(index).map(|s| *s)
    }

    pub fn map_out(&self) -> u8 {
        Self::ENUM_STORE
            .iter()
            .enumerate()
            .find_map(|(i, m)| if m == self { Some(i as u8) } else { None })
            .expect("NO WAY THIS COULD PANIC?!")
    }

    pub fn range(&self) -> u8 {
        if *self < Self::D1 {
            0
        } else if *self < Self::C1 {
            1
        } else if *self < Self::B1 {
            2
        } else if *self < Self::A1 {
            3
        } else {
            4
        }
    }
}

pub const SENSOR_MAPPING: [
    // x y width height euler_rot
    [f32; 5];34] = [
        // E1-8
        [0.0, 310.0, 104.0, 104.0, 45.0],
        [219.2031, 219.2031, 104.0,104.0,0.0],
        [310.0, 0.0, 104.0,104.0,-45.0],
        [219.2031, -219.2031, 104.0,104.0,-90.0],
        [0.0, -310.0, 104.0, 104.0, 225.0],
        [-219.2031, -219.2031, 104.0,104.0,180.0],
        [-310.0, 0.0, 104.0,104.0,-225.0],
        [-219.2031, 219.2031, 104.0,104.0,90.0],
        // D1-8
        [0.0,447.07,138.0,185.85,0.0],
        [316.1262,316.1262,138.0,185.85,-45.0],
        [447.0701,0.0,138.0,185.85,-90.0],
        [316.1262,-316.1262,138.0,185.85,225.0],
        [0.0,-447.0701,138.0,185.85,180.0],
        [-316.1262,-316.1262,138.0,185.85,-225.0],
        [-447.0701,0.0,138.0,185.85,90.0],
        [-316.1262,316.1262,138.0,185.85,45.0],
        // C1-2
        [55.535,0.0,99.405,208.246,0.0],
        [-55.535,0.0,99.405,208.246,0.0],
        // B1-8
        [82.0,198.0,165.0,136.0,-22.5],
        [198.0,82.0,165.0,136.0,-67.5],
        [198.0,-82.0,165.0,136.0,247.5],
        [82.0,-198.0,165.0,136.0,202.5],
        [-82.0,-198.0,165.0,136.0,-202.5],
        [-198.0,-82.0,165.0,136.0,-247.5],
        [-198.0,82.0,165.0,136.0,67.5],
        [-82.0,198.0,165.0,136.0,22.5],
        // A1-A8
        [164.57,397.8848,261.58,219.71,-22.5],
        [397.7156,164.9785,261.58,219.71,-67.5],
        [397.8848,-164.5701,261.58,219.71,247.5],
        [164.9784,-397.7156,261.58,219.71,202.5],
        [-164.57,-397.8848,261.58,219.71,-202.5],
        [-397.7156,-164.9785,261.58,219.71,-247.5],
        [-397.8848,164.5701,261.58,219.71,67.5],
        [-164.9784,397.7156,261.58,219.71,22.5],
        ];
