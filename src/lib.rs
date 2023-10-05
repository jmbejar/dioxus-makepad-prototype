use dioxus::core::{Component, VirtualDom, Mutation, Template, ElementId};
use dioxus::prelude::MouseData;
use std::cell::RefCell;
use std::rc::Rc;

mod makepad_app;

struct DioxusGlobals {
    vdom: VirtualDom,
    cfg: Config,
}

thread_local! {
    static DIOXUS_GLOBALS: RefCell<Option<DioxusGlobals>> = RefCell::new(None);
}

pub fn rebuild_virtual_dom() -> (String, String) {
    let mut serialized_templates: String = String::new();
    let mut serialized_edits: String = String::new();

    DIOXUS_GLOBALS.with(|cell| {
        let mut dg = cell.borrow_mut();
        let mut globals = dg.as_mut().unwrap();
        let mutations = globals.vdom.rebuild();

        serialized_templates = serde_json::to_string(&mutations.templates).unwrap();
        serialized_edits = serde_json::to_string(&mutations.edits).unwrap();
        
    });

    (serialized_templates, serialized_edits)
}

pub fn virtual_dom_handle_event(event_type: &str, element_id: ElementId) {
    DIOXUS_GLOBALS.with(|cell| {
        let mut dg = cell.borrow_mut();
        let mut dom = &mut dg.as_mut().unwrap().vdom;

        dom.handle_event(event_type, Rc::new(MouseData::default()), ElementId(2), false);
    });
}

#[derive(Default)]
pub struct Config;

pub fn launch(app: Component<()>) {
    launch_cfg(app, Config)
}

pub fn launch_cfg(app: Component<()>, cfg: Config) {
    launch_cfg_with_props(app, (), cfg)
}

pub fn launch_cfg_with_props<Props: 'static>(
    app: Component<Props>,
    props: Props,
    cfg: Config,
) {
    let mut globals = DioxusGlobals {
        vdom: VirtualDom::new_with_props(app, props),
        cfg,
    };
    DIOXUS_GLOBALS.with(move |dg| *dg.borrow_mut() = Some(globals));

    makepad_app::app_main();
}
