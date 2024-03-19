use crate::*;

pub enum Selector {
    Class(String),
    Id(String),
    Tag(String),
    Any,
}

impl ToString for Selector {
    fn to_string(&self) -> String {
        match self {
            Self::Class(x) => format!(".{x}"),
            Self::Id(x) => format!("#{x}"),
            Self::Tag(x) => x.clone(),
            Self::Any => String::new(),
        }
    }
}

pub enum Target {
    This,
    Closest(Selector),
    Find(Selector),
    Next(Selector),
    Previous(Selector),
    Selector(Selector),
}

impl ToString for Target {
    fn to_string(&self) -> String {
        match self {
            Self::This => "this".to_string(),
            Self::Closest(x) => format!("closest {}", x.to_string()),
            Self::Find(x) => format!("find {}", x.to_string()),
            Self::Next(x) => format!("next {}", x.to_string()),
            Self::Previous(x) => format!("previous {}", x.to_string()),
            Self::Selector(x) => format!("{}", x.to_string()),
        }
    }
}

pub enum Swap {
    Inner,
    Outer,
    BeforeBegin,
    AfterBegin,
    BeforeEnd,
    AfterEnd,
    Delete,
    None,
}

impl ToString for Swap {
    fn to_string(&self) -> String {
        match self {
            Self::Inner => "innerHTML".to_string(),
            Self::Outer => "outerHTML".to_string(),
            Self::BeforeBegin => "beforebegin".to_string(),
            Self::AfterBegin => "afterbegin".to_string(),
            Self::BeforeEnd => "beforeend".to_string(),
            Self::AfterEnd => "afterend".to_string(),
            Self::Delete => "delete".to_string(),
            Self::None => "none".to_string(),
        }
    }
}

pub trait Htmx {
    fn get(self, uri: &str) -> Self;
    fn post(self, uri: &str) -> Self;
    fn put(self, uri: &str) -> Self;
    fn patch(self, uri: &str) -> Self;
    fn delete(self, uri: &str) -> Self;

    fn target(self, target: Target) -> Self;
    fn swap(self, swap: Swap) -> Self;
    fn trigger(self, trigger: &str) -> Self;
}

impl Htmx for Element {
    fn get(mut self, uri: &str) -> Self {
        self.attrs.insert("hx-get".to_string(), uri.to_string());
        self
    }

    fn post(mut self, uri: &str) -> Self {
        self.attrs.insert("hx-post".to_string(), uri.to_string());
        self
    }

    fn put(mut self, uri: &str) -> Self {
        self.attrs.insert("hx-put".to_string(), uri.to_string());
        self
    }

    fn patch(mut self, uri: &str) -> Self {
        self.attrs.insert("hx-patch".to_string(), uri.to_string());
        self
    }

    fn delete(mut self, uri: &str) -> Self {
        self.attrs.insert("hx-delete".to_string(), uri.to_string());
        self
    }

    fn target(mut self, target: Target) -> Self {
        self.attrs
            .insert("hx-target".to_string(), target.to_string());
        self
    }

    fn swap(mut self, swap: Swap) -> Self {
        self.attrs.insert("hx-swap".to_string(), swap.to_string());
        self
    }
    fn trigger(mut self, trigger: &str) -> Self {
        self.attrs
            .insert("hx-trigger".to_string(), trigger.to_string());
        self
    }
}
