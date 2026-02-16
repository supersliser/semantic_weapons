pub mod convert_to_nums;
pub mod model_params;
use std::f64::consts::PI;

use fidget::context::Tree;

use crate::model_generator::model_params::ModelParams;
use crate::weapon_params::GuardPlateShape;

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
        params.blade_scale_back_right_decrement,
        params.blade_scale_back_left_decrement,
        params.blade_scale_front_right_decrement,
        params.blade_scale_front_left_decrement,
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
        params.guard_bar_z_scale,
        params.guard_bar_y_scale,
        params.guard_bar_thickness,
        params.guard_bar_height,
        params.has_guard,
        params.has_guard_bar,
        params.blade_serated_left,
        params.blade_serated_left_count,
        params.blade_serated_left_size,
        params.blade_serated_right,
        params.blade_serated_right_count,
        params.blade_serated_right_size,
        params.blade_spiked_left,
        params.blade_spiked_left_count,
        params.blade_spiked_left_size,
        params.blade_spiked_right,
        params.blade_spiked_right_count,
        params.blade_spiked_right_size,
        params.guard_plate_shape,
        params.guard_plate_curvature,
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
        let extension = (6.0) - pommel_extension;
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
    _guard_bottom: f64,
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
    guard_bar_z_scale: f64,
    guard_bar_y_scale: f64,
    guard_bar_thickness: f64,
    guard_bar_height: f64,
    has_guard_bar: bool
) -> Tree {
    // Coordinate system constants
    let guard_handle_limit = handle_bottom_limit;
    
    // y_param: parametric coordinate for curving behavior (how far along the bar's path)
    // This is used to drive the dynamic x-z offset changes as we descend
    let y_param = iy.clone() - guard_handle_limit + pommel_extension + guard_bar_bottom_offset;
    
    // Apply dynamic x-z positioning: the bar curves/repositions as it extends downward
    let mut x = ix.clone();
    let mut z = iz.clone();
    
    if guard_bar_curves_back {
        // Offsets increase with y_param, clamped by guard stops
        x += Tree::max(&y_param.clone(), guard_bar_front_offset).min(guard_front_stop);
        x -= Tree::max(&y_param.clone(), guard_bar_back_offset).min(guard_back_stop);
        z += Tree::max(&y_param.clone(), guard_bar_left_offset).min(guard_left_stop);
        z -= Tree::max(&y_param.clone(), guard_bar_right_offset).min(guard_right_stop);
    } else {
        // Static offsets if not curving
        x += guard_bar_front_offset;
        x -= guard_bar_back_offset;
        z += guard_bar_left_offset;
        z -= guard_bar_right_offset;
    }
    
    // ============ ELLIPTICAL CROSS-SECTION (x-z plane) ============
    // The bar has an elliptical profile with optional thickness scaling
    let radius_scaled = guard_bar_radius * guard_bar_thickness;
    let cross_section = (x.clone() * x.clone() / guard_bar_x_scale)
        + (z.clone() * z.clone() / guard_bar_z_scale)
        - radius_scaled;
    
    // ============ HEIGHT CONSTRAINT (y-axis bounds) ============
    // The bar starts at the guard (y = guard_handle_limit) and extends downward
    // for exactly guard_bar_height units.
    // Valid range: [bar_top_y, bar_bottom_y] where bar_top_y > bar_bottom_y
    let bar_top_y = guard_handle_limit - _guard_bottom - 1.5;  // Start right at guard reference point
    let bar_bottom_y = bar_top_y - guard_bar_height;

        // SDF constraint: distance to the valid y range
    // When iy is between bar_bottom_y and bar_top_y: constraint is negative (inside)
    // When iy is outside this range: constraint is positive (outside)
    let height_constraint = (iy.clone() - bar_top_y - 3.5).max(bar_bottom_y - iy.clone());
    
    // ============ COMBINE: Intersection of cross-section AND height ============
    // The bar exists where both constraints are simultaneously satisfied (both negative/zero)
    // In SDF space, intersection is represented by taking the max (union of boundaries)
    cross_section.max(height_constraint)
}

