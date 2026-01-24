
use crate::{
    model_generator::model_params::ModelParams,
    weapon_params::{BladeLength, BladeWidth, Direction},
};

impl ModelParams {
    pub fn set_blade_length(&mut self, value: BladeLength) {
        match value {
            BladeLength::Short => {
                self.blade_height /= 3.0;
                self.blade_left_top_slope *= 2.0;
                self.blade_right_top_slope *= 2.0;
                self.blade_radius /= 3.0;
                self.blade_scale_back_left_decrement /= 2.0;
                self.blade_scale_back_right_decrement /= 2.0;
                self.blade_scale_front_left_decrement /= 2.0;
                self.blade_scale_front_right_decrement /= 2.0;
                self.blade_front_width /= 2.0;
                self.blade_back_width /= 2.0;
                self.handle_radius /= 4.0;
                self.handle_bottom_limit *= 1.5;
                self.pommel_extension /= 2.0;
                self.guard_front_stop /= 2.0;
                self.guard_back_stop /= 2.0;
                self.guard_left_stop /= 3.0;
                self.guard_right_stop /= 3.0;
                self.guard_x_offset /= 1.5;
                self.guard_z_offset /= 1.8;
                self.guard_z_scale /= 3.0;
                self.guard_effect_radius /= 5.0;
                self.guard_bottom *= 1.1;
            }
            BladeLength::Long => {
                self.blade_height *= 2.0;
                self.handle_radius *= 1.5;
                self.blade_scale_back_left_decrement *= 1.25;
                self.blade_scale_back_right_decrement *= 1.25;
                self.blade_scale_front_left_decrement *= 1.25;
                self.blade_scale_front_right_decrement *= 1.25;
                self.handle_bottom_limit *= -0.75;
            }
            BladeLength::Great => {
                self.blade_height *= 4.0;
                self.handle_radius *= 4.0;
                self.handle_bottom_limit *= 5.0;
                self.pommel_radius *= 2.0;
                self.pommel_extension *= 2.0;
                self.handle_bottom_limit *= -1.5;
                self.blade_left_top_slope = 0.0;
                self.blade_right_top_slope = 0.0;
            }
            _ => {}
        }
    }
    pub fn set_blade_width(&mut self, value: BladeWidth, direction: Direction) {
        match value {
            BladeWidth::Narrow => {
                self.blade_radius /= 3.0;
                self.blade_front_width /= 2.0;
                self.blade_back_width /= 2.0;
                self.blade_scale_back_left_decrement *= 2.0;
                self.blade_scale_back_right_decrement *= 2.0;
                self.blade_scale_front_left_decrement *= 2.0;
                self.blade_scale_front_right_decrement *= 2.0;
                self.blade_left_top_slope *= 2.0;
                self.blade_right_top_slope *= 2.0;
                self.guard_z_offset /= 1.0;
                self.guard_z_scale = 0.75;
                self.guard_x_scale = 0.75;
                self.guard_x_offset /= 1.0;
                self.guard_left_stop /= 3.5;
                self.guard_right_stop /= 3.5;
                self.guard_front_stop /= 1.5;
                self.guard_back_stop /= 1.5;
                self.guard_effect_radius /= 1.0;
                self.guard_bottom *= 1.15;
                self.handle_grip_offset = -1.0;
                self.handle_radius /= 2.0;
                self.pommel_radius *= 2.0;
            }
            BladeWidth::Wide => {
                self.blade_radius *= 5.0;
                self.blade_back_width *= 1.5;
                self.blade_front_width *= 1.5;
                self.blade_scale_back_left_decrement *= 4.0;
                self.blade_scale_back_right_decrement *= 4.0;
                self.blade_scale_front_left_decrement *= 4.0;
                self.blade_scale_front_right_decrement *= 4.0;
                self.blade_left_top_slope *= 1.25;
                self.blade_right_top_slope *= 1.25;
                self.guard_z_offset *= 1.0;
                self.guard_x_offset *= 1.5;
                self.guard_x_scale *= 1.5;
                self.guard_z_scale *= 1.5;
                self.guard_front_stop *= 2.5;
                self.guard_back_stop *= 2.5;
                self.guard_left_stop *= 2.0;
                self.guard_right_stop *= 2.0;
                self.handle_bottom_limit /= 1.8;
                self.pommel_radius /= 3.0;
                self.handle_grip_offset *= 3.0;

                match direction {
                    Direction::Left => {
                        self.blade_scale_front_right_decrement *= 4.0;
                        self.blade_scale_back_right_decrement *= 4.0;
                        self.blade_scale_back_left_decrement /= 4.0;
                        self.blade_scale_front_left_decrement /= 4.0;
                    }
                    Direction::Right => {
                        self.blade_scale_back_left_decrement *= 4.0;
                        self.blade_scale_front_left_decrement *= 4.0;
                        self.blade_scale_front_right_decrement /= 4.0;
                        self.blade_scale_back_right_decrement /= 4.0;
                    }
                    Direction::Central => {
                        self.blade_scale_back_left_decrement /= 2.0;
                        self.blade_scale_front_left_decrement /= 2.0;
                        self.blade_scale_back_right_decrement /= 2.0;
                        self.blade_scale_front_right_decrement /= 2.0;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn set_blade_count(&mut self, direction: Direction, value: u8) {
        match value {
            1 => match direction {
                Direction::Right => {
                    self.blade_left_top_slope = 0.0;
                    self.blade_right_top_slope *= 2.0;
                    self.blade_scale_front_left_decrement /= 10.0;
                    self.blade_scale_back_left_decrement /= 10.0;
                    self.blade_scale_back_right_decrement *= 3.0;
                    self.blade_scale_front_right_decrement *= 3.0;
                    self.blade_front_width /= 1.5;
                    self.blade_back_width /= 1.5;
                    self.guard_right_stop /= 1.5;
                    self.guard_left_stop /= 2.0;
                }
                Direction::Left => {
                    self.blade_left_top_slope *= 2.0;
                    self.blade_right_top_slope = 0.0;
                    self.blade_scale_front_left_decrement *= 3.0;
                    self.blade_scale_back_left_decrement *= 3.0;
                    self.blade_scale_back_right_decrement = 0.0;
                    self.blade_scale_front_right_decrement = 0.0;
                    self.blade_front_width /= 1.5;
                    self.blade_back_width /= 1.5;
                    self.guard_right_stop /= 2.0;
                    self.guard_left_stop /= 1.5;
                }
                _ => {}
            },
            4 => {
                self.v_mirrored = true;
                if self.handle_bottom_limit < 0.0 {
                    self.handle_bottom_limit *= 2.0;
                } else {
                    self.handle_bottom_limit *= 0.5;
                }
            }
            _ => {}
        }
    }

    pub fn set_has_guard(&mut self, value: bool) {
        if value {
        } else {
            self.guard_bottom = (2.0 as f64).powf(63.0);
        }
    }

    pub fn set_blade_curvature(&mut self, direction: Direction, value: u8) {
        match direction {
            Direction::Central => {
                self.blade_lean = 0.0;
                self.blade_curvature = 0.0;
            }
            Direction::Left => {
                self.blade_lean = 90.0 - value as f64;
                self.blade_curvature = -((value as f64 / 3.0) * 2.0);
            }
            Direction::Right => {
                self.blade_lean = 90.0 - value as f64;
                self.blade_curvature = (value as f64 / 3.0) * 2.0;
            }
        }
    }
}
