#![allow(unused_variables)]
use crate::RbfType;

pub unsafe fn call_raylib(memory: &mut Vec<RbfType>, pointer: usize) {
    if let Some(result_cells) = match memory[pointer] {
        -1 => init_window(memory, pointer),
        -2 => close_window(),
        -3 => window_should_close(),
        -4 => is_window_ready(),
        -5 => is_window_fullscreen(),
        -6 => is_window_hidden(),
        -7 => is_window_minimized(),
        -8 => is_window_maximized(),
        -9 => is_window_focused(),
        -10 => is_window_resized(),
        -11 => is_window_state(memory, pointer),
        -12 => set_window_state(memory, pointer),
        -13 => clear_window_state(memory, pointer),
        -14 => toggle_fullscreen(),
        -15 => toggle_borderless_windowed(),
        -16 => maximize_window(),
        -17 => minimize_window(),
        -18 => restore_window(),
        -19 => set_window_icon(memory, pointer),
        -20 => set_window_icons(memory, pointer),
        -21 => set_window_title(memory, pointer),
        -22 => set_window_position(memory, pointer),
        -23 => set_window_monitor(memory, pointer),
        -24 => set_window_min_size(memory, pointer),
        -25 => set_window_max_size(memory, pointer),
        -26 => set_window_size(memory, pointer),
        -27 => set_window_opacity(memory, pointer),
        -28 => set_window_focused(),
        -29 => get_window_handle(),
        -30 => get_screen_width(),
        -31 => get_screen_height(),
        -32 => get_render_width(),
        -33 => get_render_height(),
        -34 => get_monitor_count(),
        -35 => get_current_monitor(),
        -36 => get_monitor_position(memory, pointer),
        -37 => get_monitor_width(memory, pointer),
        -38 => get_monitor_height(memory, pointer),
        -39 => get_monitor_physical_width(memory, pointer),
        -40 => get_monitor_physical_height(memory, pointer),
        -41 => get_monitor_refresh_rate(memory, pointer),
        -42 => get_window_position(),
        -43 => get_window_scale_d_p_i(),
        -44 => get_monitor_name(memory, pointer),
        -45 => set_clipboard_text(memory, pointer),
        -46 => get_clipboard_text(),
        -47 => enable_event_waiting(),
        -48 => disable_event_waiting(),
        -49 => show_cursor(),
        -50 => hide_cursor(),
        -51 => is_cursor_hidden(),
        -52 => enable_cursor(),
        -53 => disable_cursor(),
        -54 => is_cursor_on_screen(),
        -55 => clear_background(memory, pointer),
        -56 => begin_drawing(),
        -57 => end_drawing(),
        -58 => begin_mode2_d(memory, pointer),
        -59 => end_mode2_d(),
        -60 => begin_mode3_d(memory, pointer),
        -61 => end_mode3_d(),
        -62 => begin_texture_mode(memory, pointer),
        -63 => end_texture_mode(),
        -64 => begin_shader_mode(memory, pointer),
        -65 => end_shader_mode(),
        -66 => begin_blend_mode(memory, pointer),
        -67 => end_blend_mode(),
        -68 => begin_scissor_mode(memory, pointer),
        -69 => end_scissor_mode(),
        -70 => begin_vr_stereo_mode(memory, pointer),
        -71 => end_vr_stereo_mode(),
        -72 => load_vr_stereo_config(memory, pointer),
        -73 => unload_vr_stereo_config(memory, pointer),
        -74 => load_shader(memory, pointer),
        -75 => load_shader_from_memory(memory, pointer),
        -76 => is_shader_ready(memory, pointer),
        -77 => get_shader_location(memory, pointer),
        -78 => get_shader_location_attrib(memory, pointer),
        -79 => set_shader_value(memory, pointer),
        -80 => set_shader_value_v(memory, pointer),
        -81 => set_shader_value_matrix(memory, pointer),
        -82 => set_shader_value_texture(memory, pointer),
        -83 => unload_shader(memory, pointer),
        -84 => get_mouse_ray(memory, pointer),
        -85 => get_camera_matrix(memory, pointer),
        -86 => get_camera_matrix2_d(memory, pointer),
        -87 => get_world_to_screen(memory, pointer),
        -88 => get_screen_to_world2_d(memory, pointer),
        -89 => get_world_to_screen_ex(memory, pointer),
        -90 => get_world_to_screen2_d(memory, pointer),
        -91 => set_target_f_p_s(memory, pointer),
        -92 => get_frame_time(),
        -93 => get_time(),
        -94 => get_f_p_s(),
        -95 => swap_screen_buffer(),
        -96 => poll_input_events(),
        -97 => wait_time(memory, pointer),
        -98 => set_random_seed(memory, pointer),
        -99 => get_random_value(memory, pointer),
        -100 => load_random_sequence(memory, pointer),
        -101 => unload_random_sequence(memory, pointer),
        -102 => take_screenshot(memory, pointer),
        -103 => set_config_flags(memory, pointer),
        -104 => open_u_r_l(memory, pointer),
        -105 => trace_log(memory, pointer),
        -106 => set_trace_log_level(memory, pointer),
        -107 => mem_alloc(memory, pointer),
        -108 => mem_realloc(memory, pointer),
        -109 => mem_free(memory, pointer),
        -110 => set_trace_log_callback(memory, pointer),
        -111 => set_load_file_data_callback(memory, pointer),
        -112 => set_save_file_data_callback(memory, pointer),
        -113 => set_load_file_text_callback(memory, pointer),
        -114 => set_save_file_text_callback(memory, pointer),
        -115 => load_file_data(memory, pointer),
        -116 => unload_file_data(memory, pointer),
        -117 => save_file_data(memory, pointer),
        -118 => export_data_as_code(memory, pointer),
        -119 => load_file_text(memory, pointer),
        -120 => unload_file_text(memory, pointer),
        -121 => save_file_text(memory, pointer),
        -122 => file_exists(memory, pointer),
        -123 => directory_exists(memory, pointer),
        -124 => is_file_extension(memory, pointer),
        -125 => get_file_length(memory, pointer),
        -126 => get_file_extension(memory, pointer),
        -127 => get_file_name(memory, pointer),
        -128 => get_file_name_without_ext(memory, pointer),
        -129 => get_directory_path(memory, pointer),
        -130 => get_prev_directory_path(memory, pointer),
        -131 => get_working_directory(),
        -132 => get_application_directory(),
        -133 => change_directory(memory, pointer),
        -134 => is_path_file(memory, pointer),
        -135 => load_directory_files(memory, pointer),
        -136 => load_directory_files_ex(memory, pointer),
        -137 => unload_directory_files(memory, pointer),
        -138 => is_file_dropped(),
        -139 => load_dropped_files(),
        -140 => unload_dropped_files(memory, pointer),
        -141 => get_file_mod_time(memory, pointer),
        -142 => compress_data(memory, pointer),
        -143 => decompress_data(memory, pointer),
        -144 => encode_data_base64(memory, pointer),
        -145 => decode_data_base64(memory, pointer),
        -146 => load_automation_event_list(memory, pointer),
        -147 => unload_automation_event_list(memory, pointer),
        -148 => export_automation_event_list(memory, pointer),
        -149 => set_automation_event_list(memory, pointer),
        -150 => set_automation_event_base_frame(memory, pointer),
        -151 => start_automation_event_recording(),
        -152 => stop_automation_event_recording(),
        -153 => play_automation_event(memory, pointer),
        -154 => is_key_pressed(memory, pointer),
        -155 => is_key_pressed_repeat(memory, pointer),
        -156 => is_key_down(memory, pointer),
        -157 => is_key_released(memory, pointer),
        -158 => is_key_up(memory, pointer),
        -159 => get_key_pressed(),
        -160 => get_char_pressed(),
        -161 => set_exit_key(memory, pointer),
        -162 => is_gamepad_available(memory, pointer),
        -163 => get_gamepad_name(memory, pointer),
        -164 => is_gamepad_button_pressed(memory, pointer),
        -165 => is_gamepad_button_down(memory, pointer),
        -166 => is_gamepad_button_released(memory, pointer),
        -167 => is_gamepad_button_up(memory, pointer),
        -168 => get_gamepad_button_pressed(),
        -169 => get_gamepad_axis_count(memory, pointer),
        -170 => get_gamepad_axis_movement(memory, pointer),
        -171 => set_gamepad_mappings(memory, pointer),
        -172 => is_mouse_button_pressed(memory, pointer),
        -173 => is_mouse_button_down(memory, pointer),
        -174 => is_mouse_button_released(memory, pointer),
        -175 => is_mouse_button_up(memory, pointer),
        -176 => get_mouse_x(),
        -177 => get_mouse_y(),
        -178 => get_mouse_position(),
        -179 => get_mouse_delta(),
        -180 => set_mouse_position(memory, pointer),
        -181 => set_mouse_offset(memory, pointer),
        -182 => set_mouse_scale(memory, pointer),
        -183 => get_mouse_wheel_move(),
        -184 => get_mouse_wheel_move_v(),
        -185 => set_mouse_cursor(memory, pointer),
        -186 => get_touch_x(),
        -187 => get_touch_y(),
        -188 => get_touch_position(memory, pointer),
        -189 => get_touch_point_id(memory, pointer),
        -190 => get_touch_point_count(),
        -191 => set_gestures_enabled(memory, pointer),
        -192 => is_gesture_detected(memory, pointer),
        -193 => get_gesture_detected(),
        -194 => get_gesture_hold_duration(),
        -195 => get_gesture_drag_vector(),
        -196 => get_gesture_drag_angle(),
        -197 => get_gesture_pinch_vector(),
        -198 => get_gesture_pinch_angle(),
        -199 => update_camera(memory, pointer),
        -200 => update_camera_pro(memory, pointer),
        -201 => set_shapes_texture(memory, pointer),
        -202 => draw_pixel(memory, pointer),
        -203 => draw_pixel_v(memory, pointer),
        -204 => draw_line(memory, pointer),
        -205 => draw_line_v(memory, pointer),
        -206 => draw_line_ex(memory, pointer),
        -207 => draw_line_strip(memory, pointer),
        -208 => draw_line_bezier(memory, pointer),
        -209 => draw_circle(memory, pointer),
        -210 => draw_circle_sector(memory, pointer),
        -211 => draw_circle_sector_lines(memory, pointer),
        -212 => draw_circle_gradient(memory, pointer),
        -213 => draw_circle_v(memory, pointer),
        -214 => draw_circle_lines(memory, pointer),
        -215 => draw_circle_lines_v(memory, pointer),
        -216 => draw_ellipse(memory, pointer),
        -217 => draw_ellipse_lines(memory, pointer),
        -218 => draw_ring(memory, pointer),
        -219 => draw_ring_lines(memory, pointer),
        -220 => draw_rectangle(memory, pointer),
        -221 => draw_rectangle_v(memory, pointer),
        -222 => draw_rectangle_rec(memory, pointer),
        -223 => draw_rectangle_pro(memory, pointer),
        -224 => draw_rectangle_gradient_v(memory, pointer),
        -225 => draw_rectangle_gradient_h(memory, pointer),
        -226 => draw_rectangle_gradient_ex(memory, pointer),
        -227 => draw_rectangle_lines(memory, pointer),
        -228 => draw_rectangle_lines_ex(memory, pointer),
        -229 => draw_rectangle_rounded(memory, pointer),
        -230 => draw_rectangle_rounded_lines(memory, pointer),
        -231 => draw_triangle(memory, pointer),
        -232 => draw_triangle_lines(memory, pointer),
        -233 => draw_triangle_fan(memory, pointer),
        -234 => draw_triangle_strip(memory, pointer),
        -235 => draw_poly(memory, pointer),
        -236 => draw_poly_lines(memory, pointer),
        -237 => draw_poly_lines_ex(memory, pointer),
        -238 => draw_spline_linear(memory, pointer),
        -239 => draw_spline_basis(memory, pointer),
        -240 => draw_spline_catmull_rom(memory, pointer),
        -241 => draw_spline_bezier_quadratic(memory, pointer),
        -242 => draw_spline_bezier_cubic(memory, pointer),
        -243 => draw_spline_segment_linear(memory, pointer),
        -244 => draw_spline_segment_basis(memory, pointer),
        -245 => draw_spline_segment_catmull_rom(memory, pointer),
        -246 => draw_spline_segment_bezier_quadratic(memory, pointer),
        -247 => draw_spline_segment_bezier_cubic(memory, pointer),
        -248 => get_spline_point_linear(memory, pointer),
        -249 => get_spline_point_basis(memory, pointer),
        -250 => get_spline_point_catmull_rom(memory, pointer),
        -251 => get_spline_point_bezier_quad(memory, pointer),
        -252 => get_spline_point_bezier_cubic(memory, pointer),
        -253 => check_collision_recs(memory, pointer),
        -254 => check_collision_circles(memory, pointer),
        -255 => check_collision_circle_rec(memory, pointer),
        -256 => check_collision_point_rec(memory, pointer),
        -257 => check_collision_point_circle(memory, pointer),
        -258 => check_collision_point_triangle(memory, pointer),
        -259 => check_collision_point_poly(memory, pointer),
        -260 => check_collision_lines(memory, pointer),
        -261 => check_collision_point_line(memory, pointer),
        -262 => get_collision_rec(memory, pointer),
        -263 => load_image(memory, pointer),
        -264 => load_image_raw(memory, pointer),
        -265 => load_image_svg(memory, pointer),
        -266 => load_image_anim(memory, pointer),
        -267 => load_image_from_memory(memory, pointer),
        -268 => load_image_from_texture(memory, pointer),
        -269 => load_image_from_screen(),
        -270 => is_image_ready(memory, pointer),
        -271 => unload_image(memory, pointer),
        -272 => export_image(memory, pointer),
        -273 => export_image_to_memory(memory, pointer),
        -274 => export_image_as_code(memory, pointer),
        -275 => gen_image_color(memory, pointer),
        -276 => gen_image_gradient_linear(memory, pointer),
        -277 => gen_image_gradient_radial(memory, pointer),
        -278 => gen_image_gradient_square(memory, pointer),
        -279 => gen_image_checked(memory, pointer),
        -280 => gen_image_white_noise(memory, pointer),
        -281 => gen_image_perlin_noise(memory, pointer),
        -282 => gen_image_cellular(memory, pointer),
        -283 => gen_image_text(memory, pointer),
        -284 => image_copy(memory, pointer),
        -285 => image_from_image(memory, pointer),
        -286 => image_text(memory, pointer),
        -287 => image_text_ex(memory, pointer),
        -288 => image_format(memory, pointer),
        -289 => image_to_p_o_t(memory, pointer),
        -290 => image_crop(memory, pointer),
        -291 => image_alpha_crop(memory, pointer),
        -292 => image_alpha_clear(memory, pointer),
        -293 => image_alpha_mask(memory, pointer),
        -294 => image_alpha_premultiply(memory, pointer),
        -295 => image_blur_gaussian(memory, pointer),
        -296 => image_resize(memory, pointer),
        -297 => image_resize_n_n(memory, pointer),
        -298 => image_resize_canvas(memory, pointer),
        -299 => image_mipmaps(memory, pointer),
        -300 => image_dither(memory, pointer),
        -301 => image_flip_vertical(memory, pointer),
        -302 => image_flip_horizontal(memory, pointer),
        -303 => image_rotate(memory, pointer),
        -304 => image_rotate_c_w(memory, pointer),
        -305 => image_rotate_c_c_w(memory, pointer),
        -306 => image_color_tint(memory, pointer),
        -307 => image_color_invert(memory, pointer),
        -308 => image_color_grayscale(memory, pointer),
        -309 => image_color_contrast(memory, pointer),
        -310 => image_color_brightness(memory, pointer),
        -311 => image_color_replace(memory, pointer),
        -312 => load_image_colors(memory, pointer),
        -313 => load_image_palette(memory, pointer),
        -314 => unload_image_colors(memory, pointer),
        -315 => unload_image_palette(memory, pointer),
        -316 => get_image_alpha_border(memory, pointer),
        -317 => get_image_color(memory, pointer),
        -318 => image_clear_background(memory, pointer),
        -319 => image_draw_pixel(memory, pointer),
        -320 => image_draw_pixel_v(memory, pointer),
        -321 => image_draw_line(memory, pointer),
        -322 => image_draw_line_v(memory, pointer),
        -323 => image_draw_circle(memory, pointer),
        -324 => image_draw_circle_v(memory, pointer),
        -325 => image_draw_circle_lines(memory, pointer),
        -326 => image_draw_circle_lines_v(memory, pointer),
        -327 => image_draw_rectangle(memory, pointer),
        -328 => image_draw_rectangle_v(memory, pointer),
        -329 => image_draw_rectangle_rec(memory, pointer),
        -330 => image_draw_rectangle_lines(memory, pointer),
        -331 => image_draw(memory, pointer),
        -332 => image_draw_text(memory, pointer),
        -333 => image_draw_text_ex(memory, pointer),
        -334 => load_texture(memory, pointer),
        -335 => load_texture_from_image(memory, pointer),
        -336 => load_texture_cubemap(memory, pointer),
        -337 => load_render_texture(memory, pointer),
        -338 => is_texture_ready(memory, pointer),
        -339 => unload_texture(memory, pointer),
        -340 => is_render_texture_ready(memory, pointer),
        -341 => unload_render_texture(memory, pointer),
        -342 => update_texture(memory, pointer),
        -343 => update_texture_rec(memory, pointer),
        -344 => gen_texture_mipmaps(memory, pointer),
        -345 => set_texture_filter(memory, pointer),
        -346 => set_texture_wrap(memory, pointer),
        -347 => draw_texture(memory, pointer),
        -348 => draw_texture_v(memory, pointer),
        -349 => draw_texture_ex(memory, pointer),
        -350 => draw_texture_rec(memory, pointer),
        -351 => draw_texture_pro(memory, pointer),
        -352 => draw_texture_n_patch(memory, pointer),
        -353 => fade(memory, pointer),
        -354 => color_to_int(memory, pointer),
        -355 => color_normalize(memory, pointer),
        -356 => color_from_normalized(memory, pointer),
        -357 => color_to_h_s_v(memory, pointer),
        -358 => color_from_h_s_v(memory, pointer),
        -359 => color_tint(memory, pointer),
        -360 => color_brightness(memory, pointer),
        -361 => color_contrast(memory, pointer),
        -362 => color_alpha(memory, pointer),
        -363 => color_alpha_blend(memory, pointer),
        -364 => get_color(memory, pointer),
        -365 => get_pixel_color(memory, pointer),
        -366 => set_pixel_color(memory, pointer),
        -367 => get_pixel_data_size(memory, pointer),
        -368 => get_font_default(),
        -369 => load_font(memory, pointer),
        -370 => load_font_ex(memory, pointer),
        -371 => load_font_from_image(memory, pointer),
        -372 => load_font_from_memory(memory, pointer),
        -373 => is_font_ready(memory, pointer),
        -374 => load_font_data(memory, pointer),
        -375 => gen_image_font_atlas(memory, pointer),
        -376 => unload_font_data(memory, pointer),
        -377 => unload_font(memory, pointer),
        -378 => export_font_as_code(memory, pointer),
        -379 => draw_f_p_s(memory, pointer),
        -380 => draw_text(memory, pointer),
        -381 => draw_text_ex(memory, pointer),
        -382 => draw_text_pro(memory, pointer),
        -383 => draw_text_codepoint(memory, pointer),
        -384 => draw_text_codepoints(memory, pointer),
        -385 => set_text_line_spacing(memory, pointer),
        -386 => measure_text(memory, pointer),
        -387 => measure_text_ex(memory, pointer),
        -388 => get_glyph_index(memory, pointer),
        -389 => get_glyph_info(memory, pointer),
        -390 => get_glyph_atlas_rec(memory, pointer),
        -391 => load_u_t_f8(memory, pointer),
        -392 => unload_u_t_f8(memory, pointer),
        -393 => load_codepoints(memory, pointer),
        -394 => unload_codepoints(memory, pointer),
        -395 => get_codepoint_count(memory, pointer),
        -396 => get_codepoint(memory, pointer),
        -397 => get_codepoint_next(memory, pointer),
        -398 => get_codepoint_previous(memory, pointer),
        -399 => codepoint_to_u_t_f8(memory, pointer),
        -400 => text_copy(memory, pointer),
        -401 => text_is_equal(memory, pointer),
        -402 => text_length(memory, pointer),
        -403 => text_format(memory, pointer),
        -404 => text_subtext(memory, pointer),
        -405 => text_replace(memory, pointer),
        -406 => text_insert(memory, pointer),
        -407 => text_join(memory, pointer),
        -408 => text_split(memory, pointer),
        -409 => text_append(memory, pointer),
        -410 => text_find_index(memory, pointer),
        -411 => text_to_upper(memory, pointer),
        -412 => text_to_lower(memory, pointer),
        -413 => text_to_pascal(memory, pointer),
        -414 => text_to_integer(memory, pointer),
        -415 => draw_line3_d(memory, pointer),
        -416 => draw_point3_d(memory, pointer),
        -417 => draw_circle3_d(memory, pointer),
        -418 => draw_triangle3_d(memory, pointer),
        -419 => draw_triangle_strip3_d(memory, pointer),
        -420 => draw_cube(memory, pointer),
        -421 => draw_cube_v(memory, pointer),
        -422 => draw_cube_wires(memory, pointer),
        -423 => draw_cube_wires_v(memory, pointer),
        -424 => draw_sphere(memory, pointer),
        -425 => draw_sphere_ex(memory, pointer),
        -426 => draw_sphere_wires(memory, pointer),
        -427 => draw_cylinder(memory, pointer),
        -428 => draw_cylinder_ex(memory, pointer),
        -429 => draw_cylinder_wires(memory, pointer),
        -430 => draw_cylinder_wires_ex(memory, pointer),
        -431 => draw_capsule(memory, pointer),
        -432 => draw_capsule_wires(memory, pointer),
        -433 => draw_plane(memory, pointer),
        -434 => draw_ray(memory, pointer),
        -435 => draw_grid(memory, pointer),
        -436 => load_model(memory, pointer),
        -437 => load_model_from_mesh(memory, pointer),
        -438 => is_model_ready(memory, pointer),
        -439 => unload_model(memory, pointer),
        -440 => get_model_bounding_box(memory, pointer),
        -441 => draw_model(memory, pointer),
        -442 => draw_model_ex(memory, pointer),
        -443 => draw_model_wires(memory, pointer),
        -444 => draw_model_wires_ex(memory, pointer),
        -445 => draw_bounding_box(memory, pointer),
        -446 => draw_billboard(memory, pointer),
        -447 => draw_billboard_rec(memory, pointer),
        -448 => draw_billboard_pro(memory, pointer),
        -449 => upload_mesh(memory, pointer),
        -450 => update_mesh_buffer(memory, pointer),
        -451 => unload_mesh(memory, pointer),
        -452 => draw_mesh(memory, pointer),
        -453 => draw_mesh_instanced(memory, pointer),
        -454 => export_mesh(memory, pointer),
        -455 => get_mesh_bounding_box(memory, pointer),
        -456 => gen_mesh_tangents(memory, pointer),
        -457 => gen_mesh_poly(memory, pointer),
        -458 => gen_mesh_plane(memory, pointer),
        -459 => gen_mesh_cube(memory, pointer),
        -460 => gen_mesh_sphere(memory, pointer),
        -461 => gen_mesh_hemi_sphere(memory, pointer),
        -462 => gen_mesh_cylinder(memory, pointer),
        -463 => gen_mesh_cone(memory, pointer),
        -464 => gen_mesh_torus(memory, pointer),
        -465 => gen_mesh_knot(memory, pointer),
        -466 => gen_mesh_heightmap(memory, pointer),
        -467 => gen_mesh_cubicmap(memory, pointer),
        -468 => load_materials(memory, pointer),
        -469 => load_material_default(),
        -470 => is_material_ready(memory, pointer),
        -471 => unload_material(memory, pointer),
        -472 => set_material_texture(memory, pointer),
        -473 => set_model_mesh_material(memory, pointer),
        -474 => load_model_animations(memory, pointer),
        -475 => update_model_animation(memory, pointer),
        -476 => unload_model_animation(memory, pointer),
        -477 => unload_model_animations(memory, pointer),
        -478 => is_model_animation_valid(memory, pointer),
        -479 => check_collision_spheres(memory, pointer),
        -480 => check_collision_boxes(memory, pointer),
        -481 => check_collision_box_sphere(memory, pointer),
        -482 => get_ray_collision_sphere(memory, pointer),
        -483 => get_ray_collision_box(memory, pointer),
        -484 => get_ray_collision_mesh(memory, pointer),
        -485 => get_ray_collision_triangle(memory, pointer),
        -486 => get_ray_collision_quad(memory, pointer),
        -487 => init_audio_device(),
        -488 => close_audio_device(),
        -489 => is_audio_device_ready(),
        -490 => set_master_volume(memory, pointer),
        -491 => get_master_volume(),
        -492 => load_wave(memory, pointer),
        -493 => load_wave_from_memory(memory, pointer),
        -494 => is_wave_ready(memory, pointer),
        -495 => load_sound(memory, pointer),
        -496 => load_sound_from_wave(memory, pointer),
        -497 => load_sound_alias(memory, pointer),
        -498 => is_sound_ready(memory, pointer),
        -499 => update_sound(memory, pointer),
        -500 => unload_wave(memory, pointer),
        -501 => unload_sound(memory, pointer),
        -502 => unload_sound_alias(memory, pointer),
        -503 => export_wave(memory, pointer),
        -504 => export_wave_as_code(memory, pointer),
        -505 => play_sound(memory, pointer),
        -506 => stop_sound(memory, pointer),
        -507 => pause_sound(memory, pointer),
        -508 => resume_sound(memory, pointer),
        -509 => is_sound_playing(memory, pointer),
        -510 => set_sound_volume(memory, pointer),
        -511 => set_sound_pitch(memory, pointer),
        -512 => set_sound_pan(memory, pointer),
        -513 => wave_copy(memory, pointer),
        -514 => wave_crop(memory, pointer),
        -515 => wave_format(memory, pointer),
        -516 => load_wave_samples(memory, pointer),
        -517 => unload_wave_samples(memory, pointer),
        -518 => load_music_stream(memory, pointer),
        -519 => load_music_stream_from_memory(memory, pointer),
        -520 => is_music_ready(memory, pointer),
        -521 => unload_music_stream(memory, pointer),
        -522 => play_music_stream(memory, pointer),
        -523 => is_music_stream_playing(memory, pointer),
        -524 => update_music_stream(memory, pointer),
        -525 => stop_music_stream(memory, pointer),
        -526 => pause_music_stream(memory, pointer),
        -527 => resume_music_stream(memory, pointer),
        -528 => seek_music_stream(memory, pointer),
        -529 => set_music_volume(memory, pointer),
        -530 => set_music_pitch(memory, pointer),
        -531 => set_music_pan(memory, pointer),
        -532 => get_music_time_length(memory, pointer),
        -533 => get_music_time_played(memory, pointer),
        -534 => load_audio_stream(memory, pointer),
        -535 => is_audio_stream_ready(memory, pointer),
        -536 => unload_audio_stream(memory, pointer),
        -537 => update_audio_stream(memory, pointer),
        -538 => is_audio_stream_processed(memory, pointer),
        -539 => play_audio_stream(memory, pointer),
        -540 => pause_audio_stream(memory, pointer),
        -541 => resume_audio_stream(memory, pointer),
        -542 => is_audio_stream_playing(memory, pointer),
        -543 => stop_audio_stream(memory, pointer),
        -544 => set_audio_stream_volume(memory, pointer),
        -545 => set_audio_stream_pitch(memory, pointer),
        -546 => set_audio_stream_pan(memory, pointer),
        -547 => set_audio_stream_buffer_size_default(memory, pointer),
        -548 => set_audio_stream_callback(memory, pointer),
        -549 => attach_audio_stream_processor(memory, pointer),
        -550 => detach_audio_stream_processor(memory, pointer),
        -551 => attach_audio_mixed_processor(memory, pointer),
        -552 => detach_audio_mixed_processor(memory, pointer),
        _ => None,
    } {
        for x in 1..=result_cells.len() {
            (*memory)[pointer - x] = result_cells[x - 1];
        }
    }
}

