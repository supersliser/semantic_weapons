use crate::{model_generator::model_params::ModelParams, weapon_params::{BladeLength, BladeWidth}};

impl ModelParams {
    pub fn set_blade_length(&mut self, value: BladeLength) {
        match value {
            BladeLength::Short => {
                self.blade_height /= 3.0;
                self.blade_left_top_slope /= 4.0;
                self.blade_right_top_slope /= 4.0;
                self.handle_radius /= 4.0;
                self.handle_bottom_limit *= 2.0;
                self.pommel_extension /= 2.0;
                self.guard_front_stop /= 2.0;
                self.guard_back_stop /= 2.0;
                self.guard_left_stop /= 3.5;
                self.guard_right_stop /= 3.5;
                self.guard_x_offset /= 1.5;
                self.guard_z_offset /= 1.8;
                self.guard_z_scale /= 3.0;
                self.guard_effect_radius /= 5.0;
            }
            BladeLength::Long => {
                self.blade_height *= 2.0;
                self.handle_radius *= 1.5;
            }
            BladeLength::Great => {
                self.blade_height *= 4.0;
                self.handle_radius *= 2.0;
                self.handle_bottom_limit *= 1.75;
                self.pommel_radius *= 2.0;
                self.pommel_extension *= 2.0;
            }
            _ => {}
        }
    }

    pub fn set_blade_width(&mut self, value: BladeWidth) {
        match value {
            BladeWidth::Narrow => {
                self.blade_radius /= 4.0;
                self.blade_front_width /= 2.0;
                self.blade_back_width /= 2.0;
                self.blade_scale_decrement *= 2.0;
                self.blade_left_top_slope *= 2.0;
                self.blade_right_top_slope *= 2.0;
                self.guard_x_offset /= 2.0;
                self.guard_z_offset /= 2.0;
                self.guard_z_scale /= 3.0;
                self.guard_left_stop /= 1.5;
                self.guard_right_stop /= 1.5;
                self.guard_effect_radius /= 1.0;
            }
            BladeWidth::Wide => {
                self.blade_radius *= 2.0;
                self.blade_back_width *= 1.5;
                self.blade_front_width *= 1.5;
                self.blade_scale_decrement /= 4.0;
                self.blade_left_top_slope *= 1.25;
                self.blade_right_top_slope *= 1.25;
                self.guard_z_offset *= 1.5;
                self.guard_front_stop *= 1.5;
                self.guard_back_stop *= 1.5;
                self.guard_left_stop *= 2.0;
                self.guard_right_stop *= 2.0;
                self.guard_effect_radius /= 1.0;
            }
            _ =>{}
        }
    }
}
