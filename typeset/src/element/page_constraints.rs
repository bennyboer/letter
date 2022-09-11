use super::Size;

pub struct PageConstraints {
    pub size: Size,
    pub top: f64, // TODO Replace usize by some kind of internal unit that is easily convertible to all kinds of units you might want to work with (mm, px, ..)
    pub bottom: f64,
    pub left: f64,
    pub right: f64,
}
