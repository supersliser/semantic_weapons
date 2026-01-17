use crate::model_generator::model_params::{self, ModelParams};

impl ModelParams {
    pub fn get_blade_scale_decrement(&self) -> f64 {
        (self.blade_scale_back_right_decrement
            + self.blade_scale_back_left_decrement
            + self.blade_scale_front_left_decrement
            + self.blade_scale_front_right_decrement)
            / 4.0
    }
    pub fn set_blade_scale_decrement(&mut self, input: f64) {
        self.blade_scale_back_left_decrement = input;
        self.blade_scale_back_right_decrement = input;
        self.blade_scale_front_left_decrement = input;
        self.blade_scale_front_right_decrement = input;
    }
}
