use dioxus::prelude::*;

fn main() {
    dioxus_makepad::launch(app);
}

fn app(cx: Scope) -> Element {
    // cx.render(rsx! (
    //     div {
    //         style: "text-align: center;",
    //         h1 { "🌗 Dioxus 🚀" }
    //         h3 { "Frontend that scales." }
    //         p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
    //     }
    // ))

    cx.render(rsx! (
        p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
    ))
}