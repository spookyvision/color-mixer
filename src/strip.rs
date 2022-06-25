use chrono::{DateTime, Utc};

use palette::{convert::IntoColorUnclamped, FromColor, Hue, IntoColor, Luv, Mix, Srgb};

pub type Srgb8 = palette::rgb::Rgb<palette::encoding::Srgb, u8>;

#[derive(PartialEq, Clone)]
pub struct Segment {
    length: usize,
    bgr: bool,
    colors: [Srgb8; 2],
    speed_ms: u128,
}

impl Segment {
    pub fn new(length: usize, bgr: bool, colors: [Srgb8; 2], speed_ms: u128) -> Self {
        Self {
            length,
            bgr,
            colors,
            speed_ms,
        }
    }

    pub fn mix(&self, mut t: f32) -> Srgb8 {
        let mut c1: Luv = self.colors[0].into_format().into_color();
        let mut c2: Luv = self.colors[1].into_format().into_color();
        if t >= 0.5 {
            (c1, c2) = (c2, c1);
            t -= 0.5;
        }
        t = simple_easing::sine_in_out(t * 2.0);

        let res = c1.mix(&c2, t);
        // TODO: bgr
        let res: Srgb = res.into_color();
        res.into_format()
    }
    pub fn color_at(&self, at_millis: u128) -> Srgb8 {
        let wrapped = (at_millis % self.speed_ms) as f32;
        let speed = self.speed_ms as f32;
        let t = wrapped / speed;
        self.mix(t)
    }

    pub fn speed_ms(&self) -> u128 {
        self.speed_ms
    }
}

pub struct Control {
    start: DateTime<Utc>,
    strips: Vec<Segment>,
}

impl Control {
    pub fn new() -> Self {
        Self {
            start: Utc::now(),
            strips: vec![],
        }
    }

    pub fn tick(&self, at_ms: u128) {
        let now = Utc::now();
        let dt = now
            .signed_duration_since(self.start)
            .to_std()
            .unwrap()
            .as_millis();
        for strip in &self.strips {}
    }
}
