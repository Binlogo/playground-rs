pub trait ValueConverter {
    fn user_to_linear(&self, val: f64) -> f64;
    fn linear_to_user(&self, val: f64) -> f64;
}

pub struct DefaultValueConverter {}

impl DefaultValueConverter {
    pub fn new() -> Self {
        Self {}
    }
}

impl ValueConverter for DefaultValueConverter {
    fn user_to_linear(&self, val: f64) -> f64 {
        val
    }

    fn linear_to_user(&self, val: f64) -> f64 {
        val
    }
}

pub struct LinearValueConverter {
    pub min_user_value: f64,
    pub user_value_range: f64,
}

impl LinearValueConverter {
    pub fn new(min: f64, max: f64) -> Self {
        Self {
            min_user_value: min,
            user_value_range: max - min,
        }
    }
}

impl ValueConverter for LinearValueConverter {
    fn user_to_linear(&self, val: f64) -> f64 {
        (val - self.min_user_value) / self.user_value_range
    }

    fn linear_to_user(&self, val: f64) -> f64 {
        self.min_user_value + val * self.user_value_range
    }
}
