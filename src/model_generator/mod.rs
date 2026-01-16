pub mod convert_to_nums;
pub mod model_params;
use fidget::context::Tree;

use crate::model_generator::model_params::ModelParams;

pub fn create_basic_weapon(x: Tree, y: Tree, z: Tree, params: ModelParams) -> Tree {
    blade_and_guard(
        x,
        y,
        z,
        params.pommel_extension,
        params.pommel_radius,
        params.guard_back_stop,
        params.guard_bottom,
        params.guard_effect_radius,
        params.guard_front_stop,
        params.guard_left_stop,
        params.guard_right_stop,
        params.guard_x_offset,
        params.guard_x_scale,
        params.guard_z_offset,
        params.guard_z_scale,
        params.handle_bottom_limit,
        params.handle_radius,
        params.handle_grip_offset,
        params.handle_grip_y_scale,
        params.blade_back_width,
        params.blade_bottom,
        params.blade_front_width,
        params.blade_left_top_slope,
        params.blade_radius,
        params.blade_right_top_slope,
        params.blade_scale_decrement,
        params.blade_height,
    )
}

fn handle_mid(x: Tree, y: Tree, z: Tree, handle_radius: f64, handle_bottom_limit: f64) -> Tree {
    Tree::max(
        &(x.clone() * x.clone() + z.clone() * z.clone() - handle_radius),
        handle_bottom_limit - y.clone(),
    )
}

fn handle_pommel(x: Tree, y: Tree, z: Tree, pommel_radius: f64, pommel_extension: f64) -> Tree {
    Tree::max(
        &(x.clone() * x.clone() + z.clone() * z.clone() - y.clone() - pommel_radius),
        y.clone() - pommel_extension,
    )
}

fn handle_spiral(
    x: Tree,
    y: Tree,
    z: Tree,
    handle_radius: f64,
    handle_grip_offset: f64,
    handle_bottom_limit: f64,
    handle_grip_y_scale: f64,
) -> Tree {
    Tree::max(
        &(Tree::max(
            &((x.clone() * x.clone() + z.clone() * z.clone())
                - (handle_radius
                    + handle_grip_offset
                        * (Tree::floor(&Tree::modulo(&(y.clone() * handle_grip_y_scale), 2.0))))),
            -(x.clone() * x.clone() + z.clone() * z.clone() - handle_radius),
        )),
        -y.clone() + handle_bottom_limit,
    )
}

fn handle(
    x: Tree,
    y: Tree,
    z: Tree,
    handle_radius: f64,
    handle_bottom_limit: f64,
    pommel_extension: f64,
    pommel_radius: f64,
    handle_grip_offset: f64,
    handle_grip_y_scale: f64,
) -> Tree {
    Tree::min(
        &handle_spiral(
            x.clone(),
            y.clone(),
            z.clone(),
            handle_radius,
            handle_grip_offset,
            handle_bottom_limit,
            handle_grip_y_scale,
        ),
        Tree::min(
            &(&handle_mid(
                x.clone(),
                y.clone(),
                z.clone(),
                handle_radius,
                handle_bottom_limit,
            )),
            handle_pommel(
                x.clone(),
                y.clone() - (handle_bottom_limit - 5.0),
                z.clone(),
                pommel_radius,
                pommel_extension,
            ),
        ),
    )
}

fn guard_shape(
    x: Tree,
    y: Tree,
    z: Tree,
    guard_bottom: f64,
    guard_front_stop: f64,
    guard_back_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
) -> Tree {
    Tree::max(
        &(y.clone() + guard_bottom),
        Tree::max(
            &(-x.clone() - guard_front_stop),
            Tree::max(
                &(-z.clone() - guard_left_stop),
                Tree::max(&(z.clone() - guard_right_stop), x.clone() - guard_back_stop),
            ),
        ),
    )
}

fn guard_curve_effect(x: Tree, z: Tree, guard_effect_radius: f64) -> Tree {
    x.clone() * x.clone() + z.clone() * z.clone() - guard_effect_radius
}

