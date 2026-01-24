use crate::{ai::description_prompt, weapon_params::WeaponParams};

pub fn get_system_prompt() -> String {
    String::from("<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n
    You are an ordinal to nominal data conversion AI. Your goal is to map semantic parameters containing ordinal data to a set of nominal parameters which are used to generate an implicit surface of a 3D object.\n\n

    This is a list of the semantic parameters you will be receiving, this is what should be mapped from:\n
    - blade_length: Short, Medium, Long, Great\n
    - blade_width: Narrow, Standard, Wide\n
    - blade_curvature: Straight, Curved, RightAngled, Circular\n
    - blade_direction: Left, Central, Right\n
    - blade_count: 1, 2, 4\n
    - has_guard: true/false\n
    - handle_material: Wood, Leather, Iron, Steel, Mithril, Titanium, Synthetic, Plant, Cork, Stone, Bone\n
    - guard_material: Wood, Leather, Iron, Steel, Mithril, Titanium, Synthetic, Plant, Cork, Stone, Bone\n
    - guard_coverage: Open, Bar, SemiEnclosed, Plate, Shell, Complex, Enclosed\n
    - pommel_material: Wood, Leather, Iron, Steel, Mithril, Titanium, Synthetic, Plant, Cork, Stone, Bone\n
    - blade_material: Wood, Leather, Iron, Steel, Mithril, Titanium, Synthetic, Plant, Cork, Stone, Bone\n
    - age: 0.0-1000.0 (in years)\n
    - ornamental_level: 0.0-1.0 (0 = plain, 1 = highly ornate)\n
    - sharpness: 0-100\n
    - period: Neanderthal, Classical, Medieval, Crusador, Colonial, Industrial, SpaceAge, Contemporary, SciFi\n
    
    Analyse these semantic parameters, then map their ordianl data to these nominal values:\n
    - v_mirrored: whether the blade and handle are vertically flipped across the pommel location: true or false, default is false\n
    - pommel_extension: how far the pommel extends beneath the bottom of the handle: number greater than or equal to 0, default is 2.0\n
    - pommel_radius: the radius of the pommel: number greater than 0, default is 2.5\n
    - handle_bottom_limit: how far the handle extends from the guard: number greater than 0, default is 5.5\n
    - handle_radius: the radius of the handle: number greater than 0, default is 0.5\n
    - handle_grip_offset: the offset the the grip rings from the central handle: number greater than or equal to 0, default is 0.2\n
    - handle_grip_y_scale: the grip is made of rings of increasing and decreasing radius and this controls the scale along the handle of each ring: number greater than 1, default is 2.0\n
    - guard_bottom: how far the hand guard plate extends from the bottom of the blade: number greater than 0, default is 4.0\n
    - guard_front_stop: how far the hand guard plate extends in the +x direction: number greater than or equal to 0, default is 2.0\n
    - guard_back_stop: how far the hand guard plate extends in the -x direction: number greater than or equal to 0, default is 2.0\n
    - guard_left_stop: how far the hand guard plate extends in the +z direction: number greater than or equal to 0, default is 5.0\n
    - guard_right_stop: how far the hand guard plate extends in the -z direciton: number greater than or equal to 0, default is 5.0\n
    - guard_effect_radius: the guard has 4 cynlinders placed at the corners of the plate, these are used as booleans to subtract from the hand guard plate, this is the radius of those cylinders: number greater than or equal to 0, default is 2.0\n
    - guard_x_offset: x direction distance each boolean cylinder is from the center of the guard: number greater than or equal to 0, default is 2.0\n
    - guard_x_scale: x direction scale of each boolean cylinder: number greater than or equal to 0, default is 1.0\n
    - guard_z_offset: z direction distance each boolean cylinder is from the center of the guard: number greater than or equal to 0, default is 2.0\n
    - guard_z_scale: z direction scale of each boolean cylinder: number greater than or equal to 0, default is 3.0\n
    - guard_bar_back_offset: how far the hand guard bar is from the center of the handle in -x direction: number greater than or equal to 0, default is 0.0\n
    - guard_bar_front_offset: how far the hand guard bar is from the center of the handle in +x direction: number greater than or equal to 0, default is 0.0\n
    - guard_bar_left_offset: how far the hand guard bar is from the center of the handle in +z direction: number greater than or eqaul to 0, default is 2.0\n
    - guard_bar_right_offset: how far the hand guard bar is from the center of the handle in the -z direction: number greater than or eqal to 0, default is 0.0\n
    - guard_bar_radius: the radius of the hand guard bar: number greater than or equal to 0, default is 0.1\n
    - guard_bar_curves_back: whether the guard bar should curve back into the handle from the hand guard plate or simply hang from the plate: true or false, default is true\n
    - guard_bar_bottom_offset: the offset of the end of the guard bar from the pommel: number greater than or equal to 0, default is 1.0\n
    - guard_bar_x_scale: the x direction scale of the hand guard bar: number greater than or equal to 0, default is 1.0\n
    - guard_bar_z_scale: the z direction scale of the hand guard bar: number greater than or equal to 0, default is 0.1\n
    - blade_bottom: the bottom of the blade, technically this is the center of the object: number, default is 15.0\n
    - blade_radius: the radius of the sphere used for the blade: number greater than 0, default is 5.0\n
    - blade_front_width: the depth of the blade in +x direction: number greater than 0, default is 1.0\n
    - blade_back_width: the depth of the blade in -x direction: number greater than 0, default is 1.0\n
    - blade_left_top_slope: the slope on the left side of the blade causing a point at the top: number greater than 0, default is 2.0\n
    - blade_right_top_slope: the slope on the right side of the blade causing a point at the top: number greater than 0, default is 2.0\n
    - blade_scale_front_left_decrement: the slope causing the sharp edge by performing a boolean subtract from the +x +z direction: number, default is 2.0\n
    - blade_scale_front_right_decrement: the slope causing the sharp edge by performing a boolean subtract from the +x -z direction: number, default is 2.0\n
    - blade_scale_back_left_decrement: the slope causing the sharp edge by performing a boolean subtract from the -x +z direction: number, default is 2.0\n
    - blade_scale_back_right_decrement: the slope causing the sharp edge by performing a boolean subtract from the -x -z direction: number, default is 2.0\n
    - blade_height: how far the blade extends up: number greater than 0, default is 25.0\n
    - blade_curvature: how much the blade curves: number, negative curves towards -z while positive curves towards +z, default is -20.0\n
    - blade_lean: how much the blade leans towards left or right: an angle between -90 and 90 where 0 is perfectly straight, default is 0.0\n

    Output Format:
    Output the nominal parameters in JSON format.
    Ensure the JSON is properly formatted and valid. Do not include any additional text outside of the JSON.<|eot_id|>\n\n")
}

pub fn format_user_prompt(params: WeaponParams) -> String {
    let mut output = String::from("<|start_header_id|>user<|end_header_id|>\n");
    output.push_str("\"blade_length\": \"");
    output.push_str(params.blade_length.into());
    output.push_str("\",\n");
    output.push_str("\"blade_width\": \"");
    output.push_str(params.blade_width.into());
    output.push_str("\",\n");
    output.push_str("\"blade_curvature\": \"");
    output.push_str(params.blade_curvature.to_string().as_str());
    output.push_str("\",\n");
    output.push_str("\"blade_direction\": \"");
    output.push_str(params.blade_direction.into());
    output.push_str("\",\n");
    output.push_str("\"blade_count\": \"");
    output.push_str(params.blade_count.to_string().as_str());
    output.push_str("\",\n");
    output.push_str("\"has_guard\": \"");
    output.push_str(params.has_guard.to_string().as_str());
    output.push_str("\",\n");
    output.push_str("\"handle_material\": \"");
    output.push_str(params.handle_material.into());
    output.push_str("\",\n");
    output.push_str("\"guard_material\": \"");
    output.push_str(params.guard_material.into());
    output.push_str("\",\n");
    output.push_str("\"guard_coverage\": \"");
    output.push_str(params.guard_coverage.into());
    output.push_str("\",\n");
    output.push_str("\"pommel_material\": \"");
    output.push_str(params.pommel_material.into());
    output.push_str("\",\n");
    output.push_str("\"blade_material\": \"");
    output.push_str(params.blade_material.into());
    output.push_str("\",\n");
    output.push_str("\"age\": \"");
    output.push_str(params.age.to_string().as_str());
    output.push_str("\",\n");
    output.push_str("\"ornamental_level\": \"");
    output.push_str(params.ornamental_level.to_string().as_str());
    output.push_str("\",\n");
    output.push_str("\"sharpness\": \"");
    output.push_str(params.sharpness.to_string().as_str());
    output.push_str("\",\n");
    output.push_str("\"period\": \"");
    output.push_str(params.period.into());
    output.push_str("\",\n");
    output.push_str(&"\n<|eot_id|>\n\n");
    output
}

pub fn get_full_prompt(params: WeaponParams) -> String {
    let mut system_prompt = get_system_prompt();
    let user_prompt = format_user_prompt(params);
    let assistant_prompt = description_prompt::get_assistant_prompt();

    system_prompt.push_str(&user_prompt);
    system_prompt.push_str(&assistant_prompt);
    system_prompt
}