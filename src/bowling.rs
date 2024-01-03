pub struct Bowling {
    score: i16
}

impl Bowling {
    pub(crate) fn new() -> Bowling {
        return Bowling {
            score: 0
        }
    }

    pub fn roll(&mut self, pins_down: i16) {
        self.score += pins_down;
    }

    pub fn score(&self) -> i16 {
        return self.score;
    }
}