fn guard(
    x: Tree,
    y: Tree,
    z: Tree,
    guard_bottom: f64,
    guard_back_stop: f64,
    guard_effect_radius: f64,
    guard_front_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    guard_x_offset: f64,
    guard_x_scale: f64,
    guard_z_scale: f64,
    guard_z_offset: f64,
) -> Tree {
    Tree::max(
        &(Tree::max(
            &(Tree::max(
                &(Tree::max(
                    &(guard_shape(
                        x.clone(),
                        y.clone(),
                        z.clone(),
                        guard_bottom,
                        guard_front_stop,
                        guard_back_stop,
                        guard_left_stop,
                        guard_right_stop,
                    )),
                    -guard_curve_effect(
                        (x.clone() / guard_x_scale) + guard_x_offset,
                        (z.clone() / guard_z_scale) + guard_z_offset,
                        guard_effect_radius,
                    ),
                )),
                -guard_curve_effect(
                    (x.clone() / guard_x_scale) + guard_x_offset,
                    (z.clone() / guard_z_scale) - guard_z_offset,
                    guard_effect_radius,
                ),
            )),
            -guard_curve_effect(
                (x.clone() / guard_x_scale) - guard_x_offset,
                (z.clone() / guard_z_scale) - guard_z_offset,
                guard_effect_radius,
            ),
        )),
        -guard_curve_effect(
            (x.clone() / guard_x_scale) - guard_x_offset,
            (z.clone() / guard_z_scale) + guard_z_offset,
            guard_effect_radius,
        ),
    )
}

fn handle_and_guard(
    x: Tree,
    y: Tree,
    z: Tree,
    pommel_extension: f64,
    pommel_radius: f64,
    guard_back_stop: f64,
    guard_bottom: f64,
    guard_effect_radius: f64,
    guard_front_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    guard_x_offset: f64,
    guard_x_scale: f64,
    guard_z_offset: f64,
    guard_z_scale: f64,
    handle_bottom_limit: f64,
    handle_radius: f64,
    handle_grip_offset: f64,
    handle_grip_y_scale: f64,
) -> Tree {
    Tree::max(
        &(Tree::min(
            &(handle(
                x.clone(),
                y.clone(),
                z.clone(),
                handle_radius,
                handle_bottom_limit,
                pommel_extension,
                pommel_radius,
                handle_grip_offset,
                handle_grip_y_scale,
            )),
            guard(
                x.clone(),
                (30.0 - y.clone()) - 20.0,
                z.clone(),
                guard_bottom,
                guard_back_stop,
                guard_effect_radius,
                guard_front_stop,
                guard_left_stop,
                guard_right_stop,
                guard_x_offset,
                guard_x_scale,
                guard_z_scale,
                guard_z_offset,
            ),
        )),
        0.0 - (15.0 - y.clone()) + 0,
    )
}

fn blade(
    x: Tree,
    y: Tree,
    z: Tree,
    blade_radius: f64,
    blade_back_width: f64,
    blade_front_width: f64,
    blade_left_top_slope: f64,
    blade_right_top_slope: f64,
    blade_height: f64,
) -> Tree {
    Tree::max(
        &(Tree::max(
            &(Tree::max(
                &(Tree::max(
                    &(Tree::max(
                        &Tree::max(
                            &((x.clone() * x.clone()) + (z.clone() * z.clone()) - blade_radius),
                            -y.clone(),
                        ),
                        -(-y.clone() + blade_height),
                    )),
                    -(x.clone() + blade_back_width),
                )),
                -(-x.clone() + blade_front_width),
            )),
            -(z.clone() * blade_left_top_slope + -(y.clone() - blade_height)),
        )),
        -(-z.clone() * blade_right_top_slope + -(y.clone() - blade_height)),
    )
}

