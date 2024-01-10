pub struct Bowling {
    rolls: Vec<i16>,
}

const TOTAL_PINS: i8 = 10;
const LAST_FRAME: i8 = 10;

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
        let mut score: i16 = 0;

        for frame in 0..LAST_FRAME {
            let roll = (frame * 2) as usize;

            if self.is_spare(roll) {
                score += self.rolls[roll + 2];
            }

            score += self.rolls[roll] + self.rolls[roll + 1];
        }

        return score;
    }

    fn is_spare(&self, roll: usize) -> bool {
        self.rolls[roll] + self.rolls[roll + 1] == TOTAL_PINS as i16
    }
}
