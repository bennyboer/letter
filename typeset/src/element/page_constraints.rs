use unit::Distance;

use super::Size;

pub struct PageConstraints {
    pub size: Size,
    pub top: Distance,
    pub bottom: Distance,
    pub left: Distance,
    pub right: Distance,
}
