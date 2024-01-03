#[path = "../src/bowling.rs"]
mod bowling;

mod bowling_test {
    use crate::bowling::Bowling;

    #[test]
    fn score_is_zero_when_every_roll_is_a_gutter_ball() {
        let mut  bowling = Bowling::new();

        for _ in 0..20 {
            bowling.roll(0);
        }

        assert_eq!(bowling.score(), 0);
    }

    // write the same test as above with a game of ones
    #[test]
    fn score_is_twenty_when_every_roll_is_a_one() {
        let mut bowling = Bowling::new();

        for _ in 0..20 {
            bowling.roll(1);
        }

        assert_eq!(bowling.score(), 20);
    }
}