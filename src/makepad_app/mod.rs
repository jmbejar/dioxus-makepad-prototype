
use makepad_widgets::*;
use crate::virtual_dom_rebuild;
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
    }
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
   
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        if let Event::Draw(event) = event {
            let mut cx = Cx2d::new(cx, event);
            return self.ui.draw_widget_all(&mut cx);
        }

        self.ui.handle_widget_event(cx, event);
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
            virtual_dom_rebuild(&mut |template| {
                for (idx, root) in template.roots.iter().enumerate() {
                    self.add_node_from_template_node(idx, cx, root, &main_view);
                }
            });
        }
    }
}

impl App {
    fn add_node_from_template_node(&mut self, idx: usize, cx: &mut Cx, node: &TemplateNode, parent_view: &ViewRef) {
        match node {
            TemplateNode::Element {
                tag,
                attrs,
                children,
                ..
            } => {
                match *tag {
                    "p" => {
                        let liveid = LiveId::from_str(&format!("widget{}", idx));
                        let label = parent_view
                            .append_child(cx, liveid, self.label_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            dbg!(&text);
                            label.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    "h1" => {
                        let liveid = LiveId::from_str(&format!("widget{}", idx));
                        let label = parent_view
                            .append_child(cx, liveid, self.heading1_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            dbg!(&text);
                            label.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    "h3" => {
                        let liveid = LiveId::from_str(&format!("widget{}", idx));
                        let label = parent_view
                            .append_child(cx, liveid, self.heading3_template)
                            .unwrap();

                        if let TemplateNode::Text { text } = &children[0] {
                            dbg!(&text);
                            label.apply_over(cx, live!{
                                text: (text.clone())
                            });
                        }
                    },
                    "div" => {
                        let liveid = LiveId::from_str(&format!("widget{}", idx));
                        parent_view
                            .append_child(cx, liveid, self.view_template)
                            .unwrap();
                        
                        let view = parent_view.view(&[liveid]);
                        for (i, child) in children.iter().enumerate() {
                            self.add_node_from_template_node(i, cx, child, &view);
                        }
                    },
                    &_ => ()
                }
            },
            TemplateNode::Text { .. } | TemplateNode::Dynamic { .. } | TemplateNode::DynamicText { .. } => ()
        }
    }
}