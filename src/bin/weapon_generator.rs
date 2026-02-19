use fidget::{
    context::Tree,
    jit::JitShape,
    mesh::{Octree, Settings},
};
use nalgebra::Matrix4;
use semantic_weapons::{
    model_generator::{create_basic_weapon, model_params::ModelParams},
    weapon_params::{BladeLength, BladeType, BladeWidth, Direction, GuardCoverage, GuardPlateShape, HandleLength, WeaponParams},
};

use std::{env, fs::File, io::{Read, Write}, time::SystemTime};

fn main() {
    let start = SystemTime::now();

    let mut quality: u8 = 10;

    let mut params = ModelParams::default();
    let mut semantic = WeaponParams::default();

    let args: Vec<String> = env::args().collect();
    if args.iter().count() <= 1 {
        panic!("Please make sure to provide arguements")
    }
    let mut loaded_from_json = false;
    for arg in 1..args.iter().count() {
        match args[arg].as_str() {
            "-f" => {
                if arg + 1 > args.iter().count() || arg + 2 < args.iter().count() {
                    panic!("Please provide a json file")
                }
                let mut file = File::open(args[arg + 1].clone()).unwrap();
                let mut json_str = String::from("");
                file.read_to_string(&mut json_str).unwrap();

                // Deserialize with defaults for missing fields
                let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
                let default_params = ModelParams::default();

                // Merge JSON values into default params
                if let serde_json::Value::Object(map) = json_value {
                    let merged_json = serde_json::to_value(&default_params).unwrap();
                    if let serde_json::Value::Object(mut merged_map) = merged_json {
                        for (key, value) in map {
                            merged_map.insert(key, value);
                        }
                        params =
                            serde_json::from_value(serde_json::Value::Object(merged_map)).unwrap();
                    }
                } else {
                    params = default_params;
                }
                loaded_from_json = true;
                break;
            }
            "-s" => {
                let mut file = File::open(args[arg + 1].clone()).unwrap();
                let mut json_str = String::from("");
                file.read_to_string(&mut json_str).unwrap();

                // Deserialize with defaults for missing fields
                let json_value: serde_json::Value = serde_json::from_str(&json_str).unwrap();
                let default_params = WeaponParams::default();

                // Merge JSON values into default params
                if let serde_json::Value::Object(map) = json_value {
                    let merged_json = serde_json::to_value(&default_params).unwrap();
                    if let serde_json::Value::Object(mut merged_map) = merged_json {
                        for (key, value) in map {
                            merged_map.insert(key, value);
                        }
                        semantic =
                            serde_json::from_value(serde_json::Value::Object(merged_map)).unwrap();
                    }
                } else {
                    semantic = default_params;
                }
            }
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
                semantic.blade_length = Some(match args[arg + 1].to_lowercase().as_str() {
                    "short" => 
                        BladeLength::Short,
                    
                    "medium" => 
                        BladeLength::Medium,
                    
                    "long" => 
                        BladeLength::Long,
                   
                    "great" => 
                        BladeLength::Great,
                    
                    _ => {
                        panic!(
                            "Please provide a blade-length value, valid options are Short, Medium, Long and Great"
                        )
                    }
                
                }).unwrap();
            }
            "-blade-width" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a blade-width value, valid options are Narrow, Standard, Wide"
                    )
                }
                semantic.blade_width = Some(match args[arg + 1].to_lowercase().as_str() {
                    "narrow" => 
                        BladeWidth::Narrow,
                    
                    "standard" => 
                        BladeWidth::Standard,
                    
                    "wide" => 
                        BladeWidth::Wide,
                    
                    _ => {
                        panic!(
                            "Please provide a blade-width value, valid options are Narrow, Standard, Wide"
                        )
                    }
                }).unwrap();
            }
            "-blade-thickness" => {
                if arg + 1 > args.iter().count() {
                    panic!("Please provide a blade-thickness value between 0.0 and 100.0")
                }
                let parsed = args[arg + 1].parse::<f64>().ok();
                if parsed.is_none() || parsed.unwrap() < 0.0 || parsed.unwrap() > 100.0 {
                    panic!("Invalid input: blade-thickness must be between 0.0 and 100.0")
                }
                semantic.blade_thickness = parsed.unwrap();
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
                semantic.blade_count = args[arg + 1].parse::<u8>().unwrap();
            }
            "-has-guard" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a boolean for if the weapon should have a guard, valid options are true or false"
                    )
                }
                semantic.has_guard = Some(match args[arg + 1].to_lowercase().as_str() {
                    "true" => 
                        true,
                    "false" => 
                        false,
                    _ => {
                        panic!(
                            "Please provide a boolean for if the weapon should have a guard, valid options are true or false"
                        )
                    }
                }).unwrap();
            }
            "-guard-coverage" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a guard coverage, valid options are Open, Bar, SemiEnclosed, Plate, Shell, Complex, Enclosed"
                    )
                }
                let normalized = args[arg + 1]
                    .to_lowercase()
                    .replace('-', "")
                    .replace('_', "");
                semantic.guard_coverage = Some(match normalized.as_str() {
                    "open" => GuardCoverage::Open,
                    "bar" => GuardCoverage::Bar,
                    "semienclosed" => GuardCoverage::SemiEnclosed,
                    "plate" => GuardCoverage::Plate,
                    "shell" => GuardCoverage::Shell,
                    "complex" => GuardCoverage::Complex,
                    "enclosed" => GuardCoverage::Enclosed,
                    _ => {
                        panic!(
                            "Please provide a guard coverage, valid options are Open, Bar, SemiEnclosed, Plate, Shell, Complex, Enclosed"
                        )
                    }
                }).unwrap();
            }
            "-guard-plate-shape" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a guard plate shape, valid options are Flat, Horseshoe, Dome, Bowl, BowlCapped, Upturned"
                    )
                }
                let normalized = args[arg + 1]
                    .to_lowercase()
                    .replace('-', "")
                    .replace('_', "");
                semantic.guard_plate_shape = Some(match normalized.as_str() {
                    "flat" => GuardPlateShape::Flat,
                    "horseshoe" => GuardPlateShape::Horseshoe,
                    "dome" => GuardPlateShape::Dome,
                    "bowl" => GuardPlateShape::Bowl,
                    "bowlcapped" => GuardPlateShape::BowlCapped,
                    "upturned" => GuardPlateShape::Upturned,
                    _ => {
                        panic!(
                            "Please provide a guard plate shape, valid options are Flat, Horseshoe, Dome, Bowl, BowlCapped, Upturned"
                        )
                    }
                }).unwrap();
            }
            "-guard-plate-curvature" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a guard plate curvature value between 0.1 and 3.0"
                    )
                }
                match args[arg + 1].parse::<f64>() {
                    Ok(value) => {
                        if value < 0.1 || value > 3.0 {
                            panic!(
                                "Guard plate curvature must be between 0.1 (shallow) and 3.0 (deep)"
                            )
                        }
                        semantic.guard_plate_curvature = Some(value).unwrap();
                    }
                    Err(_) => {
                        panic!(
                            "Please provide a valid floating-point number for guard plate curvature"
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
                semantic.blade_direction = Some(match args[arg + 1].to_lowercase().as_str() {
                    "left" => 
                        Direction::Left,
                    
                    "right" => 
                        Direction::Right,
                    
                    "central" => 
                        Direction::Central,
                    
                    _ => {
                        panic!(
                            "Please provide a valid direction for the blade, either left, central or right"
                        )
                    }
                }).unwrap();
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
                semantic.blade_curvature = num_arg as f64;
            }
            "-handle-length" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a length for the handle, valid options are Dagger, OneHanded, TwoHanded, ForearmLength, Polearm"
                    )
                }
                semantic.handle_length = Some(match args[arg + 1].to_lowercase().as_str() {
                    "dagger" => 
                        HandleLength::Dagger,
                    
                    "onehanded" => 
                        HandleLength::OneHanded,
                    
                    "twohanded" => 
                        HandleLength::TwoHanded,
                    
                    "forearmlength" => HandleLength::ForearmLength,
                    "polearm" => HandleLength::Polearm,
                    _ => {
                        panic!(
                            "Please provide a length for the handle, valid options are Dagger, OneHanded, TwoHanded, ForearmLength, Polearm"
                        )
                    }
                }).unwrap();
            }
            "-blade-type" => {
                if arg + 1 > args.iter().count() {
                    panic!(
                        "Please provide a type for the blade, valid options are Dull, Sharp, Spikey, Serated, SpikeyAndSerated"
                    )
                }
                semantic.blade_type = Some(match args[arg + 1].to_lowercase().as_str() {
                    "dull" => 
                        BladeType::Dull,
                    
                    "sharp" => 
                        BladeType::Sharp,
                    
                    "spikey" => 
                        BladeType::Spikey,
                    
                    "serated" => BladeType::Serated,
                    "spikeyandserated" => BladeType::SpikeyAndSerated,
                    _ => {
                        panic!(
                            "Please provide a type for the blade, valid options are Dull, Sharp, Spikey, Serated, SpikeyAndSerated"
                        )
                    }
                }).unwrap();
            }
            _ => {
                continue;
            }
        }
    }

    if (semantic.blade_direction == Direction::Central && semantic.blade_curvature != 0.0)
        || (semantic.blade_count == 1 && semantic.blade_direction == Direction::Central)
    {
        panic!(
            "You have provided a parameter that requires the -blade-direction argument, please provide this data as either left or right"
        );
    }

    if params == ModelParams::default() && !loaded_from_json {
        params.set_blade_length(semantic.blade_length);
        params.set_blade_width(semantic.blade_width, semantic.blade_direction);
        params.set_blade_count(semantic.blade_direction, semantic.blade_count);
        params.set_has_guard(semantic.has_guard);
            params.guard_coverage(semantic.guard_coverage);
            params.set_guard_plate_shape(semantic.guard_plate_shape);
            params.set_guard_plate_curvature(semantic.guard_plate_curvature);
        params.set_blade_curvature(semantic.blade_direction, semantic.blade_curvature as u8);
        params.set_handle_length(semantic.handle_length);
        params.set_blade_type(semantic.blade_type, semantic.blade_direction);
            params.set_blade_thickness(semantic.blade_thickness);
}
    let sdf = create_basic_weapon(Tree::x(), Tree::y(), Tree::z(), params);

    //scaling shape to fit inside bounding box (1, 1, 1)
    let mut scaling = params.blade_height + params.blade_bottom;
    scaling += 15.0;
    if params.v_mirrored || params.handle_bottom_limit < -1.0 {
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
