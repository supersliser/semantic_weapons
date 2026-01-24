pub mod convert_to_nums;
pub mod model_params;
use fidget::{context::Tree, eval::MathFunction};

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
        params.blade_scale_back_left_decrement,
        params.blade_scale_back_right_decrement,
        params.blade_scale_front_left_decrement,
        params.blade_scale_front_right_decrement,
        params.blade_height,
        params.v_mirrored,
        params.blade_curvature,
        params.blade_lean,
        params.guard_bar_back_offset,
        params.guard_bar_front_offset,
        params.guard_bar_left_offset,
        params.guard_bar_radius,
        params.guard_bar_right_offset,
        params.guard_bar_curves_back,
        params.guard_bar_bottom_offset,
        params.guard_bar_x_scale,
        params.guard_bar_z_scale
    )
}

fn handle_mid(x: Tree, y: Tree, z: Tree, handle_radius: f64, handle_bottom_limit: f64) -> Tree {
    (x.clone() * x.clone() + z.clone() * z.clone() - handle_radius)
        .max(handle_bottom_limit - y.clone())
}

fn handle_pommel(
    x: Tree,
    y: Tree,
    z: Tree,
    pommel_radius: f64,
    pommel_extension: f64,
    handle_bottom_limit: f64,
) -> Tree {
    (x.clone() * x.clone() + z.clone() * z.clone() - (y.clone() * (pommel_radius * 0.1)))
        .max((y.clone()) - (handle_bottom_limit - pommel_extension))
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
    ((x.clone() * x.clone() + z.clone() * z.clone())
        - (handle_radius
            + handle_grip_offset * (y.clone() * handle_grip_y_scale).modulo(2.0).floor()))
    .max(-(x.clone() * x.clone() + z.clone() * z.clone() - handle_radius))
    .max(-y.clone() + handle_bottom_limit)
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
    mirrored_v: bool,
) -> Tree {
    let mut handle = handle_mid(
        x.clone(),
        y.clone(),
        z.clone(),
        handle_radius,
        handle_bottom_limit,
    );
    if !mirrored_v {
        let mut extension = (6.0) - pommel_extension;
        handle = handle.min(handle_pommel(
            x.clone(),
            y.clone() - extension,
            z.clone(),
            pommel_radius,
            extension,
            handle_bottom_limit,
        ));
    }
    handle.min(handle_spiral(
        x.clone(),
        y.clone(),
        z.clone(),
        handle_radius,
        handle_grip_offset,
        handle_bottom_limit,
        handle_grip_y_scale,
    ))
}

