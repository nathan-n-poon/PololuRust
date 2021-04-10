use std::cmp;

pub(crate) fn within_tolerance(actual: u16, expected: u16, tolerance: u16) -> bool {
    return cmp::max(expected-actual, actual-expected) < tolerance;
}
