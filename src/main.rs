#![allow(unused)]

pub mod strip;

use std::{collections::HashMap, rc::Rc};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dioxus::{core::to_owned, prelude::*};
use gloo::{
    events::EventListener,
    timers::{callback::Timeout, future::TimeoutFuture},
};
use indexmap::IndexMap;
use palette::{IntoColor, Srgb};
use strip::{Control, Segment, Srgb8, State};

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
}
struct ColorBridge {
    val: UseState<String>,
    backing: Rc<Srgb>,
}

#[inline_props]
fn Color<'a>(cx: Scope, val: UseState<String>, seg: &'a Segment) -> Element {
    let val: f32 = val.get().parse().unwrap();
    let val = (val / 100.).clamp(0.0, 1.0);

    let at = (val * seg.speed_ms() as f32) as _;
    let col = seg.color_at(at);

    cx.render(rsx!(div {
        class: "square",
        style: format_args!("background-color: #{:x}", col)
    }))
}

use strip::C as Container;

// #[inline_props]
// fn Dat(cx: Scope, val: UseState<String>, now: u128) -> Element {
//     const PRIMES: &[u128] = &[
//         2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
//         97,
//     ];

//     let fac: u128 = val.get().parse().unwrap();

//     let segs = vec![
//         Segment::new(
//             100,
//             false,
//             [
//                 Container::new(Srgb::new(240, 200, 5)),
//                 Rc::new(Srgb::new(255, 20, 200)),
//             ],
//             PRIMES[10] * fac,
//         ),
//         Segment::new(
//             100,
//             false,
//             [
//                 Rc::new(Srgb::new(255, 20, 80)),
//                 Rc::new(Srgb::new(25, 250, 20)),
//             ],
//             PRIMES[11] * fac,
//         ),
//     ];

//     let cols = segs.into_iter().enumerate().map(|(id, seg)| {
//         let col = seg.color_at(*now);
//         rsx! {
//             div {
//                 key: "dat-{id}",
//                 class: "square",
//                 style: format_args!("background-color: #{:x}", col)
//             }
//         }
//     });

//     cx.render(rsx!(cols))
// }
const PRIMES: &[u128] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
];

#[inline_props]
fn Color2(
    cx: Scope,
    prime_idx: usize,
    fac: u128,
    now: u128,
    c1: UseState<Srgb8>,
    c2: UseState<Srgb8>,
) -> Element {
    let seg = Segment::new(
        100,
        false,
        [Container::new(**c1), Container::new(**c2)],
        PRIMES[*prime_idx] * fac,
    );
    let col = seg.color_at(*now);

    cx.render(rsx!(div {
        class: "square",
        style: format_args!("background-color: #{:x}", col)
    }))
}

#[inline_props]
fn ColorInput(cx: Scope, val: UseState<Srgb8>) -> Element {
    cx.render(rsx! {
        input {
            r#type: "color",
            //name: "val",
            value: format_args!("#{:x}", **val),
            //id: "val",
            oninput: move |ev| {
                let color: Srgb8 = ev.value.parse().unwrap();
                val.set(color);

            },
        }
    })
}

// #[inline_props]
// fn Segment(
//     cx: Scope,
//     c1: UseState<String>,
//     c2: UseState<String>,
//     prime_idx: usize,
//     fac: u128,
//     now: u128,
// ) -> Element {
//     const PRIMES: &[u128] = &[
//         2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
//         97,
//     ];

//     let seg = Segment::new(
//         100,
//         false,
//         [Srgb::new(240, 200, 5), Srgb::new(255, 20, 200)],
//         PRIMES[*prime_idx] * fac,
//     );

//     cx.render(rsx!(
//         ColorInput{val: c1.clone()}
//         ColorInput{val: c2.clone()}

//     ))
// }

#[inline_props]
fn SegmentN(cx: Scope, seg: Segment, prime_idx: usize, fac: u128, now: u128) -> Element {
    let c1 = use_state(&cx, || seg.color_1().to_owned());
    let c2 = use_state(&cx, || seg.color_1().to_owned());

    let seg = use_state(&cx, || {
        Segment::new(
            100,
            false,
            [Container::new(**c1), Container::new(**c2)],
            PRIMES[*prime_idx] * fac,
        )
    });

    /*    prime_idx: usize,
    fac: u128,
    now: u128,
    c1: UseState<Srgb8>,
    c2: UseState<Srgb8>,*/
    cx.render(rsx!(
        ColorInput{val: c1.clone()}
        ColorInput{val: c2.clone()}
        Color2{prime_idx: *prime_idx, fac: *fac, now: *now, c1: c1.clone(), c2: c2.clone()}

    ))
}

#[inline_props]
fn Segments(cx: Scope, state: State, fac: UseState<String>, now: u128) -> Element {
    to_owned![state];
    let fac: u128 = fac.get().parse().unwrap();
    // let c = state.with(|state| {
    //     let content = state.iter().enumerate().map(|(id, seg)| {});
    //     log::debug!("");
    //     log::debug!("");
    //     content
    // });

    let content = rsx!(p {});

    let content = state.iter().enumerate().map(|(id, seg)| {
        rsx! {
            SegmentN{seg:seg.clone(), prime_idx: 10, fac: fac, now: *now}
            p{}
        }
    });

    cx.render(rsx!(content))
}

fn app(cx: Scope) -> Element {
    let control = use_ref(&cx, || Control::new());
    let state = use_ref(&cx, || State::new_empty());

    let state = State::new([Segment::default()].into_iter());

    let now = control.write().tick();
    let now = use_state(&cx, || now);

    let some_color = use_state(&cx, || "#1122ff".to_string());

    const PRIMES: &[u128] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    const FAC: u128 = 3000;

    // to_owned![hsegs];

    // hsegs.with(|segs| segs.iter().enumerate().map(|(i, (k, v))| rsx!(div {})));

    let initial_val = "400".to_string();

    let val = use_state(&cx, || initial_val.clone());

    to_owned![now, control];
    let nc = now.clone();

    let dog = use_future(&cx, &control, |c| async move {
        let dat_now = c.with_mut(|c| c.tick());
        nc.set(dat_now);
    });

    let ticker: &CoroutineHandle<()> = use_coroutine(&cx, |_rx| async move {
        for i in 0..5 {
            //now.set(control.tick());
            control.with_mut(|c| c.tick());
            TimeoutFuture::new(30).await;
        }
    });

    // segs.read().iter().enumerate().map(|(id, seg)| {
    //     rsx! {
    //         Color2 {key: "color2-{id}", at: *now, seg: seg}
    //     }
    // });

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "Dioxus" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
            p { "{now}"}
            form {
                input {
                    r#type: "range",
                    name: "val",
                    value: "{val}",
                    id: "val",
                    min: "30",
                    max: "2000",
                    oninput: move |ev| val.set(ev.value.clone()),
                }
            }
            Segments {state: state.clone(), fac: val.clone(), now: *now}

            p { "{val}"}

        }
   ))
}
