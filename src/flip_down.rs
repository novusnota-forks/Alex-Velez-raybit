use crate::BaseType;

pub unsafe fn call(memory: &mut Vec<BaseType>, pointer: usize) {
    if let Some(result_cells) = match memory[pointer] {
        0 => set_shapes_texture(memory, pointer),
        1 => draw_pixel(memory, pointer),
        2 => draw_pixel_v(memory, pointer),
        3 => draw_line(memory, pointer),
        4 => draw_line_v(memory, pointer),
        5 => draw_line_ex(memory, pointer),
        6 => draw_line_strip(memory, pointer),
        7 => draw_line_bezier(memory, pointer),
        8 => draw_circle(memory, pointer),
        9 => draw_circle_sector(memory, pointer),
        10 => draw_circle_sector_lines(memory, pointer),
        11 => draw_circle_gradient(memory, pointer),
        12 => draw_circle_v(memory, pointer),
        13 => draw_circle_lines(memory, pointer),
        14 => draw_circle_lines_v(memory, pointer),
        15 => draw_ellipse(memory, pointer),
        16 => draw_ellipse_lines(memory, pointer),
        17 => draw_ring(memory, pointer),
        18 => draw_ring_lines(memory, pointer),
        19 => draw_rectangle(memory, pointer),
        20 => draw_rectangle_v(memory, pointer),
        21 => draw_rectangle_rec(memory, pointer),
        22 => draw_rectangle_pro(memory, pointer),
        23 => draw_rectangle_gradient_v(memory, pointer),
        24 => draw_rectangle_gradient_h(memory, pointer),
        25 => draw_rectangle_gradient_ex(memory, pointer),
        26 => draw_rectangle_lines(memory, pointer),
        27 => draw_rectangle_lines_ex(memory, pointer),
        28 => draw_rectangle_rounded(memory, pointer),
        29 => draw_rectangle_rounded_lines(memory, pointer),
        30 => draw_triangle(memory, pointer),
        31 => draw_triangle_lines(memory, pointer),
        32 => draw_triangle_fan(memory, pointer),
        33 => draw_triangle_strip(memory, pointer),
        34 => draw_poly(memory, pointer),
        35 => draw_poly_lines(memory, pointer),
        36 => draw_poly_lines_ex(memory, pointer),
        37 => draw_spline_linear(memory, pointer),
        38 => draw_spline_basis(memory, pointer),
        39 => draw_spline_catmull_rom(memory, pointer),
        40 => draw_spline_bezier_quadratic(memory, pointer),
        41 => draw_spline_bezier_cubic(memory, pointer),
        42 => draw_spline_segment_linear(memory, pointer),
        43 => draw_spline_segment_basis(memory, pointer),
        44 => draw_spline_segment_catmull_rom(memory, pointer),
        45 => draw_spline_segment_bezier_quadratic(memory, pointer),
        46 => draw_spline_segment_bezier_cubic(memory, pointer),
        47 => get_spline_point_linear(memory, pointer),
        48 => get_spline_point_basis(memory, pointer),
        49 => get_spline_point_catmull_rom(memory, pointer),
        50 => get_spline_point_bezier_quad(memory, pointer),
        51 => get_spline_point_bezier_cubic(memory, pointer),
        52 => check_collision_recs(memory, pointer),
        53 => check_collision_circles(memory, pointer),
        54 => check_collision_circle_rec(memory, pointer),
        55 => check_collision_point_rec(memory, pointer),
        56 => check_collision_point_circle(memory, pointer),
        57 => check_collision_point_triangle(memory, pointer),
        58 => check_collision_point_poly(memory, pointer),
        59 => check_collision_lines(memory, pointer),
        60 => check_collision_point_line(memory, pointer),
        61 => get_collision_rec(memory, pointer),
        62 => draw_line3_d(memory, pointer),
        63 => draw_point3_d(memory, pointer),
        64 => draw_circle3_d(memory, pointer),
        65 => draw_triangle3_d(memory, pointer),
        66 => draw_triangle_strip3_d(memory, pointer),
        67 => draw_cube(memory, pointer),
        68 => draw_cube_v(memory, pointer),
        69 => draw_cube_wires(memory, pointer),
        70 => draw_cube_wires_v(memory, pointer),
        71 => draw_sphere(memory, pointer),
        72 => draw_sphere_ex(memory, pointer),
        73 => draw_sphere_wires(memory, pointer),
        74 => draw_cylinder(memory, pointer),
        75 => draw_cylinder_ex(memory, pointer),
        76 => draw_cylinder_wires(memory, pointer),
        77 => draw_cylinder_wires_ex(memory, pointer),
        78 => draw_capsule(memory, pointer),
        79 => draw_capsule_wires(memory, pointer),
        80 => draw_plane(memory, pointer),
        81 => draw_ray(memory, pointer),
        82 => draw_grid(memory, pointer),
        83 => load_model(memory, pointer),
        84 => load_model_from_mesh(memory, pointer),
        85 => is_model_ready(memory, pointer),
        86 => unload_model(memory, pointer),
        87 => get_model_bounding_box(memory, pointer),
        88 => draw_model(memory, pointer),
        89 => draw_model_ex(memory, pointer),
        90 => draw_model_wires(memory, pointer),
        91 => draw_model_wires_ex(memory, pointer),
        92 => draw_bounding_box(memory, pointer),
        93 => draw_billboard(memory, pointer),
        94 => draw_billboard_rec(memory, pointer),
        95 => draw_billboard_pro(memory, pointer),
        96 => upload_mesh(memory, pointer),
        97 => update_mesh_buffer(memory, pointer),
        98 => unload_mesh(memory, pointer),
        99 => draw_mesh(memory, pointer),
        100 => draw_mesh_instanced(memory, pointer),
        101 => export_mesh(memory, pointer),
        102 => get_mesh_bounding_box(memory, pointer),
        103 => gen_mesh_tangents(memory, pointer),
        104 => gen_mesh_poly(memory, pointer),
        105 => gen_mesh_plane(memory, pointer),
        106 => gen_mesh_cube(memory, pointer),
        107 => gen_mesh_sphere(memory, pointer),
        108 => gen_mesh_hemi_sphere(memory, pointer),
        109 => gen_mesh_cylinder(memory, pointer),
        110 => gen_mesh_cone(memory, pointer),
        111 => gen_mesh_torus(memory, pointer),
        112 => gen_mesh_knot(memory, pointer),
        113 => gen_mesh_heightmap(memory, pointer),
        114 => gen_mesh_cubicmap(memory, pointer),
        115 => load_materials(memory, pointer),
        116 => load_material_default(),
        117 => is_material_ready(memory, pointer),
        118 => unload_material(memory, pointer),
        119 => set_material_texture(memory, pointer),
        120 => set_model_mesh_material(memory, pointer),
        121 => load_model_animations(memory, pointer),
        122 => update_model_animation(memory, pointer),
        123 => unload_model_animation(memory, pointer),
        124 => unload_model_animations(memory, pointer),
        125 => is_model_animation_valid(memory, pointer),
        126 => check_collision_spheres(memory, pointer),
        127 => check_collision_boxes(memory, pointer),
        128 => check_collision_box_sphere(memory, pointer),
        129 => get_ray_collision_sphere(memory, pointer),
        130 => get_ray_collision_box(memory, pointer),
        131 => get_ray_collision_mesh(memory, pointer),
        132 => get_ray_collision_triangle(memory, pointer),
        133 => get_ray_collision_quad(memory, pointer),
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
unsafe fn load_material_default() -> Option<Vec<BaseType>> {
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
