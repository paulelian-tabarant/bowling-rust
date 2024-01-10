pub struct Bowling {
    rolls: Vec<i16>,
}

const TOTAL_PINS: i16 = 10;

impl Bowling {
    pub(crate) fn new() -> Bowling {
        return Bowling {
            rolls: Vec::new()
        };
    }

    pub fn roll(&mut self, pins_down: i16) {
        self.rolls.push(pins_down);
    }

    pub fn score(&self) -> i16 {
        let mut score = 0;

        for frame in 0..10 {
            let roll = frame * 2;

            if self.is_spare(roll) {
                score += self.rolls[roll + 2];
            }
            score += self.rolls[roll] + self.rolls[roll + 1];
        }

        return score;
    }

    fn is_spare(&self, roll: usize) -> bool {
        roll < self.rolls.len() - 1 && self.rolls[roll] + self.rolls[roll + 1] == TOTAL_PINS
    }
}
