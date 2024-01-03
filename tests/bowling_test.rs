use std::ffi::c_int;

struct Bowling();

impl Bowling {
    pub(crate) fn score(&self) -> c_int {
        todo!()
    }
}

#[test]
fn test_bowling_score_is_zero_when_bowling_object_is_created() {
    let bowling = Bowling();
    assert_eq!(bowling.score(), 0);
}