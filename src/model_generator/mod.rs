use fidget::{Context, context::Tree, shapes::{Sphere, types::Vec3}};

use crate::weapon_params::*;

pub fn begin(parameters: WeaponParams) {
    let mut ctx = Context::new();
    let mut sphere = Tree::from(Sphere {center: Vec3 {x: 0.0, y: 0.0, z:0.0}, radius: 1.0});
    let node = ctx.import(&sphere);
    node.
    
}tree