fn blade_scale_decrease(
    x: Tree,
    y: Tree,
    z: Tree,
    blade_radius: f64,
    blade_back_width: f64,
    blade_front_width: f64,
    blade_left_top_slope: f64,
    blade_right_top_slope: f64,
    blade_height: f64,
    blade_scale_decrement: f64,
) -> Tree {
    Tree::max(
        &Tree::max(
        &Tree::max(
        &Tree::max(
        &Tree::max(
            &blade(
                x.clone(),
                y.clone(),
                z.clone(),
                blade_radius,
                blade_back_width,
                blade_front_width,
                blade_left_top_slope,
                blade_right_top_slope,
                blade_height,
            ),
            -Tree::min(
                &(blade_scale_v_decrease_modifier(
                    x.clone(),
                    -z.clone(),
                    blade_radius,
                    blade_scale_decrement,
                )),
                Tree::min(
                    &(blade_scale_v_decrease_modifier(
                        x.clone(),
                        z.clone(),
                        blade_radius,
                        blade_scale_decrement,
                    )),
                    Tree::min(
                        &(blade_scale_v_decrease_modifier(
                            -x.clone(),
                            z.clone(),
                            blade_radius,
                            blade_scale_decrement,
                        )),
                        blade_scale_v_decrease_modifier(
                            -x.clone(),
                            -z.clone(),
                            blade_radius,
                            blade_scale_decrement,
                        ),
                    ),
                ),
            ),
        ),
        blade_scale_d_decrease_modifier(
            x.clone()*blade_right_top_slope,
            y.clone(),
            z.clone()*blade_right_top_slope,
            blade_height,
            blade_scale_decrement,
        )),blade_scale_d_decrease_modifier(
            -x.clone()*blade_right_top_slope,
            y.clone(),
            z.clone()*blade_right_top_slope,
            blade_height,
            blade_scale_decrement,
        )),blade_scale_d_decrease_modifier(
            x.clone()*blade_left_top_slope,
            y.clone(),
            -z.clone()*blade_left_top_slope,
            blade_height,
            blade_scale_decrement,
        )),blade_scale_d_decrease_modifier(
            -x.clone()*blade_left_top_slope,
            y.clone(),
            -z.clone()*blade_left_top_slope,
            blade_height,
            blade_scale_decrement,
        ),
    )
}

fn blade_scale_v_decrease_modifier(
    x: Tree,
    z: Tree,
    blade_radius: f64,
    blade_scale_decrement: f64,
) -> Tree {
    x.clone() * blade_scale_decrement + blade_radius + z.clone() * blade_scale_decrement
}

fn blade_scale_d_decrease_modifier(
    x: Tree,
    y: Tree,
    z: Tree,
    blade_height: f64,
    blade_scale_decrement: f64,
) -> Tree {
    x.clone() * blade_scale_decrement + y.clone() - blade_height + z.clone() * blade_scale_decrement
}

fn blade_and_guard(
    x: Tree,
    y: Tree,
    z: Tree,
    pommel_extension: f64,
    pommel_radius: f64,
    guard_back_stop: f64,
    guard_bottom: f64,
    guard_effect_radius: f64,
    guard_front_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    guard_x_offset: f64,
    guard_x_scale: f64,
    guard_z_offset: f64,
    guard_z_scale: f64,
    handle_bottom_limit: f64,
    handle_radius: f64,
    handle_grip_offset: f64,
    handle_grip_y_scale: f64,
    blade_back_width: f64,
    blade_bottom: f64,
    blade_front_width: f64,
    blade_left_top_slope: f64,
    blade_radius: f64,
    blade_right_top_slope: f64,
    blade_scale_decrement: f64,
    blade_height: f64,
) -> Tree {
    -Tree::min(
        &(handle_and_guard(
            x.clone(),
            y.clone(),
            z.clone(),
            pommel_extension,
            pommel_radius,
            guard_back_stop,
            guard_bottom,
            guard_effect_radius,
            guard_front_stop,
            guard_left_stop,
            guard_right_stop,
            guard_x_offset,
            guard_x_scale,
            guard_z_offset,
            guard_z_scale,
            handle_bottom_limit,
            handle_radius,
            handle_grip_offset,
            handle_grip_y_scale,
        )),
        blade_scale_decrease(
            x.clone(),
            y.clone() - blade_bottom,
            z.clone(),
            blade_radius,
            blade_back_width,
            blade_front_width,
            blade_left_top_slope,
            blade_right_top_slope,
            blade_height,
            blade_scale_decrement,
        ),
    )
}
