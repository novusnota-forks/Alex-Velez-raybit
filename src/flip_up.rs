use crate::BaseType;

const FN_COUNT: usize = 200;
const FN_MAP: [unsafe fn(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>>;
    FN_COUNT] = [
    init_window,
    close_window,
    window_should_close,
    is_window_ready,
    is_window_fullscreen,
    is_window_hidden,
    is_window_minimized,
    is_window_maximized,
    is_window_focused,
    is_window_resized,
    is_window_state,
    set_window_state,
    clear_window_state,
    toggle_fullscreen,
    toggle_borderless_windowed,
    maximize_window,
    minimize_window,
    restore_window,
    set_window_icon,
    set_window_icons,
    set_window_title,
    set_window_position,
    set_window_monitor,
    set_window_min_size,
    set_window_max_size,
    set_window_size,
    set_window_opacity,
    set_window_focused,
    get_window_handle,
    get_screen_width,
    get_screen_height,
    get_render_width,
    get_render_height,
    get_monitor_count,
    get_current_monitor,
    get_monitor_position,
    get_monitor_width,
    get_monitor_height,
    get_monitor_physical_width,
    get_monitor_physical_height,
    get_monitor_refresh_rate,
    get_window_position,
    get_window_scale_d_p_i,
    get_monitor_name,
    set_clipboard_text,
    get_clipboard_text,
    enable_event_waiting,
    disable_event_waiting,
    show_cursor,
    hide_cursor,
    is_cursor_hidden,
    enable_cursor,
    disable_cursor,
    is_cursor_on_screen,
    clear_background,
    begin_drawing,
    end_drawing,
    begin_mode2_d,
    end_mode2_d,
    begin_mode3_d,
    end_mode3_d,
    begin_texture_mode,
    end_texture_mode,
    begin_shader_mode,
    end_shader_mode,
    begin_blend_mode,
    end_blend_mode,
    begin_scissor_mode,
    end_scissor_mode,
    begin_vr_stereo_mode,
    end_vr_stereo_mode,
    load_vr_stereo_config,
    unload_vr_stereo_config,
    load_shader,
    load_shader_from_memory,
    is_shader_ready,
    get_shader_location,
    get_shader_location_attrib,
    set_shader_value,
    set_shader_value_v,
    set_shader_value_matrix,
    set_shader_value_texture,
    unload_shader,
    get_mouse_ray,
    get_camera_matrix,
    get_camera_matrix2_d,
    get_world_to_screen,
    get_screen_to_world2_d,
    get_world_to_screen_ex,
    get_world_to_screen2_d,
    set_target_f_p_s,
    get_frame_time,
    get_time,
    get_f_p_s,
    swap_screen_buffer,
    poll_input_events,
    wait_time,
    set_random_seed,
    get_random_value,
    load_random_sequence,
    unload_random_sequence,
    take_screenshot,
    set_config_flags,
    open_u_r_l,
    trace_log,
    set_trace_log_level,
    mem_alloc,
    mem_realloc,
    mem_free,
    set_trace_log_callback,
    set_load_file_data_callback,
    set_save_file_data_callback,
    set_load_file_text_callback,
    set_save_file_text_callback,
    load_file_data,
    unload_file_data,
    save_file_data,
    export_data_as_code,
    load_file_text,
    unload_file_text,
    save_file_text,
    file_exists,
    directory_exists,
    is_file_extension,
    get_file_length,
    get_file_extension,
    get_file_name,
    get_file_name_without_ext,
    get_directory_path,
    get_prev_directory_path,
    get_working_directory,
    get_application_directory,
    change_directory,
    is_path_file,
    load_directory_files,
    load_directory_files_ex,
    unload_directory_files,
    is_file_dropped,
    load_dropped_files,
    unload_dropped_files,
    get_file_mod_time,
    compress_data,
    decompress_data,
    encode_data_base64,
    decode_data_base64,
    load_automation_event_list,
    unload_automation_event_list,
    export_automation_event_list,
    set_automation_event_list,
    set_automation_event_base_frame,
    start_automation_event_recording,
    stop_automation_event_recording,
    play_automation_event,
    is_key_pressed,
    is_key_pressed_repeat,
    is_key_down,
    is_key_released,
    is_key_up,
    get_key_pressed,
    get_char_pressed,
    set_exit_key,
    is_gamepad_available,
    get_gamepad_name,
    is_gamepad_button_pressed,
    is_gamepad_button_down,
    is_gamepad_button_released,
    is_gamepad_button_up,
    get_gamepad_button_pressed,
    get_gamepad_axis_count,
    get_gamepad_axis_movement,
    set_gamepad_mappings,
    is_mouse_button_pressed,
    is_mouse_button_down,
    is_mouse_button_released,
    is_mouse_button_up,
    get_mouse_x,
    get_mouse_y,
    get_mouse_position,
    get_mouse_delta,
    set_mouse_position,
    set_mouse_offset,
    set_mouse_scale,
    get_mouse_wheel_move,
    get_mouse_wheel_move_v,
    set_mouse_cursor,
    get_touch_x,
    get_touch_y,
    get_touch_position,
    get_touch_point_id,
    get_touch_point_count,
    set_gestures_enabled,
    is_gesture_detected,
    get_gesture_detected,
    get_gesture_hold_duration,
    get_gesture_drag_vector,
    get_gesture_drag_angle,
    get_gesture_pinch_vector,
    get_gesture_pinch_angle,
    update_camera,
    update_camera_pro,
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

// Initialize window and OpenGL context
unsafe fn init_window(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][width][width][height][height][str:ptr][str:ptr]
    let input_cells = crate::get_input_cells(memory, pointer, 6);
    let width = crate::cells_to_unsigned(&input_cells[0..2]);
    let height = crate::cells_to_unsigned(&input_cells[2..4]);
    let title_ptr = crate::cells_to_unsigned(&input_cells[4..6]);
    let title = crate::get_string(memory, title_ptr);
    raylib::ffi::InitWindow(width as i32, height as i32, title.as_ptr() as *const i8);
    None
}

// Close window and unload OpenGL context
unsafe fn close_window(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::CloseWindow();
    None
}

// Check if application should close (KEY_ESCAPE pressed or windows close icon clicked)
unsafe fn window_should_close(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::WindowShouldClose() as BaseType])
}

