#[path = "../src/bowling.rs"]
mod bowling;

mod bowling_test {
    use crate::bowling::Bowling;

    #[test]
    fn score_is_zero_when_every_roll_is_a_gutter_ball() {
        let bowling = Bowling();

        for _ in 0..20 {
            bowling.roll(0);
        }

        assert_eq!(bowling.score(), 0);
    }
}