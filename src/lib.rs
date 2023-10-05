use dioxus::core::{Component, VirtualDom};
mod makepad;
mod virtual_dom;

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
    _cfg: Config,
) {
    let vdom = VirtualDom::new_with_props(app, props);
    virtual_dom::init(vdom);

    makepad::app::app_main();
}