// Check if window has been initialized successfully
unsafe fn is_window_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::IsWindowReady() as BaseType])
}

// Check if window is currently fullscreen
unsafe fn is_window_fullscreen(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::IsWindowFullscreen() as BaseType])
}

// Check if window is currently hidden (only PLATFORM_DESKTOP)
unsafe fn is_window_hidden(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::IsWindowHidden() as BaseType])
}

// Check if window is currently minimized (only PLATFORM_DESKTOP)
unsafe fn is_window_minimized(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::IsWindowMinimized() as BaseType])
}

// Check if window is currently maximized (only PLATFORM_DESKTOP)
unsafe fn is_window_maximized(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::IsWindowMaximized() as BaseType])
}

// Check if window is currently focused (only PLATFORM_DESKTOP)
unsafe fn is_window_focused(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::IsWindowFocused() as BaseType])
}

// Check if window has been resized last frame
unsafe fn is_window_resized(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@]
    Some(vec![raylib::ffi::IsWindowResized() as BaseType])
}

/// Check if one specific window flag is enabled
unsafe fn is_window_state(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [0/1][@][flag][flag][flag][flag]
    let input_cells = crate::get_input_cells(memory, pointer, 4);
    let flag = crate::cells_to_unsigned(&input_cells) as u32;
    Some(vec![raylib::ffi::IsWindowState(flag) as BaseType])
}

