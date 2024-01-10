pub struct Bowling {
    rolls: Vec<i16>
}

impl Bowling {
    pub(crate) fn new() -> Bowling {
        return Bowling {
            rolls: Vec::new()
        }
    }

    pub fn roll(&mut self, pins_down: i16) {
        self.rolls.push(pins_down);
    }

    pub fn score(&self) -> i16 {
        return self.rolls.iter().sum();
    }
}
