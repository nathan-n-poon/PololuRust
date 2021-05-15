use std::cmp;

pub(crate) fn within_tolerance(actual: u16, expected: u16, tolerance: u16) -> bool {
    if expected > actual
    {
        return (expected - actual) < tolerance;
    }
    return (actual - expected) < tolerance;
}
