pub mod strip;

use std::rc::Rc;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dioxus::{core::to_owned, prelude::*};
use gloo::{
    events::EventListener,
    timers::{callback::Timeout, future::TimeoutFuture},
};
use palette::{IntoColor, Srgb};
use strip::{Control, Segment};

fn main() {
    // init debug tool for WebAssembly
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();

    dioxus::web::launch(app);
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

#[inline_props]
fn Dat(cx: Scope, val: UseState<String>, now: u128) -> Element {
    const PRIMES: &[u128] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];

    let fac: u128 = val.get().parse().unwrap();

    let segs = vec![
        Segment::new(
            100,
            false,
            [Srgb::new(240, 200, 5), Srgb::new(255, 20, 200)],
            PRIMES[10] * fac,
        ),
        Segment::new(
            100,
            false,
            [Srgb::new(255, 20, 80), Srgb::new(25, 250, 20)],
            PRIMES[11] * fac,
        ),
    ];

    let cols = segs.into_iter().enumerate().map(|(id, seg)| {
        let col = seg.color_at(*now);
        rsx! {
            div {
                key: "dat-{id}",
                class: "square",
                style: format_args!("background-color: #{:x}", col)
            }
        }
    });

    cx.render(rsx!(cols))
}

#[inline_props]
fn Color2<'a>(cx: Scope, at: u128, seg: &'a Segment) -> Element {
    let col = seg.color_at(*at);

    cx.render(rsx!(div {
        class: "square",
        style: format_args!("background-color: #{:x}", col)
    }))
}

fn app(cx: Scope) -> Element {
    let control = use_ref(&cx, || Control::new());
    let now = control.write().tick();
    let now = use_state(&cx, || now);

    const PRIMES: &[u128] = &[
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89,
        97,
    ];
    const FAC: u128 = 3000;

    let segs = cx.use_hook(|_| {
        vec![
            Segment::new(
                100,
                false,
                [Srgb::new(240, 200, 5), Srgb::new(255, 20, 200)],
                PRIMES[10] * FAC,
            ),
            Segment::new(
                100,
                false,
                [Srgb::new(255, 20, 80), Srgb::new(25, 250, 20)],
                PRIMES[11] * FAC,
            ),
        ]
    });

    let initial_val = "400".to_string();

    let val = use_state(&cx, || initial_val.clone());

    to_owned![control];
    let nc = now.clone();

    let dog = use_future(&cx, (&control), |(c)| async move {
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
            Dat { val: val.clone(), now: **now }

            p { "{val}"}

            segs.iter().enumerate().map(|(id, seg)| rsx!{
                Color2 {key: "color2-{id}", at: **now, seg: seg}
            })


        }
   ))
}