mod rbf {
    use super::RbfType;

    pub fn get_input_cells(
        memory: &mut Vec<RbfType>,
        pointer: usize,
        input_size: usize,
    ) -> Vec<i32> {
        memory[(pointer + 1)..(pointer + input_size + 1)]
            .iter()
            .cloned()
            .collect()
    }

    pub fn get_string(memory: &mut Vec<RbfType>, str_ptr: i32) -> std::ffi::CString {
        // validate str ptr
        assert!(str_ptr >= 0);
        let str_ptr = str_ptr as usize;

        // read string array starting at str_ptr to null
        let mut str_bytes: Vec<RbfType> = Vec::new();
        let mut x = 0;
        loop {
            if memory[str_ptr + x] == 0 {
                break;
            }
            // validate ascii
            assert!(memory[str_ptr + x] >= 0);
            str_bytes.push(memory[str_ptr + x]);
            x += 1;
        }

        // turn string vec (i32) into string bytes (u8)
        let ascii_bytes: Vec<u8> = str_bytes
            .iter()
            .map(|&x| {
                // validate byte
                assert!(x >= 0);
                x as u8
            })
            .collect();

        // turn string bytes into String
        let ascii_str: String = String::from_utf8_lossy(&ascii_bytes).to_string();

        // turn string into C string
        let c_str = std::ffi::CString::new(ascii_str).unwrap();

        // return C string
        return c_str;
    }

