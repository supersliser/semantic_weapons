use rand::RngExt;

use crate::{
    model_generator::model_params::ModelParams,
    weapon_params::{BladeLength, BladeType, BladeWidth, Direction, GuardCoverage, GuardPlateShape, HandleLength},
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
                self.guard_left_stop /= 2.5;
                self.guard_right_stop /= 2.5;
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
                        self.blade_scale_front_right_decrement *= 2.0;
                        self.blade_scale_back_right_decrement *= 2.0;
                        self.blade_scale_back_left_decrement /= 2.0;
                        self.blade_scale_front_left_decrement /= 2.0;
                    }
                    Direction::Right => {
                        self.blade_scale_back_left_decrement *= 2.0;
                        self.blade_scale_front_left_decrement *= 2.0;
                        self.blade_scale_front_right_decrement /= 2.0;
                        self.blade_scale_back_right_decrement /= 2.0;
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

    pub fn set_blade_thickness(&mut self, value: f64) {
        let clamped = value.clamp(0.0, 1.0);
        let thickness_scale = clamped * 2.0;
        self.blade_radius *= thickness_scale;
        self.blade_front_width *= thickness_scale;
        self.blade_back_width *= thickness_scale;
    }

    pub fn set_blade_count(&mut self, direction: Direction, value: u8) {
        match value {
            1 => match direction {
                Direction::Right => {
                    self.blade_left_top_slope = 0.0;
                    self.blade_right_top_slope *= 2.0;
                    self.blade_scale_front_left_decrement *= 2.0;
                    self.blade_scale_back_left_decrement *= 2.0;
                    self.blade_scale_back_right_decrement *= -1.5;
                    self.blade_scale_front_right_decrement *= -1.5;
                    self.blade_front_width /= 1.5;
                    self.blade_back_width /= 1.5;
                    self.guard_right_stop /= 1.5;
                    self.guard_left_stop /= 2.0;
                }
                Direction::Left => {
                    self.blade_left_top_slope *= 2.0;
                    self.blade_right_top_slope = 0.0;
                    self.blade_scale_front_left_decrement *= -1.5;
                    self.blade_scale_back_left_decrement *= -1.5;
                    self.blade_scale_back_right_decrement *= 2.0;
                    self.blade_scale_front_right_decrement *= 2.0;
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
        self.has_guard = value;
        self.has_guard_bar = value
    }

    /// Adjusts guard coverage from minimal (Open) to complete (Enclosed).
    /// 
    /// Guard structure mechanics:
    ///   - Guard bar: looping structural bar for hand/finger protection
    ///   - Guard effect cylinders: 4 positioned at corners for curved plate closure
    ///
    /// Coverage spectrum reflects the defense strategy:
    ///   Open      → minimal bar, minimal corner cylinders
    ///   Bar       → large structural bar, small corner cylinders
    ///   Semi      → thickened bar + moderate corner cylinders
    ///   Plate     → no bar, prominent corner cylinders (curved sides are defense)
    ///   Shell     → no bar, very large corner cylinders (protective shell)
    ///   Complex   → both large bar + large corner cylinders (baroque ornate)
    ///   Enclosed  → no bar, maximum corner cylinders (full wrapping protection)
    pub fn set_guard_plate_shape(&mut self, shape: GuardPlateShape) {
        self.guard_plate_shape = shape;
        // Reset curvature to default for new shape
        self.guard_plate_curvature = 1.0;
    }

    /// Adjusts guard plate curvature strength (1.0 = normal, 0.1 = shallow, 3.0 = deep)
    pub fn set_guard_plate_curvature(&mut self, value: f64) {
        // Clamp between 0.1 and 3.0 for reasonable curvature amounts
        self.guard_plate_curvature = value.max(0.1).min(3.0);
    }

    pub fn guard_coverage(&mut self, value: GuardCoverage) {
        match value {
            GuardCoverage::Open => {
                // Minimal guard: thin bar, tiny corner cylinders
                self.has_guard_bar = false;
                self.guard_front_stop *= 0.4;
                self.guard_back_stop *= 0.4;
                self.guard_left_stop *= 0.45;
                self.guard_right_stop *= 0.45;
                self.guard_effect_radius *= 0.5;
                self.guard_x_scale *= 0.6;
                self.guard_z_scale *= 0.6;
                self.guard_bar_thickness *= 0.6;
                self.guard_bar_height *= 0.5;
            }
            GuardCoverage::Bar => {
                // Bar-dominant hand protection: large looping bar, minimal corner cylinders
                self.has_guard_bar = true;
                self.guard_front_stop *= 0.55;
                self.guard_back_stop *= 0.55;
                self.guard_left_stop *= 0.6;
                self.guard_right_stop *= 0.6;
                // Corner cylinders minimized since bar is primary defense
                self.guard_effect_radius *= 0.65;
                self.guard_x_scale *= 0.8;
                self.guard_z_scale *= 0.8;
                // Large structural bar for gripping hand
                self.guard_bar_radius *= 1.6;
                self.guard_bar_x_scale *= 1.4;
                self.guard_bar_z_scale *= 1.4;
                self.guard_bar_left_offset *= 1.3;
                self.guard_bar_right_offset *= 1.3;
                self.guard_bar_front_offset *= 1.1;
                self.guard_bar_back_offset *= 1.1;
                self.guard_bar_thickness *= 2.2;
                self.guard_bar_height *= 1.8;
            }
            GuardCoverage::SemiEnclosed => {
                // Medium: balance between bar structure and corner cylinder protection
                self.has_guard_bar = true;
                self.guard_front_stop *= 1.1;
                self.guard_back_stop *= 1.1;
                self.guard_left_stop *= 1.15;
                self.guard_right_stop *= 1.15;
                // Moderate corner cylinders
                self.guard_effect_radius *= 1.1;
                self.guard_x_offset *= 1.05;
                self.guard_z_offset *= 1.05;
                self.guard_x_scale *= 1.0;
                self.guard_z_scale *= 1.0;
                // Moderate bar
                self.guard_bar_radius *= 1.15;
                self.guard_bar_x_scale *= 1.05;
                self.guard_bar_z_scale *= 1.05;
                self.guard_bar_left_offset *= 1.1;
                self.guard_bar_right_offset *= 1.1;
                self.guard_bar_front_offset *= 1.0;
                self.guard_bar_back_offset *= 1.0;
                self.guard_bar_thickness *= 1.4;
                self.guard_bar_height *= 1.2;
            }
            GuardCoverage::Plate => {
                // Pure plate guard: no bar, corner cylinders are primary defense
                self.has_guard_bar = false;
                self.guard_front_stop *= 1.25;
                self.guard_back_stop *= 1.25;
                self.guard_left_stop *= 1.3;
                self.guard_right_stop *= 1.3;
                // Prominent corner cylinders for curved plate sides
                self.guard_effect_radius *= 1.35;
                self.guard_x_offset *= 1.15;
                self.guard_z_offset *= 1.15;
                self.guard_x_scale *= 1.05;
                self.guard_z_scale *= 1.05;
                self.guard_bar_thickness *= 0.1;  // Nearly invisible
                self.guard_bar_height *= 0.8;
            }
            GuardCoverage::Shell => {
                // Curved shell: very large corner cylinders create protective shell
                self.has_guard_bar = false;
                self.guard_front_stop *= 1.5;
                self.guard_back_stop *= 1.5;
                // self.guard_left_stop *= 1.6;
                // self.guard_right_stop *= 1.6;
                // Large corner cylinders for shell curvature
                self.guard_effect_radius *= 1.7;
                self.guard_x_offset *= 1.5;
                self.guard_z_offset *= 1.5;
                self.guard_x_scale *= 1.1;
                self.guard_z_scale *= 1.1;
                self.guard_bar_thickness *= 0.05;  // Nearly invisible
                self.guard_bar_height *= 0.6;
            }
            GuardCoverage::Complex => {
                // Ornate baroque: both large bar + large corner cylinders
                self.has_guard_bar = true;
                self.guard_front_stop *= 1.6;
                self.guard_back_stop *= 1.6;
                self.guard_left_stop *= 1.75;
                self.guard_right_stop *= 1.75;
                // Large corner cylinders for decoration
                self.guard_effect_radius *= 1.5;
                self.guard_x_offset *= 1.4;
                self.guard_z_offset *= 1.4;
                self.guard_x_scale *= 1.0;
                self.guard_z_scale *= 1.0;
                // Large ornate bar
                self.guard_bar_radius *= 1.7;
                self.guard_bar_x_scale *= 1.5;
                self.guard_bar_z_scale *= 1.5;
                self.guard_bar_left_offset *= 1.6;
                self.guard_bar_right_offset *= 1.6;
                self.guard_bar_front_offset *= 1.4;
                self.guard_bar_back_offset *= 1.4;
                self.guard_bar_thickness *= 2.0;
                // self.guard_bar_height *= 3.0;
            }
            GuardCoverage::Enclosed => {
                // Maximum protection: no bar, maximum corner cylinders wrap the hand
                self.has_guard_bar = false;
                self.guard_front_stop *= 1.8;
                self.guard_back_stop *= 1.8;
                self.guard_left_stop *= 2.0;
                self.guard_right_stop *= 2.0;
                // Maximum corner cylinders for full wrapping
                self.guard_effect_radius *= 2.0;
                self.guard_x_offset *= 1.7;
                self.guard_z_offset *= 1.7;
                self.guard_x_scale *= 1.3;
                self.guard_z_scale *= 1.3;
                self.guard_bar_thickness *= 0.01;  // Nearly invisible
                self.guard_bar_height *= 0.4;
            }
        }
    }
    /// Generated by Github Copilot

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

    pub fn set_handle_length(&mut self, value: HandleLength) {
        match value {
            HandleLength::Dagger => {
                self.handle_bottom_limit *= 1.75;
                self.pommel_radius /= 6.0;
                self.pommel_extension /= 3.0;
            }
            HandleLength::OneHanded => {
                self.handle_bottom_limit *= 1.25;
                self.pommel_radius /= 4.0;
            }
            HandleLength::TwoHanded => {
                self.handle_bottom_limit *= 0.75;
                self.pommel_radius /= 3.0;
                self.pommel_extension *= 2.0;
            }
            HandleLength::ForearmLength => {
                self.handle_bottom_limit *= -1.0;
            }
            HandleLength::Polearm => {
                self.handle_bottom_limit *= -10.0;
            }
        }
    }

    pub fn set_blade_type(&mut self, value: BladeType, direction: Direction) {
        match value {
            BladeType::Dull => {
                self.blade_scale_back_left_decrement = 0.0;
                self.blade_scale_back_right_decrement = 0.0;
                self.blade_scale_front_left_decrement = 0.0;
                self.blade_scale_front_right_decrement = 0.0;
            }
            BladeType::Spikey => match direction {
                Direction::Left => {
                    self.blade_spiked_left = true;
                }
                Direction::Right => {
                    self.blade_spiked_right = true;
                }
                Direction::Central => {
                    self.blade_spiked_left = true;
                    self.blade_spiked_right = true;
                }
            },
            BladeType::Serated => match direction {
                Direction::Left => {
                    self.blade_serated_left = true;
                }
                Direction::Right => {
                    self.blade_serated_right = true;
                }
                Direction::Central => {
                    self.blade_serated_left = true;
                    self.blade_serated_right = true;
                }
            },
            BladeType::SpikeyAndSerated => {
                let mut rand = rand::rng();
                if rand.random_bool(0.5) {
                    self.blade_serated_left = true;
                    self.blade_spiked_right = true;
                } else {
                    self.blade_spiked_left = true;
                    self.blade_serated_right = true;
                }
            }
            BladeType::Sharp => {}
        }
    }
}
