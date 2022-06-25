pub mod strip;

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use dioxus::prelude::*;
use palette::{IntoColor, Srgb};
use strip::Segment;
use gloo::{events::EventListener, timers::callback::Timeout};

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

fn app(cx: Scope) -> Element {
    let segs = cx.use_hook(|_| {
        vec![
            Segment::new(
                100,
                false,
                [Srgb::new(240, 200, 5), Srgb::new(255, 20, 200)],
                1500,
            ),
            Segment::new(
                100,
                false,
                [Srgb::new(255, 20, 80), Srgb::new(25, 250, 20)],
                1500,
            ),
        ]
    });
    let val = use_state(&cx, || "10".to_string());

    let fut = use_future(&cx, (), |_| async move {
        log::debug!("ohai");
    });

    let ticker: &CoroutineHandle<()> = use_coroutine(&cx, |_rx| async move {
        for i in 0..5 {
            Timeout::new(1_000, move || {
                log::debug!("whorp");
            })
            .forget();
        }

    });

    cx.render(rsx! (
        div {
            style: "text-align: center;",
            h1 { "Dioxus" }
            h3 { "Frontend that scales." }
            p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
            form {

                input {
                    r#type: "range",
                    name: "val",
                    id: "val",
                    min: "0",
                    max: "100",
                    oninput: move |ev| val.set(ev.value.clone()),
                }
            }

            segs.iter().enumerate().map(|(id, seg)| rsx!{
                Color {key: "color{id}", val: val.clone(), seg: seg}
            })


        }
    ))
}
