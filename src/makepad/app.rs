
use makepad_widgets::*;

use dioxus::core::{ElementId, Template};
use dioxus::prelude::{TemplateNode, TemplateAttribute};
use std::sync::atomic::{AtomicU64, Ordering};

static UNIQUE_WIDGET_LIVE_ID: AtomicU64 = AtomicU64::new(1);

live_design! {
    import makepad_widgets::base::*;
    import makepad_widgets::theme_desktop_dark::*;

    App = {{App}} {
        ui: <Window> {
            body = <View> {
                margin: {top: 50, left: 30, right: 30},
                align: {x: 0, y: 0},
                width: Fill,
                height: Fill,
            }
        }

        view_template: <View> {
            flow: Down,
            width: Fill,
            height: Fit,
        }

        label_template: <Label> {
            width: Fit,
        }

        heading1_template: <Label> {
            width: Fit,
            draw_text: {
                text_style: { font_size: 20. },
            }
        }

        heading3_template: <Label> {
            width: Fit,
            draw_text: {
                text_style: { font_size: 16. },
            }
        }

        button_template: <View> {
            width: Fill,
            height: Fit,
            button = <Button> {
                width: Fit,
            }
        }
    }
}

#[derive(Debug)]
pub struct DioxusTemplate {
    makepad_el: LiveId,
    dioxus_path: Vec<usize>,
    dioxus_el: Option<ElementId>,
}

#[derive(Debug)]
pub struct DioxusListener {
    name: String,
    makepad_el: LiveId,
    dioxus_el: ElementId,
}

app_main!(App);

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,

    // TODO For a quick prototype we are using a single template for all elements
    // and we rely on retained mode adding instances to a root view
    // Let's consider later if relying on inmediate mode is a better option

    #[live]
    view_template: Option<LivePtr>,
    #[live]
    label_template: Option<LivePtr>,
    #[live]
    heading1_template: Option<LivePtr>,
    #[live]
    heading3_template: Option<LivePtr>,
    #[live]
    button_template: Option<LivePtr>,

    #[rust]
    dioxus_templates: Vec<DioxusTemplate>,
    #[rust]
    dioxus_listeners: Vec<DioxusListener>, 
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            let mut cx = Cx2d::new(cx, event);
            return self.ui.draw_widget_all(&mut cx);
        }

        let actions = self.ui.handle_widget_event(cx, event);

        let mut was_event_handled = false;
        for button_listeners in self.dioxus_listeners.iter() {
            let button_id = button_listeners.makepad_el;
            if self.ui.view(&[button_id]).button(id!(button)).clicked(&actions) {
                was_event_handled = true;
                crate::virtual_dom::handle_event("click", button_listeners.dioxus_el);
            }
        }

        if was_event_handled {
            self.process_pending_mutations(cx);
        }
    }
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc() {
            let main_view = self.ui.view(id!(body));

            main_view.clear_children();
            self.dioxus_templates.clear();
            self.dioxus_listeners.clear();

            let (serialized_templates, serialized_edits) = crate::virtual_dom::rebuild();

            let templates: Vec<Template> = serde_json::from_str(&serialized_templates).unwrap();
            let mutations: Vec<serde_json::Value> = serde_json::from_str(&serialized_edits).unwrap();

            for template in templates {
                for (idx, root) in template.roots.iter().enumerate() {
                    self.add_node_from_template_node(vec![idx], cx, root, &main_view);
                }
            }

            for mutation in mutations {
                self.process_mutation(cx, &mutation);
            }
        }
    }
}

