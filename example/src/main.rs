use dioxus::prelude::*;

fn main() {
    dioxus_makepad::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        div {
            div {
                style: "text-align: center;",
                h1 { "Dioxus" }
                h3 { "Frontend that scales." }
                p { "Dioxus is a portable, performant, and ergonomic framework for building cross-platform user interfaces in Rust." }
            }
            button {
                onclick: |_| {
                    println!("hello makepad!");
                },
                "hello, desktop!"
            }
        }
    ))
}