use crate::BaseType;

pub unsafe fn call(memory: &mut Vec<BaseType>, pointer: usize) {
    if let Some(result_cells) = match memory[pointer] {
        0 => init_window(memory, pointer),
        1 => close_window(),
        2 => window_should_close(),
        3 => is_window_ready(),
        4 => is_window_fullscreen(),
        5 => is_window_hidden(),
        6 => is_window_minimized(),
        7 => is_window_maximized(),
        8 => is_window_focused(),
        9 => is_window_resized(),
        10 => is_window_state(memory, pointer),
        11 => set_window_state(memory, pointer),
        12 => clear_window_state(memory, pointer),
        13 => toggle_fullscreen(),
        14 => toggle_borderless_windowed(),
        15 => maximize_window(),
        16 => minimize_window(),
        17 => restore_window(),
        18 => set_window_icon(memory, pointer),
        19 => set_window_icons(memory, pointer),
        20 => set_window_title(memory, pointer),
        21 => set_window_position(memory, pointer),
        22 => set_window_monitor(memory, pointer),
        23 => set_window_min_size(memory, pointer),
        24 => set_window_max_size(memory, pointer),
        25 => set_window_size(memory, pointer),
        26 => set_window_opacity(memory, pointer),
        27 => set_window_focused(),
        28 => get_window_handle(),
        29 => get_screen_width(),
        30 => get_screen_height(),
        31 => get_render_width(),
        32 => get_render_height(),
        33 => get_monitor_count(),
        34 => get_current_monitor(),
        35 => get_monitor_position(memory, pointer),
        36 => get_monitor_width(memory, pointer),
        37 => get_monitor_height(memory, pointer),
        38 => get_monitor_physical_width(memory, pointer),
        39 => get_monitor_physical_height(memory, pointer),
        40 => get_monitor_refresh_rate(memory, pointer),
        41 => get_window_position(),
        42 => get_window_scale_d_p_i(),
        43 => get_monitor_name(memory, pointer),
        44 => set_clipboard_text(memory, pointer),
        45 => get_clipboard_text(),
        46 => enable_event_waiting(),
        47 => disable_event_waiting(),
        48 => show_cursor(),
        49 => hide_cursor(),
        50 => is_cursor_hidden(),
        51 => enable_cursor(),
        52 => disable_cursor(),
        53 => is_cursor_on_screen(),
        54 => clear_background(memory, pointer),
        55 => begin_drawing(),
        56 => end_drawing(),
        57 => begin_mode2_d(memory, pointer),
        58 => end_mode2_d(),
        59 => begin_mode3_d(memory, pointer),
        60 => end_mode3_d(),
        61 => begin_texture_mode(memory, pointer),
        62 => end_texture_mode(),
        63 => begin_shader_mode(memory, pointer),
        64 => end_shader_mode(),
        65 => begin_blend_mode(memory, pointer),
        66 => end_blend_mode(),
        67 => begin_scissor_mode(memory, pointer),
        68 => end_scissor_mode(),
        69 => begin_vr_stereo_mode(memory, pointer),
        70 => end_vr_stereo_mode(),
        71 => load_vr_stereo_config(memory, pointer),
        72 => unload_vr_stereo_config(memory, pointer),
        73 => load_shader(memory, pointer),
        74 => load_shader_from_memory(memory, pointer),
        75 => is_shader_ready(memory, pointer),
        76 => get_shader_location(memory, pointer),
        77 => get_shader_location_attrib(memory, pointer),
        78 => set_shader_value(memory, pointer),
        79 => set_shader_value_v(memory, pointer),
        80 => set_shader_value_matrix(memory, pointer),
        81 => set_shader_value_texture(memory, pointer),
        82 => unload_shader(memory, pointer),
        83 => get_mouse_ray(memory, pointer),
        84 => get_camera_matrix(memory, pointer),
        85 => get_camera_matrix2_d(memory, pointer),
        86 => get_world_to_screen(memory, pointer),
        87 => get_screen_to_world2_d(memory, pointer),
        88 => get_world_to_screen_ex(memory, pointer),
        89 => get_world_to_screen2_d(memory, pointer),
        90 => set_target_f_p_s(memory, pointer),
        91 => get_frame_time(),
        92 => get_time(),
        93 => get_f_p_s(),
        94 => swap_screen_buffer(),
        95 => poll_input_events(),
        96 => wait_time(memory, pointer),
        97 => set_random_seed(memory, pointer),
        98 => get_random_value(memory, pointer),
        99 => load_random_sequence(memory, pointer),
        100 => unload_random_sequence(memory, pointer),
        101 => take_screenshot(memory, pointer),
        102 => set_config_flags(memory, pointer),
        103 => open_u_r_l(memory, pointer),
        104 => trace_log(memory, pointer),
        105 => set_trace_log_level(memory, pointer),
        106 => mem_alloc(memory, pointer),
        107 => mem_realloc(memory, pointer),
        108 => mem_free(memory, pointer),
        109 => set_trace_log_callback(memory, pointer),
        110 => set_load_file_data_callback(memory, pointer),
        111 => set_save_file_data_callback(memory, pointer),
        112 => set_load_file_text_callback(memory, pointer),
        113 => set_save_file_text_callback(memory, pointer),
        114 => load_file_data(memory, pointer),
        115 => unload_file_data(memory, pointer),
        116 => save_file_data(memory, pointer),
        117 => export_data_as_code(memory, pointer),
        118 => load_file_text(memory, pointer),
        119 => unload_file_text(memory, pointer),
        120 => save_file_text(memory, pointer),
        121 => file_exists(memory, pointer),
        122 => directory_exists(memory, pointer),
        123 => is_file_extension(memory, pointer),
        124 => get_file_length(memory, pointer),
        125 => get_file_extension(memory, pointer),
        126 => get_file_name(memory, pointer),
        127 => get_file_name_without_ext(memory, pointer),
        128 => get_directory_path(memory, pointer),
        129 => get_prev_directory_path(memory, pointer),
        130 => get_working_directory(),
        131 => get_application_directory(),
        132 => change_directory(memory, pointer),
        133 => is_path_file(memory, pointer),
        134 => load_directory_files(memory, pointer),
        135 => load_directory_files_ex(memory, pointer),
        136 => unload_directory_files(memory, pointer),
        137 => is_file_dropped(),
        138 => load_dropped_files(),
        139 => unload_dropped_files(memory, pointer),
        140 => get_file_mod_time(memory, pointer),
        141 => compress_data(memory, pointer),
        142 => decompress_data(memory, pointer),
        143 => encode_data_base64(memory, pointer),
        144 => decode_data_base64(memory, pointer),
        145 => load_automation_event_list(memory, pointer),
        146 => unload_automation_event_list(memory, pointer),
        147 => export_automation_event_list(memory, pointer),
        148 => set_automation_event_list(memory, pointer),
        149 => set_automation_event_base_frame(memory, pointer),
        150 => start_automation_event_recording(),
        151 => stop_automation_event_recording(),
        152 => play_automation_event(memory, pointer),
        153 => is_key_pressed(memory, pointer),
        154 => is_key_pressed_repeat(memory, pointer),
        155 => is_key_down(memory, pointer),
        156 => is_key_released(memory, pointer),
        157 => is_key_up(memory, pointer),
        158 => get_key_pressed(),
        159 => get_char_pressed(),
        160 => set_exit_key(memory, pointer),
        161 => is_gamepad_available(memory, pointer),
        162 => get_gamepad_name(memory, pointer),
        163 => is_gamepad_button_pressed(memory, pointer),
        164 => is_gamepad_button_down(memory, pointer),
        165 => is_gamepad_button_released(memory, pointer),
        166 => is_gamepad_button_up(memory, pointer),
        167 => get_gamepad_button_pressed(),
        168 => get_gamepad_axis_count(memory, pointer),
        169 => get_gamepad_axis_movement(memory, pointer),
        170 => set_gamepad_mappings(memory, pointer),
        171 => is_mouse_button_pressed(memory, pointer),
        172 => is_mouse_button_down(memory, pointer),
        173 => is_mouse_button_released(memory, pointer),
        174 => is_mouse_button_up(memory, pointer),
        175 => get_mouse_x(),
        176 => get_mouse_y(),
        177 => get_mouse_position(),
        178 => get_mouse_delta(),
        179 => set_mouse_position(memory, pointer),
        180 => set_mouse_offset(memory, pointer),
        181 => set_mouse_scale(memory, pointer),
        182 => get_mouse_wheel_move(),
        183 => get_mouse_wheel_move_v(),
        184 => set_mouse_cursor(memory, pointer),
        185 => get_touch_x(),
        186 => get_touch_y(),
        187 => get_touch_position(memory, pointer),
        188 => get_touch_point_id(memory, pointer),
        189 => get_touch_point_count(),
        190 => set_gestures_enabled(memory, pointer),
        191 => is_gesture_detected(memory, pointer),
        192 => get_gesture_detected(),
        193 => get_gesture_hold_duration(),
        194 => get_gesture_drag_vector(),
        195 => get_gesture_drag_angle(),
        196 => get_gesture_pinch_vector(),
        197 => get_gesture_pinch_angle(),
        198 => update_camera(memory, pointer),
        199 => update_camera_pro(memory, pointer),
        _ => None,
    } {
        for x in 1..=result_cells.len() {
            (*memory)[pointer - x] = result_cells[x - 1];
        }
    }
}

