use fidget::{
    context::Tree,
    jit::JitShape,
    mesh::{Octree, Settings},
};
use nalgebra::Matrix4;
use semantic_weapons::{
    model_generator::{create_basic_weapon, model_params::ModelParams},
    weapon_params::{BladeLength, BladeWidth, Direction},
};

use core::num;
use std::{env, time::SystemTime};

fn main() {
    let start = SystemTime::now();

    let mut quality: u8 = 10;
    let mut blade_length = BladeLength::Medium;
    let mut blade_width = BladeWidth::Standard;
    let mut bladed_edge_count: u8 = 2;
    let mut has_guard = true;
    let mut blade_direction = Direction::Central;
    let mut blade_angle = 0;

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
            "-blade-length" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a blade-length value, valid options are Short, Medium, Long and Great"
                    )
                }
                match args[arg + 1].to_lowercase().as_str() {
                    "short" => {
                        blade_length = BladeLength::Short;
                    }
                    "medium" => {
                        blade_length = BladeLength::Medium;
                    }
                    "long" => {
                        blade_length = BladeLength::Long;
                    }
                    "great" => {
                        blade_length = BladeLength::Great;
                    }
                    _ => {
                        panic!(
                            "Please provide a blade-length value, valid options are Short, Medium, Long and Great"
                        )
                    }
                }
            }
            "-blade-width" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a blade-width value, valid options are Narrow, Standard, Wide"
                    )
                }
                match args[arg + 1].to_lowercase().as_str() {
                    "narrow" => {
                        blade_width = BladeWidth::Narrow;
                    }
                    "standard" => {
                        blade_width = BladeWidth::Standard;
                    }
                    "wide" => {
                        blade_width = BladeWidth::Wide;
                    }
                    _ => {
                        panic!(
                            "Please provide a blade-width value, valid options are Narrow, Standard, Wide"
                        )
                    }
                }
            }
            "-bladed-edge-count" => {
                if arg + 1 > args.iter().count() {
                    panic!("Please provide an edge count, valid numbers are 1, 2 or 4")
                }
                if args[arg + 1].parse::<u8>().is_err()
                    || args[arg + 1].parse::<u8>().ok() < Some(1)
                    || args[arg + 1].parse::<u8>().ok() > Some(4)
                    || args[arg + 1].parse::<u8>().ok() == Some(3)
                {
                    panic!(
                        "Invalid input: Please provide a quality number between 1 and 255 inclusively"
                    )
                }
                bladed_edge_count = args[arg + 1].parse::<u8>().unwrap();
            }
            "-has-guard" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a boolean for if the weapon should have a guard, valid options are true or false"
                    )
                }
                match args[arg + 1].to_lowercase().as_str() {
                    "true" => {
                        has_guard = true;
                    }
                    "false" => {
                        has_guard = false;
                    }
                    _ => {
                        panic!(
                            "Please provide a boolean for if the weapon should have a guard, valid options are true or false"
                        )
                    }
                }
            }
            "-blade-direction" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a direction for the curve of the blade, valid options are left, central or right"
                    )
                }
                match args[arg + 1].to_lowercase().as_str() {
                    "left" => {
                        blade_direction = Direction::Left;
                    }
                    "right" => {
                        blade_direction = Direction::Right;
                    }
                    "central" => {
                        blade_direction = Direction::Central;
                    }
                    _ => {
                        panic!(
                            "Please provide a valid direction for the blade, either left, central or right"
                        )
                    }
                }
            }
            "-blade-bend" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a number for the curve of the blade, valid options are between 0 and 90"
                    )
                }
                if args[arg + 1].parse::<u8>().is_err()
                    || args[arg + 1].parse::<u8>().ok().is_none()
                {
                    panic!("Please provide a number between 0 and 90")
                }
                let num_arg = args[arg + 1].parse::<u8>().ok().unwrap();
                if num_arg > 90 {
                    panic!("Please provide a number between 0 and 90")
                }
                blade_angle = num_arg;
            }
            _ => {
                continue;
            }
        }
    }

    if (blade_direction == Direction::Central && blade_angle != 0)
        || (bladed_edge_count == 1 && blade_direction == Direction::Central)
    {
        panic!(
            "You have provided a parameter that requires the -blade-direction argument, please provide this data as either left or right"
        );
    }

    let mut params = ModelParams::default();
    params.set_blade_length(blade_length);
    params.set_blade_width(blade_width, blade_direction);
    params.set_blade_count(blade_direction, bladed_edge_count);
    params.set_has_guard(has_guard);
    params.set_blade_curvature(blade_direction, blade_angle);
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
    println!(
        "Mesh written after: {} secs",
        start.elapsed().unwrap().as_secs()
    );
}
