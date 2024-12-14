use crate::core::component::{Component, Element, ElementKind, Message};

pub struct Button {
    pub label: String,
    on_click: Option<Box<dyn Fn()>>,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Button {
            label: label.to_string(),
            on_click: None,
        }
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl Component for Button {
    fn view(&self) -> Element {
        Element {
            kind: ElementKind::Button,
            children: vec![],
            style: Default::default(),
        }
    }

    fn update(&mut self, message: Message) {
        if let Message::Click = message {
            if let Some(handler) = &self.on_click {
                handler();
            }
        }
    }
}