/// Flat rectangular guard plate (traditional flat slab shape)
fn guard_shape_flat(
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

/// Horseshoe-shaped guard plate (curved inward in x-z plane, concave)
/// Creates a U-shaped or horseshoe profile for better hand grip coverage
fn guard_shape_horseshoe(
    x: Tree,
    y: Tree,
    z: Tree,
    guard_bottom: f64,
    guard_front_stop: f64,
    guard_back_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    curvature: f64,
) -> Tree {
    // Base y constraint
    let y_constraint = y.clone() + guard_bottom;
    
    // Horseshoe is created by blending the plate stops with a curved surface
    // The curve contracts inward as you move toward the center (x-z plane)
    // This creates a bowl-like profile in the horizontal plane
    
    // Central distance from plate center (normalized, no sqrt/abs for speed)
    let x_scale = guard_front_stop.max(guard_back_stop).max(1.0);
    let z_scale = guard_left_stop.max(guard_right_stop).max(1.0);
    let x_norm = x.clone() / x_scale;
    let z_norm = z.clone() / z_scale;
    let radial_dist = x_norm.clone() * x_norm.clone() + z_norm.clone() * z_norm.clone();
    
    // Curvature factor: higher curvature = tighter inward curve
    // Clamp so stops never expand outward (prevents unbounded plate)
    let curve_factor = ((1.0 - (radial_dist.clone() * curvature)) as Tree).max(0.2);
    
    // Apply curvature to the plate stops
    let front_constraint = -x.clone() - guard_front_stop * curve_factor.clone();
    let back_constraint = x.clone() - guard_back_stop * curve_factor.clone();
    let left_constraint = -z.clone() - guard_left_stop * curve_factor.clone();
    let right_constraint = z.clone() - guard_right_stop * curve_factor.clone();
    
    y_constraint.max(front_constraint).max(left_constraint).max(right_constraint).max(back_constraint)
}

/// Spherical dome-shaped guard plate (curved up and inward)
/// Creates a protective dome that curves upward from edges
fn guard_shape_dome(
    x: Tree,
    y: Tree,
    z: Tree,
    guard_bottom: f64,
    guard_front_stop: f64,
    guard_back_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    curvature: f64,
) -> Tree {
    // Dome creates a spherical surface above the guard plate
    // The surface curves both inward (toward center) and upward (higher y)
    
    // Calculate radial distance in x-z plane (normalized, no sqrt/abs for speed)
    let x_scale = guard_front_stop.max(guard_back_stop).max(1.0);
    let z_scale = guard_left_stop.max(guard_right_stop).max(1.0);
    let x_norm = x.clone() / x_scale;
    let z_norm = z.clone() / z_scale;
    let radial_dist = x_norm.clone() * x_norm.clone() + z_norm.clone() * z_norm.clone();
    
    // Dome height decreases from center to edges
    // Using a simple quadratic dome: height = max_height * (1 - r^2)
    let max_dome_height = 2.0 * curvature;
    let dome_height = ((1.0 - radial_dist.clone()) as Tree).max(0.0) * max_dome_height;
    
    // The constraint: y would need to be below dome_surface to be inside
    let y_at_radius = guard_bottom - dome_height;
    
    // Combine with planar constraints for plate sides
    let y_constraint = (y.clone() + guard_bottom).max(y.clone() - y_at_radius);
    
    let front_constraint = -x.clone() - guard_front_stop;
    let back_constraint = x.clone() - guard_back_stop;
    let left_constraint = -z.clone() - guard_left_stop;
    let right_constraint = z.clone() - guard_right_stop;
    
    y_constraint.max(front_constraint).max(left_constraint).max(right_constraint).max(back_constraint)
}

/// Bowl-shaped guard plate (deep concave, curves both inward and downward)
/// Creates a protective bowl that deeply encompasses the hand
fn guard_shape_bowl(
    x: Tree,
    y: Tree,
    z: Tree,
    guard_bottom: f64,
    guard_front_stop: f64,
    guard_back_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    curvature: f64,
) -> Tree {
    // Bowl creates a deep concave surface
    // The surface curves inward (toward center) and can also curve downward
    
    // Normalize coordinates to [-1, 1] range based on plate extents
    let x_max = guard_front_stop.max(guard_back_stop).max(1.0);
    let z_max = guard_left_stop.max(guard_right_stop).max(1.0);
    let x_norm = x.clone() / x_max;
    let z_norm = z.clone() / z_max;
    
    // Radial distance from center (squared, no sqrt for speed)
    let radial_dist = x_norm.clone() * x_norm.clone() + z_norm.clone() * z_norm.clone();
    
    // Bowl depth increases with curvature
    // At center (r=0): depth = 0
    // At edges (r=1): depth = bowl_depth
    let max_bowl_depth = 3.0 * curvature;
    let bowl_depth = radial_dist.clone() * max_bowl_depth;
    
    // Y constraint: bowl surface slopes downward from center
    let bowl_surface_y = guard_bottom + bowl_depth;
    let y_constraint = y.clone() - bowl_surface_y;
    
    // Planar constraints are curved by the radial distance
    // At center, stops are minimal; at edges, stops expand
    let stop_scale: Tree = 1.0 + radial_dist.clone() * (curvature - 1.0).max(0.0);
    
    let front_constraint = -x.clone() - guard_front_stop * stop_scale.clone();
    let back_constraint = x.clone() - guard_back_stop * stop_scale.clone();
    let left_constraint = -z.clone() - guard_left_stop * stop_scale.clone();
    let right_constraint = z.clone() - guard_right_stop * stop_scale.clone();
    
    y_constraint.max(front_constraint).max(left_constraint).max(right_constraint).max(back_constraint)
}

/// Upturned guard plate (edges curve upward away from the handle)
/// Creates a flared, wing-like silhouette while keeping the center flat
fn guard_shape_upturned(
    x: Tree,
    y: Tree,
    z: Tree,
    guard_bottom: f64,
    guard_front_stop: f64,
    guard_back_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    curvature: f64,
) -> Tree {
    // Normalize coordinates to [-1, 1] range based on plate extents
    let x_max = guard_front_stop.max(guard_back_stop).max(1.0);
    let z_max = guard_left_stop.max(guard_right_stop).max(1.0);
    let x_norm = x.clone() / x_max;
    let z_norm = z.clone() / z_max;

    // Radial distance from center (squared, no sqrt for speed)
    let radial_dist = x_norm.clone() * x_norm.clone() + z_norm.clone() * z_norm.clone();

    // Upturn height increases toward the edges
    let max_upturn = 2.0 * curvature;
    let upturn = radial_dist.clone() * max_upturn;

    // Y constraint: edges can rise higher than the center
    let y_constraint = y.clone() + guard_bottom - upturn;

    let front_constraint = -x.clone() - guard_front_stop;
    let back_constraint = x.clone() - guard_back_stop;
    let left_constraint = -z.clone() - guard_left_stop;
    let right_constraint = z.clone() - guard_right_stop;

    y_constraint.max(front_constraint).max(left_constraint).max(right_constraint).max(back_constraint)
}

/// Dispatcher function that selects the appropriate guard shape based on type
fn guard_shape(
    x: Tree,
    y: Tree,
    z: Tree,
    guard_bottom: f64,
    guard_front_stop: f64,
    guard_back_stop: f64,
    guard_left_stop: f64,
    guard_right_stop: f64,
    guard_plate_shape: GuardPlateShape,
    guard_plate_curvature: f64,
) -> Tree {
    match guard_plate_shape {
        GuardPlateShape::Flat => {
            guard_shape_flat(
                x, y, z,
                guard_bottom,
                guard_front_stop, guard_back_stop,
                guard_left_stop, guard_right_stop,
            )
        }
        GuardPlateShape::Horseshoe => {
            guard_shape_horseshoe(
                x, y, z,
                guard_bottom,
                guard_front_stop, guard_back_stop,
                guard_left_stop, guard_right_stop,
                guard_plate_curvature,
            )
        }
        GuardPlateShape::Dome => {
            guard_shape_dome(
                x, y, z,
                guard_bottom,
                guard_front_stop, guard_back_stop,
                guard_left_stop, guard_right_stop,
                guard_plate_curvature,
            )
        }
        GuardPlateShape::Bowl => {
            guard_shape_bowl(
                x, y, z,
                guard_bottom,
                guard_front_stop, guard_back_stop,
                guard_left_stop, guard_right_stop,
                guard_plate_curvature,
            )
        }
        GuardPlateShape::Upturned => {
            guard_shape_upturned(
                x, y, z,
                guard_bottom,
                guard_front_stop, guard_back_stop,
                guard_left_stop, guard_right_stop,
                guard_plate_curvature,
            )
        }
    }
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
    guard_bar_y_scale: f64,
    guard_bar_thickness: f64,
    guard_bar_height: f64,
    has_guard_bar: bool,
    guard_plate_shape: GuardPlateShape,
    guard_plate_curvature: f64,
) -> Tree {
    let guard = guard_shape(
        x.clone(),
        y.clone(),
        z.clone(),
        guard_bottom,
        guard_front_stop,
        guard_back_stop,
        guard_left_stop,
        guard_right_stop,
        guard_plate_shape,
        guard_plate_curvature,
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
    ));
    if !has_guard_bar {
        return guard;
    }
    guard.min(guard_bar(
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
        guard_bar_z_scale,
        guard_bar_y_scale,
        guard_bar_thickness,
        guard_bar_height,
        has_guard_bar
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
    guard_bar_z_scale: f64,
    guard_bar_y_scale: f64,
    guard_bar_thickness: f64,
    guard_bar_height: f64,
    has_guard: bool,
    blade_bottom: f64,
    has_guard_bar: bool,
    guard_plate_shape: GuardPlateShape,
    guard_plate_curvature: f64,
) -> Tree {
    let handle = handle(
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
    );
    if has_guard {
        return handle
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
                guard_bar_z_scale,
                guard_bar_y_scale,
                guard_bar_thickness,
                guard_bar_height,
                has_guard_bar,
                guard_plate_shape,
                guard_plate_curvature,
            ))
            .max(0.0 - (blade_bottom - y.clone()) + 0);
    }
    handle.max(0.0 - (blade_bottom - y.clone()) + 0)
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

// Creates a blade with beveled/tapered edges controlled by scale_decrement parameters
// 
// BLADE EDGE SHARPNESS CONTROL (blade_scale_*_decrement parameters):
// These control how sharp or dull each edge quadrant is:
//   - front_left: front edge on positive z side
//   - front_right: front edge on negative z side  
//   - back_left: back edge on positive z side
//   - back_right: back edge on negative z side
//
// VALUE MEANINGS:
//   Positive values (1.0 to 10.0+): Sharper edge - cuts away more material
//   Zero (0.0): No beveling - flat edge
//   Negative values (-1.0 to -10.0): Thicker edge - adds material back
//
// TYPICAL VALUES:
//   Sharp sword edge: 3.0 to 6.0
//   Dull spine: -3.0 to -6.0
//   Blunt/thick edge: -5.0 to -10.0
//
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
    blade_serated_left: bool,
    blade_serated_left_count: f64,
    blade_serated_left_size: f64,
    blade_serated_right: bool,
    blade_serated_right_count: f64,
    blade_serated_right_size: f64,
    blade_spiked_left: bool,
    blade_spiked_left_count: f64,
    blade_spiked_left_size: f64,
    blade_spiked_right: bool,
    blade_spiked_right_count: f64,
    blade_spiked_right_size: f64,
) -> Tree {
    let z_curved = apply_curvature(y.clone(), z.clone(), blade_curvature, blade_lean);

    // Determine base blade edge for positive z (left side)
    let mut blade_left_edge = blade(
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
    );
    
    // Apply left edge texture if enabled
    if blade_spiked_left || blade_serated_left {
        let height_taper = ((blade_height - y.clone()) / blade_height).max(0.0).sqrt();
        let left_spike_offset = if blade_spiked_left {
            let period = blade_spiked_left_count;
            let phase = y.clone().modulo(period);
            let spike_wave = (phase.clone() / period * 2.0 - 1.0).abs();
            (1.0 - spike_wave) * blade_spiked_left_size * height_taper.clone()
        } else {
            let seration_wave = (y.clone() * PI / blade_serated_left_count).sin();
            seration_wave * blade_serated_left_size * height_taper.clone()
        };
        blade_left_edge = blade_left_edge.max(z_curved.clone() - left_spike_offset);
    }

    // Determine base blade edge for negative z (right side)
    let mut blade_right_edge = blade(
        x.clone(),
        y.clone(),
        -z.clone(),
        blade_radius,
        blade_back_width,
        blade_front_width,
        blade_left_top_slope,
        blade_right_top_slope,
        blade_height,
        blade_curvature,
        blade_lean,
    );
    
    // Apply right edge texture if enabled
    if blade_spiked_right || blade_serated_right {
        let height_taper = ((blade_height - y.clone()) / blade_height).max(0.0).sqrt();
        let right_spike_offset = if blade_spiked_right {
            let period = blade_spiked_right_count;
            let phase = y.clone().modulo(period);
            let spike_wave = (phase.clone() / period * 2.0 - 1.0).abs();
            (1.0 - spike_wave) * blade_spiked_right_size * height_taper.clone()
        } else {
            let seration_wave = (y.clone() * PI / blade_serated_right_count).sin();
            seration_wave * blade_serated_right_size * height_taper.clone()
        };
        blade_right_edge = blade_right_edge.max(-z_curved.clone() - right_spike_offset);
    }

    // Constrain to each side's half-space and combine
    let blade_left = blade_left_edge.max(-z.clone());
    let blade_right = blade_right_edge.max(z.clone());
    let blade = blade_left.min(blade_right);

    // Apply edge beveling/sharpness modifiers
    // Each decrement creates a cutting plane from a corner of the blade cross-section
    // The four v_decrease modifiers (combined with min) create the cross-sectional bevel
    // The four d_decrease modifiers create lengthwise tapers toward the tip
    blade
        .max(
            -(blade_scale_v_decrease_modifier(
                x.clone(),
                -z_curved.clone(),
                blade_radius,
                blade_scale_back_right_decrement,  // Back-right corner sharpness
            )
            .min(
                blade_scale_v_decrease_modifier(
                    x.clone(),
                    z_curved.clone(),
                    blade_radius,
                    blade_scale_back_left_decrement,  // Back-left corner sharpness
                )
                .min(
                    blade_scale_v_decrease_modifier(
                        -x.clone(),
                        z_curved.clone(),
                        blade_radius,
                        blade_scale_front_left_decrement,  // Front-left corner sharpness
                    )
                    .min(blade_scale_v_decrease_modifier(
                        -x.clone(),
                        -z_curved.clone(),
                        blade_radius,
                        blade_scale_front_right_decrement,  // Front-right corner sharpness
                    )),
                ),
            )),
        )
        .max(blade_scale_d_decrease_modifier(
            x.clone() * blade_right_top_slope,
            y.clone(),
            z_curved.clone(),
            blade_height,
            blade_scale_back_left_decrement,  // Lengthwise taper on back-left
        ))
        .max(blade_scale_d_decrease_modifier(
            -x.clone() * blade_right_top_slope,
            y.clone(),
            z_curved.clone(),
            blade_height,
            blade_scale_front_left_decrement,  // Lengthwise taper on front-left
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

// Vertical cross-section edge bevel modifier
// Creates a cutting plane from a corner of the blade cross-section
// Parameters:
//   x, z: position coordinates (one may be negated to select corner)
//   blade_radius: base blade thickness
//   blade_scale_decrement: controls bevel angle
//     - Higher positive = steeper bevel = sharper edge
//     - Negative = inverted bevel = thicker edge
fn blade_scale_v_decrease_modifier(
    x: Tree,
    z: Tree,
    blade_radius: f64,
    blade_scale_decrement: f64,
) -> Tree {
    // Creates a cutting plane for edge sharpness
    // Higher decrement = steeper plane = sharper edge
    x.clone() * blade_scale_decrement + z.clone() * blade_scale_decrement + blade_radius
}

// Diagonal lengthwise taper modifier  
// Creates a cutting plane along the blade length toward the tip
// Parameters:
//   x, z: position coordinates scaled by top slope
//   y: height along blade
//   blade_height: total blade length
//   blade_scale_decrement: controls taper angle
fn blade_scale_d_decrease_modifier(
    x: Tree,
    y: Tree,
    z: Tree,
    blade_height: f64,
    blade_scale_decrement: f64,
) -> Tree {
    // Creates a diagonal cutting plane along blade length
    // Higher decrement = sharper taper toward tip
    x.clone() * blade_scale_decrement + z.clone() * blade_scale_decrement + y.clone() - blade_height
}

fn apply_curvature(y: Tree, z: Tree, blade_curvature: f64, blade_lean: f64) -> Tree {
    if blade_lean == 0.0 {
        z
    } else {
        let curve_shape = (y / blade_lean).pow(2);
        z + curve_shape * blade_curvature
    }
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
    guard_bar_z_scale: f64,
    guard_bar_y_scale: f64,
    guard_bar_thickness: f64,
    guard_bar_height: f64,
    has_guard: bool,
    has_guard_bar: bool,
    blade_serated_left: bool,
    blade_serated_left_count: f64,
    blade_serated_left_size: f64,
    blade_serated_right: bool,
    blade_serated_right_count: f64,
    blade_serated_right_size: f64,
    blade_spiked_left: bool,
    blade_spiked_left_count: f64,
    blade_spiked_left_size: f64,
    blade_spiked_right: bool,
    blade_spiked_right_count: f64,
    blade_spiked_right_size: f64,
    guard_plate_shape: GuardPlateShape,
    guard_plate_curvature: f64,
) -> Tree {
    // let blade_left_shift =
    //     ((blade_scale_back_left_decrement + blade_scale_front_left_decrement) / 2.0) / 20.0;
    // let blade_right_shift =
    //     ((blade_scale_back_right_decrement + blade_scale_front_right_decrement) / 2.0) / 20.0;
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
            guard_bar_z_scale,
            guard_bar_y_scale,
            guard_bar_thickness,
            guard_bar_height,
            has_guard,
            blade_bottom,
            has_guard_bar,
            guard_plate_shape,
            guard_plate_curvature,
        )
        .min(blade_scale_decrease(
            x.clone(),
            y.clone() - blade_bottom,
            // z.clone() - blade_right_shift + blade_left_shift,
            z.clone(),
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
            blade_serated_left,
            blade_serated_left_count,
            blade_serated_left_size,
            blade_serated_right,
            blade_serated_right_count,
            blade_serated_right_size,
            blade_spiked_left,
            blade_spiked_left_count,
            blade_spiked_left_size,
            blade_spiked_right,
            blade_spiked_right_count,
            blade_spiked_right_size,
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
                guard_bar_z_scale,
                guard_bar_y_scale,
                guard_bar_thickness,
                guard_bar_height,
                has_guard,
                blade_bottom,
                has_guard_bar,
                guard_plate_shape,
                guard_plate_curvature,
            )
            .min(blade_scale_decrease(
                x.clone(),
                2.0 * handle_bottom_limit - (y.clone() + blade_bottom),
                // 2.0 * 0.0 - z.clone() - blade_right_shift + blade_left_shift,
                2.0 * 0.0 - z.clone(),
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
                blade_serated_left,
                blade_serated_left_count,
                blade_serated_left_size,
                blade_serated_right,
                blade_serated_right_count,
                blade_serated_right_size,
                blade_spiked_left,
                blade_spiked_left_count,
                blade_spiked_left_size,
                blade_spiked_right,
                blade_spiked_right_count,
                blade_spiked_right_size,
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
            guard_bar_z_scale,
            guard_bar_y_scale,
            guard_bar_thickness,
            guard_bar_height,
            has_guard,
            blade_bottom,
            has_guard_bar,
            guard_plate_shape,
            guard_plate_curvature
        )
        .min(blade_scale_decrease(
            x.clone(),
            y.clone() - blade_bottom,
            // z.clone() - blade_right_shift + blade_left_shift,
            z.clone(),
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
            blade_serated_left,
            blade_serated_left_count,
            blade_serated_left_size,
            blade_serated_right,
            blade_serated_right_count,
            blade_serated_right_size,
            blade_spiked_left,
            blade_spiked_left_count,
            blade_spiked_left_size,
            blade_spiked_right,
            blade_spiked_right_count,
            blade_spiked_right_size,
        ))
    }
}
