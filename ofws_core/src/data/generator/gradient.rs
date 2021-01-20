use crate::data::generator::Generator;

/// Returns a linear gradient between a start and an end value along the x-axis.
pub struct LinearGradientX {
    gradient: Gradient,
    start: u32,
}

impl LinearGradientX {
    pub fn new(value_start: u8, value_end: u8, start: u32, max_distance: u32) -> LinearGradientX {
        LinearGradientX {
            gradient: Gradient::new(value_start, value_end, max_distance),
            start,
        }
    }
}

impl Generator for LinearGradientX {
    /// Returns a linear gradient between a start and an end value along the x-axis.
    ///
    /// ```
    ///# use ofws_core::data::generator::Generator;
    ///# use ofws_core::data::generator::gradient::LinearGradientX;
    /// let generator = LinearGradientX::new(100, 200, 1000, 100);
    ///
    /// assert_eq!(generator.generate(   0,  0), 100);
    /// assert_eq!(generator.generate( 500,  0), 100);
    /// assert_eq!(generator.generate(1000,  0), 100);
    /// assert_eq!(generator.generate(1001,  5), 101);
    /// assert_eq!(generator.generate(1050, 10), 150);
    /// assert_eq!(generator.generate(1099, 15), 199);
    /// assert_eq!(generator.generate(1100, 20), 200);
    /// assert_eq!(generator.generate(1101, 20), 200);
    /// assert_eq!(generator.generate(1200, 20), 200);
    /// ```
    fn generate(&self, x: u32, _y: u32) -> u8 {
        if x < self.start {
            return self.gradient.value_start;
        }

        self.gradient.generate(x - self.start)
    }
}

/// Returns a linear gradient between a start and an end value along the y-axis.
pub struct LinearGradientY {
    gradient: Gradient,
    start: u32,
}

impl LinearGradientY {
    pub fn new(value_start: u8, value_end: u8, start: u32, max_distance: u32) -> LinearGradientY {
        LinearGradientY {
            gradient: Gradient::new(value_start, value_end, max_distance),
            start,
        }
    }
}

impl Generator for LinearGradientY {
    /// Returns a linear gradient between a start and an end value along the y-axis.
    ///
    /// ```
    ///# use ofws_core::data::generator::Generator;
    ///# use ofws_core::data::generator::gradient::LinearGradientY;
    /// let generator = LinearGradientY::new(100, 200, 1000, 100);
    ///
    /// assert_eq!(generator.generate( 0,    0), 100);
    /// assert_eq!(generator.generate( 0,  500), 100);
    /// assert_eq!(generator.generate( 0, 1000), 100);
    /// assert_eq!(generator.generate( 5, 1001), 101);
    /// assert_eq!(generator.generate(10, 1050), 150);
    /// assert_eq!(generator.generate(15, 1099), 199);
    /// assert_eq!(generator.generate(20, 1100), 200);
    /// assert_eq!(generator.generate(20, 1101), 200);
    /// assert_eq!(generator.generate(20, 1200), 200);
    /// ```
    fn generate(&self, _x: u32, y: u32) -> u8 {
        if y < self.start {
            return self.gradient.value_start;
        }

        self.gradient.generate(y - self.start)
    }
}

struct Gradient {
    value_start: u8,
    value_end: u8,
    max_distance: u32,
}

impl Gradient {
    pub fn new(value_start: u8, value_end: u8, max_distance: u32) -> Gradient {
        Gradient {
            value_start,
            value_end,
            max_distance,
        }
    }
}

impl Gradient {
    pub fn generate(&self, distance: u32) -> u8 {
        let factor = distance as f32 / self.max_distance as f32;

        println!("distance={} factor={}", distance, factor);

        if factor > 1.0 {
            return self.value_end;
        }

        if self.value_end >= self.value_start {
            let diff = (self.value_end - self.value_start) as f32;
            return self.value_start + (diff * factor) as u8;
        }

        let diff = (self.value_start - self.value_end) as f32;

        self.value_start - (diff * factor) as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_gradient_x_from_high_to_low() {
        let generator = LinearGradientX::new(150, 50, 100, 50);

        assert_eq!(generator.generate(0, 0), 150);
        assert_eq!(generator.generate(100, 0), 150);
        assert_eq!(generator.generate(101, 5), 148);
        assert_eq!(generator.generate(125, 10), 100);
        assert_eq!(generator.generate(149, 15), 52);
        assert_eq!(generator.generate(150, 20), 50);
        assert_eq!(generator.generate(151, 25), 50);
        assert_eq!(generator.generate(200, 15), 50);
    }
}
