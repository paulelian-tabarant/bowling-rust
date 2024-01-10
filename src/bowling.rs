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
        let mut score = 0;

        for i in 0..self.rolls.len() {
            if i < self.rolls.len() - 1 && self.rolls[i] + self.rolls[i + 1] == 10 {
                score += self.rolls[i + 2];
            }
            score += self.rolls[i];
        }

        return score;
    }
}