/// Initialize window and OpenGL context
unsafe fn init_window(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][w][w][h][h][sptr][sptr]
    let input_cells = crate::get_input_cells(memory, pointer, 6);
    let width = crate::cells_to_unsigned(&input_cells[0..2]);
    let height = crate::cells_to_unsigned(&input_cells[2..4]);
    let title_ptr = crate::cells_to_unsigned(&input_cells[4..6]);
    let title = crate::get_string(memory, title_ptr);
    raylib::ffi::InitWindow(width as i32, height as i32, title.as_ptr() as *const i8);
    None
}

/// Close window and unload OpenGL context
unsafe fn close_window() -> Option<Vec<BaseType>> {
    raylib::ffi::CloseWindow();
    None
}

/// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
unsafe fn window_should_close() -> Option<Vec<BaseType>> {
    Some(vec![raylib::ffi::WindowShouldClose() as BaseType])
}

/// Check if window has been initialized successfully
unsafe fn is_window_ready() -> Option<Vec<BaseType>> {
    raylib::ffi::IsWindowReady();
    None
}

/// Check if window is currently fullscreen
unsafe fn is_window_fullscreen() -> Option<Vec<BaseType>> {
    raylib::ffi::IsWindowFullscreen();
    None
}

/// Check if window is currently hidden (only PLATFORM_DESKTOP)
unsafe fn is_window_hidden() -> Option<Vec<BaseType>> {
    raylib::ffi::IsWindowHidden();
    None
}

