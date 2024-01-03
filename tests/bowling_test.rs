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
    fn score_is_the_sum_of_every_roll_in_a_classic_game() {
        let mut bowling = Bowling::new();
        let one_pin = 1;

        for _ in 0..20 {
            bowling.roll(one_pin);
        }

        assert_eq!(bowling.score(), 20);
    }

    #[test]
    fn score_includes_next_roll_after_a_spare() {
        let mut bowling = Bowling::new();
        let next_roll = 3;

        bowling.roll(2);
        bowling.roll(8);
        bowling.roll(next_roll);
        for _ in 0..17 {
            bowling.roll(0);
        }

        assert_eq!(bowling.score(), (5 + 5 + next_roll) + next_roll);
    }
}