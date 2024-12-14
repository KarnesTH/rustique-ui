pub trait Component {
    fn view(&self) -> Element;
    fn update(&mut self, message: Message);
}

pub struct Element {
    pub kind: ElementKind,
    pub children: Vec<Element>,
    pub style: Style,
}

pub enum ElementKind {
    Container,
    Button,
    Label,
}

pub struct Style {}

pub enum Message {
    Click,
}