/// Check if window is currently minimized (only PLATFORM_DESKTOP)
unsafe fn is_window_minimized() -> Option<Vec<BaseType>> {
    raylib::ffi::IsWindowMinimized();
    None
}

/// Check if window is currently maximized (only PLATFORM_DESKTOP)
unsafe fn is_window_maximized() -> Option<Vec<BaseType>> {
    raylib::ffi::IsWindowMaximized();
    None
}

/// Check if window is currently focused (only PLATFORM_DESKTOP)
unsafe fn is_window_focused() -> Option<Vec<BaseType>> {
    raylib::ffi::IsWindowFocused();
    None
}

/// Check if window has been resized last frame
unsafe fn is_window_resized() -> Option<Vec<BaseType>> {
    raylib::ffi::IsWindowResized();
    None
}

/// Check if one specific window flag is enabled
unsafe fn is_window_state(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window configuration state using flags (only PLATFORM_DESKTOP)
unsafe fn set_window_state(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Clear window configuration state flags
unsafe fn clear_window_state(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_fullscreen() -> Option<Vec<BaseType>> {
    raylib::ffi::ToggleFullscreen();
    None
}

/// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_borderless_windowed() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::ToggleBorderlessWindowed()");
}

/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
unsafe fn maximize_window() -> Option<Vec<BaseType>> {
    raylib::ffi::MaximizeWindow();
    None
}

/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
unsafe fn minimize_window() -> Option<Vec<BaseType>> {
    raylib::ffi::MinimizeWindow();
    None
}

/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
unsafe fn restore_window() -> Option<Vec<BaseType>> {
    raylib::ffi::RestoreWindow();
    None
}

/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icon(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icons(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
unsafe fn set_window_title(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window position on screen (only PLATFORM_DESKTOP)
unsafe fn set_window_position(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set monitor for the current window
unsafe fn set_window_monitor(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_min_size(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_max_size(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window dimensions
unsafe fn set_window_size(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
unsafe fn set_window_opacity(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window focused (only PLATFORM_DESKTOP)
unsafe fn set_window_focused() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::SetWindowFocused()")
}

/// Get native window handle
unsafe fn get_window_handle() -> Option<Vec<BaseType>> {
    raylib::ffi::GetWindowHandle();
    None
}

/// Get current screen width
unsafe fn get_screen_width() -> Option<Vec<BaseType>> {
    raylib::ffi::GetScreenWidth();
    None
}

/// Get current screen height
unsafe fn get_screen_height() -> Option<Vec<BaseType>> {
    raylib::ffi::GetScreenHeight();
    None
}

/// Get current render width (it considers HiDPI)
unsafe fn get_render_width() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetRenderWidth()")
}

/// Get current render height (it considers HiDPI)
unsafe fn get_render_height() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetRenderHeight()");
}

/// Get number of connected monitors
unsafe fn get_monitor_count() -> Option<Vec<BaseType>> {
    raylib::ffi::GetMonitorCount();
    None
}

/// Get current connected monitor
unsafe fn get_current_monitor() -> Option<Vec<BaseType>> {
    raylib::ffi::GetCurrentMonitor();
    None
}

/// Get specified monitor position
unsafe fn get_monitor_position(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get specified monitor width (current video mode used by monitor)
unsafe fn get_monitor_width(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get specified monitor height (current video mode used by monitor)
unsafe fn get_monitor_height(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get specified monitor physical width in millimetres
unsafe fn get_monitor_physical_width(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get specified monitor physical height in millimetres
unsafe fn get_monitor_physical_height(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get specified monitor refresh rate
unsafe fn get_monitor_refresh_rate(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get window position XY on monitor
unsafe fn get_window_position() -> Option<Vec<BaseType>> {
    raylib::ffi::GetWindowPosition();
    None
}

/// Get window scale DPI factor
unsafe fn get_window_scale_d_p_i() -> Option<Vec<BaseType>> {
    raylib::ffi::GetWindowScaleDPI();
    None
}

/// Get the human-readable, UTF-8 encoded name of the specified monitor
unsafe fn get_monitor_name(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set clipboard text content
unsafe fn set_clipboard_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get clipboard text content
unsafe fn get_clipboard_text() -> Option<Vec<BaseType>> {
    raylib::ffi::GetClipboardText();
    None
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
unsafe fn enable_event_waiting() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::EnableEventWaiting()")
}

/// Disable waiting for events on EndDrawing(), automatic events polling
unsafe fn disable_event_waiting() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::DisableEventWaiting()")
}

/// Shows cursor
unsafe fn show_cursor() -> Option<Vec<BaseType>> {
    raylib::ffi::ShowCursor();
    None
}

/// Hides cursor
unsafe fn hide_cursor() -> Option<Vec<BaseType>> {
    raylib::ffi::HideCursor();
    None
}

/// Check if cursor is not visible
unsafe fn is_cursor_hidden() -> Option<Vec<BaseType>> {
    raylib::ffi::IsCursorHidden();
    None
}

/// Enables cursor (unlock cursor)
unsafe fn enable_cursor() -> Option<Vec<BaseType>> {
    raylib::ffi::EnableCursor();
    None
}

/// Disables cursor (lock cursor)
unsafe fn disable_cursor() -> Option<Vec<BaseType>> {
    raylib::ffi::DisableCursor();
    None
}

/// Check if cursor is on the screen
unsafe fn is_cursor_on_screen() -> Option<Vec<BaseType>> {
    raylib::ffi::IsCursorOnScreen();
    None
}

/// Set background color (framebuffer clear color)
unsafe fn clear_background(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][cptr][cptr]
    let input_cells = crate::get_input_cells(memory, pointer, 2);
    let color_ptr = crate::cells_to_unsigned(&input_cells[0..2]);
    let color = crate::get_color(memory, color_ptr);
    raylib::ffi::ClearBackground(color);
    None
}

/// Setup canvas (framebuffer) to start drawing
unsafe fn begin_drawing() -> Option<Vec<BaseType>> {
    raylib::ffi::BeginDrawing();
    None
}

/// End canvas drawing and swap buffers (double buffering)
unsafe fn end_drawing() -> Option<Vec<BaseType>> {
    raylib::ffi::EndDrawing();
    None
}

/// Begin 2D mode with custom camera (2D)
unsafe fn begin_mode2_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Ends 2D mode with custom camera
unsafe fn end_mode2_d() -> Option<Vec<BaseType>> {
    raylib::ffi::EndMode2D();
    None
}

/// Begin 3D mode with custom camera (3D)
unsafe fn begin_mode3_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Ends 3D mode and returns to default 2D orthographic mode
unsafe fn end_mode3_d() -> Option<Vec<BaseType>> {
    raylib::ffi::EndMode3D();
    None
}

/// Begin drawing to render texture
unsafe fn begin_texture_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Ends drawing to render texture
unsafe fn end_texture_mode() -> Option<Vec<BaseType>> {
    raylib::ffi::EndTextureMode();
    None
}

/// Begin custom shader drawing
unsafe fn begin_shader_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// End custom shader drawing (use default shader)
unsafe fn end_shader_mode() -> Option<Vec<BaseType>> {
    raylib::ffi::EndShaderMode();
    None
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
unsafe fn begin_blend_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// End blending mode (reset to default: alpha blending)
unsafe fn end_blend_mode() -> Option<Vec<BaseType>> {
    raylib::ffi::EndBlendMode();
    None
}

/// Begin scissor mode (define screen area for following drawing)
unsafe fn begin_scissor_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// End scissor mode
unsafe fn end_scissor_mode() -> Option<Vec<BaseType>> {
    raylib::ffi::EndScissorMode();
    None
}

/// Begin stereo rendering (requires VR simulator)
unsafe fn begin_vr_stereo_mode(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// End stereo rendering (requires VR simulator)
unsafe fn end_vr_stereo_mode() -> Option<Vec<BaseType>> {
    raylib::ffi::EndVrStereoMode();
    None
}

/// Load VR stereo config for VR simulator device parameters
unsafe fn load_vr_stereo_config(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload VR stereo config
unsafe fn unload_vr_stereo_config(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load shader from files and bind default locations
unsafe fn load_shader(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load shader from code strings and bind default locations
unsafe fn load_shader_from_memory(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a shader is ready
unsafe fn is_shader_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get shader uniform location
unsafe fn get_shader_location(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get shader attribute location
unsafe fn get_shader_location_attrib(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set shader uniform value
unsafe fn set_shader_value(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set shader uniform value vector
unsafe fn set_shader_value_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set shader uniform value (matrix 4x4)
unsafe fn set_shader_value_matrix(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set shader uniform value for texture (sampler2d)
unsafe fn set_shader_value_texture(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload shader from GPU memory (VRAM)
unsafe fn unload_shader(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get a ray trace from mouse position
unsafe fn get_mouse_ray(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get camera transform matrix (view matrix)
unsafe fn get_camera_matrix(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get camera 2d transform matrix
unsafe fn get_camera_matrix2_d(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get the screen space position for a 3d world space position
unsafe fn get_world_to_screen(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get the world space position for a 2d camera screen space position
unsafe fn get_screen_to_world2_d(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get size position for a 3d world space position
unsafe fn get_world_to_screen_ex(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get the screen space position for a 2d camera world space position
unsafe fn get_world_to_screen2_d(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set target FPS (maximum)
unsafe fn set_target_f_p_s(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get time in seconds for last frame drawn (delta time)
unsafe fn get_frame_time() -> Option<Vec<BaseType>> {
    raylib::ffi::GetFrameTime();
    None
}

/// Get elapsed time in seconds since InitWindow()
unsafe fn get_time() -> Option<Vec<BaseType>> {
    raylib::ffi::GetTime();
    None
}

/// Get current FPS
unsafe fn get_f_p_s() -> Option<Vec<BaseType>> {
    raylib::ffi::GetFPS();
    None
}

/// Swap back buffer with front buffer (screen drawing)
unsafe fn swap_screen_buffer() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::SwapScreenBuffer()")
}

/// Register all input events
unsafe fn poll_input_events() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::PollInputEvents()")
}

/// Wait for some time (halt program execution)
unsafe fn wait_time(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set the seed for the random number generator
unsafe fn set_random_seed(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get a random value between min and max (both included)
unsafe fn get_random_value(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load random values sequence, no values repeated
unsafe fn load_random_sequence(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload random values sequence
unsafe fn unload_random_sequence(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Takes a screenshot of current screen (filename extension defines format)
unsafe fn take_screenshot(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Setup init configuration flags (view FLAGS)
unsafe fn set_config_flags(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Open URL with default system browser (if available)
unsafe fn open_u_r_l(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Show trace log messages (LOG_DEBUG, LOG_INFO, LOG_WARNING, LOG_ERROR...)
unsafe fn trace_log(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set the current threshold (minimum) log level
unsafe fn set_trace_log_level(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Internal memory allocator
unsafe fn mem_alloc(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Internal memory reallocator
unsafe fn mem_realloc(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Internal memory free
unsafe fn mem_free(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set custom trace log
unsafe fn set_trace_log_callback(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set custom file binary data loader
unsafe fn set_load_file_data_callback(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set custom file binary data saver
unsafe fn set_save_file_data_callback(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set custom file text data loader
unsafe fn set_load_file_text_callback(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set custom file text data saver
unsafe fn set_save_file_text_callback(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load file data as byte array (read)
unsafe fn load_file_data(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload file data allocated by LoadFileData()
unsafe fn unload_file_data(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Save data to file from byte array (write), returns true on success
unsafe fn save_file_data(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export data to code (.h), returns true on success
unsafe fn export_data_as_code(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load text data from file (read), returns a '\0' terminated string
unsafe fn load_file_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload file text data allocated by LoadFileText()
unsafe fn unload_file_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Save text data to file (write), string must be '\0' terminated, returns true on success
unsafe fn save_file_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if file exists
unsafe fn file_exists(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a directory path exists
unsafe fn directory_exists(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check file extension (including point: .png, .wav)
unsafe fn is_file_extension(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get file length in bytes (NOTE: GetFileSize() conflicts with windows.h)
unsafe fn get_file_length(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get pointer to extension for a filename string (includes dot: '.png')
unsafe fn get_file_extension(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get pointer to filename for a path string
unsafe fn get_file_name(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get filename string without extension (uses static string)
unsafe fn get_file_name_without_ext(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get full path for a given fileName with path (uses static string)
unsafe fn get_directory_path(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get previous directory path for a given path (uses static string)
unsafe fn get_prev_directory_path(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get current working directory (uses static string)
unsafe fn get_working_directory() -> Option<Vec<BaseType>> {
    raylib::ffi::GetWorkingDirectory();
    None
}

/// Get the directory of the running application (uses static string)
unsafe fn get_application_directory() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetApplicationDirectory()")
}

/// Change working directory, return true on success
unsafe fn change_directory(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a given path is a file or a directory
unsafe fn is_path_file(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load directory filepaths
unsafe fn load_directory_files(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load directory filepaths with extension filtering and recursive directory scan
unsafe fn load_directory_files_ex(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload filepaths
unsafe fn unload_directory_files(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a file has been dropped into window
unsafe fn is_file_dropped() -> Option<Vec<BaseType>> {
    raylib::ffi::IsFileDropped();
    None
}

/// Load dropped filepaths
unsafe fn load_dropped_files() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::LoadDroppedFiles()")
}

/// Unload dropped filepaths
unsafe fn unload_dropped_files(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get file modification time (last write time)
unsafe fn get_file_mod_time(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Compress data (DEFLATE algorithm), memory must be MemFree()
unsafe fn compress_data(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Decompress data (DEFLATE algorithm), memory must be MemFree()
unsafe fn decompress_data(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Encode data to Base64 string, memory must be MemFree()
unsafe fn encode_data_base64(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Decode Base64 string data, memory must be MemFree()
unsafe fn decode_data_base64(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load automation events list from file, NULL for empty list, capacity = MAX_AUTOMATION_EVENTS
unsafe fn load_automation_event_list(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload automation events list from file
unsafe fn unload_automation_event_list(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export automation events list as text file
unsafe fn export_automation_event_list(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set automation event list to record to
unsafe fn set_automation_event_list(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set automation event internal base frame to start recording
unsafe fn set_automation_event_base_frame(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Start recording automation events (AutomationEventList must be set)
unsafe fn start_automation_event_recording() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::StartAutomationEventRecording()")
}

/// Stop recording automation events
unsafe fn stop_automation_event_recording() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::StopAutomationEventRecording()")
}

/// Play a recorded automation event
unsafe fn play_automation_event(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a key has been pressed once
unsafe fn is_key_pressed(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a key has been pressed again (Only PLATFORM_DESKTOP)
unsafe fn is_key_pressed_repeat(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a key is being pressed
unsafe fn is_key_down(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a key has been released once
unsafe fn is_key_released(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a key is NOT being pressed
unsafe fn is_key_up(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get key pressed (keycode), call it multiple times for keys queued, returns 0 when the queue is empty
unsafe fn get_key_pressed() -> Option<Vec<BaseType>> {
    raylib::ffi::GetKeyPressed();
    None
}

/// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
unsafe fn get_char_pressed() -> Option<Vec<BaseType>> {
    raylib::ffi::GetCharPressed();
    None
}

/// Set a custom key to exit program (default is ESC)
unsafe fn set_exit_key(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a gamepad is available
unsafe fn is_gamepad_available(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get gamepad internal name id
unsafe fn get_gamepad_name(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a gamepad button has been pressed once
unsafe fn is_gamepad_button_pressed(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a gamepad button is being pressed
unsafe fn is_gamepad_button_down(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a gamepad button has been released once
unsafe fn is_gamepad_button_released(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a gamepad button is NOT being pressed
unsafe fn is_gamepad_button_up(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get the last gamepad button pressed
unsafe fn get_gamepad_button_pressed() -> Option<Vec<BaseType>> {
    raylib::ffi::GetGamepadButtonPressed();
    None
}

/// Get gamepad axis count for a gamepad
unsafe fn get_gamepad_axis_count(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get axis movement value for a gamepad axis
unsafe fn get_gamepad_axis_movement(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set internal gamepad mappings (SDL_GameControllerDB)
unsafe fn set_gamepad_mappings(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a mouse button has been pressed once
unsafe fn is_mouse_button_pressed(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a mouse button is being pressed
unsafe fn is_mouse_button_down(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a mouse button has been released once
unsafe fn is_mouse_button_released(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a mouse button is NOT being pressed
unsafe fn is_mouse_button_up(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get mouse position X
unsafe fn get_mouse_x() -> Option<Vec<BaseType>> {
    raylib::ffi::GetMouseX();
    None
}

/// Get mouse position Y
unsafe fn get_mouse_y() -> Option<Vec<BaseType>> {
    raylib::ffi::GetMouseY();
    None
}

/// Get mouse position XY
unsafe fn get_mouse_position() -> Option<Vec<BaseType>> {
    raylib::ffi::GetMousePosition();
    None
}

/// Get mouse delta between frames
unsafe fn get_mouse_delta() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetMouseDelta()")
}

/// Set mouse position XY
unsafe fn set_mouse_position(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set mouse offset
unsafe fn set_mouse_offset(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set mouse scaling
unsafe fn set_mouse_scale(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get mouse wheel movement for X or Y, whichever is larger
unsafe fn get_mouse_wheel_move() -> Option<Vec<BaseType>> {
    raylib::ffi::GetMouseWheelMove();
    None
}

/// Get mouse wheel movement for both X and Y
unsafe fn get_mouse_wheel_move_v() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetMouseWheelMoveV()")
}

/// Set mouse cursor
unsafe fn set_mouse_cursor(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get touch position X for touch point 0 (relative to screen size)
unsafe fn get_touch_x() -> Option<Vec<BaseType>> {
    raylib::ffi::GetTouchX();
    None
}

/// Get touch position Y for touch point 0 (relative to screen size)
unsafe fn get_touch_y() -> Option<Vec<BaseType>> {
    raylib::ffi::GetTouchY();
    None
}

/// Get touch position XY for a touch point index (relative to screen size)
unsafe fn get_touch_position(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get touch point identifier for given index
unsafe fn get_touch_point_id(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get number of touch points
unsafe fn get_touch_point_count() -> Option<Vec<BaseType>> {
    Some(vec![raylib::ffi::GetTouchPointsCount() as BaseType])
}

/// Enable a set of gestures using flags
unsafe fn set_gestures_enabled(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a gesture have been detected
unsafe fn is_gesture_detected(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get latest detected gesture
unsafe fn get_gesture_detected() -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureDetected();
    None
}

/// Get gesture hold time in milliseconds
unsafe fn get_gesture_hold_duration() -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureHoldDuration();
    None
}

/// Get gesture drag vector
unsafe fn get_gesture_drag_vector() -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureDragVector();
    None
}

/// Get gesture drag angle
unsafe fn get_gesture_drag_angle() -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureDragAngle();
    None
}

/// Get gesture pinch delta
unsafe fn get_gesture_pinch_vector() -> Option<Vec<BaseType>> {
    raylib::ffi::GetGesturePinchVector();
    None
}

/// Get gesture pinch angle
unsafe fn get_gesture_pinch_angle() -> Option<Vec<BaseType>> {
    raylib::ffi::GetGesturePinchAngle();
    None
}

/// Update camera position for selected mode
unsafe fn update_camera(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Update camera movement/rotation
unsafe fn update_camera_pro(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}
