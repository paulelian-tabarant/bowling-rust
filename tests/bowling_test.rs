pub struct Bowling();

impl Bowling {
    pub fn roll(&self, pins: i8) {
    }

    pub fn score(&self) -> i16 {
        return 0;
    }
}

#[test]
fn score_is_zero_when_every_roll_is_a_gutter_ball() {
    let bowling = Bowling();

    for _ in 0..20 {
        bowling.roll(0);
    }

    assert_eq!(bowling.score(), 0);
}