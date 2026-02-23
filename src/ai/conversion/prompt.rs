use crate::weapon_params;


pub fn get_system_prompt() -> String {
    String::from("<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n
    You are a parametric mapping AI. Your goal is to map a set of semantic parameters from the user input containing ordinal data to a set of nominal parameters which are used in an implicit surface algorithm.\n\n
    
    Analyse the semantic parameters from the user input, then map their ordianl data to these nominal values:\n
    - v_mirrored: whether the blade and handle are vertically flipped across the pommel location: true or false, default is false\n
    - pommel_extension: how far the pommel extends beneath the bottom of the handle: number greater than or equal to 0, default is 2.0\n
    - pommel_radius: the radius of the pommel: number greater than 0, default is 2.5\n
    - handle_bottom_limit: how far the handle extends from the guard: number greater than 0, default is 5.5\n
    - handle_radius: the radius of the handle: number greater than 0, default is 0.5\n
    - handle_grip_offset: the offset the the grip rings from the central handle: number greater than or equal to 0, default is 0.2\n
    - handle_grip_y_scale: the grip is made of rings of increasing and decreasing radius and this controls the scale along the handle of each ring: number greater than 1, default is 2.0\n
    - guard_bottom: how far the hand guard plate extends from the bottom of the blade: number greater than 0, default is 4.0\n
    - guard_front_stop: how far the hand guard plate extends in the +x direction, 0.1 is closer to the center than 2.0: number greater than or equal to 0, default is 1.0\n
    - guard_back_stop: how far the hand guard plate extends in the -x direction, 0.1 is closer to the center than 2.0: number greater than or equal to 0, default is 1.0\n
    - guard_left_stop: how far the hand guard plate extends in the +z direction: number greater than or equal to 0, default is 5.0\n
    - guard_right_stop: how far the hand guard plate extends in the -z direciton: number greater than or equal to 0, default is 5.0\n
    - guard_effect_radius: the guard has 4 cynlinders placed at the corners of the plate, these are used as booleans to subtract from the hand guard plate, this is the radius of those cylinders: number greater than or equal to 0, default is 2.0\n
    - guard_x_offset: x direction distance each boolean cylinder is from the center of the guard: number greater than or equal to 0, default is 2.0\n
    - guard_x_scale: x direction scale of each boolean cylinder: number greater than or equal to 0, default is 1.0\n
    - guard_z_offset: z direction distance each boolean cylinder is from the center of the guard: number greater than or equal to 0, default is 2.0\n
    - guard_z_scale: z direction scale of each boolean cylinder: number greater than or equal to 0, default is 3.0\n
    - has_guard: whether the weapon has a hand guard plate at the meeting point between the blade and handle: true or false, default is true\n
    - has_guard_bar: whether the weapon has a bar travelling the length of the handle from the hand guard plate: true or false, default is true\n
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
    - blade_serated_left: whether the +z side of the blade is serated: true or false, default is false\n
    - blade_serated_left_count: the width of the serations along the +z side of the blade: number greater than 0, default is 1.0\n
    - blade_serated_left_size: the depth of the serations along the +z side of the blade: number greater than 0, default is 2.0\n
    - blade_serated_right: whether the -z side of the blade is serated: true or false, default is false\n
    - blade_serated_right_count: the width of the serations along the -z side of the blade: number greater than 0, default is 1.0\n
    - blade_serated_right_size: the depth of the serations along the -z side of the blade: number greater than 0, default is 2.0\n
    - blade_spiked_left: whether the +z side of the blade is spiked: true or false, default is false\n
    - blade_spiked_left_count: the width of the spikes along the +z side of the blade: number greater than 0, default is 1.0\n
    - blade_spiked_left_size: the length of the spikes along the +z side of the blade: number greater than 0, default is 2.0\n
    - blade_spiked_right: whether the -z side of the blade is spiked: true or false, default is false\n
    - blade_spiked_right_count: the width of the spikes along the -z side of the blade: number greater than 0, default is 1.0\n
    - blade_spiked_right_size: the length of the spikes along the -z side of the blade: number greater than 0, default is 2.0\n


    Output Format:
    Output the data in JSON format as shown below:\n
    {\n
    \"v_mirrored\": true/false,
    \"pommel_extension\": value,
    \"pommel_radius\": value,
    \"handle_bottom_limit\": value,
    \"handle_radius\": value,
    \"handle_grip_offset\": value,
    \"handle_grip_y_scale\": value,
    \"guard_bottom\": value,
    \"guard_front_stop\": value,
    \"guard_back_stop\": value,
    \"guard_left_stop\": value,
    \"guard_right_stop\": value,
    \"guard_effect_radius\": value,
    \"guard_x_offset\": value,
    \"guard_x_scale\": value,
    \"guard_z_offset\": value,
    \"guard_z_scale\": value,
    \"has_guard\": true/false,
    \"has_guard_bar\": true/false,
    \"guard_bar_back_offset\": value,
    \"guard_bar_front_offset\": value,
    \"guard_bar_left_offset\": value,
    \"guard_bar_right_offset\": value,
    \"guard_bar_radius\": value,
    \"guard_bar_curves_back\": true/false,
    \"guard_bar_bottom_offset\": value,
    \"guard_bar_x_scale\": value,
    \"guard_bar_z_scale\": value,
    \"blade_bottom\": value,
    \"blade_radius\": value,
    \"blade_front_width\": value,
    \"blade_back_width\": value,
    \"blade_left_top_slope\": value,
    \"blade_right_top_slope\": value,
    \"blade_scale_front_left_decrement\": value,
    \"blade_scale_front_right_decrement\": value,
    \"blade_scale_back_left_decrement\": value,
    \"blade_scale_back_right_decrement\": value,
    \"blade_serated_left\": true/false,
    \"blade_serated_left_count\": value,
    \"blade_serated_left_width\": value,
    \"blade_serated_right\": true/false,
    \"blade_serated_right_count\": value,
    \"blade_serated_right_width\": value,
    \"blade_spiked_left\": true/false,
    \"blade_spiked_left_count\": value,
    \"blade_spiked_left_width\": value,
    \"blade_spiked_right\": true/false,
    \"blade_spiked_right_count\": value,
    \"blade_spiked_right_width\": value,
    \"blade_height\": value,
    \"blade_curvature\": value,
    \"blade_lean\": value
    }\n
    \nEnsure the JSON is properly formatted and valid. If a value is not going to be used, enter it as the default parameter. Do not include any additional text outside of the JSON. Include all parameters listed within the JSON format and nothing else. Never introduce keys that are not in the list above, never omit keys, and never wrap the JSON in markdown or prose<|eot_id|>\n\n")
}

pub fn format_user_prompt(params: weapon_params::WeaponParams) -> String {
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
    // output.push_str("\"handle_material\": \"");
    // output.push_str(params.handle_material.into());
    // output.push_str("\",\n");
    // output.push_str("\"guard_material\": \"");
    // output.push_str(params.guard_material.into());
    // output.push_str("\",\n");
    output.push_str("\"guard_coverage\": \"");
    output.push_str(params.guard_coverage.into());
    output.push_str("\",\n");
    // output.push_str("\"pommel_material\": \"");
    // output.push_str(params.pommel_material.into());
    // output.push_str("\",\n");
    // output.push_str("\"blade_material\": \"");
    // output.push_str(params.blade_material.into());
    // output.push_str("\",\n");
    // output.push_str("\"age\": \"");
    // output.push_str(params.age.to_string().as_str());
    // output.push_str("\",\n");
    // output.push_str("\"ornamental_level\": \"");
    // output.push_str(params.ornamental_level.to_string().as_str());
    // output.push_str("\",\n");
    output.push_str("\"sharpness\": \"");
    output.push_str(params.blade_thickness.to_string().as_str());
    output.push_str("\",\n");
    // output.push_str("\"period\": \"");
    // output.push_str(params.period.into());
    // output.push_str("\",\n");
    output.push_str("\"blade_type\": \"");
    output.push_str(params.blade_type.into());
    output.push_str("\",\n");
    output.push_str("\"handle_length\": \"");
    output.push_str(params.handle_length.into());
    output.push_str("\",\n");
    output.push_str(&"\n<|eot_id|>\n\n<|end_of_text|>");
    output
}

pub fn get_assistant_prompt() -> String {
    let mut output = String::from("<|start_header_id|>assistant<|end_header_id|>\n
    This is the default values for the user input, use this to gain additional context on the mapping between the ordinal and nominal parameters:
    ");
    output.push_str(&serde_json::to_string(&weapon_params::WeaponParams::default()).unwrap());
    output.push_str(&String::from("<|eot_id|>"));
    output
}

pub fn get_full_prompt(params: weapon_params::WeaponParams) -> String {
    let mut system_prompt = get_system_prompt();
    let user_prompt = format_user_prompt(params);
    let assistant_prompt = get_assistant_prompt();

    system_prompt.push_str(&assistant_prompt);
    system_prompt.push_str(&user_prompt);
    system_prompt
}