/// Set window configuration state using flags (only PLATFORM_DESKTOP)
unsafe fn set_window_state(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][fc][flag_1][][][]...[flag_fc][][][]
    let flag_count = memory[pointer + 1] as usize;
    let input_cells = crate::get_input_cells(memory, pointer, flag_count * 4);
    let mut flags = 0x0;
    for fcells in input_cells.chunks(4) {
        let flag = crate::cells_to_unsigned(&fcells) as u32;
        flags |= flag;
    }
    raylib::ffi::SetWindowState(flags);
    None
}

/// Clear window configuration state flags
unsafe fn clear_window_state(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][flag_count][flag_1][][][]...[flag_x][][][]
    let flag_count = memory[pointer + 1] as usize;
    let input_cells = crate::get_input_cells(memory, pointer, flag_count * 4);
    let mut flags = 0x0;
    for fcells in input_cells.chunks(4) {
        let flag = crate::cells_to_unsigned(&fcells) as u32;
        flags |= flag;
    }
    raylib::ffi::ClearWindowState(flags);
    None
}

/// Toggle window state: fullscreen/windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_fullscreen(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::ToggleFullscreen();
    None
}

/// Toggle window state: borderless windowed (only PLATFORM_DESKTOP)
unsafe fn toggle_borderless_windowed(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::ToggleBorderlessWindowed()");
}

/// Set window state: maximized, if resizable (only PLATFORM_DESKTOP)
unsafe fn maximize_window(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::MaximizeWindow();
    None
}

/// Set window state: minimized, if resizable (only PLATFORM_DESKTOP)
unsafe fn minimize_window(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::MinimizeWindow();
    None
}

/// Set window state: not minimized/maximized (only PLATFORM_DESKTOP)
unsafe fn restore_window(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::RestoreWindow();
    None
}

/// Set icon for window (single image, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icon(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][image:ptr][image:ptr]
    let input_cells = crate::get_input_cells(memory, pointer, 2);
    let image_ptr = crate::cells_to_unsigned(&input_cells);
    let image = crate::get_image(memory, image_ptr);
    raylib::ffi::SetWindowIcon(image);
    None
}

/// Set icon for window (multiple images, RGBA 32bit, only PLATFORM_DESKTOP)
unsafe fn set_window_icons(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][image_count][image_1:ptr][image_1:ptr]...[image_X:ptr][image_X:ptr]
    let image_count = memory[pointer + 1] as usize;
    let input_cells = crate::get_input_cells(memory, pointer + 1, image_count * 2);
    for ptr_cells in input_cells.chunks(2) {
        let image_ptr = crate::cells_to_unsigned(ptr_cells);
        let image = crate::get_image(memory, image_ptr);
        raylib::ffi::SetWindowIcon(image);
    }
    None
}

/// Set title for window (only PLATFORM_DESKTOP and PLATFORM_WEB)
unsafe fn set_window_title(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][str:ptr][str:ptr]
    let input_cells = crate::get_input_cells(memory, pointer, 2);
    let title_ptr = crate::cells_to_unsigned_u16(&input_cells);
    let title = crate::get_string(memory, title_ptr as usize);
    raylib::ffi::SetWindowTitle(title.as_ptr() as *const i8);
    None
}

/// Set window position on screen (only PLATFORM_DESKTOP)
unsafe fn set_window_position(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][x][x][y][y]
    let input_cells = crate::get_input_cells(memory, pointer, 4);
    let x = crate::cells_to_unsigned_u16(&input_cells[0..2]);
    let y = crate::cells_to_unsigned_u16(&input_cells[2..4]);
    raylib::ffi::SetWindowPosition(x as i32, y as i32);
    None
}

/// Set monitor for the current window
unsafe fn set_window_monitor(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][monitor]
    let input_cells = crate::get_input_cells(memory, pointer, 1);
    let monitor = input_cells[0];
    raylib::ffi::SetWindowMonitor(monitor as i32);
    None
}

/// Set window minimum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_min_size(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][width][width][height][height]
    let input_cells = crate::get_input_cells(memory, pointer, 4);
    let width = crate::cells_to_unsigned(&input_cells[0..2]);
    let height = crate::cells_to_unsigned(&input_cells[2..4]);
    raylib::ffi::SetWindowMinSize(width as i32, height as i32);
    None
}

