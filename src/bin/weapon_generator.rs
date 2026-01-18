use fidget::{
    context::Tree,
    jit::JitShape,
    mesh::{Octree, Settings},
};
use nalgebra::Matrix4;
use semantic_weapons::model_generator::{create_basic_weapon, model_params::ModelParams};

use std::{env, time::SystemTime};

fn main() {

    let start = SystemTime::now();

    let mut quality: u8 = 0;

    let args: Vec<String> = env::args().collect();

    if args.iter().count() <= 1 {
        panic!("Please make sure to provide arguements")
    }

    for arg in 0..args.iter().count() - 1 {
        match args[arg].as_str() {
            "-quality" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a quality number, valid numbers are between 1 and 255 inclusively"
                    )
                }
                if args[arg + 1].parse::<u8>().is_err()
                    || args[arg + 1].parse::<u8>().ok() == Some(0)
                {
                    panic!(
                        "Invalid input: Please provide a quality number between 1 and 255 inclusively"
                    )
                }
                quality = args[arg + 1].parse::<u8>().unwrap();
            }
            _ => {
                continue;
            }
        }
    }

    let mut params = ModelParams::default();
    params.set_blade_length(semantic_weapons::weapon_params::BladeLength::Long);
    params.set_blade_width(semantic_weapons::weapon_params::BladeWidth::Standard);
    params.set_blade_count(1);
    params.set_has_guard(false);
    let sdf = create_basic_weapon(Tree::x(), Tree::y(), Tree::z(), params);

    //scaling shape to fit inside bounding box (1, 1, 1)
    let mut scaling = params.blade_height + params.blade_bottom;
    scaling += 10.0;
    if params.v_mirrored {
        scaling *= 2.0;
    }
    let world_to_model = Matrix4::new_translation(&nalgebra::Vector3::new(0.0, 0.0, 0.0))
        * Matrix4::new_scaling(scaling as f32);
    let shape = JitShape::from(sdf);

    //converting shape to mesh
    let settings = Settings {
        depth: quality,
        world_to_model: world_to_model,
        ..Default::default()
    };
    let o = Octree::build(&shape, &settings).unwrap();
    let mesh = o.walk_dual();

    //writing file
    let mut f = std::fs::File::create("out.stl").unwrap();
    mesh.write_stl(&mut f).unwrap();

    //speed should on average be on a 4x multiplier where x = detail-1. detail level 10 = ~4 secs
    println!("Mesh written after: {} secs", start.elapsed().unwrap().as_secs());
}
