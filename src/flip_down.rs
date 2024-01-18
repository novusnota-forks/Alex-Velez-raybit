use crate::BaseType;

const FN_COUNT: usize = 134;
const FN_MAP: [unsafe fn(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>>;
    FN_COUNT] = [
    set_shapes_texture,
    draw_pixel,
    draw_pixel_v,
    draw_line,
    draw_line_v,
    draw_line_ex,
    draw_line_strip,
    draw_line_bezier,
    draw_circle,
    draw_circle_sector,
    draw_circle_sector_lines,
    draw_circle_gradient,
    draw_circle_v,
    draw_circle_lines,
    draw_circle_lines_v,
    draw_ellipse,
    draw_ellipse_lines,
    draw_ring,
    draw_ring_lines,
    draw_rectangle,
    draw_rectangle_v,
    draw_rectangle_rec,
    draw_rectangle_pro,
    draw_rectangle_gradient_v,
    draw_rectangle_gradient_h,
    draw_rectangle_gradient_ex,
    draw_rectangle_lines,
    draw_rectangle_lines_ex,
    draw_rectangle_rounded,
    draw_rectangle_rounded_lines,
    draw_triangle,
    draw_triangle_lines,
    draw_triangle_fan,
    draw_triangle_strip,
    draw_poly,
    draw_poly_lines,
    draw_poly_lines_ex,
    draw_spline_linear,
    draw_spline_basis,
    draw_spline_catmull_rom,
    draw_spline_bezier_quadratic,
    draw_spline_bezier_cubic,
    draw_spline_segment_linear,
    draw_spline_segment_basis,
    draw_spline_segment_catmull_rom,
    draw_spline_segment_bezier_quadratic,
    draw_spline_segment_bezier_cubic,
    get_spline_point_linear,
    get_spline_point_basis,
    get_spline_point_catmull_rom,
    get_spline_point_bezier_quad,
    get_spline_point_bezier_cubic,
    check_collision_recs,
    check_collision_circles,
    check_collision_circle_rec,
    check_collision_point_rec,
    check_collision_point_circle,
    check_collision_point_triangle,
    check_collision_point_poly,
    check_collision_lines,
    check_collision_point_line,
    get_collision_rec,
    draw_line3_d,
    draw_point3_d,
    draw_circle3_d,
    draw_triangle3_d,
    draw_triangle_strip3_d,
    draw_cube,
    draw_cube_v,
    draw_cube_wires,
    draw_cube_wires_v,
    draw_sphere,
    draw_sphere_ex,
    draw_sphere_wires,
    draw_cylinder,
    draw_cylinder_ex,
    draw_cylinder_wires,
    draw_cylinder_wires_ex,
    draw_capsule,
    draw_capsule_wires,
    draw_plane,
    draw_ray,
    draw_grid,
    load_model,
    load_model_from_mesh,
    is_model_ready,
    unload_model,
    get_model_bounding_box,
    draw_model,
    draw_model_ex,
    draw_model_wires,
    draw_model_wires_ex,
    draw_bounding_box,
    draw_billboard,
    draw_billboard_rec,
    draw_billboard_pro,
    upload_mesh,
    update_mesh_buffer,
    unload_mesh,
    draw_mesh,
    draw_mesh_instanced,
    export_mesh,
    get_mesh_bounding_box,
    gen_mesh_tangents,
    gen_mesh_poly,
    gen_mesh_plane,
    gen_mesh_cube,
    gen_mesh_sphere,
    gen_mesh_hemi_sphere,
    gen_mesh_cylinder,
    gen_mesh_cone,
    gen_mesh_torus,
    gen_mesh_knot,
    gen_mesh_heightmap,
    gen_mesh_cubicmap,
    load_materials,
    load_material_default,
    is_material_ready,
    unload_material,
    set_material_texture,
    set_model_mesh_material,
    load_model_animations,
    update_model_animation,
    unload_model_animation,
    unload_model_animations,
    is_model_animation_valid,
    check_collision_spheres,
    check_collision_boxes,
    check_collision_box_sphere,
    get_ray_collision_sphere,
    get_ray_collision_box,
    get_ray_collision_mesh,
    get_ray_collision_triangle,
    get_ray_collision_quad,
];

pub unsafe fn call(memory: &mut Vec<BaseType>, pointer: usize) {
    if let Some(result_cells) = match memory[pointer] as usize {
        id if id <= FN_COUNT => FN_MAP[id](memory, pointer),
        _ => None,
    } {
        for x in 1..=result_cells.len() {
            (*memory)[pointer - x] = result_cells[x - 1];
        }
    }
}

/// Set texture and rectangle to be used on shapes drawing
unsafe fn set_shapes_texture(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a pixel
unsafe fn draw_pixel(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a pixel (Vector version)
unsafe fn draw_pixel_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a line
unsafe fn draw_line(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a line (using gl lines)
unsafe fn draw_line_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a line (using triangles/quads)
unsafe fn draw_line_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw lines sequence (using gl lines)
unsafe fn draw_line_strip(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw line segment cubic-bezier in-out interpolation
unsafe fn draw_line_bezier(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled circle
unsafe fn draw_circle(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a piece of a circle
unsafe fn draw_circle_sector(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw circle sector outline
unsafe fn draw_circle_sector_lines(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a gradient-filled circle
unsafe fn draw_circle_gradient(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled circle (Vector version)
unsafe fn draw_circle_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw circle outline
unsafe fn draw_circle_lines(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw circle outline (Vector version)
unsafe fn draw_circle_lines_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw ellipse
unsafe fn draw_ellipse(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw ellipse outline
unsafe fn draw_ellipse_lines(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw ring
unsafe fn draw_ring(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw ring outline
unsafe fn draw_ring_lines(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle
unsafe fn draw_rectangle(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle (Vector version)
unsafe fn draw_rectangle_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle
unsafe fn draw_rectangle_rec(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle with pro parameters
unsafe fn draw_rectangle_pro(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a vertical-gradient-filled rectangle
unsafe fn draw_rectangle_gradient_v(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a horizontal-gradient-filled rectangle
unsafe fn draw_rectangle_gradient_h(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a gradient-filled rectangle with custom vertex colors
unsafe fn draw_rectangle_gradient_ex(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle outline
unsafe fn draw_rectangle_lines(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle outline with extended parameters
unsafe fn draw_rectangle_lines_ex(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle with rounded edges
unsafe fn draw_rectangle_rounded(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle with rounded edges outline
unsafe fn draw_rectangle_rounded_lines(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
unsafe fn draw_triangle(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw triangle outline (vertex in counter-clockwise order!)
unsafe fn draw_triangle_lines(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a triangle fan defined by points (first vertex is the center)
unsafe fn draw_triangle_fan(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a triangle strip defined by points
unsafe fn draw_triangle_strip(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a regular polygon (Vector version)
unsafe fn draw_poly(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a polygon outline of n sides
unsafe fn draw_poly_lines(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a polygon outline of n sides with extended parameters
unsafe fn draw_poly_lines_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline: Linear, minimum 2 points
unsafe fn draw_spline_linear(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline: B-Spline, minimum 4 points
unsafe fn draw_spline_basis(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline: Catmull-Rom, minimum 4 points
unsafe fn draw_spline_catmull_rom(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
unsafe fn draw_spline_bezier_quadratic(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
unsafe fn draw_spline_bezier_cubic(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline segment: Linear, 2 points
unsafe fn draw_spline_segment_linear(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline segment: B-Spline, 4 points
unsafe fn draw_spline_segment_basis(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline segment: Catmull-Rom, 4 points
unsafe fn draw_spline_segment_catmull_rom(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
unsafe fn draw_spline_segment_bezier_quadratic(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw spline segment: Cubic Bezier, 2 points, 2 control points
unsafe fn draw_spline_segment_bezier_cubic(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Linear
unsafe fn get_spline_point_linear(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: B-Spline
unsafe fn get_spline_point_basis(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Catmull-Rom
unsafe fn get_spline_point_catmull_rom(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Quadratic Bezier
unsafe fn get_spline_point_bezier_quad(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Cubic Bezier
unsafe fn get_spline_point_bezier_cubic(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check collision between two rectangles
unsafe fn check_collision_recs(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check collision between two circles
unsafe fn check_collision_circles(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check collision between circle and rectangle
unsafe fn check_collision_circle_rec(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if point is inside rectangle
unsafe fn check_collision_point_rec(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if point is inside circle
unsafe fn check_collision_point_circle(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if point is inside a triangle
unsafe fn check_collision_point_triangle(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if point is within a polygon described by array of vertices
unsafe fn check_collision_point_poly(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
unsafe fn check_collision_lines(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
unsafe fn check_collision_point_line(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get collision rectangle for two rectangles collision
unsafe fn get_collision_rec(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a line in 3D world space
unsafe fn draw_line3_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a point in 3D space, actually a small line
unsafe fn draw_point3_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a circle in 3D world space
unsafe fn draw_circle3_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
unsafe fn draw_triangle3_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a triangle strip defined by points
unsafe fn draw_triangle_strip3_d(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw cube
unsafe fn draw_cube(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw cube (Vector version)
unsafe fn draw_cube_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw cube wires
unsafe fn draw_cube_wires(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw cube wires (Vector version)
unsafe fn draw_cube_wires_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw sphere
unsafe fn draw_sphere(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw sphere with extended parameters
unsafe fn draw_sphere_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw sphere wires
unsafe fn draw_sphere_wires(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a cylinder/cone
unsafe fn draw_cylinder(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a cylinder with base at startPos and top at endPos
unsafe fn draw_cylinder_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a cylinder/cone wires
unsafe fn draw_cylinder_wires(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a cylinder wires with base at startPos and top at endPos
unsafe fn draw_cylinder_wires_ex(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a capsule with the center of its sphere caps at startPos and endPos
unsafe fn draw_capsule(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
unsafe fn draw_capsule_wires(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a plane XZ
unsafe fn draw_plane(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a ray line
unsafe fn draw_ray(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a grid (centered at (0, 0, 0))
unsafe fn draw_grid(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load model from files (meshes and materials)
unsafe fn load_model(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load model from generated mesh (default material)
unsafe fn load_model_from_mesh(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a model is ready
unsafe fn is_model_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload model (including meshes) from memory (RAM and/or VRAM)
unsafe fn unload_model(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Compute model bounding box limits (considers all meshes)
unsafe fn get_model_bounding_box(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a model (with texture if set)
unsafe fn draw_model(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a model with extended parameters
unsafe fn draw_model_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a model wires (with texture if set)
unsafe fn draw_model_wires(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a model wires (with texture if set) with extended parameters
unsafe fn draw_model_wires_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw bounding box (wires)
unsafe fn draw_bounding_box(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a billboard texture
unsafe fn draw_billboard(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a billboard texture defined by source
unsafe fn draw_billboard_rec(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a billboard texture defined by source and rotation
unsafe fn draw_billboard_pro(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Upload mesh vertex data in GPU and provide VAO/VBO ids
unsafe fn upload_mesh(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Update mesh vertex data in GPU for a specific buffer index
unsafe fn update_mesh_buffer(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload mesh data from CPU and GPU
unsafe fn unload_mesh(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a 3d mesh with material and transform
unsafe fn draw_mesh(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw multiple mesh instances with material and different transforms
unsafe fn draw_mesh_instanced(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export mesh data to file, returns true on success
unsafe fn export_mesh(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Compute mesh bounding box limits
unsafe fn get_mesh_bounding_box(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Compute mesh tangents
unsafe fn gen_mesh_tangents(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate polygonal mesh
unsafe fn gen_mesh_poly(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate plane mesh (with subdivisions)
unsafe fn gen_mesh_plane(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate cuboid mesh
unsafe fn gen_mesh_cube(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate sphere mesh (standard sphere)
unsafe fn gen_mesh_sphere(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate half-sphere mesh (no bottom cap)
unsafe fn gen_mesh_hemi_sphere(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate cylinder mesh
unsafe fn gen_mesh_cylinder(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate cone/pyramid mesh
unsafe fn gen_mesh_cone(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate torus mesh
unsafe fn gen_mesh_torus(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate trefoil knot mesh
unsafe fn gen_mesh_knot(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate heightmap mesh from image data
unsafe fn gen_mesh_heightmap(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate cubes-based map mesh from image data
unsafe fn gen_mesh_cubicmap(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load materials from model file
unsafe fn load_materials(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
unsafe fn load_material_default(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::LoadMaterialDefault();
    None
}

/// Check if a material is ready
unsafe fn is_material_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload material from GPU memory (VRAM)
unsafe fn unload_material(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
unsafe fn set_material_texture(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set material for a mesh
unsafe fn set_model_mesh_material(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load model animations from file
unsafe fn load_model_animations(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Update model animation pose
unsafe fn update_model_animation(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload animation data
unsafe fn unload_model_animation(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload animation array data
unsafe fn unload_model_animations(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check model animation skeleton match
unsafe fn is_model_animation_valid(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check collision between two spheres
unsafe fn check_collision_spheres(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check collision between two bounding boxes
unsafe fn check_collision_boxes(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check collision between box and sphere
unsafe fn check_collision_box_sphere(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get collision info between ray and sphere
unsafe fn get_ray_collision_sphere(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get collision info between ray and box
unsafe fn get_ray_collision_box(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get collision info between ray and mesh
unsafe fn get_ray_collision_mesh(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get collision info between ray and triangle
unsafe fn get_ray_collision_triangle(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get collision info between ray and quad
unsafe fn get_ray_collision_quad(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}
