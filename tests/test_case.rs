use drift_macros::assert_no_slop;
use static_assertions::const_assert_eq;

#[assert_no_slop]
pub struct TestStruct {
    pub a: u64,
    pub b: u64,
}

fn main() {}