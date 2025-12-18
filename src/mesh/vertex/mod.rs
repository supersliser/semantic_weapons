use std::ops::Div;

use glam::{Vec2, Vec3, Vec4};

#[derive(PartialEq)]
pub struct Vertex {
    position: Vec3,
    normal: Vec3,
    uv: Vec2,
    color: Vec4,
}

impl Vertex {
    pub fn new(position: Vec3) -> Self {
        Self {
            position,
            normal: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::ZERO,
            color: Vec4::ONE,
        }
    }
    pub fn set_normal(&mut self, normal: Vec3) {
        self.normal = Vec3::normalize(normal);
    }
    fn set_uv(&mut self, uv: Vec2) {
        self.uv = Vec2::clamp(uv, Vec2::ZERO, Vec2::ONE);
    }
    pub fn set_color(&mut self, color: Vec4) {
        if color.x > 1.0 || color.y > 1.0 || color.z > 1.0 || color.w > 1.0 {
            self.color = Vec4::div(color, 255.0);
        } else {
            self.color = color;
        }
    }
    pub fn set_position(&mut self, position: Vec3) {
        self.position = position;
    }
    pub fn get_position(&self) -> Vec3 {
        self.position
    }
    pub fn get_normal(&self) -> Vec3 {
        self.normal
    }
    pub fn get_uv(&self) -> Vec2 {
        self.uv
    }
    pub fn get_color(&self) -> Vec4 {
        self.color
    }
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vec3::ZERO,
            normal: Vec3::new(0.0, 0.0, 1.0),
            uv: Vec2::ZERO,
            color: Vec4::ONE,
        }
    }
}
