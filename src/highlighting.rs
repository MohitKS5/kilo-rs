use termion::color;

#[derive(Default, Debug, PartialEq)]
pub enum Class {
    #[default]
    None,
    Number,
}

impl Class {
    pub fn color(&self) -> impl color::Color {
        match self {
            Class::Number => color::Rgb(220, 163, 163),
            _ => color::Rgb(255, 255, 255),
        }
    }
}
