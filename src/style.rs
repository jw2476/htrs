use crate::Element;

pub const fn px(length: u32) -> Length {
    Length::Px(length)
}

pub const fn rem(length: f32) -> Length {
    Length::Rem(length)
}

#[derive(Clone, Copy, Debug)]
pub enum Length {
    Px(u32),
    Rem(f32),
}

impl ToString for Length {
    fn to_string(&self) -> String {
        match self {
            Self::Px(x) => format!("{x}px"),
            Self::Rem(x) => format!("{x}rem"),
        }
    }
}

pub struct Edges<T> {
    top: T,
    bottom: T,
    left: T,
    right: T,
}

impl<T: Copy> Edges<T> {
    pub fn all(data: T) -> Self {
        Self {
            top: data,
            bottom: data,
            left: data,
            right: data,
        }
    }

    pub fn top(mut self, top: T) -> Self {
        self.top = top;
        self
    }

    pub fn bottom(mut self, bottom: T) -> Self {
        self.bottom = bottom;
        self
    }

    pub fn left(mut self, left: T) -> Self {
        self.left = left;
        self
    }

    pub fn right(mut self, right: T) -> Self {
        self.right = right;
        self
    }
}

impl<T: ToString> ToString for Edges<T> {
    fn to_string(&self) -> String {
        format!(
            "{} {} {} {}",
            self.top.to_string(),
            self.right.to_string(),
            self.bottom.to_string(),
            self.left.to_string()
        )
    }
}

pub enum Style {
    Padding(Edges<Length>),
    Margin(Edges<Length>),
    Border(Edges<Length>),
}

impl ToString for Style {
    fn to_string(&self) -> String {
        match self {
            Self::Padding(padding) => format!("padding: {};", padding.to_string()),
            Self::Margin(margin) => format!("margin: {};", margin.to_string()),
            Self::Border(border) => format!("border: {};", border.to_string()),
        }
    }
}

pub trait Stylable: Sized {
    fn style(self, style: Style) -> Self;

    fn padding(self, padding: Edges<Length>) -> Self {
        self.style(Style::Padding(padding))
    }

    fn margin(self, margin: Edges<Length>) -> Self {
        self.style(Style::Margin(margin))
    }

    fn border(self, border: Edges<Length>) -> Self {
        self.style(Style::Border(border))
    }
}

impl Stylable for Element {
    fn style(mut self, style: Style) -> Self {
        match self.attrs.get_mut("style") {
            Some(s) => *s = s.to_string() + &style.to_string(),
            None => {
                self.attrs.insert("style".to_string(), style.to_string());
            }
        }

        self
    }
}
