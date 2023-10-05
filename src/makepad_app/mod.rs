
use makepad_widgets::*;
use crate::{rebuild_virtual_dom, virtual_dom_handle_event};

use dioxus::core::{ElementId, Template};
use dioxus::prelude::TemplateNode;

live_design! {
  import makepad_widgets::base::*;
  import makepad_widgets::theme_desktop_dark::*;

  App = {{App}} {
        ui: <Window> {
            root = <View> {
                width: Fill,
                height: Fill,
            }
        }

        view_template: <View> {
            flow: Down,
            width: Fill,
            height: Fill,
        }

        label_template: <Label> {
            text: "Hello, world!"
        }

        heading1_template: <Label> {
            draw_text: {
                text_style: { font_size: 20. },
            }
            text: "Hello, world!"
        }

        heading3_template: <Label> {
            draw_text: {
                text_style: { font_size: 16. },
            }
            text: "Hello, world!"
        }

        button_template: <Button> {
            text: "Click me!"
        }
    }
}

pub struct DioxusTemplate {
    makepad_el: LiveId,
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

        for button_listeners in self.dioxus_listeners.iter() {
            let button_id = button_listeners.makepad_el;
            if self.ui.button(&[button_id]).clicked(&actions) {
                virtual_dom_handle_event("click", button_listeners.dioxus_el);
            }
        }
    }
}

impl LiveHook for App {
    fn before_live_design(cx: &mut Cx) {
        makepad_widgets::live_design(cx);
    }

    fn after_apply(&mut self, cx: &mut Cx, from: ApplyFrom, _index: usize, _nodes: &[LiveNode]) {
        if from.is_from_doc() {
            let main_view = self.ui.view(id!(root));

            main_view.clear_children();
            self.dioxus_templates.clear();
            self.dioxus_listeners.clear();

            let (serialized_templates, serialized_edits) = rebuild_virtual_dom();

            let templates: Vec<Template> = serde_json::from_str(&serialized_templates).unwrap();
            let mutations: Vec<serde_json::Value> = serde_json::from_str(&serialized_edits).unwrap();

            for template in templates {
                for (idx, root) in template.roots.iter().enumerate() {
                    self.add_node_from_template_node(idx, cx, root, &main_view);
                }
            }

            for mutation in mutations {
                self.process_mutation(&mutation);
            }
        }
    }
}

impl App {
    fn add_node_from_template_node(&mut self, idx: usize, cx: &mut Cx, node: &TemplateNode, parent_view: &ViewRef) {
        let liveid = LiveId::from_str(&format!("widget{}", idx));
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
                            self.add_node_from_template_node(i, cx, child, &view);
                        }
                    },
                    "button" => {
                        let label = parent_view
                            .append_child(cx, liveid, self.button_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            label.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    &_ => ()
                }
            },
            TemplateNode::Text { .. } | TemplateNode::Dynamic { .. } | TemplateNode::DynamicText { .. } => ()
        }

        self.dioxus_templates.push(
            DioxusTemplate {
                makepad_el: liveid,
                dioxus_el: None,
            }
        );
    }

    fn process_mutation(&mut self, mutation: &serde_json::Value) {
        match mutation["type"].as_str().unwrap() {
            "AssignId" => {
                let index = mutation["path"][0].as_u64().unwrap() as usize;
                self.dioxus_templates[index].dioxus_el =
                    Some(ElementId(mutation["id"].as_u64().unwrap() as usize));
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
            _ => ()
        }
    }
}