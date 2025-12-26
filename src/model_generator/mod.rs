use fidget::context::Tree;

pub fn create_basic_weapon(x: Tree, y: Tree, z: Tree) -> Tree {
    let pommel_shift = -0.5;
    let pommel_extension = 4.5;
    let pommel_radius = -3.5;
    let handle_bottom_limit = 5.5;
    let handle_radius = 0.5;
    let handle_spiral_offset = 0.2;

    let guard_bottom = 4.0;
    let guard_front_stop = 2.0;
    let guard_back_stop = 2.0;
    let guard_left_stop = 5.0;
    let guard_right_stop = 5.0;
    let guard_effect_radius = 2.0;
    let guard_x_offset = 2.0;
    let guard_z_offset = 2.0;
    let guard_z_scale = 3.0;

    let blade_bottom = 15.0;
    let blade_radius = 2.0;
    let blade_front_width = 1.0;
    let blade_back_width = 1.0;
    let blade_left_top_slope = 2.0;
    let blade_right_top_slope = 2.0;
    let blade_scale = 3.0;
    let blade_scale_decrement = 10.0;

    blade_and_guard(
        x,
        y,
        z,
        pommel_extension,
        pommel_radius,
        pommel_shift,
        guard_back_stop,
        guard_bottom,
        guard_effect_radius,
        guard_front_stop,
        guard_left_stop,
        guard_right_stop,
        guard_x_offset,
        guard_z_offset,
        guard_z_scale,
        handle_bottom_limit,
        handle_radius,
        handle_spiral_offset,
        blade_back_width,
        blade_bottom,
        blade_front_width,
        blade_left_top_slope,
        blade_radius,
        blade_right_top_slope,
        blade_scale,
        blade_scale_decrement,
    )
}

fn handle_mid(x: Tree, y: Tree, z: Tree, handle_radius: f64, handle_bottom_radius: f64) -> Tree {
    Tree::max(
        &(x.clone() * x.clone() + z.clone() * z.clone() - handle_radius),
        handle_bottom_radius - y.clone(),
    )
}

fn handle_pommel(
    x: Tree,
    y: Tree,
    z: Tree,
    pommel_shift: f64,
    pommel_radius: f64,
    pommel_extension: f64,
) -> Tree {
    Tree::max(
        &(x.clone() * x.clone() + z.clone() * z.clone()
            - (y.clone() + pommel_shift)
            - pommel_radius),
        (y.clone() + pommel_shift) - pommel_extension,
    )
}

fn handle_spiral(
    x: Tree,
    y: Tree,
    z: Tree,
    handle_radius: f64,
    handle_spiral_offset: f64,
    handle_bottom_limit: f64,
) -> Tree {
    Tree::max(
        &(Tree::max(
            &(Tree::max(
                &((x.clone() * x.clone() + z.clone() * z.clone())
                    - (handle_radius + handle_spiral_offset)),
                Tree::sin(&(x.clone() * x.clone() - y.clone() * y.clone())),
            )),
            -(x.clone() * x.clone() + z.clone() * z.clone() - handle_radius),
        )),
        -y.clone() + handle_bottom_limit,
    )
}

fn handle(
    x: Tree,
    y: Tree,
    z: Tree,
    pommel_shift: f64,
    handle_radius: f64,
    handle_bottom_limit: f64,
    pommel_extension: f64,
    pommel_radius: f64,
    handle_spiral_offset: f64,
) -> Tree {
    Tree::min(
        &handle_spiral(
            x.clone(),
            y.clone(),
            z.clone(),
            handle_radius,
            handle_spiral_offset,
            handle_bottom_limit,
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
                y.clone() + pommel_shift,
                z.clone(),
                pommel_shift,
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

fn guard_curve_effect(x: Tree, y: Tree, z: Tree, guard_effect_radius: f64) -> Tree {
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
                        guard_right_stop
                    )),
                    -guard_curve_effect(
                        x.clone() + guard_x_offset,
                        y.clone(),
                        (z.clone() / guard_z_scale) + guard_z_offset,
                        guard_effect_radius,
                    ),
                )),
                -guard_curve_effect(
                    x.clone() + guard_x_offset,
                    y.clone(),
                    (z.clone() / guard_z_scale) - guard_z_offset,
                    guard_effect_radius,
                ),
            )),
            -guard_curve_effect(
                x.clone() - guard_x_offset,
                y.clone(),
                (z.clone() / guard_z_scale) - guard_z_offset,
                guard_effect_radius,
            ),
        )),
        -guard_curve_effect(
            x.clone() - guard_x_offset,
            y.clone(),
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
    pommel_shift: f64,
    guard_back_stop: f64,
    guard_bottom: f64,
    guard_effect_radius: f64,
    guard_front_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    guard_x_offset: f64,
    guard_z_offset: f64,
    guard_z_scale: f64,
    handle_bottom_limit: f64,
    handle_radius: f64,
    handle_spiral_offset: f64,
) -> Tree {
    Tree::max(
        &(Tree::min(
            &(handle(
                x.clone(),
                y.clone(),
                z.clone(),
                pommel_shift,
                handle_radius,
                handle_bottom_limit,
                pommel_extension,
                pommel_radius,
                handle_spiral_offset,
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
    blade_bottom: f64,
    blade_back_width: f64,
    blade_front_width: f64,
    blade_left_top_slope: f64,
    blade_right_top_slope: f64,
) -> Tree {
    Tree::max(
        &(Tree::max(
            &(Tree::max(
                &(Tree::max(
                    &(Tree::max(
                        &(x.clone() * x.clone() + z.clone() * z.clone() - blade_radius),
                        -(y.clone() - blade_bottom),
                    )),
                    -(x.clone() + blade_back_width),
                )),
                -(-x.clone() + blade_front_width),
            )),
            -(z.clone() * blade_left_top_slope + -(y.clone() - 40)),
        )),
        -(-z.clone() * blade_right_top_slope + -(y.clone() - 40)),
    )
}

fn blade_and_guard(
    x: Tree,
    y: Tree,
    z: Tree,
    pommel_extension: f64,
    pommel_radius: f64,
    pommel_shift: f64,
    guard_back_stop: f64,
    guard_bottom: f64,
    guard_effect_radius: f64,
    guard_front_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    guard_x_offset: f64,
    guard_z_offset: f64,
    guard_z_scale: f64,
    handle_bottom_limit: f64,
    handle_radius: f64,
    handle_spiral_offset: f64,
    blade_back_width: f64,
    blade_bottom: f64,
    blade_front_width: f64,
    blade_left_top_slope: f64,
    blade_radius: f64,
    blade_right_top_slope: f64,
    blade_scale: f64,
    blade_scale_decrement: f64,
) -> Tree {
    Tree::min(
        &(handle_and_guard(
            x.clone(),
            y.clone(),
            z.clone(),
            pommel_extension,
            pommel_radius,
            pommel_shift,
            guard_back_stop,
            guard_bottom,
            guard_effect_radius,
            guard_front_stop,
            guard_left_stop,
            guard_right_stop,
            guard_x_offset,
            guard_z_offset,
            guard_z_scale,
            handle_bottom_limit,
            handle_radius,
            handle_spiral_offset,
        )),
        blade(
            x.clone() * (blade_scale + (y.clone() / blade_scale_decrement)),
            y.clone(),
            z.clone(),
            blade_radius,
            blade_bottom,
            blade_back_width,
            blade_front_width,
            blade_left_top_slope,
            blade_right_top_slope,
        ),
    )
}