impl App {
    fn add_node_from_template_node(
        &mut self,
        path: Vec<usize>,
        cx: &mut Cx,
        node: &TemplateNode,
        parent_view: &ViewRef
    ) {
        // TODO Define a better way to generate ids
        let liveid = LiveId::from_str(&format!("widget{}", UNIQUE_WIDGET_LIVE_ID.fetch_add(1, Ordering::SeqCst)));
        match node {
            TemplateNode::Element {
                tag,
                attrs,
                children,
                ..
            } => {
                match *tag {
                    "p" => {
                        let label = parent_view
                            .append_child(cx, liveid, self.label_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            label.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    "h1" => {
                        let label = parent_view
                            .append_child(cx, liveid, self.heading1_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            label.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    "h3" => {
                        let label = parent_view
                            .append_child(cx, liveid, self.heading3_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            label.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    "div" => {
                        parent_view
                            .append_child(cx, liveid, self.view_template)
                            .unwrap();
                        
                        let view = parent_view.view(&[liveid]);
                        for (i, child) in children.iter().enumerate() {
                            let mut new_path = path.clone();
                            new_path.push(i);
                            self.add_node_from_template_node(new_path, cx, child, &view);
                        }
                    },
                    "button" => {
                        let elem = parent_view
                            .append_child(cx, liveid, self.button_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            let button = elem.button(id!(button));
                            button.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    &_ => ()
                }

                self.process_attrs(cx, liveid, attrs);
            },
            TemplateNode::Text { .. } | TemplateNode::Dynamic { .. } | TemplateNode::DynamicText { .. } => ()
        }

        self.dioxus_templates.push(
            DioxusTemplate {
                makepad_el: liveid,
                dioxus_path: path,
                dioxus_el: None,
            }
        );
    }

    fn process_mutation(&mut self, cx: &mut Cx, mutation: &serde_json::Value) {
        match mutation["type"].as_str().unwrap() {
            "AssignId" => {
                let path_array = mutation["path"].as_array().unwrap();
                let mut path: Vec<usize> = path_array.iter().map(|p| p.as_u64().unwrap() as usize).collect();

                // Reconcialiation of paths: this mutations does not include the root node index (0)
                path.insert(0, 0);

                if let Some(mut template) = self.dioxus_templates
                    .iter_mut()
                    .find(|t| t.dioxus_path == path)
                {
                    template.dioxus_el = Some(ElementId(mutation["id"].as_u64().unwrap() as usize));
                }
            },
            "NewEventListener" => {
                let id = ElementId(mutation["id"].as_u64().unwrap() as usize);
                let template = self.dioxus_templates.iter().find(|t| t.dioxus_el == Some(id)).unwrap();
                self.dioxus_listeners.push(
                    DioxusListener {
                        makepad_el: template.makepad_el,
                        dioxus_el: id,
                        name: mutation["name"].to_string(),
                    }
                );
            },
            "HydrateText" => {
                let path_array = mutation["path"].as_array().unwrap();
                let mut path: Vec<usize> = path_array.iter().map(|p| p.as_u64().unwrap() as usize).collect();

                // Reconcialiation of paths: this mutations does not include the root node index (0),
                // and also, it includes an extra 0 for the nested DynamicText node that we don't track
                path.insert(0, 0);
                path.pop();

                if let Some(mut template) = self.dioxus_templates
                    .iter_mut()
                    .find(|t| t.dioxus_path == path)
                {
                    // Store id because it is the reference of future SetText mutations
                    let id = ElementId(mutation["id"].as_u64().unwrap() as usize);
                    template.dioxus_el = Some(id);

                    let value = mutation["value"].as_str().unwrap().to_string();
                    self.ui.label(&[template.makepad_el]).apply_over(cx, live!{
                        text: (value)
                    });
                }
            },
            "SetText" => {
                let id = ElementId(mutation["id"].as_u64().unwrap() as usize);
                let template = self.dioxus_templates.iter().find(|t| t.dioxus_el == Some(id)).unwrap();
                let value = mutation["value"].as_str().unwrap().to_string();

                self.ui.label(&[template.makepad_el]).apply_over(cx, live!{
                    text: (value)
                });
            }
            _ => ()
        }
    }

    fn process_attrs(&mut self, cx: &mut Cx, liveid: LiveId, attrs: &[TemplateAttribute]) {
        for attr in attrs {
            match attr {
                TemplateAttribute::Static {
                    name,
                    value,
                    ..
                } => {
                    if *name == "style" {
                        self.process_style(cx, liveid, value);
                    }
                },
                &_ => ()
            }
        }
    }

    fn process_style(&mut self, cx: &mut Cx, liveid: LiveId, value: &str) {
        for style_attr in value.split(";") {
            let mut style_attr = style_attr.split(":");
            if style_attr.clone().count() != 2 {
                continue;
            }

            let name = style_attr.next().unwrap().trim();
            let value = style_attr.next().unwrap().trim();

            match name {
                "text-align" => {
                    match value {
                        "center" => {
                            self.ui.widget(&[liveid]).apply_over(cx, live!{
                                align: {x: 0.5}
                            });
                        },
                        &_ => ()
                    }
                },
                "margin-top" => {
                    let number = value
                        // TODO Remove suffix, but we should differentiate between px and other units
                        .trim_end_matches(char::is_alphabetic)
                        .parse::<u32>()
                        .unwrap();
                    self.ui.widget(&[liveid]).apply_over(cx, live!{
                        margin: {top: (number)}
                    })
                },
                &_ => ()
            }
        }
    }

    fn process_pending_mutations(&mut self, cx: &mut Cx) {
        let serialized_edits = crate::virtual_dom::pending_mutations();
        let mutations: Vec<serde_json::Value> = serde_json::from_str(&serialized_edits).unwrap();

        for mutation in &mutations {
            self.process_mutation(cx, &mutation);
        }

        if !mutations.is_empty() {
            self.ui.redraw(cx);
        }
    }
}