pub mod circular;
pub mod linear;

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