/// Set window maximum dimensions (for FLAG_WINDOW_RESIZABLE)
unsafe fn set_window_max_size(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][width][width][height][height]
    let input_cells = crate::get_input_cells(memory, pointer, 4);
    let width = crate::cells_to_unsigned(&input_cells[0..2]);
    let height = crate::cells_to_unsigned(&input_cells[2..4]);
    // raylib::ffi::SetWindowMaxSize(width as i32, height as i32);
    unimplemented!()
}

/// Set window dimensions
unsafe fn set_window_size(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][width][width][height][height]
    let input_cells = crate::get_input_cells(memory, pointer, 4);
    let width = crate::cells_to_unsigned(&input_cells[0..2]);
    let height = crate::cells_to_unsigned(&input_cells[2..4]);
    raylib::ffi::SetWindowSize(width as i32, height as i32);
    None
}

/// Set window opacity [0.0f..1.0f] (only PLATFORM_DESKTOP)
unsafe fn set_window_opacity(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set window focused (only PLATFORM_DESKTOP)
unsafe fn set_window_focused(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::SetWindowFocused()")
}

/// Get native window handle
unsafe fn get_window_handle(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetWindowHandle();
    None
}

/// Get current screen width
unsafe fn get_screen_width(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    // [width][@]
    Some(vec![raylib::ffi::GetScreenWidth() as BaseType])
}

/// Get current screen height
unsafe fn get_screen_height(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    // [height][@]
    Some(vec![raylib::ffi::GetScreenHeight() as BaseType])
}

/// Get current render width (it considers HiDPI)
unsafe fn get_render_width(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetRenderWidth()")
}

/// Get current render height (it considers HiDPI)
unsafe fn get_render_height(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetRenderHeight()");
}

/// Get number of connected monitors
unsafe fn get_monitor_count(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    // [monitor_count][@]
    Some(vec![raylib::ffi::GetMonitorCount() as BaseType])
}

/// Get current connected monitor
unsafe fn get_current_monitor(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    // [monitor_index][@]
    Some(vec![raylib::ffi::GetCurrentMonitor() as BaseType])
}

/// Get specified monitor position
unsafe fn get_monitor_position(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    // [pos.y][pos.y][pos.x][pos.x][@][index][index]
    let input_cells = crate::get_input_cells(memory, pointer, 2);
    let index = crate::cells_to_unsigned_u16(&input_cells);
    let pos = raylib::ffi::GetMonitorPosition(index as i32);
    let x = pos.x;

    // Some(vec![raylib::ffi::GetMonitorPosition(index) as BaseType])
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
unsafe fn get_window_position(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetWindowPosition();
    None
}

/// Get window scale DPI factor
unsafe fn get_window_scale_d_p_i(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn get_clipboard_text(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetClipboardText();
    None
}

/// Enable waiting for events on EndDrawing(), no automatic event polling
unsafe fn enable_event_waiting(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::EnableEventWaiting()")
}

/// Disable waiting for events on EndDrawing(), automatic events polling
unsafe fn disable_event_waiting(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::DisableEventWaiting()")
}

/// Shows cursor
unsafe fn show_cursor(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::ShowCursor();
    None
}

/// Hides cursor
unsafe fn hide_cursor(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::HideCursor();
    None
}

/// Check if cursor is not visible
unsafe fn is_cursor_hidden(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::IsCursorHidden();
    None
}

/// Enables cursor (unlock cursor)
unsafe fn enable_cursor(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::EnableCursor();
    None
}

/// Disables cursor (lock cursor)
unsafe fn disable_cursor(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::DisableCursor();
    None
}

/// Check if cursor is on the screen
unsafe fn is_cursor_on_screen(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn begin_drawing(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::BeginDrawing();
    None
}

/// End canvas drawing and swap buffers (double buffering)
unsafe fn end_drawing(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::EndDrawing();
    None
}

/// Begin 2D mode with custom camera (2D)
unsafe fn begin_mode2_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Ends 2D mode with custom camera
unsafe fn end_mode2_d(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::EndMode2D();
    None
}

/// Begin 3D mode with custom camera (3D)
unsafe fn begin_mode3_d(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Ends 3D mode and returns to default 2D orthographic mode
unsafe fn end_mode3_d(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::EndMode3D();
    None
}

/// Begin drawing to render texture
unsafe fn begin_texture_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Ends drawing to render texture
unsafe fn end_texture_mode(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::EndTextureMode();
    None
}

/// Begin custom shader drawing
unsafe fn begin_shader_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// End custom shader drawing (use default shader)
unsafe fn end_shader_mode(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::EndShaderMode();
    None
}

/// Begin blending mode (alpha, additive, multiplied, subtract, custom)
unsafe fn begin_blend_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// End blending mode (reset to default: alpha blending)
unsafe fn end_blend_mode(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::EndBlendMode();
    None
}

/// Begin scissor mode (define screen area for following drawing)
unsafe fn begin_scissor_mode(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// End scissor mode
unsafe fn end_scissor_mode(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
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
unsafe fn end_vr_stereo_mode(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn get_frame_time(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetFrameTime();
    None
}

/// Get elapsed time in seconds since InitWindow()
unsafe fn get_time(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetTime();
    None
}

/// Get current FPS
unsafe fn get_f_p_s(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetFPS();
    None
}

/// Swap back buffer with front buffer (screen drawing)
unsafe fn swap_screen_buffer(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::SwapScreenBuffer()")
}

/// Register all input events
unsafe fn poll_input_events(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
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
unsafe fn get_working_directory(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetWorkingDirectory();
    None
}

/// Get the directory of the running application (uses static string)
unsafe fn get_application_directory(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn is_file_dropped(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::IsFileDropped();
    None
}

/// Load dropped filepaths
unsafe fn load_dropped_files(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn start_automation_event_recording(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::StartAutomationEventRecording()")
}

/// Stop recording automation events
unsafe fn stop_automation_event_recording(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn get_key_pressed(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetKeyPressed();
    None
}

/// Get char pressed (unicode), call it multiple times for chars queued, returns 0 when the queue is empty
unsafe fn get_char_pressed(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
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
unsafe fn get_gamepad_button_pressed(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn get_mouse_x(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetMouseX();
    None
}

/// Get mouse position Y
unsafe fn get_mouse_y(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetMouseY();
    None
}

/// Get mouse position XY
unsafe fn get_mouse_position(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetMousePosition();
    None
}

/// Get mouse delta between frames
unsafe fn get_mouse_delta(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
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
unsafe fn get_mouse_wheel_move(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetMouseWheelMove();
    None
}

/// Get mouse wheel movement for both X and Y
unsafe fn get_mouse_wheel_move_v(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetMouseWheelMoveV()")
}

/// Set mouse cursor
unsafe fn set_mouse_cursor(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get touch position X for touch point 0 (relative to screen size)
unsafe fn get_touch_x(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::GetTouchX();
    None
}

/// Get touch position Y for touch point 0 (relative to screen size)
unsafe fn get_touch_y(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
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
unsafe fn get_touch_point_count(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
unsafe fn get_gesture_detected(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureDetected();
    None
}

/// Get gesture hold time in milliseconds
unsafe fn get_gesture_hold_duration(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureHoldDuration();
    None
}

/// Get gesture drag vector
unsafe fn get_gesture_drag_vector(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureDragVector();
    None
}

/// Get gesture drag angle
unsafe fn get_gesture_drag_angle(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetGestureDragAngle();
    None
}

/// Get gesture pinch delta
unsafe fn get_gesture_pinch_vector(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::GetGesturePinchVector();
    None
}

/// Get gesture pinch angle
unsafe fn get_gesture_pinch_angle(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
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