fn guard_bar(
    ix: Tree,
    iy: Tree,
    iz: Tree,
    guard_bottom: f64,
    guard_front_stop: f64,
    guard_back_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    guard_bar_front_offset: f64,
    guard_bar_back_offset: f64,
    guard_bar_left_offset: f64,
    guard_bar_right_offset: f64,
    guard_bar_radius: f64,
    guard_bar_curves_back: bool,
    guard_bar_bottom_offset: f64,
    handle_bottom_limit: f64,
    pommel_extension: f64,
    guard_bar_x_scale: f64,
    guard_bar_z_scale: f64
) -> Tree {
    let mut x = ix.clone();
    let mut z = iz.clone();
    let y = iy.clone() - handle_bottom_limit + pommel_extension + guard_bar_bottom_offset;
    if guard_bar_curves_back {
        x += Tree::max(&y.clone(), guard_bar_front_offset).min(guard_front_stop);
        x -= Tree::max(&y.clone(), guard_bar_back_offset).min(guard_back_stop);
        z += Tree::max(&y.clone(), guard_bar_left_offset).min(guard_left_stop);
        z -= Tree::max(&y.clone(), guard_bar_right_offset).min(guard_right_stop);
    } else {
        // x += Tree::min(&y.clone(), guard_front_stop);
        // x -= Tree::min(&y.clone(), guard_back_stop);
        // z += Tree::min(&y.clone(), guard_left_stop);
        // z -= Tree::min(&y.clone(), guard_right_stop);
        x += guard_bar_front_offset;
        x -= guard_bar_back_offset;
        z += guard_bar_left_offset;
        z -= guard_bar_right_offset;
    }
    let bar = ((x.clone() * x.clone() / guard_bar_x_scale)  + (z.clone() * z.clone() / guard_bar_z_scale) - guard_bar_radius)
        .max(iy.clone() - handle_bottom_limit);
    bar
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
    (y.clone() + guard_bottom).max(
        (-x.clone() - guard_front_stop).max(
            (-z.clone() - guard_left_stop)
                .max((z.clone() - guard_right_stop).max(x.clone() - guard_back_stop)),
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
    guard_bar_back_offset: f64,
    guard_bar_front_offset: f64,
    guard_bar_left_offset: f64,
    guard_bar_radius: f64,
    guard_bar_right_offset: f64,
    guard_bar_curves_back: bool,
    handle_bottom_limit: f64,
    pommel_extension: f64,
    guard_bar_bottom_offset: f64,
    guard_bar_x_scale: f64,
    guard_bar_z_scale: f64,
) -> Tree {
    guard_shape(
        x.clone(),
        y.clone(),
        z.clone(),
        guard_bottom,
        guard_front_stop,
        guard_back_stop,
        guard_left_stop,
        guard_right_stop,
    )
    .max(-guard_curve_effect(
        (x.clone() / guard_x_scale) + guard_x_offset,
        (z.clone() / guard_z_scale) + guard_z_offset,
        guard_effect_radius,
    ))
    .max(-guard_curve_effect(
        (x.clone() / guard_x_scale) + guard_x_offset,
        (z.clone() / guard_z_scale) - guard_z_offset,
        guard_effect_radius,
    ))
    .max(-guard_curve_effect(
        (x.clone() / guard_x_scale) - guard_x_offset,
        (z.clone() / guard_z_scale) - guard_z_offset,
        guard_effect_radius,
    ))
    .max(-guard_curve_effect(
        (x.clone() / guard_x_scale) - guard_x_offset,
        (z.clone() / guard_z_scale) + guard_z_offset,
        guard_effect_radius,
    ))
    .min(guard_bar(
        x,
        y,
        z,
        guard_bottom,
        guard_front_stop,
        guard_back_stop,
        guard_left_stop,
        guard_right_stop,
        guard_bar_front_offset,
        guard_bar_back_offset,
        guard_bar_left_offset,
        guard_bar_right_offset,
        guard_bar_radius,
        guard_bar_curves_back,
        guard_bar_bottom_offset,
        handle_bottom_limit,
        pommel_extension,
        guard_bar_x_scale,
        guard_bar_z_scale
    ))
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
    mirrored_v: bool,
    guard_bar_back_offset: f64,
    guard_bar_front_offset: f64,
    guard_bar_left_offset: f64,
    guard_bar_radius: f64,
    guard_bar_right_offset: f64,
    guard_bar_curves_back: bool,
    guard_bar_bottom_offset: f64,
    guard_bar_x_scale: f64,
    guard_bar_z_scale: f64
) -> Tree {
    handle(
        x.clone(),
        y.clone(),
        z.clone(),
        handle_radius,
        handle_bottom_limit,
        pommel_extension,
        pommel_radius,
        handle_grip_offset,
        handle_grip_y_scale,
        mirrored_v,
    )
    .min(guard(
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
        guard_bar_back_offset,
        guard_bar_front_offset,
        guard_bar_left_offset,
        guard_bar_radius,
        guard_bar_right_offset,
        guard_bar_curves_back,
        handle_bottom_limit,
        pommel_extension,
        guard_bar_bottom_offset,
        guard_bar_x_scale,
        guard_bar_z_scale
    ))
    .max(0.0 - (15.0 - y.clone()) + 0)
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
    blade_curvature: f64,
    blade_lean: f64,
) -> Tree {
    let z_curved = apply_curvature(y.clone(), z.clone(), blade_curvature, blade_lean);
    ((x.clone() * x.clone()) + (z_curved.clone() * z_curved.clone()) - blade_radius)
        .max(-y.clone())
        .max(-(-y.clone() + blade_height))
        .max(-(x.clone() + blade_back_width))
        .max(-(-x.clone() + blade_front_width))
        .max(-(z_curved.clone() * blade_left_top_slope - (y.clone() - blade_height)))
        .max(-(-z_curved.clone() * blade_right_top_slope - (y.clone() - blade_height)))
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
    blade_scale_front_left_decrement: f64,
    blade_scale_front_right_decrement: f64,
    blade_scale_back_left_decrement: f64,
    blade_scale_back_right_decrement: f64,
    blade_curvature: f64,
    blade_lean: f64,
) -> Tree {
    let z_curved = apply_curvature(y.clone(), z.clone(), blade_curvature, blade_lean);
    blade(
        x.clone(),
        y.clone(),
        z.clone(),
        blade_radius,
        blade_back_width,
        blade_front_width,
        blade_left_top_slope,
        blade_right_top_slope,
        blade_height,
        blade_curvature,
        blade_lean,
    )
    .max(
        -(blade_scale_v_decrease_modifier(
            x.clone(),
            -z_curved.clone(),
            blade_radius,
            blade_scale_back_right_decrement,
        )
        .min(
            blade_scale_v_decrease_modifier(
                x.clone(),
                z_curved.clone(),
                blade_radius,
                blade_scale_back_left_decrement,
            )
            .min(
                blade_scale_v_decrease_modifier(
                    -x.clone(),
                    z_curved.clone(),
                    blade_radius,
                    blade_scale_front_left_decrement,
                )
                .min(blade_scale_v_decrease_modifier(
                    -x.clone(),
                    -z_curved.clone(),
                    blade_radius,
                    blade_scale_front_right_decrement,
                )),
            ),
        )),
    )
    .max(blade_scale_d_decrease_modifier(
        x.clone() * blade_right_top_slope,
        y.clone(),
        z_curved.clone(),
        blade_height,
        blade_scale_back_left_decrement,
    ))
    .max(blade_scale_d_decrease_modifier(
        -x.clone() * blade_right_top_slope,
        y.clone(),
        z_curved.clone(),
        blade_height,
        blade_scale_front_left_decrement,
    ))
    .max(blade_scale_d_decrease_modifier(
        x.clone() * blade_left_top_slope,
        y.clone(),
        -z_curved.clone(),
        blade_height,
        blade_scale_back_right_decrement,
    ))
    .max(blade_scale_d_decrease_modifier(
        -x.clone() * blade_left_top_slope,
        y.clone(),
        -z_curved.clone(),
        blade_height,
        blade_scale_front_right_decrement,
    ))
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
    y: Tree, // This represents the vertical height
    z: Tree,
    blade_height: f64,
    blade_scale_decrement: f64,
) -> Tree {
    x.clone() * blade_scale_decrement + y.clone() - blade_height + z.clone() * blade_scale_decrement
}

fn apply_curvature(y: Tree, z: Tree, blade_curvature: f64, blade_lean: f64) -> Tree {
    let curve_shape = (y / blade_lean).pow(2);
    z + curve_shape * blade_curvature
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
    blade_scale_back_left_decrement: f64,
    blade_scale_back_right_decrement: f64,
    blade_scale_front_left_decrement: f64,
    blade_scale_front_right_decrement: f64,
    blade_height: f64,
    mirrored_v: bool,
    blade_curvature: f64,
    blade_lean: f64,
    guard_bar_back_offset: f64,
    guard_bar_front_offset: f64,
    guard_bar_left_offset: f64,
    guard_bar_radius: f64,
    guard_bar_right_offset: f64,
    guard_bar_curves_back: bool,
    guard_bar_bottom_offset: f64,
    guard_bar_x_scale: f64,
    guard_bar_z_scale: f64
) -> Tree {
    let blade_left_shift =
        (blade_scale_back_left_decrement + blade_scale_front_left_decrement) / 20.0;
    let blade_right_shift =
        (blade_scale_back_right_decrement + blade_scale_front_right_decrement) / 20.0;
    if mirrored_v {
        -handle_and_guard(
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
            mirrored_v,
            guard_bar_back_offset,
            guard_bar_front_offset,
            guard_bar_left_offset,
            guard_bar_radius,
            guard_bar_right_offset,
            guard_bar_curves_back,
            guard_bar_bottom_offset,
            guard_bar_x_scale,
            guard_bar_z_scale
        )
        .min(blade_scale_decrease(
            x.clone(),
            y.clone() - blade_bottom,
            z.clone() - blade_right_shift + blade_left_shift,
            blade_radius,
            blade_back_width,
            blade_front_width,
            blade_left_top_slope,
            blade_right_top_slope,
            blade_height,
            blade_scale_front_left_decrement,
            blade_scale_front_right_decrement,
            blade_scale_back_left_decrement,
            blade_scale_back_right_decrement,
            blade_curvature,
            blade_lean,
        ))
        .min(
            handle_and_guard(
                x.clone(),
                2.0 * handle_bottom_limit - y.clone(),
                2.0 * 0.0 - z.clone(),
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
                mirrored_v,
                guard_bar_back_offset,
                guard_bar_front_offset,
                guard_bar_left_offset,
                guard_bar_radius,
                guard_bar_right_offset,
                guard_bar_curves_back,
                guard_bar_bottom_offset,
                guard_bar_x_scale,
                guard_bar_z_scale
            )
            .min(blade_scale_decrease(
                x.clone(),
                2.0 * handle_bottom_limit - (y.clone() + blade_bottom),
                2.0 * 0.0 - z.clone() - blade_right_shift + blade_left_shift,
                blade_radius,
                blade_back_width,
                blade_front_width,
                blade_left_top_slope,
                blade_right_top_slope,
                blade_height,
                blade_scale_front_left_decrement,
                blade_scale_front_right_decrement,
                blade_scale_back_left_decrement,
                blade_scale_back_right_decrement,
                blade_curvature,
                blade_lean,
            )),
        )
    } else {
        -handle_and_guard(
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
            mirrored_v,
            guard_bar_back_offset,
            guard_bar_front_offset,
            guard_bar_left_offset,
            guard_bar_radius,
            guard_bar_right_offset,
            guard_bar_curves_back,
            guard_bar_bottom_offset,
            guard_bar_x_scale,
            guard_bar_z_scale
        )
        .min(blade_scale_decrease(
            x.clone(),
            y.clone() - blade_bottom,
            z.clone() - blade_right_shift + blade_left_shift,
            blade_radius,
            blade_back_width,
            blade_front_width,
            blade_left_top_slope,
            blade_right_top_slope,
            blade_height,
            blade_scale_front_left_decrement,
            blade_scale_front_right_decrement,
            blade_scale_back_left_decrement,
            blade_scale_back_right_decrement,
            blade_curvature,
            blade_lean,
        ))
    }
}
