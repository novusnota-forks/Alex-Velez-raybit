use crate::BaseType;

pub unsafe fn call(memory: &mut Vec<BaseType>, pointer: usize) {
    if let Some(result_cells) = match memory[pointer] {
        0 => load_image(memory, pointer),
        1 => load_image_raw(memory, pointer),
        2 => load_image_svg(memory, pointer),
        3 => load_image_anim(memory, pointer),
        4 => load_image_from_memory(memory, pointer),
        5 => load_image_from_texture(memory, pointer),
        6 => load_image_from_screen(),
        7 => is_image_ready(memory, pointer),
        8 => unload_image(memory, pointer),
        9 => export_image(memory, pointer),
        10 => export_image_to_memory(memory, pointer),
        11 => export_image_as_code(memory, pointer),
        12 => gen_image_color(memory, pointer),
        13 => gen_image_gradient_linear(memory, pointer),
        14 => gen_image_gradient_radial(memory, pointer),
        15 => gen_image_gradient_square(memory, pointer),
        16 => gen_image_checked(memory, pointer),
        17 => gen_image_white_noise(memory, pointer),
        18 => gen_image_perlin_noise(memory, pointer),
        19 => gen_image_cellular(memory, pointer),
        20 => gen_image_text(memory, pointer),
        21 => image_copy(memory, pointer),
        22 => image_from_image(memory, pointer),
        23 => image_text(memory, pointer),
        24 => image_text_ex(memory, pointer),
        25 => image_format(memory, pointer),
        26 => image_to_p_o_t(memory, pointer),
        27 => image_crop(memory, pointer),
        28 => image_alpha_crop(memory, pointer),
        29 => image_alpha_clear(memory, pointer),
        30 => image_alpha_mask(memory, pointer),
        31 => image_alpha_premultiply(memory, pointer),
        32 => image_blur_gaussian(memory, pointer),
        33 => image_resize(memory, pointer),
        34 => image_resize_n_n(memory, pointer),
        35 => image_resize_canvas(memory, pointer),
        36 => image_mipmaps(memory, pointer),
        37 => image_dither(memory, pointer),
        38 => image_flip_vertical(memory, pointer),
        39 => image_flip_horizontal(memory, pointer),
        40 => image_rotate(memory, pointer),
        41 => image_rotate_c_w(memory, pointer),
        42 => image_rotate_c_c_w(memory, pointer),
        43 => image_color_tint(memory, pointer),
        44 => image_color_invert(memory, pointer),
        45 => image_color_grayscale(memory, pointer),
        46 => image_color_contrast(memory, pointer),
        47 => image_color_brightness(memory, pointer),
        48 => image_color_replace(memory, pointer),
        49 => load_image_colors(memory, pointer),
        50 => load_image_palette(memory, pointer),
        51 => unload_image_colors(memory, pointer),
        52 => unload_image_palette(memory, pointer),
        53 => get_image_alpha_border(memory, pointer),
        54 => get_image_color(memory, pointer),
        55 => image_clear_background(memory, pointer),
        56 => image_draw_pixel(memory, pointer),
        57 => image_draw_pixel_v(memory, pointer),
        58 => image_draw_line(memory, pointer),
        59 => image_draw_line_v(memory, pointer),
        60 => image_draw_circle(memory, pointer),
        61 => image_draw_circle_v(memory, pointer),
        62 => image_draw_circle_lines(memory, pointer),
        63 => image_draw_circle_lines_v(memory, pointer),
        64 => image_draw_rectangle(memory, pointer),
        65 => image_draw_rectangle_v(memory, pointer),
        66 => image_draw_rectangle_rec(memory, pointer),
        67 => image_draw_rectangle_lines(memory, pointer),
        68 => image_draw(memory, pointer),
        69 => image_draw_text(memory, pointer),
        70 => image_draw_text_ex(memory, pointer),
        71 => load_texture(memory, pointer),
        72 => load_texture_from_image(memory, pointer),
        73 => load_texture_cubemap(memory, pointer),
        74 => load_render_texture(memory, pointer),
        75 => is_texture_ready(memory, pointer),
        76 => unload_texture(memory, pointer),
        77 => is_render_texture_ready(memory, pointer),
        78 => unload_render_texture(memory, pointer),
        79 => update_texture(memory, pointer),
        80 => update_texture_rec(memory, pointer),
        81 => gen_texture_mipmaps(memory, pointer),
        82 => set_texture_filter(memory, pointer),
        83 => set_texture_wrap(memory, pointer),
        84 => draw_texture(memory, pointer),
        85 => draw_texture_v(memory, pointer),
        86 => draw_texture_ex(memory, pointer),
        87 => draw_texture_rec(memory, pointer),
        88 => draw_texture_pro(memory, pointer),
        89 => draw_texture_n_patch(memory, pointer),
        90 => fade(memory, pointer),
        91 => color_to_int(memory, pointer),
        92 => color_normalize(memory, pointer),
        93 => color_from_normalized(memory, pointer),
        94 => color_to_h_s_v(memory, pointer),
        95 => color_from_h_s_v(memory, pointer),
        96 => color_tint(memory, pointer),
        97 => color_brightness(memory, pointer),
        98 => color_contrast(memory, pointer),
        99 => color_alpha(memory, pointer),
        100 => color_alpha_blend(memory, pointer),
        101 => get_color(memory, pointer),
        102 => get_pixel_color(memory, pointer),
        103 => set_pixel_color(memory, pointer),
        104 => get_pixel_data_size(memory, pointer),
        105 => get_font_default(),
        106 => load_font(memory, pointer),
        107 => load_font_ex(memory, pointer),
        108 => load_font_from_image(memory, pointer),
        109 => load_font_from_memory(memory, pointer),
        110 => is_font_ready(memory, pointer),
        111 => load_font_data(memory, pointer),
        112 => gen_image_font_atlas(memory, pointer),
        113 => unload_font_data(memory, pointer),
        114 => unload_font(memory, pointer),
        115 => export_font_as_code(memory, pointer),
        116 => draw_f_p_s(memory, pointer),
        117 => draw_text(memory, pointer),
        118 => draw_text_ex(memory, pointer),
        119 => draw_text_pro(memory, pointer),
        120 => draw_text_codepoint(memory, pointer),
        121 => draw_text_codepoints(memory, pointer),
        122 => set_text_line_spacing(memory, pointer),
        123 => measure_text(memory, pointer),
        124 => measure_text_ex(memory, pointer),
        125 => get_glyph_index(memory, pointer),
        126 => get_glyph_info(memory, pointer),
        127 => get_glyph_atlas_rec(memory, pointer),
        128 => load_u_t_f8(memory, pointer),
        129 => unload_u_t_f8(memory, pointer),
        130 => load_codepoints(memory, pointer),
        131 => unload_codepoints(memory, pointer),
        132 => get_codepoint_count(memory, pointer),
        133 => get_codepoint(memory, pointer),
        134 => get_codepoint_next(memory, pointer),
        135 => get_codepoint_previous(memory, pointer),
        136 => codepoint_to_u_t_f8(memory, pointer),
        137 => text_copy(memory, pointer),
        138 => text_is_equal(memory, pointer),
        139 => text_length(memory, pointer),
        140 => text_format(memory, pointer),
        141 => text_subtext(memory, pointer),
        142 => text_replace(memory, pointer),
        143 => text_insert(memory, pointer),
        144 => text_join(memory, pointer),
        145 => text_split(memory, pointer),
        146 => text_append(memory, pointer),
        147 => text_find_index(memory, pointer),
        148 => text_to_upper(memory, pointer),
        149 => text_to_lower(memory, pointer),
        150 => text_to_pascal(memory, pointer),
        151 => text_to_integer(memory, pointer),
        _ => None,
    } {
        for x in 1..=result_cells.len() {
            (*memory)[pointer - x] = result_cells[x - 1];
        }
    }
}

