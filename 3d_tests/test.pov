#include "colors.inc"

camera { location <10, 50, 10> look_at <0, 30, 0> }
light_source { <10, 10, 10> White }
background { Blue }

#declare pommel_shift = 2.5;
#declare pommel_extension = 4.5;
#declare pommel_radius = -2.5;
#declare handle_bottom_limit = -0.5;
#declare handle_radius = 2;
#declare handle_spiral_offset = 0.2;

#declare guard_effect_radius = 2;
#declare guard_x_offset = 2;
#declare guard_z_offset = 2;
#declare guard_z_scale = 3;

#declare fn_handle_mid = function {max(x*x + z*z - handle_radius, handle_bottom_limit - y)}
#declare fn_handle_pommel = function {max(x*x + z*z - (y + pommel_shift) - pommel_radius, (y + pommel_shift) - pommel_extension)}
#declare fn_handle_spiral = function {max(max(max((x*x + z*z) - (handle_radius + handle_spiral_offset), sin((x * x - (y) * (y)))), -(x*x+z*z-handle_radius)), -y)}

#declare fn_handle = function {min(fn_handle_mid(x,y,z), fn_handle_pommel(x,y+pommel_shift,z), fn_handle_spiral(x, y, z))}


#declare fn_hand_guard_shape = function {max(y + 3, max((-x - 1.5), (x - 1.5)))}
#declare fn_hand_guard_curve_effect = function {(x*x+z*z)-guard_effect_radius}
#declare fn_hand_guard = function {max(fn_hand_guard_shape(x,y,z), -fn_hand_guard_curve_effect(x + guard_x_offset, y, (z / guard_z_scale) + guard_z_offset), -fn_hand_guard_curve_effect(x + guard_x_offset, y, (z / guard_z_scale) - guard_z_offset),-fn_hand_guard_curve_effect(x - guard_x_offset, y, (z / guard_z_scale) - guard_z_offset),-fn_hand_guard_curve_effect(x - guard_x_offset, y, (z / guard_z_scale) + guard_z_offset))}

#declare fn_handle_and_guard = function {max(min(fn_handle(x, y, z), fn_hand_guard(x, (30 - y) - 20, z)), -(15-y))}


#declare fn_blade = function {max(x*x+z*z-2, -(y-15), -(x+1), -(-x+1), -(z + -(y-40)),-(-z + -(y-40)))}



isosurface {
  function {min(fn_handle_and_guard(x, y, z), fn_blade(x*(3+(y/10)), y, z))} 
  threshold 0
  contained_by { box { <5, 40, 8>, <-5, -5, -8> } }
  max_gradient 50
  pigment { Red } 
}