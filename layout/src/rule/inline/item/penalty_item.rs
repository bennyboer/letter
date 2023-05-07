use unit::Distance;

pub(crate) type Penalty = i32;

#[derive(Debug)]
pub(crate) struct PenaltyItem {
    width: Distance,
    penalty: Penalty,
    flagged: bool,
}

impl PenaltyItem {
    pub fn new(width: Distance, penalty: Penalty, flagged: bool) -> Self {
        Self {
            width,
            penalty,
            flagged,
        }
    }

    pub fn width(&self) -> Distance {
        self.width
    }

    pub fn penalty(&self) -> Penalty {
        self.penalty
    }

    pub fn flagged(&self) -> bool {
        self.flagged
    }
}
