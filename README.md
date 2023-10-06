# Dioxus meets Makepad experiment

### What we tested here

The file `example/src/main.rs` contains a very simple Dioxus aplication. It renders a button and a text indicating how many times the button was clicked.
In the `src` folder we implemented the minimal glue to have this application rendered using Makepad. It only supports a few of HTML tags (div, h1, h3, p, button), but it should be easy to expand this list.

Regarding event handlers, it currently is able to handle click events for buttons. It is capable to send to connect Makepad events to the specific handler defined in the Dioxus app, calculate what has to change in the UI and get back to Makepad. Note that async code is not supported in this prototype yet. One limitation is that buttons should be not nested in order to work (an issue that is not hard to fix though).

### How to run the demo application

```bash
cd examples
cargo run
```

### What's next?

Most of the code is experimenatal, so everything could be implemented much better.

We're using JSON serialization for templates/mutations coming from Dioxus virtual DOM just because it was quicker way to have this demo working, but I would implement a Rust middle layer to represent that information with appropiate data types.

Also, the metadata included in templates/mutations is only partially captured by Makepad side. We have to discover what we should persist in Makepad to better process mutations as they comes from Dioxus.

CSS styles are also a pending task, nothing was implemented in this demo so far.

And, of course, keep iterating with more demo apps, in order to be able to render and respond to more events.
