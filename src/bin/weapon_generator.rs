use fidget::{
    context::Tree,
    jit::JitShape,
    mesh::{Octree, Settings},
};
use nalgebra::Matrix4;
use semantic_weapons::model_generator::{create_basic_weapon, model_params::ModelParams};

use std::env;

fn main() {

    let mut quality: u8 = 0;


    let args: Vec<String> = env::args().collect();

    if args.iter().count() <= 1 {
        panic!("Please make sure to provide arguements")
    }

    for arg in 0..args.iter().count()-1 {
        match args[arg].as_str() {
            "-quality" => {
                if arg + 1 > args.iter().count() {
                    panic!("Please provide a quality number, valid numbers are between 1 and 255 inclusively")
                }
                if args[arg+1].parse::<u8>().is_err() || args[arg+1].parse::<u8>().ok() == Some(0) {
                    panic!("Invalid input: Please provide a quality number between 1 and 255 inclusively")
                }
                quality = args[arg+1].parse::<u8>().unwrap();
            }
            _=> {continue;}
        }
    }

    let mut params = ModelParams::default();
    params.set_blade_length(semantic_weapons::weapon_params::BladeLength::Long);
    params.set_blade_width(semantic_weapons::weapon_params::BladeWidth::Standard);
    params.set_blade_count(4);
    let sdf = create_basic_weapon(Tree::x(), Tree::y(), Tree::z(), params);
    let world_to_model = Matrix4::new_translation(&nalgebra::Vector3::new(0.0, 0.0, 0.0))
        * Matrix4::new_scaling(((params.blade_height + params.blade_bottom) * (params.v_mirrored as i32 as f64 * 2.0)) as f32);
    let shape = JitShape::from(sdf);
    let settings = Settings {
        depth: quality,
        world_to_model: world_to_model,
        ..Default::default()
    };
    let o = Octree::build(&shape, &settings).unwrap();
    let mesh = o.walk_dual();
    let mut f = std::fs::File::create("out.stl").unwrap();
    mesh.write_stl(&mut f).unwrap();
}
