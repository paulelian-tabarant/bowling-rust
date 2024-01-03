pub struct Bowling {
    toto: i8
}

impl Bowling {
    pub(crate) fn new() -> Bowling {
        Bowling {
            toto: 0
        }
    }

    pub fn roll(&mut self, pins: i8) {
        self.toto = pins;
    }

    pub fn score(&self) -> i16 {
        return 0;
    }
}
