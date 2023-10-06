use dioxus::core::{VirtualDom, ElementId};
use dioxus::prelude::MouseData;
use std::cell::RefCell;
use std::rc::Rc;

thread_local! {
  static DIOXUS_VIRTUAL_DOM: RefCell<Option<VirtualDom>> = RefCell::new(None);
}

pub fn init(vdom: VirtualDom) {
  DIOXUS_VIRTUAL_DOM.with(move |cell| *cell.borrow_mut() = Some(vdom));
}

pub fn rebuild() -> (String, String) {
  let mut serialized_templates: String = String::new();
  let mut serialized_edits: String = String::new();

  DIOXUS_VIRTUAL_DOM.with(|cell| {
      let mut dg = cell.borrow_mut();
      let vdom = dg.as_mut().unwrap();
      let mutations = vdom.rebuild();

      serialized_templates = serde_json::to_string(&mutations.templates).unwrap();
      serialized_edits = serde_json::to_string(&mutations.edits).unwrap(); 
  });

  (serialized_templates, serialized_edits)
}

pub fn handle_event(event_type: &str, element_id: ElementId) {
  DIOXUS_VIRTUAL_DOM.with(|cell| {
      let mut dg = cell.borrow_mut();
      let vdom = &mut dg.as_mut().unwrap();

      vdom.handle_event(event_type, Rc::new(MouseData::default()), element_id, false);
  });
}

pub fn pending_mutations() -> String {
  DIOXUS_VIRTUAL_DOM.with(|cell| {
      let mut dg = cell.borrow_mut();
      let vdom = &mut dg.as_mut().unwrap();

      let mutations = vdom.render_immediate();
      serde_json::to_string(&mutations.edits).unwrap()
  })
}