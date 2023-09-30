
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

        label_template: <Label> {
            text: "Hello, world!"
        }
    }
}

#[derive(Live)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[live]
    label_template: Option<LivePtr>,

    #[rust]
    templates: ComponentMap<usize, WidgetRef>,
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
            self.templates.clear();

            virtual_dom_rebuild(&mut |template| {
                dbg!(&template);

                for (idx, node) in template.roots.iter().enumerate() {
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
                                    let label = self.ui
                                        .view(id!(root))
                                        .append_child(cx, liveid, self.label_template)
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
                }
            });
        }
    }
}

app_main!(App);