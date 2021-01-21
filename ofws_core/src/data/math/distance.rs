/// Returns the absolute difference between 2 unsigned integers.
/// ```
///# use ofws_core::data::math::distance::abs_diff;
///
/// assert_eq!(abs_diff(0, 0), 0);
/// assert_eq!(abs_diff(10, 6), 4);
/// assert_eq!(abs_diff(6, 10), 4);
/// assert_eq!(abs_diff(u32::max_value(), 0), u32::max_value());
/// assert_eq!(abs_diff(0, u32::max_value()), u32::max_value());
/// ```
pub fn abs_diff(a: u32, b:u32) -> u32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}