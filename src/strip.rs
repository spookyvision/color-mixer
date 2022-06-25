use std::{
    cell::{Ref, RefCell},
    fmt::Debug,
    hash::Hash,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use chrono::{DateTime, Utc};
use derive_more::{AsRef, Deref, DerefMut, Display, From, Into};
use palette::{convert::IntoColorUnclamped, FromColor, Hue, IntoColor, Luv, Mix, Srgb};

pub type Srgb8 = palette::rgb::Rgb<palette::encoding::Srgb, u8>;

#[cfg(not(target_arch = "wasm32"))]
impl Default for Segment {
    fn default() -> Self {
        Self::new(
            10,
            false,
            [
                RcWrap::new(Srgb8::new(255, 150, 0)),
                RcWrap::new(Srgb8::new(255, 10, 220)),
            ],
            2000,
        )
    }
}

mod std_imp {
    use std::{
        cell::{Ref, RefCell},
        rc::Rc,
    };

    use derive_more::{Deref, DerefMut, From, Into};

    #[derive(Clone, PartialEq, From, Into, Deref, DerefMut)]
    pub struct RcWrap<T>(Rc<RefCell<T>>);

    impl<T> RcWrap<T> {
        pub fn new(t: T) -> Self {
            Self(Rc::new(RefCell::new(t)))
        }
        pub fn read(&self) -> Ref<T> {
            self.0.borrow()
        }
    }

    pub type C<T> = RcWrap<T>;
}

mod wasm_imp {
    use dioxus::prelude::UseRef;
    pub type C<T> = UseRef<T>;
}

#[cfg(target_arch = "wasm32")]
use wasm_imp::C;

#[cfg(not(target_arch = "wasm32"))]
use std_imp::C;

#[derive(Clone, PartialEq, From, Into, Deref, DerefMut)]
struct Srgb8H(C<Srgb8>);

impl Debug for Srgb8H {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Srgb8H").field(&self.0.read()).finish()
    }
}

impl Hash for Srgb8H {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.read().red.hash(state);
        self.0.read().green.hash(state);
        self.0.read().blue.hash(state);
    }
}
#[derive(PartialEq, Clone, Hash, Debug, AsRef)]
pub struct Segment {
    length: usize,
    bgr: bool,
    colors: [Srgb8H; 2],
    speed_ms: u128,
}

impl Segment {
    pub fn new(length: usize, bgr: bool, colors: [C<Srgb8>; 2], speed_ms: u128) -> Self {
        Self {
            length,
            bgr,
            colors: [colors[0].into(), colors[1].into()],
            speed_ms,
        }
    }

    pub fn mix(&self, mut t: f32) -> Srgb8 {
        let mut c1: Luv = self.color_1().into_format().into_color();
        let mut c2: Luv = self.color_2().into_format().into_color();
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

    pub fn color_1(&self) -> Ref<Srgb8> {
        self.colors[0].read()
    }

    pub fn color_2(&self) -> Ref<Srgb8> {
        self.colors[1].read()
    }
}

pub struct Control {
    start: DateTime<Utc>,
    now: DateTime<Utc>,
}

impl Control {
    pub fn new() -> Self {
        let now = Utc::now();
        Self { start: now, now }
    }

    pub fn tick(&mut self) -> u128 {
        self.now = Utc::now();
        let dt = self
            .now
            .signed_duration_since(self.start)
            .to_std()
            .unwrap()
            .as_millis();
        dt
    }
}

#[derive(PartialEq, Hash, Clone, Debug)]
pub struct State {
    segments: Vec<Segment>,
}

impl State {
    pub fn new(segments: impl Iterator<Item = Segment>) -> Self {
        Self {
            segments: segments.collect(),
        }
    }

    pub fn new_empty() -> Self {
        Self { segments: vec![] }
    }
}

impl Deref for State {
    type Target = Vec<Segment>;

    fn deref(&self) -> &Self::Target {
        &self.segments
    }
}

impl DerefMut for State {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.segments
    }
}
