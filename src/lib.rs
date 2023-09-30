use dioxus::core::{Component, VirtualDom, Template};
use std::cell::RefCell;

//use dioxus_native_core::prelude::RealDom;

mod makepad_app;

struct DioxusGlobals {
    vdom: VirtualDom,
    cfg: Config,
}

thread_local! {
    static DIOXUS_GLOBALS: RefCell<Option<DioxusGlobals>> = RefCell::new(None);
}

pub fn virtual_dom_rebuild(handle_template: &mut dyn FnMut(&Template)) {
    DIOXUS_GLOBALS.with(|cell| {
        let mut dg = cell.borrow_mut();
        let mutations = dg.as_mut().unwrap().vdom.rebuild();
        
        // let mut rdom = RealDom::new([]);
        // let mut dioxus_state = DioxusState::create(&mut rdom);
        // dioxus_state.apply_mutations(&mut rdom, mutations);

        for template in mutations.templates {
            //dbg!(&template);
            handle_template(&template);
        };

        for edit in mutations.edits {
            dbg!(&edit);
        };
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
