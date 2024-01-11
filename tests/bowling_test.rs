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

        assert_eq!(bowling.score(), (2 + 8 + next_roll) + next_roll);
    }

    #[test]
    fn score_does_not_consider_two_rolls_that_sum_ten_on_two_adjacent_frames_as_a_spare() {
        let mut bowling = Bowling::new();

        bowling.roll(2);
        bowling.roll(4);
        bowling.roll(6);
        bowling.roll(5);
        for _ in 0..16 {
            bowling.roll(0);
        }

        assert_eq!(bowling.score(), 2 + 4 + 6 + 5);
    }

    #[test]
    fn score_is_computed_correctly_even_when_frames_are_strikes() {
        let mut bowling = Bowling::new();

        bowling.roll(10);
        for _ in 0..18 {
            bowling.roll(0);
        }

        assert_eq!(bowling.score(), 10);
    }

    #[test]
    fn score_of_the_next_two_rolls_are_added_when_a_strike_is_made() {
        let mut bowling = Bowling::new();
        let next = 3;
        let next_next = 4;

        bowling.roll(10);
        bowling.roll(next);
        bowling.roll(next_next);
        for _ in 0..2 * 8 {
            bowling.roll(0);
        }

        assert_eq!(bowling.score(), (10 + next + next_next) + (next + next_next));
    }

    #[test]
    // acceptance test
    fn score_is_300_when_every_roll_is_a_strike() {
        let mut bowling = Bowling::new();

        for _ in 0..12 {
            bowling.roll(10);
        }

        assert_eq!(bowling.score(), 300);
    }
}