    pub fn get_color(memory: &mut Vec<RbfType>, color_ptr: i32) -> raylib::ffi::Color {
        // validate ptr
        assert!(color_ptr >= 0);
        let color_ptr = color_ptr as usize;

        // read 4 bytes starting at ptr
        let mut color_bytes: [u8; 4] = [0, 0, 0, 0];
        for x in 0..4 {
            // validate rgba value
            assert!(memory[color_ptr + x] >= 0);
            assert!(memory[color_ptr + x] <= 255);
            color_bytes[x] = memory[color_ptr + x] as u8;
        }

        // build color
        raylib::ffi::Color {
            r: color_bytes[0],
            g: color_bytes[1],
            b: color_bytes[2],
            a: color_bytes[3],
        }
    }
}

/// Initialize window and OpenGL context
unsafe fn init_window(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    let input_cells = rbf::get_input_cells(memory, pointer, 3);

    // get inputs
    let width = input_cells[0];
    let height = input_cells[1];
    let title = rbf::get_string(memory, input_cells[2]);

    // init raylib window
    raylib::ffi::InitWindow(width, height, title.as_ptr() as *const i8);
    None
}

/// Close window and unload OpenGL context
unsafe fn close_window() -> Option<Vec<RbfType>> {
    raylib::ffi::CloseWindow();
    None
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
unsafe fn window_should_close() -> Option<Vec<RbfType>> {
    Some(vec![raylib::ffi::WindowShouldClose() as RbfType])
}

/// Check if window has been initialized successfully
unsafe fn is_window_ready() -> Option<Vec<RbfType>> {
    raylib::ffi::IsWindowReady();
    None
}

/// Check if window is currently fullscreen
unsafe fn is_window_fullscreen() -> Option<Vec<RbfType>> {
    raylib::ffi::IsWindowFullscreen();
    None
}

/// Check if window is currently hidden (only PLATFORM_DESKTOP)
unsafe fn is_window_hidden() -> Option<Vec<RbfType>> {
    raylib::ffi::IsWindowHidden();
    None
}

/// Check if window is currently minimized (only PLATFORM_DESKTOP)
unsafe fn is_window_minimized() -> Option<Vec<RbfType>> {
    raylib::ffi::IsWindowMinimized();
    None
}

/// Check if window is currently maximized (only PLATFORM_DESKTOP)
unsafe fn is_window_maximized() -> Option<Vec<RbfType>> {
    raylib::ffi::IsWindowMaximized();
    None
}

/// Check if window is currently focused (only PLATFORM_DESKTOP)
unsafe fn is_window_focused() -> Option<Vec<RbfType>> {
    raylib::ffi::IsWindowFocused();
    None
}

/// Check if window has been resized last frame
unsafe fn is_window_resized() -> Option<Vec<RbfType>> {
    raylib::ffi::IsWindowResized();
    None
}

/// Check if one specific window flag is enabled
unsafe fn is_window_state(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set window configuration state using flags (only PLATFORM_DESKTOP)
unsafe fn set_window_state(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Clear window configuration state flags
unsafe fn clear_window_state(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_fullscreen() -> Option<Vec<RbfType>> {
    raylib::ffi::ToggleFullscreen();
    None
}

/// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_borderless_windowed() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::ToggleBorderlessWindowed()");
}

/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
unsafe fn maximize_window() -> Option<Vec<RbfType>> {
    raylib::ffi::MaximizeWindow();
    None
}

/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
unsafe fn minimize_window() -> Option<Vec<RbfType>> {
    raylib::ffi::MinimizeWindow();
    None
}

/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
unsafe fn restore_window() -> Option<Vec<RbfType>> {
    raylib::ffi::RestoreWindow();
    None
}

/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icon(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icons(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
unsafe fn set_window_title(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set window position on screen (only PLATFORM_DESKTOP)
unsafe fn set_window_position(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set monitor for the current window
unsafe fn set_window_monitor(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_min_size(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_max_size(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set window dimensions
unsafe fn set_window_size(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
unsafe fn set_window_opacity(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set window focused (only PLATFORM_DESKTOP)
unsafe fn set_window_focused() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::SetWindowFocused()")
}

/// Get native window handle
unsafe fn get_window_handle() -> Option<Vec<RbfType>> {
    raylib::ffi::GetWindowHandle();
    None
}

/// Get current screen width
unsafe fn get_screen_width() -> Option<Vec<RbfType>> {
    raylib::ffi::GetScreenWidth();
    None
}

/// Get current screen height
unsafe fn get_screen_height() -> Option<Vec<RbfType>> {
    raylib::ffi::GetScreenHeight();
    None
}

/// Get current render width (it considers HiDPI)
unsafe fn get_render_width() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::GetRenderWidth()")
}

/// Get current render height (it considers HiDPI)
unsafe fn get_render_height() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::GetRenderHeight()");
}

/// Get number of connected monitors
unsafe fn get_monitor_count() -> Option<Vec<RbfType>> {
    raylib::ffi::GetMonitorCount();
    None
}

/// Get current connected monitor
unsafe fn get_current_monitor() -> Option<Vec<RbfType>> {
    raylib::ffi::GetCurrentMonitor();
    None
}

/// Get specified monitor position
unsafe fn get_monitor_position(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get specified monitor width (current video mode used by monitor)
unsafe fn get_monitor_width(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get specified monitor height (current video mode used by monitor)
unsafe fn get_monitor_height(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get specified monitor physical width in millimetres
unsafe fn get_monitor_physical_width(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get specified monitor physical height in millimetres
unsafe fn get_monitor_physical_height(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get specified monitor refresh rate
unsafe fn get_monitor_refresh_rate(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get window position XY on monitor
unsafe fn get_window_position() -> Option<Vec<RbfType>> {
    raylib::ffi::GetWindowPosition();
    None
}

/// Get window scale DPI factor
unsafe fn get_window_scale_d_p_i() -> Option<Vec<RbfType>> {
    raylib::ffi::GetWindowScaleDPI();
    None
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
unsafe fn get_monitor_name(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set clipboard text content
unsafe fn set_clipboard_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get clipboard text content
unsafe fn get_clipboard_text() -> Option<Vec<RbfType>> {
    raylib::ffi::GetClipboardText();
    None
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
unsafe fn enable_event_waiting() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::EnableEventWaiting()")
}

/// Disable waiting for events on EndDrawing(), automatic events polling
unsafe fn disable_event_waiting() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::DisableEventWaiting()")
}

/// Shows cursor
unsafe fn show_cursor() -> Option<Vec<RbfType>> {
    raylib::ffi::ShowCursor();
    None
}

/// Hides cursor
unsafe fn hide_cursor() -> Option<Vec<RbfType>> {
    raylib::ffi::HideCursor();
    None
}

/// Check if cursor is not visible
unsafe fn is_cursor_hidden() -> Option<Vec<RbfType>> {
    raylib::ffi::IsCursorHidden();
    None
}

/// Enables cursor (unlock cursor)
unsafe fn enable_cursor() -> Option<Vec<RbfType>> {
    raylib::ffi::EnableCursor();
    None
}

/// Disables cursor (lock cursor)
unsafe fn disable_cursor() -> Option<Vec<RbfType>> {
    raylib::ffi::DisableCursor();
    None
}

/// Check if cursor is on the screen
unsafe fn is_cursor_on_screen() -> Option<Vec<RbfType>> {
    raylib::ffi::IsCursorOnScreen();
    None
}

/// Set background color (framebuffer clear color)
unsafe fn clear_background(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    let input_cells = rbf::get_input_cells(memory, pointer, 1);

    // get color
    let color = rbf::get_color(memory, input_cells[0]);

    // clear background
    raylib::ffi::ClearBackground(color);
    None
}

/// Setup canvas (framebuffer) to start drawing
unsafe fn begin_drawing() -> Option<Vec<RbfType>> {
    raylib::ffi::BeginDrawing();
    None
}

/// End canvas drawing and swap buffers (double buffering)
unsafe fn end_drawing() -> Option<Vec<RbfType>> {
    raylib::ffi::EndDrawing();
    None
}

/// Begin 2D mode with custom camera (2D)
unsafe fn begin_mode2_d(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Ends 2D mode with custom camera
unsafe fn end_mode2_d() -> Option<Vec<RbfType>> {
    raylib::ffi::EndMode2D();
    None
}

/// Begin 3D mode with custom camera (3D)
unsafe fn begin_mode3_d(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Ends 3D mode and returns to default 2D orthographic mode
unsafe fn end_mode3_d() -> Option<Vec<RbfType>> {
    raylib::ffi::EndMode3D();
    None
}

/// Begin drawing to render texture
unsafe fn begin_texture_mode(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Ends drawing to render texture
unsafe fn end_texture_mode() -> Option<Vec<RbfType>> {
    raylib::ffi::EndTextureMode();
    None
}

/// Begin custom shader drawing
unsafe fn begin_shader_mode(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// End custom shader drawing (use default shader)
unsafe fn end_shader_mode() -> Option<Vec<RbfType>> {
    raylib::ffi::EndShaderMode();
    None
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
unsafe fn begin_blend_mode(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// End blending mode (reset to default: alpha blending)
unsafe fn end_blend_mode() -> Option<Vec<RbfType>> {
    raylib::ffi::EndBlendMode();
    None
}

/// Begin scissor mode (define screen area for following drawing)
unsafe fn begin_scissor_mode(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// End scissor mode
unsafe fn end_scissor_mode() -> Option<Vec<RbfType>> {
    raylib::ffi::EndScissorMode();
    None
}

/// Begin stereo rendering (requires VR simulator)
unsafe fn begin_vr_stereo_mode(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// End stereo rendering (requires VR simulator)
unsafe fn end_vr_stereo_mode() -> Option<Vec<RbfType>> {
    raylib::ffi::EndVrStereoMode();
    None
}

/// Load VR stereo config for VR simulator device parameters
unsafe fn load_vr_stereo_config(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload VR stereo config
unsafe fn unload_vr_stereo_config(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load shader from files and bind default locations
unsafe fn load_shader(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load shader from code strings and bind default locations
unsafe fn load_shader_from_memory(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a shader is ready
unsafe fn is_shader_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get shader uniform location
unsafe fn get_shader_location(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get shader attribute location
unsafe fn get_shader_location_attrib(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set shader uniform value
unsafe fn set_shader_value(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set shader uniform value vector
unsafe fn set_shader_value_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set shader uniform value (matrix 4x4)
unsafe fn set_shader_value_matrix(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set shader uniform value for texture (sampler2d)
unsafe fn set_shader_value_texture(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload shader from GPU memory (VRAM)
unsafe fn unload_shader(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get a ray trace from mouse position
unsafe fn get_mouse_ray(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get camera transform matrix (view matrix)
unsafe fn get_camera_matrix(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get camera 2d transform matrix
unsafe fn get_camera_matrix2_d(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get the screen space position for a 3d world space position
unsafe fn get_world_to_screen(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get the world space position for a 2d camera screen space position
unsafe fn get_screen_to_world2_d(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get size position for a 3d world space position
unsafe fn get_world_to_screen_ex(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get the screen space position for a 2d camera world space position
unsafe fn get_world_to_screen2_d(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set target FPS (maximum)
unsafe fn set_target_f_p_s(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get time in seconds for last frame drawn (delta time)
unsafe fn get_frame_time() -> Option<Vec<RbfType>> {
    raylib::ffi::GetFrameTime();
    None
}

/// Get elapsed time in seconds since InitWindow()
unsafe fn get_time() -> Option<Vec<RbfType>> {
    raylib::ffi::GetTime();
    None
}

/// Get current FPS
unsafe fn get_f_p_s() -> Option<Vec<RbfType>> {
    raylib::ffi::GetFPS();
    None
}

/// Swap back buffer with front buffer (screen drawing)
unsafe fn swap_screen_buffer() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::SwapScreenBuffer()")
}

/// Register all input events
unsafe fn poll_input_events() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::PollInputEvents()")
}

/// Wait for some time (halt program execution)
unsafe fn wait_time(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set the seed for the random number generator
unsafe fn set_random_seed(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get a random value between min and max (both included)
unsafe fn get_random_value(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load random values sequence, no values repeated
unsafe fn load_random_sequence(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload random values sequence
unsafe fn unload_random_sequence(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Takes a screenshot of current screen (filename extension defines format)
unsafe fn take_screenshot(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Setup init configuration flags (view FLAGS)
unsafe fn set_config_flags(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Open URL with default system browser (if available)
unsafe fn open_u_r_l(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
unsafe fn trace_log(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set the current threshold (minimum) log level
unsafe fn set_trace_log_level(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Internal memory allocator
unsafe fn mem_alloc(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Internal memory reallocator
unsafe fn mem_realloc(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Internal memory free
unsafe fn mem_free(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set custom trace log
unsafe fn set_trace_log_callback(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set custom file binary data loader
unsafe fn set_load_file_data_callback(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set custom file binary data saver
unsafe fn set_save_file_data_callback(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set custom file text data loader
unsafe fn set_load_file_text_callback(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set custom file text data saver
unsafe fn set_save_file_text_callback(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load file data as byte array (read)
unsafe fn load_file_data(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload file data allocated by LoadFileData()
unsafe fn unload_file_data(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Save data to file from byte array (write), returns true on success
unsafe fn save_file_data(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export data to code (.h), returns true on success
unsafe fn export_data_as_code(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load text data from file (read), returns a '\0' terminated string
unsafe fn load_file_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload file text data allocated by LoadFileText()
unsafe fn unload_file_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Save text data to file (write), string must be '\0' terminated, returns true on success
unsafe fn save_file_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if file exists
unsafe fn file_exists(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a directory path exists
unsafe fn directory_exists(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check file extension (including point: .png, .wav)
unsafe fn is_file_extension(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
unsafe fn get_file_length(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get pointer to extension for a filename string (includes dot: '.png')
unsafe fn get_file_extension(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get pointer to filename for a path string
unsafe fn get_file_name(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get filename string without extension (uses static string)
unsafe fn get_file_name_without_ext(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get full path for a given fileName with path (uses static string)
unsafe fn get_directory_path(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get previous directory path for a given path (uses static string)
unsafe fn get_prev_directory_path(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get current working directory (uses static string)
unsafe fn get_working_directory() -> Option<Vec<RbfType>> {
    raylib::ffi::GetWorkingDirectory();
    None
}

/// Get the directory of the running application (uses static string)
unsafe fn get_application_directory() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::GetApplicationDirectory()")
}

/// Change working directory, return true on success
unsafe fn change_directory(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a given path is a file or a directory
unsafe fn is_path_file(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load directory filepaths
unsafe fn load_directory_files(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load directory filepaths with extension filtering and recursive directory scan
unsafe fn load_directory_files_ex(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload filepaths
unsafe fn unload_directory_files(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a file has been dropped into window
unsafe fn is_file_dropped() -> Option<Vec<RbfType>> {
    raylib::ffi::IsFileDropped();
    None
}

/// Load dropped filepaths
unsafe fn load_dropped_files() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::LoadDroppedFiles()")
}

/// Unload dropped filepaths
unsafe fn unload_dropped_files(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get file modification time (last write time)
unsafe fn get_file_mod_time(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Compress data (DEFLATE algorithm), memory must be MemFree()
unsafe fn compress_data(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Decompress data (DEFLATE algorithm), memory must be MemFree()
unsafe fn decompress_data(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Encode data to Base64 string, memory must be MemFree()
unsafe fn encode_data_base64(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Decode Base64 string data, memory must be MemFree()
unsafe fn decode_data_base64(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
unsafe fn load_automation_event_list(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload automation events list from file
unsafe fn unload_automation_event_list(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export automation events list as text file
unsafe fn export_automation_event_list(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set automation event list to record to
unsafe fn set_automation_event_list(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set automation event internal base frame to start recording
unsafe fn set_automation_event_base_frame(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Start recording automation events (AutomationEventList must be set)
unsafe fn start_automation_event_recording() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::StartAutomationEventRecording()")
}

/// Stop recording automation events
unsafe fn stop_automation_event_recording() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::StopAutomationEventRecording()")
}

/// Play a recorded automation event
unsafe fn play_automation_event(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a key has been pressed once
unsafe fn is_key_pressed(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a key has been pressed again (Only PLATFORM_DESKTOP)
unsafe fn is_key_pressed_repeat(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a key is being pressed
unsafe fn is_key_down(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a key has been released once
unsafe fn is_key_released(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a key is NOT being pressed
unsafe fn is_key_up(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
unsafe fn get_key_pressed() -> Option<Vec<RbfType>> {
    raylib::ffi::GetKeyPressed();
    None
}

/// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
unsafe fn get_char_pressed() -> Option<Vec<RbfType>> {
    raylib::ffi::GetCharPressed();
    None
}

/// Set a custom key to exit program (default is ESC)
unsafe fn set_exit_key(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a gamepad is available
unsafe fn is_gamepad_available(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get gamepad internal name id
unsafe fn get_gamepad_name(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a gamepad button has been pressed once
unsafe fn is_gamepad_button_pressed(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a gamepad button is being pressed
unsafe fn is_gamepad_button_down(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a gamepad button has been released once
unsafe fn is_gamepad_button_released(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a gamepad button is NOT being pressed
unsafe fn is_gamepad_button_up(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get the last gamepad button pressed
unsafe fn get_gamepad_button_pressed() -> Option<Vec<RbfType>> {
    raylib::ffi::GetGamepadButtonPressed();
    None
}

/// Get gamepad axis count for a gamepad
unsafe fn get_gamepad_axis_count(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get axis movement value for a gamepad axis
unsafe fn get_gamepad_axis_movement(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set internal gamepad mappings (SDL_GameControllerDB)
unsafe fn set_gamepad_mappings(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a mouse button has been pressed once
unsafe fn is_mouse_button_pressed(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a mouse button is being pressed
unsafe fn is_mouse_button_down(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a mouse button has been released once
unsafe fn is_mouse_button_released(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a mouse button is NOT being pressed
unsafe fn is_mouse_button_up(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get mouse position X
unsafe fn get_mouse_x() -> Option<Vec<RbfType>> {
    raylib::ffi::GetMouseX();
    None
}

/// Get mouse position Y
unsafe fn get_mouse_y() -> Option<Vec<RbfType>> {
    raylib::ffi::GetMouseY();
    None
}

/// Get mouse position XY
unsafe fn get_mouse_position() -> Option<Vec<RbfType>> {
    raylib::ffi::GetMousePosition();
    None
}

/// Get mouse delta between frames
unsafe fn get_mouse_delta() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::GetMouseDelta()")
}

/// Set mouse position XY
unsafe fn set_mouse_position(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set mouse offset
unsafe fn set_mouse_offset(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set mouse scaling
unsafe fn set_mouse_scale(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get mouse wheel movement for X or Y, whichever is larger
unsafe fn get_mouse_wheel_move() -> Option<Vec<RbfType>> {
    raylib::ffi::GetMouseWheelMove();
    None
}

/// Get mouse wheel movement for both X and Y
unsafe fn get_mouse_wheel_move_v() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::GetMouseWheelMoveV()")
}

/// Set mouse cursor
unsafe fn set_mouse_cursor(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get touch position X for touch point 0 (relative to screen size)
unsafe fn get_touch_x() -> Option<Vec<RbfType>> {
    raylib::ffi::GetTouchX();
    None
}

/// Get touch position Y for touch point 0 (relative to screen size)
unsafe fn get_touch_y() -> Option<Vec<RbfType>> {
    raylib::ffi::GetTouchY();
    None
}

/// Get touch position XY for a touch point index (relative to screen size)
unsafe fn get_touch_position(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get touch point identifier for given index
unsafe fn get_touch_point_id(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get number of touch points
unsafe fn get_touch_point_count() -> Option<Vec<RbfType>> {
    Some(vec![raylib::ffi::GetTouchPointsCount()])
}

/// Enable a set of gestures using flags
unsafe fn set_gestures_enabled(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a gesture have been detected
unsafe fn is_gesture_detected(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get latest detected gesture
unsafe fn get_gesture_detected() -> Option<Vec<RbfType>> {
    raylib::ffi::GetGestureDetected();
    None
}

/// Get gesture hold time in milliseconds
unsafe fn get_gesture_hold_duration() -> Option<Vec<RbfType>> {
    raylib::ffi::GetGestureHoldDuration();
    None
}

/// Get gesture drag vector
unsafe fn get_gesture_drag_vector() -> Option<Vec<RbfType>> {
    raylib::ffi::GetGestureDragVector();
    None
}

/// Get gesture drag angle
unsafe fn get_gesture_drag_angle() -> Option<Vec<RbfType>> {
    raylib::ffi::GetGestureDragAngle();
    None
}

/// Get gesture pinch delta
unsafe fn get_gesture_pinch_vector() -> Option<Vec<RbfType>> {
    raylib::ffi::GetGesturePinchVector();
    None
}

/// Get gesture pinch angle
unsafe fn get_gesture_pinch_angle() -> Option<Vec<RbfType>> {
    raylib::ffi::GetGesturePinchAngle();
    None
}

/// Update camera position for selected mode
unsafe fn update_camera(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Update camera movement/rotation
unsafe fn update_camera_pro(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set texture and rectangle to be used on shapes drawing
unsafe fn set_shapes_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a pixel
unsafe fn draw_pixel(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a pixel (Vector version)
unsafe fn draw_pixel_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a line
unsafe fn draw_line(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a line (using gl lines)
unsafe fn draw_line_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a line (using triangles/quads)
unsafe fn draw_line_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw lines sequence (using gl lines)
unsafe fn draw_line_strip(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw line segment cubic-bezier in-out interpolation
unsafe fn draw_line_bezier(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled circle
unsafe fn draw_circle(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a piece of a circle
unsafe fn draw_circle_sector(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw circle sector outline
unsafe fn draw_circle_sector_lines(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a gradient-filled circle
unsafe fn draw_circle_gradient(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled circle (Vector version)
unsafe fn draw_circle_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw circle outline
unsafe fn draw_circle_lines(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw circle outline (Vector version)
unsafe fn draw_circle_lines_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw ellipse
unsafe fn draw_ellipse(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw ellipse outline
unsafe fn draw_ellipse_lines(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw ring
unsafe fn draw_ring(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw ring outline
unsafe fn draw_ring_lines(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle
unsafe fn draw_rectangle(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle (Vector version)
unsafe fn draw_rectangle_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle
unsafe fn draw_rectangle_rec(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled rectangle with pro parameters
unsafe fn draw_rectangle_pro(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a vertical-gradient-filled rectangle
unsafe fn draw_rectangle_gradient_v(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a horizontal-gradient-filled rectangle
unsafe fn draw_rectangle_gradient_h(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a gradient-filled rectangle with custom vertex colors
unsafe fn draw_rectangle_gradient_ex(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle outline
unsafe fn draw_rectangle_lines(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle outline with extended parameters
unsafe fn draw_rectangle_lines_ex(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle with rounded edges
unsafe fn draw_rectangle_rounded(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle with rounded edges outline
unsafe fn draw_rectangle_rounded_lines(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
unsafe fn draw_triangle(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw triangle outline (vertex in counter-clockwise order!)
unsafe fn draw_triangle_lines(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a triangle fan defined by points (first vertex is the center)
unsafe fn draw_triangle_fan(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a triangle strip defined by points
unsafe fn draw_triangle_strip(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a regular polygon (Vector version)
unsafe fn draw_poly(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a polygon outline of n sides
unsafe fn draw_poly_lines(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a polygon outline of n sides with extended parameters
unsafe fn draw_poly_lines_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline: Linear, minimum 2 points
unsafe fn draw_spline_linear(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline: B-Spline, minimum 4 points
unsafe fn draw_spline_basis(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline: Catmull-Rom, minimum 4 points
unsafe fn draw_spline_catmull_rom(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline: Quadratic Bezier, minimum 3 points (1 control point): [p1, c2, p3, c4...]
unsafe fn draw_spline_bezier_quadratic(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline: Cubic Bezier, minimum 4 points (2 control points): [p1, c2, c3, p4, c5, c6...]
unsafe fn draw_spline_bezier_cubic(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline segment: Linear, 2 points
unsafe fn draw_spline_segment_linear(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline segment: B-Spline, 4 points
unsafe fn draw_spline_segment_basis(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline segment: Catmull-Rom, 4 points
unsafe fn draw_spline_segment_catmull_rom(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline segment: Quadratic Bezier, 2 points, 1 control point
unsafe fn draw_spline_segment_bezier_quadratic(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw spline segment: Cubic Bezier, 2 points, 2 control points
unsafe fn draw_spline_segment_bezier_cubic(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Linear
unsafe fn get_spline_point_linear(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: B-Spline
unsafe fn get_spline_point_basis(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Catmull-Rom
unsafe fn get_spline_point_catmull_rom(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Quadratic Bezier
unsafe fn get_spline_point_bezier_quad(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get (evaluate) spline point: Cubic Bezier
unsafe fn get_spline_point_bezier_cubic(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check collision between two rectangles
unsafe fn check_collision_recs(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check collision between two circles
unsafe fn check_collision_circles(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check collision between circle and rectangle
unsafe fn check_collision_circle_rec(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if point is inside rectangle
unsafe fn check_collision_point_rec(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if point is inside circle
unsafe fn check_collision_point_circle(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if point is inside a triangle
unsafe fn check_collision_point_triangle(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if point is within a polygon described by array of vertices
unsafe fn check_collision_point_poly(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check the collision between two lines defined by two points each, returns collision point by reference
unsafe fn check_collision_lines(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if point belongs to line created between two points [p1] and [p2] with defined margin in pixels [threshold]
unsafe fn check_collision_point_line(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get collision rectangle for two rectangles collision
unsafe fn get_collision_rec(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load image from file into CPU memory (RAM)
unsafe fn load_image(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load image from RAW file data
unsafe fn load_image_raw(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load image from SVG file data or string with specified size
unsafe fn load_image_svg(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load image sequence from file (frames appended to image.data)
unsafe fn load_image_anim(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
unsafe fn load_image_from_memory(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load image from GPU texture data
unsafe fn load_image_from_texture(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load image from screen buffer and (screenshot)
unsafe fn load_image_from_screen() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::LoadImageFromScreen()")
}

/// Check if an image is ready
unsafe fn is_image_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload image from CPU memory (RAM)
unsafe fn unload_image(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export image data to file, returns true on success
unsafe fn export_image(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export image to memory buffer
unsafe fn export_image_to_memory(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export image as code file defining an array of bytes, returns true on success
unsafe fn export_image_as_code(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: plain color
unsafe fn gen_image_color(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
unsafe fn gen_image_gradient_linear(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: radial gradient
unsafe fn gen_image_gradient_radial(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: square gradient
unsafe fn gen_image_gradient_square(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: checked
unsafe fn gen_image_checked(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: white noise
unsafe fn gen_image_white_noise(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: perlin noise
unsafe fn gen_image_perlin_noise(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: cellular algorithm, bigger tileSize means bigger cells
unsafe fn gen_image_cellular(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image: grayscale image from text data
unsafe fn gen_image_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Create an image duplicate (useful for transformations)
unsafe fn image_copy(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Create an image from another image piece
unsafe fn image_from_image(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Create an image from text (default font)
unsafe fn image_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Create an image from text (custom sprite font)
unsafe fn image_text_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Convert image data to desired format
unsafe fn image_format(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Convert image to POT (power-of-two)
unsafe fn image_to_p_o_t(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Crop an image to a defined rectangle
unsafe fn image_crop(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Crop image depending on alpha value
unsafe fn image_alpha_crop(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Clear alpha channel to desired color
unsafe fn image_alpha_clear(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Apply alpha mask to image
unsafe fn image_alpha_mask(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Premultiply alpha channel
unsafe fn image_alpha_premultiply(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Apply Gaussian blur using a box blur approximation
unsafe fn image_blur_gaussian(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Resize image (Bicubic scaling algorithm)
unsafe fn image_resize(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Resize image (Nearest-Neighbor scaling algorithm)
unsafe fn image_resize_n_n(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Resize canvas and fill with color
unsafe fn image_resize_canvas(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Compute all mipmap levels for a provided image
unsafe fn image_mipmaps(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
unsafe fn image_dither(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Flip image vertically
unsafe fn image_flip_vertical(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Flip image horizontally
unsafe fn image_flip_horizontal(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Rotate image by input angle in degrees (-359 to 359)
unsafe fn image_rotate(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Rotate image clockwise 90deg
unsafe fn image_rotate_c_w(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Rotate image counter-clockwise 90deg
unsafe fn image_rotate_c_c_w(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Modify image color: tint
unsafe fn image_color_tint(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Modify image color: invert
unsafe fn image_color_invert(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Modify image color: grayscale
unsafe fn image_color_grayscale(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Modify image color: contrast (-100 to 100)
unsafe fn image_color_contrast(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Modify image color: brightness (-255 to 255)
unsafe fn image_color_brightness(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Modify image color: replace color
unsafe fn image_color_replace(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load color data from image as a Color array (RGBA - 32bit)
unsafe fn load_image_colors(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load colors palette from image as a Color array (RGBA - 32bit)
unsafe fn load_image_palette(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload color data loaded with LoadImageColors()
unsafe fn unload_image_colors(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload colors palette loaded with LoadImagePalette()
unsafe fn unload_image_palette(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get image alpha border rectangle
unsafe fn get_image_alpha_border(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get image pixel color at (x, y) position
unsafe fn get_image_color(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Clear image background with given color
unsafe fn image_clear_background(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw pixel within an image
unsafe fn image_draw_pixel(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw pixel within an image (Vector version)
unsafe fn image_draw_pixel_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw line within an image
unsafe fn image_draw_line(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw line within an image (Vector version)
unsafe fn image_draw_line_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a filled circle within an image
unsafe fn image_draw_circle(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a filled circle within an image (Vector version)
unsafe fn image_draw_circle_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw circle outline within an image
unsafe fn image_draw_circle_lines(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw circle outline within an image (Vector version)
unsafe fn image_draw_circle_lines_v(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle within an image
unsafe fn image_draw_rectangle(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle within an image (Vector version)
unsafe fn image_draw_rectangle_v(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle within an image
unsafe fn image_draw_rectangle_rec(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw rectangle lines within an image
unsafe fn image_draw_rectangle_lines(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a source image within a destination image (tint applied to source)
unsafe fn image_draw(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw text (using default font) within an image (destination)
unsafe fn image_draw_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw text (custom sprite font) within an image (destination)
unsafe fn image_draw_text_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load texture from file into GPU memory (VRAM)
unsafe fn load_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load texture from image data
unsafe fn load_texture_from_image(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load cubemap from image, multiple image cubemap layouts supported
unsafe fn load_texture_cubemap(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load texture for rendering (framebuffer)
unsafe fn load_render_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a texture is ready
unsafe fn is_texture_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload texture from GPU memory (VRAM)
unsafe fn unload_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a render texture is ready
unsafe fn is_render_texture_ready(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload render texture from GPU memory (VRAM)
unsafe fn unload_render_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Update GPU texture with new data
unsafe fn update_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Update GPU texture rectangle with new data
unsafe fn update_texture_rec(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate GPU mipmaps for a texture
unsafe fn gen_texture_mipmaps(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set texture scaling filter mode
unsafe fn set_texture_filter(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set texture wrapping mode
unsafe fn set_texture_wrap(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a Texture2D
unsafe fn draw_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a Texture2D with position defined as Vector2
unsafe fn draw_texture_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a Texture2D with extended parameters
unsafe fn draw_texture_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a part of a texture defined by a rectangle
unsafe fn draw_texture_rec(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a part of a texture defined by a rectangle with 'pro' parameters
unsafe fn draw_texture_pro(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draws a texture (or part of it) that stretches or shrinks nicely
unsafe fn draw_texture_n_patch(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
unsafe fn fade(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get hexadecimal value for a Color
unsafe fn color_to_int(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get Color normalized as float [0..1]
unsafe fn color_normalize(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get Color from normalized values [0..1]
unsafe fn color_from_normalized(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
unsafe fn color_to_h_s_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
unsafe fn color_from_h_s_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get color multiplied with another color
unsafe fn color_tint(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
unsafe fn color_brightness(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get color with contrast correction, contrast values between -1.0f and 1.0f
unsafe fn color_contrast(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
unsafe fn color_alpha(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get src alpha-blended into dst color with tint
unsafe fn color_alpha_blend(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get Color structure from hexadecimal value
unsafe fn get_color(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get Color from a source pixel pointer of certain format
unsafe fn get_pixel_color(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set color formatted into destination pixel pointer
unsafe fn set_pixel_color(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get pixel data size in bytes for certain format
unsafe fn get_pixel_data_size(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get the default Font
unsafe fn get_font_default() -> Option<Vec<RbfType>> {
    raylib::ffi::GetFontDefault();
    None
}

/// Load font from file into GPU memory (VRAM)
unsafe fn load_font(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set
unsafe fn load_font_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load font from Image (XNA style)
unsafe fn load_font_from_image(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
unsafe fn load_font_from_memory(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a font is ready
unsafe fn is_font_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load font data for further use
unsafe fn load_font_data(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate image font atlas using chars info
unsafe fn gen_image_font_atlas(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload font chars info data (RAM)
unsafe fn unload_font_data(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload font from GPU memory (VRAM)
unsafe fn unload_font(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export font as code file, returns true on success
unsafe fn export_font_as_code(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw current FPS
unsafe fn draw_f_p_s(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw text (using default font)
unsafe fn draw_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    let input_cells = rbf::get_input_cells(memory, pointer, 5);

    // get input
    let text = rbf::get_string(memory, input_cells[0]);
    let pos_x = input_cells[1];
    let pos_y = input_cells[2];
    assert!(input_cells[3] >= 0);
    let font_size = input_cells[3];
    let color = rbf::get_color(memory, input_cells[4]);

    // draw text
    raylib::ffi::DrawText(text.as_ptr() as *const i8, pos_x, pos_y, font_size, color);
    None
}

/// Draw text using font and additional parameters
unsafe fn draw_text_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw text using Font and pro parameters (rotation)
unsafe fn draw_text_pro(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw one character (codepoint)
unsafe fn draw_text_codepoint(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw multiple character (codepoint)
unsafe fn draw_text_codepoints(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set vertical line spacing when drawing with line-breaks
unsafe fn set_text_line_spacing(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Measure string width for default font
unsafe fn measure_text(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Measure string size for Font
unsafe fn measure_text_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_index(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_info(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_atlas_rec(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load UTF-8 text encoded from codepoints array
unsafe fn load_u_t_f8(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload UTF-8 text encoded from codepoints array
unsafe fn unload_u_t_f8(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
unsafe fn load_codepoints(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload codepoints data from memory
unsafe fn unload_codepoints(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get total number of codepoints in a UTF-8 encoded string
unsafe fn get_codepoint_count(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint_next(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint_previous(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
unsafe fn codepoint_to_u_t_f8(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Copy one string to another, returns bytes copied
unsafe fn text_copy(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if two text string are equal
unsafe fn text_is_equal(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get text length, checks for '\0' ending
unsafe fn text_length(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Text formatting with variables (sprintf() style)
unsafe fn text_format(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get a piece of a text string
unsafe fn text_subtext(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Replace text string (WARNING: memory must be freed!)
unsafe fn text_replace(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Insert text in a position (WARNING: memory must be freed!)
unsafe fn text_insert(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Join text strings with delimiter
unsafe fn text_join(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Split text into multiple strings
unsafe fn text_split(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Append text at specific position and move cursor!
unsafe fn text_append(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Find first text occurrence within a string
unsafe fn text_find_index(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get upper case version of provided string
unsafe fn text_to_upper(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get lower case version of provided string
unsafe fn text_to_lower(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get Pascal case notation version of provided string
unsafe fn text_to_pascal(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get integer value from text (negative values not supported)
unsafe fn text_to_integer(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a line in 3D world space
unsafe fn draw_line3_d(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a point in 3D space, actually a small line
unsafe fn draw_point3_d(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a circle in 3D world space
unsafe fn draw_circle3_d(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a color-filled triangle (vertex in counter-clockwise order!)
unsafe fn draw_triangle3_d(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a triangle strip defined by points
unsafe fn draw_triangle_strip3_d(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw cube
unsafe fn draw_cube(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw cube (Vector version)
unsafe fn draw_cube_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw cube wires
unsafe fn draw_cube_wires(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw cube wires (Vector version)
unsafe fn draw_cube_wires_v(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw sphere
unsafe fn draw_sphere(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw sphere with extended parameters
unsafe fn draw_sphere_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw sphere wires
unsafe fn draw_sphere_wires(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a cylinder/cone
unsafe fn draw_cylinder(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a cylinder with base at startPos and top at endPos
unsafe fn draw_cylinder_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a cylinder/cone wires
unsafe fn draw_cylinder_wires(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a cylinder wires with base at startPos and top at endPos
unsafe fn draw_cylinder_wires_ex(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a capsule with the center of its sphere caps at startPos and endPos
unsafe fn draw_capsule(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw capsule wireframe with the center of its sphere caps at startPos and endPos
unsafe fn draw_capsule_wires(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a plane XZ
unsafe fn draw_plane(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a ray line
unsafe fn draw_ray(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a grid (centered at (0, 0, 0))
unsafe fn draw_grid(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load model from files (meshes and materials)
unsafe fn load_model(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load model from generated mesh (default material)
unsafe fn load_model_from_mesh(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a model is ready
unsafe fn is_model_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload model (including meshes) from memory (RAM and/or VRAM)
unsafe fn unload_model(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Compute model bounding box limits (considers all meshes)
unsafe fn get_model_bounding_box(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a model (with texture if set)
unsafe fn draw_model(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a model with extended parameters
unsafe fn draw_model_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a model wires (with texture if set)
unsafe fn draw_model_wires(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a model wires (with texture if set) with extended parameters
unsafe fn draw_model_wires_ex(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw bounding box (wires)
unsafe fn draw_bounding_box(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a billboard texture
unsafe fn draw_billboard(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a billboard texture defined by source
unsafe fn draw_billboard_rec(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a billboard texture defined by source and rotation
unsafe fn draw_billboard_pro(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Upload mesh vertex data in GPU and provide VAO/VBO ids
unsafe fn upload_mesh(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Update mesh vertex data in GPU for a specific buffer index
unsafe fn update_mesh_buffer(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload mesh data from CPU and GPU
unsafe fn unload_mesh(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw a 3d mesh with material and transform
unsafe fn draw_mesh(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Draw multiple mesh instances with material and different transforms
unsafe fn draw_mesh_instanced(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export mesh data to file, returns true on success
unsafe fn export_mesh(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Compute mesh bounding box limits
unsafe fn get_mesh_bounding_box(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Compute mesh tangents
unsafe fn gen_mesh_tangents(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate polygonal mesh
unsafe fn gen_mesh_poly(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate plane mesh (with subdivisions)
unsafe fn gen_mesh_plane(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate cuboid mesh
unsafe fn gen_mesh_cube(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate sphere mesh (standard sphere)
unsafe fn gen_mesh_sphere(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate half-sphere mesh (no bottom cap)
unsafe fn gen_mesh_hemi_sphere(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate cylinder mesh
unsafe fn gen_mesh_cylinder(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate cone/pyramid mesh
unsafe fn gen_mesh_cone(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate torus mesh
unsafe fn gen_mesh_torus(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate trefoil knot mesh
unsafe fn gen_mesh_knot(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate heightmap mesh from image data
unsafe fn gen_mesh_heightmap(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Generate cubes-based map mesh from image data
unsafe fn gen_mesh_cubicmap(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load materials from model file
unsafe fn load_materials(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load default material (Supports: DIFFUSE, SPECULAR, NORMAL maps)
unsafe fn load_material_default() -> Option<Vec<RbfType>> {
    raylib::ffi::LoadMaterialDefault();
    None
}

/// Check if a material is ready
unsafe fn is_material_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload material from GPU memory (VRAM)
unsafe fn unload_material(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set texture for a material map type (MATERIAL_MAP_DIFFUSE, MATERIAL_MAP_SPECULAR...)
unsafe fn set_material_texture(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set material for a mesh
unsafe fn set_model_mesh_material(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load model animations from file
unsafe fn load_model_animations(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Update model animation pose
unsafe fn update_model_animation(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload animation data
unsafe fn unload_model_animation(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload animation array data
unsafe fn unload_model_animations(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check model animation skeleton match
unsafe fn is_model_animation_valid(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check collision between two spheres
unsafe fn check_collision_spheres(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check collision between two bounding boxes
unsafe fn check_collision_boxes(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check collision between box and sphere
unsafe fn check_collision_box_sphere(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get collision info between ray and sphere
unsafe fn get_ray_collision_sphere(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get collision info between ray and box
unsafe fn get_ray_collision_box(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get collision info between ray and mesh
unsafe fn get_ray_collision_mesh(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get collision info between ray and triangle
unsafe fn get_ray_collision_triangle(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get collision info between ray and quad
unsafe fn get_ray_collision_quad(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Initialize audio device and context
unsafe fn init_audio_device() -> Option<Vec<RbfType>> {
    raylib::ffi::InitAudioDevice();
    None
}

/// Close the audio device and context
unsafe fn close_audio_device() -> Option<Vec<RbfType>> {
    raylib::ffi::CloseAudioDevice();
    None
}

/// Check if audio device has been initialized successfully
unsafe fn is_audio_device_ready() -> Option<Vec<RbfType>> {
    raylib::ffi::IsAudioDeviceReady();
    None
}

/// Set master volume (listener)
unsafe fn set_master_volume(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get master volume (listener)
unsafe fn get_master_volume() -> Option<Vec<RbfType>> {
    unimplemented!("raylib::ffi::GetMasterVolume()");
}

/// Load wave data from file
unsafe fn load_wave(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
unsafe fn load_wave_from_memory(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Checks if wave data is ready
unsafe fn is_wave_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load sound from file
unsafe fn load_sound(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load sound from wave data
unsafe fn load_sound_from_wave(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Create a new sound that shares the same sample data as the source sound, does not own the sound data
unsafe fn load_sound_alias(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Checks if a sound is ready
unsafe fn is_sound_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Update sound buffer with new data
unsafe fn update_sound(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload wave data
unsafe fn unload_wave(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload sound
unsafe fn unload_sound(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload a sound alias (does not deallocate sample data)
unsafe fn unload_sound_alias(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export wave data to file, returns true on success
unsafe fn export_wave(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Export wave sample data to code (.h), returns true on success
unsafe fn export_wave_as_code(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Play a sound
unsafe fn play_sound(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Stop playing a sound
unsafe fn stop_sound(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Pause a sound
unsafe fn pause_sound(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Resume a paused sound
unsafe fn resume_sound(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if a sound is currently playing
unsafe fn is_sound_playing(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set volume for a sound (1.0 is max level)
unsafe fn set_sound_volume(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set pitch for a sound (1.0 is base level)
unsafe fn set_sound_pitch(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set pan for a sound (0.5 is center)
unsafe fn set_sound_pan(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Copy a wave to a new wave
unsafe fn wave_copy(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Crop a wave to defined samples range
unsafe fn wave_crop(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Convert wave data to desired format
unsafe fn wave_format(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load samples data from wave as a 32bit float data array
unsafe fn load_wave_samples(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload samples data loaded with LoadWaveSamples()
unsafe fn unload_wave_samples(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load music stream from file
unsafe fn load_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load music stream from data
unsafe fn load_music_stream_from_memory(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Checks if a music stream is ready
unsafe fn is_music_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload music stream
unsafe fn unload_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Start music playing
unsafe fn play_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if music is playing
unsafe fn is_music_stream_playing(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Updates buffers for music streaming
unsafe fn update_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Stop music playing
unsafe fn stop_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Pause music playing
unsafe fn pause_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Resume playing paused music
unsafe fn resume_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Seek music to a position (in seconds)
unsafe fn seek_music_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set volume for music (1.0 is max level)
unsafe fn set_music_volume(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set pitch for a music (1.0 is base level)
unsafe fn set_music_pitch(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set pan for a music (0.5 is center)
unsafe fn set_music_pan(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get music time length (in seconds)
unsafe fn get_music_time_length(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Get current music time played (in seconds)
unsafe fn get_music_time_played(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Load audio stream (to stream raw audio pcm data)
unsafe fn load_audio_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Checks if an audio stream is ready
unsafe fn is_audio_stream_ready(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Unload audio stream and free memory
unsafe fn unload_audio_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Update audio stream buffers with data
unsafe fn update_audio_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if any audio stream buffers requires refill
unsafe fn is_audio_stream_processed(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Play audio stream
unsafe fn play_audio_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Pause audio stream
unsafe fn pause_audio_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Resume audio stream
unsafe fn resume_audio_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Check if audio stream is playing
unsafe fn is_audio_stream_playing(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Stop audio stream
unsafe fn stop_audio_stream(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set volume for audio stream (1.0 is max level)
unsafe fn set_audio_stream_volume(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set pitch for audio stream (1.0 is base level)
unsafe fn set_audio_stream_pitch(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Set pan for audio stream (0.5 is centered)
unsafe fn set_audio_stream_pan(memory: &mut Vec<RbfType>, pointer: usize) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Default size for new audio streams
unsafe fn set_audio_stream_buffer_size_default(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Audio thread callback to request new data
unsafe fn set_audio_stream_callback(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Attach audio stream processor to stream, receives the samples as <float>s
unsafe fn attach_audio_stream_processor(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Detach audio stream processor from stream
unsafe fn detach_audio_stream_processor(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Attach audio stream processor to the entire audio pipeline, receives the samples as <float>s
unsafe fn attach_audio_mixed_processor(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}

/// Detach audio stream processor from the entire audio pipeline
unsafe fn detach_audio_mixed_processor(
    memory: &mut Vec<RbfType>,
    pointer: usize,
) -> Option<Vec<RbfType>> {
    unimplemented!()
}
