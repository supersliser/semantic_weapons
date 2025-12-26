pub struct ModelParams {
    pub pommel_shift: f64,
    pub pommel_extension: f64,
    pub pommel_radius: f64,

    pub handle_bottom_limit: f64,
    pub handle_radius: f64,
    pub handle_spiral_offset: f64,

    pub guard_bottom: f64,
    pub guard_front_stop: f64,
    pub guard_back_stop: f64,
    pub guard_left_stop: f64,
    pub guard_right_stop: f64,
    pub guard_effect_radius: f64,
    pub guard_x_offset: f64,
    pub guard_z_offset: f64,
    pub guard_z_scale: f64,

    pub blade_bottom: f64,
    pub blade_radius: f64,
    pub blade_front_width: f64,
    pub blade_back_width: f64,
    pub blade_left_top_slope: f64,
    pub blade_right_top_slope: f64,
    pub blade_scale: f64,
    pub blade_scale_decrement: f64,
    pub blade_height: f64,
}

impl Default for ModelParams {
    fn default() -> Self {
        ModelParams {
            pommel_shift: -0.5,
            pommel_extension: 4.5,
            pommel_radius: -3.5,
            handle_bottom_limit: 5.5,
            handle_radius: 0.5,
            handle_spiral_offset: 0.2,
            guard_bottom: 4.0,
            guard_front_stop: 2.0,
            guard_back_stop: 2.0,
            guard_left_stop: 5.0,
            guard_right_stop: 5.0,
            guard_effect_radius: 2.0,
            guard_x_offset: 2.0,
            guard_z_offset: 2.0,
            guard_z_scale: 3.0,
            blade_bottom: 15.0,
            blade_radius: 1.5,
            blade_front_width: 1.0,
            blade_back_width: 1.0,
            blade_left_top_slope: 2.0,
            blade_right_top_slope: 2.0,
            blade_scale: 2.0,
            blade_scale_decrement: 10.0,
            blade_height: 25.0,
        }
    }
}
