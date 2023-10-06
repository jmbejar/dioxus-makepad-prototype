use dioxus::prelude::*;

fn main() {
    dioxus_makepad::launch(app);
}

fn app(cx: Scope) -> Element {
    let counter = use_state(cx, || 0);
    cx.render(rsx! (
        div {
            div {
                style: "text-align: center;",
                h1 { "Dioxus" }
                h3 { "Frontend that scales." }
                p { "Count: {counter}" }
                p { "Count: {counter}" }
            }
            button {
                onclick: |_| {
                    *counter.make_mut() += 1;
                    println!("hello makepad!");
                },
                "Click to increment counter"
            }
        }
    ))
}