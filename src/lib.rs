pub mod htmx;
pub mod style;

use std::collections::HashMap;

pub enum Child {
    Text(String),
    Element(Box<Element>),
}

pub struct Element {
    tag: String,
    attrs: HashMap<String, String>,
    children: Vec<Child>,
}

impl Element {
    pub fn new<C: Children, T: ToString>(tag: T, children: C) -> Element {
        Element {
            tag: tag.to_string(),
            attrs: HashMap::new(),
            children: children.children(),
        }
    }

    pub fn new_content<A: ToString, B: ToString>(tag: A, text: B) -> Element {
        Element {
            tag: tag.to_string(),
            attrs: HashMap::new(),
            children: vec![Child::Text(text.to_string())],
        }
    }

    pub fn render(self) -> String {
        let inline = match self.children.get(0) {
            Some(Child::Text(_)) => self.children.len() == 1,
            None => true,
            _ => false
        };

        let mut attrs = self
            .attrs
            .into_iter()
            .map(|(k, v)| format!("{k}=\"{v}\""))
            .collect::<Vec<_>>()
            .join(" ");
        if !attrs.is_empty() {
            attrs = format!(" {attrs}");
        }

        let children = self
            .children
            .into_iter()
            .map(|child| match child {
                Child::Text(text) => text,
                Child::Element(element) => element.render(),
            })
            .collect::<Vec<_>>()
            .join("\n");

        if inline {
            format!("<{0}{1}>{2}</{0}>", self.tag, attrs, children)
        } else {
            format!(
                "<{0}{1}>\n{2}\n</{0}>",
                self.tag,
                attrs,
                children
                    .split("\n")
                    .map(|x| format!("  {x}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        }
    }
}

pub trait Children {
    fn children(self) -> Vec<Child>;
}

impl Children for Element {
    fn children(self) -> Vec<Child> {
        vec![Child::Element(Box::new(self))]
    }
}

impl Children for (Element, Element) {
    fn children(self) -> Vec<Child> {
        vec![
            Child::Element(Box::new(self.0)),
            Child::Element(Box::new(self.1)),
        ]
    }
}

impl Children for Vec<Element> {
    fn children(self) -> Vec<Child> {
        self.into_iter()
            .map(|element| Child::Element(Box::new(element)))
            .collect()
    }
}

macro_rules! parents {
    ($( $tag:ident ),*) => {
        $(
            pub fn $tag<T: Children>(children: T) -> Element {
                Element::new(stringify!($tag), children)
            }
        )*
    }
}

macro_rules! content {
    ($( $tag:ident ),*) => {
        $(
            pub fn $tag(text: &str) -> Element {
                Element::new_content(stringify!($tag), text)
            }
        )*
    }
}

parents!(html, body, head, div, ol, ul);
content!(p, li, title, button, h1, h2, h3, h4, h5, h6, input, label);

pub fn image<T: ToString>(src: T) -> Element {
    Element {
        tag: "image".to_string(),
        attrs: HashMap::from([("src".to_string(), src.to_string())]),
        children: Vec::new(),
    }
}

pub fn script<T: ToString>(src: T) -> Element {
    Element {
        tag: "script".to_string(),
        attrs: HashMap::from([("src".to_string(), src.to_string())]),
        children: Vec::new(),
    }
}

pub fn link<A: ToString, B: ToString>(href: A, rel: B) -> Element {
    Element {
        tag: "link".to_string(),
        attrs: HashMap::from([("href".to_string(), href.to_string()), ("rel".to_string(), rel.to_string())]),
        children: Vec::new(),
    }
}
