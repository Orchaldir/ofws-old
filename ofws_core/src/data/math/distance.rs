use num_integer::Roots;

/// Returns distance between 2 points in 2d space.
///
/// ```
///# use ofws_core::data::math::distance::calculate_distance;
///
/// assert_eq!(calculate_distance(0, 0, 0, 0), 0);
/// assert_eq!(calculate_distance(0, 0, 3, 4), 5);
/// assert_eq!(calculate_distance(3, 4, 0, 0), 5);
/// ```
///
/// # Panics
///
/// Panics if the distance is greater than the possible maximum.
///
/// ```should_panic
///# use ofws_core::data::math::distance::calculate_distance;
///
/// calculate_distance(0, 0, u32::max_value(), u32::max_value());
/// ```
pub fn calculate_distance(x0: u32, y0: u32, x1: u32, y1: u32) -> u32 {
    let diff_x = abs_diff(x0, x1);
    let diff_y = abs_diff(y0, y1);
    (diff_x.pow(2) + diff_y.pow(2)).sqrt()
}

/// Returns the absolute difference between 2 unsigned integers.
///
/// ```
///# use ofws_core::data::math::distance::abs_diff;
///
/// assert_eq!(abs_diff(0, 0), 0);
/// assert_eq!(abs_diff(10, 6), 4);
/// assert_eq!(abs_diff(6, 10), 4);
/// assert_eq!(abs_diff(u32::max_value(), 0), u32::max_value());
/// assert_eq!(abs_diff(0, u32::max_value()), u32::max_value());
/// ```
pub fn abs_diff(a: u32, b: u32) -> u32 {
    if a < b {
        b - a
    } else {
        a - b
    }
}

/// Returns the absolute difference between 2 unsigned integers.
///
/// ```
///# use ofws_core::data::math::distance::is_close;
///
/// assert!(is_close(0.0, 0.0, 0.001));
/// assert!(is_close(0.0, 0.1, 0.15));
/// assert!(!is_close(0.0, 0.2, 0.15));
/// assert!(is_close(5.0, 4.9, 0.2));
/// assert!(!is_close(5.0, 4.0, 0.2));
/// ```
pub fn is_close(a: f32, b: f32, epsilon: f32) -> bool {
    if a < b {
        b - a < epsilon
    } else {
        a - b < epsilon
    }
}
