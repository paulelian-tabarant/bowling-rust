#[path = "../src/bowling.rs"]
mod bowling;

mod bowling_test {
    use crate::bowling::Bowling;

    #[test]
    fn score_is_zero_when_every_roll_is_a_gutter_ball() {
        let mut bowling = Bowling::new();
        let gutter_ball = 0;

        for _ in 0..20 {
            bowling.roll(gutter_ball);
        }

        assert_eq!(bowling.score(), 0);
    }

    #[test]
    fn score_is_twenty_when_every_roll_is_a_one() {
        let mut bowling = Bowling::new();
        let one_pin = 1;

        for _ in 0..20 {
            bowling.roll(one_pin);
        }

        assert_eq!(bowling.score(), 20);
    }

    // test the same with twos
    #[test]
    fn score_is_forty_when_every_roll_is_a_two() {
        let mut bowling = Bowling::new();
        let two_pins = 2;

        for _ in 0..20 {
            bowling.roll(two_pins);
        }

        assert_eq!(bowling.score(), 40);
    }
}