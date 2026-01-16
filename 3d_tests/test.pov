#include "colors.inc"

camera { location <30, 30, 30> look_at <0, 25, 0> angle 60 }
light_source { <10, 20, 10> White }
background { Blue }

#declare pommel_shift = -0.5;
#declare pommel_extension = 4.5;
#declare pommel_radius = -3.5;
#declare handle_bottom_limit = 5.5;
#declare handle_radius = 0.5;
#declare handle_spiral_offset = 0.2;

#declare guard_bottom = 4;
#declare guard_height_size = 10;
#declare guard_height = 20;
#declare guard_front_stop = 2;
#declare guard_back_stop = 2;
#declare guard_effect_radius = 2;
#declare guard_x_offset = 2;
#declare guard_z_offset = 2;
#declare guard_z_scale = 3;

#declare blade_bottom = 15;
#declare blade_radius = 2;
#declare blade_front_width = 1;
#declare blade_back_width = 1;
#declare left_blade_top_slope = 2;
#declare right_blade_top_slope = 2;

#declare fn_handle_mid = function {max(x*x + z*z - handle_radius, handle_bottom_limit - y)}
#declare fn_handle_pommel = function {max(x*x + z*z - (y + pommel_shift) - pommel_radius, (y + pommel_shift) - pommel_extension)}
#declare fn_handle_spiral = function {max(max(max((x*x + z*z) - (handle_radius + handle_spiral_offset), sin((x * x - (y) * (y)))), -(x*x+z*z-handle_radius)), -y+handle_bottom_limit)}

#declare fn_handle = function {min(fn_handle_mid(x,y,z), fn_handle_pommel(x,y+pommel_shift,z), fn_handle_spiral(x, y, z))}


#declare fn_hand_guard_shape = function {max(y + guard_bottom, max((-x - guard_front_stop), (x - guard_back_stop)))}
#declare fn_hand_guard_curve_effect = function {(x*x+z*z)-guard_effect_radius}

#declare fn_hand_guard = function {max(fn_hand_guard_shape(x,y,z), -fn_hand_guard_curve_effect(x + guard_x_offset, y, (z / guard_z_scale) + guard_z_offset), 
-fn_hand_guard_curve_effect(x + guard_x_offset, y, (z / guard_z_scale) - guard_z_offset),
-fn_hand_guard_curve_effect(x - guard_x_offset, y, (z / guard_z_scale) - guard_z_offset),
-fn_hand_guard_curve_effect(x - guard_x_offset, y, (z / guard_z_scale) + guard_z_offset))}

#declare fn_handle_and_guard = function {max(min(fn_handle(x, y, z), fn_hand_guard(x, (guard_height_size + guard_height - y) - guard_height, z)), -(blade_bottom-y))}


#declare fn_blade = function {max(x*x+z*z-blade_radius, 
-(y-blade_bottom), 
-(x+blade_back_width), 
-(-x+blade_front_width), 
-(z*left_blade_top_slope + -(y-40)),
-(-z*right_blade_top_slope + -(y-40)))}


isosurface {
  function {min(fn_handle_and_guard(x, y, z), fn_blade(x*(3+(y/10)), y, z))} 
  threshold 0
  contained_by { box { <5, 40, 7>, <-5, -5, -7> } }
  max_gradient 50
  pigment { Red } 
}