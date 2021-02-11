use super::*;
pub enum Widget {
    Button,
    Slider,
}

pub trait Parameter {
    fn name(&self) -> String;
    fn default_user_value(&self) -> f64;
    fn widght(&self) -> Widget {
        Widget::Slider
    }
    fn make_value_converter(&self) -> Box<dyn ValueConverter> {
        Box::new(DefaultValueConverter::new())
    }
}

pub struct BoolParameter {
    pub name: String,
    pub default_user_value: bool,
}

impl BoolParameter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            default_user_value: false,
        }
    }

    pub fn default_user_value(mut self, default_value: bool) -> Self {
        self.default_user_value = default_value;
        self
    }
}

impl Parameter for BoolParameter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn default_user_value(&self) -> f64 {
        if self.default_user_value {
            1.0
        } else {
            0.0
        }
    }

    fn widght(&self) -> Widget {
        Widget::Button
    }
}

pub struct FloatParameter {
    pub name: String,
    pub unit: String,
    pub min_user_value: f64,
    pub max_user_value: f64,
    pub default_user_value: f64,
    pub value_converter_maker: fn(&FloatParameter) -> Box<dyn ValueConverter>,
}

impl FloatParameter {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            unit: String::default(),
            min_user_value: 0.0,
            max_user_value: 1.0,
            default_user_value: 0.0,
            value_converter_maker: linear_value_converter,
        }
    }

    pub fn unit(mut self, unit: &str) -> Self {
        self.unit = unit.to_string();
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min_user_value = min;
        self.max_user_value = max;
        self
    }

    pub fn default_user_value(mut self, default: f64) -> Self {
        self.default_user_value = default;
        self
    }

    pub fn value_converter(
        mut self,
        converter: fn(&FloatParameter) -> Box<dyn ValueConverter>,
    ) -> Self {
        self.value_converter_maker = converter;
        self
    }
}

impl Parameter for FloatParameter {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn default_user_value(&self) -> f64 {
        self.default_user_value
    }

    fn make_value_converter(&self) -> Box<dyn ValueConverter> {
        (self.value_converter_maker)(self)
    }
}

pub fn linear_value_converter(parameter: &FloatParameter) -> Box<dyn ValueConverter> {
    Box::new(LinearValueConverter::new(
        parameter.min_user_value,
        parameter.max_user_value,
    ))
}

pub trait ParameterProvider {
    fn parameter_count() -> usize;
    fn parameter(id: usize) -> Box<dyn Parameter>;
}