/// Load image from file into CPU memory (RAM)
unsafe fn load_image(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load image from RAW file data
unsafe fn load_image_raw(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load image from SVG file data or string with specified size
unsafe fn load_image_svg(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load image sequence from file (frames appended to image.data)
unsafe fn load_image_anim(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load image from memory buffer, fileType refers to extension: i.e. '.png'
unsafe fn load_image_from_memory(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load image from GPU texture data
unsafe fn load_image_from_texture(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load image from screen buffer and (screenshot)
unsafe fn load_image_from_screen() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::LoadImageFromScreen()")
}

/// Check if an image is ready
unsafe fn is_image_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload image from CPU memory (RAM)
unsafe fn unload_image(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export image data to file, returns true on success
unsafe fn export_image(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export image to memory buffer
unsafe fn export_image_to_memory(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export image as code file defining an array of bytes, returns true on success
unsafe fn export_image_as_code(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: plain color
unsafe fn gen_image_color(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: linear gradient, direction in degrees [0..360], 0=Vertical gradient
unsafe fn gen_image_gradient_linear(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: radial gradient
unsafe fn gen_image_gradient_radial(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: square gradient
unsafe fn gen_image_gradient_square(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: checked
unsafe fn gen_image_checked(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: white noise
unsafe fn gen_image_white_noise(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: perlin noise
unsafe fn gen_image_perlin_noise(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: cellular algorithm, bigger tileSize means bigger cells
unsafe fn gen_image_cellular(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image: grayscale image from text data
unsafe fn gen_image_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Create an image duplicate (useful for transformations)
unsafe fn image_copy(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Create an image from another image piece
unsafe fn image_from_image(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Create an image from text (default font)
unsafe fn image_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Create an image from text (custom sprite font)
unsafe fn image_text_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Convert image data to desired format
unsafe fn image_format(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Convert image to POT (power-of-two)
unsafe fn image_to_p_o_t(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Crop an image to a defined rectangle
unsafe fn image_crop(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Crop image depending on alpha value
unsafe fn image_alpha_crop(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Clear alpha channel to desired color
unsafe fn image_alpha_clear(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Apply alpha mask to image
unsafe fn image_alpha_mask(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Premultiply alpha channel
unsafe fn image_alpha_premultiply(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Apply Gaussian blur using a box blur approximation
unsafe fn image_blur_gaussian(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Resize image (Bicubic scaling algorithm)
unsafe fn image_resize(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Resize image (Nearest-Neighbor scaling algorithm)
unsafe fn image_resize_n_n(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Resize canvas and fill with color
unsafe fn image_resize_canvas(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Compute all mipmap levels for a provided image
unsafe fn image_mipmaps(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Dither image data to 16bpp or lower (Floyd-Steinberg dithering)
unsafe fn image_dither(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Flip image vertically
unsafe fn image_flip_vertical(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Flip image horizontally
unsafe fn image_flip_horizontal(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Rotate image by input angle in degrees (-359 to 359)
unsafe fn image_rotate(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Rotate image clockwise 90deg
unsafe fn image_rotate_c_w(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Rotate image counter-clockwise 90deg
unsafe fn image_rotate_c_c_w(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Modify image color: tint
unsafe fn image_color_tint(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Modify image color: invert
unsafe fn image_color_invert(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Modify image color: grayscale
unsafe fn image_color_grayscale(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Modify image color: contrast (-100 to 100)
unsafe fn image_color_contrast(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Modify image color: brightness (-255 to 255)
unsafe fn image_color_brightness(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Modify image color: replace color
unsafe fn image_color_replace(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load color data from image as a Color array (RGBA - 32bit)
unsafe fn load_image_colors(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load colors palette from image as a Color array (RGBA - 32bit)
unsafe fn load_image_palette(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload color data loaded with LoadImageColors()
unsafe fn unload_image_colors(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload colors palette loaded with LoadImagePalette()
unsafe fn unload_image_palette(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get image alpha border rectangle
unsafe fn get_image_alpha_border(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get image pixel color at (x, y) position
unsafe fn get_image_color(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Clear image background with given color
unsafe fn image_clear_background(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw pixel within an image
unsafe fn image_draw_pixel(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw pixel within an image (Vector version)
unsafe fn image_draw_pixel_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw line within an image
unsafe fn image_draw_line(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw line within an image (Vector version)
unsafe fn image_draw_line_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a filled circle within an image
unsafe fn image_draw_circle(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a filled circle within an image (Vector version)
unsafe fn image_draw_circle_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw circle outline within an image
unsafe fn image_draw_circle_lines(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw circle outline within an image (Vector version)
unsafe fn image_draw_circle_lines_v(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle within an image
unsafe fn image_draw_rectangle(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle within an image (Vector version)
unsafe fn image_draw_rectangle_v(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle within an image
unsafe fn image_draw_rectangle_rec(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw rectangle lines within an image
unsafe fn image_draw_rectangle_lines(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a source image within a destination image (tint applied to source)
unsafe fn image_draw(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw text (using default font) within an image (destination)
unsafe fn image_draw_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw text (custom sprite font) within an image (destination)
unsafe fn image_draw_text_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load texture from file into GPU memory (VRAM)
unsafe fn load_texture(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load texture from image data
unsafe fn load_texture_from_image(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load cubemap from image, multiple image cubemap layouts supported
unsafe fn load_texture_cubemap(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load texture for rendering (framebuffer)
unsafe fn load_render_texture(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a texture is ready
unsafe fn is_texture_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload texture from GPU memory (VRAM)
unsafe fn unload_texture(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a render texture is ready
unsafe fn is_render_texture_ready(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload render texture from GPU memory (VRAM)
unsafe fn unload_render_texture(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Update GPU texture with new data
unsafe fn update_texture(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Update GPU texture rectangle with new data
unsafe fn update_texture_rec(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate GPU mipmaps for a texture
unsafe fn gen_texture_mipmaps(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set texture scaling filter mode
unsafe fn set_texture_filter(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set texture wrapping mode
unsafe fn set_texture_wrap(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a Texture2D
unsafe fn draw_texture(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a Texture2D with position defined as Vector2
unsafe fn draw_texture_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a Texture2D with extended parameters
unsafe fn draw_texture_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a part of a texture defined by a rectangle
unsafe fn draw_texture_rec(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw a part of a texture defined by a rectangle with 'pro' parameters
unsafe fn draw_texture_pro(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draws a texture (or part of it) that stretches or shrinks nicely
unsafe fn draw_texture_n_patch(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
unsafe fn fade(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get hexadecimal value for a Color
unsafe fn color_to_int(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get Color normalized as float [0..1]
unsafe fn color_normalize(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get Color from normalized values [0..1]
unsafe fn color_from_normalized(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get HSV values for a Color, hue [0..360], saturation/value [0..1]
unsafe fn color_to_h_s_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get a Color from HSV values, hue [0..360], saturation/value [0..1]
unsafe fn color_from_h_s_v(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get color multiplied with another color
unsafe fn color_tint(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get color with brightness correction, brightness factor goes from -1.0f to 1.0f
unsafe fn color_brightness(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get color with contrast correction, contrast values between -1.0f and 1.0f
unsafe fn color_contrast(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get color with alpha applied, alpha goes from 0.0f to 1.0f
unsafe fn color_alpha(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get src alpha-blended into dst color with tint
unsafe fn color_alpha_blend(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get Color structure from hexadecimal value
unsafe fn get_color(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get Color from a source pixel pointer of certain format
unsafe fn get_pixel_color(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set color formatted into destination pixel pointer
unsafe fn set_pixel_color(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get pixel data size in bytes for certain format
unsafe fn get_pixel_data_size(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get the default Font
unsafe fn get_font_default() -> Option<Vec<BaseType>> {
    raylib::ffi::GetFontDefault();
    None
}

/// Load font from file into GPU memory (VRAM)
unsafe fn load_font(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load font from file with extended parameters, use NULL for codepoints and 0 for codepointCount to load the default character set
unsafe fn load_font_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load font from Image (XNA style)
unsafe fn load_font_from_image(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load font from memory buffer, fileType refers to extension: i.e. '.ttf'
unsafe fn load_font_from_memory(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a font is ready
unsafe fn is_font_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load font data for further use
unsafe fn load_font_data(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Generate image font atlas using chars info
unsafe fn gen_image_font_atlas(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload font chars info data (RAM)
unsafe fn unload_font_data(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload font from GPU memory (VRAM)
unsafe fn unload_font(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export font as code file, returns true on success
unsafe fn export_font_as_code(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw current FPS
unsafe fn draw_f_p_s(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw text (using default font)
unsafe fn draw_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    // [@][sptr][sptr][x][x][y][y][font][cptr][cptr]
    let input_cells = crate::get_input_cells(memory, pointer, 9);
    let text_ptr = crate::cells_to_unsigned(&input_cells[0..2]);
    let text = crate::get_string(memory, text_ptr);
    let pos_x = crate::cells_to_unsigned(&input_cells[2..4]) as i32;
    let pos_y = crate::cells_to_unsigned(&input_cells[4..6]) as i32;
    let font_size = crate::cells_to_unsigned(&input_cells[6..7]) as i32;
    let color_ptr = crate::cells_to_unsigned(&input_cells[7..9]);
    let color = crate::get_color(memory, color_ptr);
    raylib::ffi::DrawText(text.as_ptr() as *const i8, pos_x, pos_y, font_size, color);
    None
}

/// Draw text using font and additional parameters
unsafe fn draw_text_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw text using Font and pro parameters (rotation)
unsafe fn draw_text_pro(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw one character (codepoint)
unsafe fn draw_text_codepoint(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Draw multiple character (codepoint)
unsafe fn draw_text_codepoints(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set vertical line spacing when drawing with line-breaks
unsafe fn set_text_line_spacing(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Measure string width for default font
unsafe fn measure_text(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Measure string size for Font
unsafe fn measure_text_ex(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get glyph index position in font for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_index(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get glyph font info data for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_info(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get glyph rectangle in font atlas for a codepoint (unicode character), fallback to '?' if not found
unsafe fn get_glyph_atlas_rec(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load UTF-8 text encoded from codepoints array
unsafe fn load_u_t_f8(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload UTF-8 text encoded from codepoints array
unsafe fn unload_u_t_f8(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load all codepoints from a UTF-8 text string, codepoints count returned by parameter
unsafe fn load_codepoints(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload codepoints data from memory
unsafe fn unload_codepoints(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get total number of codepoints in a UTF-8 encoded string
unsafe fn get_codepoint_count(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get next codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint_next(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get previous codepoint in a UTF-8 encoded string, 0x3f('?') is returned on failure
unsafe fn get_codepoint_previous(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Encode one codepoint into UTF-8 byte array (array length returned as parameter)
unsafe fn codepoint_to_u_t_f8(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Copy one string to another, returns bytes copied
unsafe fn text_copy(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if two text string are equal
unsafe fn text_is_equal(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get text length, checks for '\0' ending
unsafe fn text_length(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Text formatting with variables (sprintf() style)
unsafe fn text_format(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get a piece of a text string
unsafe fn text_subtext(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Replace text string (WARNING: memory must be freed!)
unsafe fn text_replace(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Insert text in a position (WARNING: memory must be freed!)
unsafe fn text_insert(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Join text strings with delimiter
unsafe fn text_join(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Split text into multiple strings
unsafe fn text_split(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Append text at specific position and move cursor!
unsafe fn text_append(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Find first text occurrence within a string
unsafe fn text_find_index(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get upper case version of provided string
unsafe fn text_to_upper(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get lower case version of provided string
unsafe fn text_to_lower(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get Pascal case notation version of provided string
unsafe fn text_to_pascal(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get integer value from text (negative values not supported)
unsafe fn text_to_integer(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}
