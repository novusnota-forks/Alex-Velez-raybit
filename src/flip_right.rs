use crate::BaseType;

pub unsafe fn call(memory: &mut Vec<BaseType>, pointer: usize) {
    if let Some(result_cells) = match memory[pointer] {
        0 => init_audio_device(),
        1 => close_audio_device(),
        2 => is_audio_device_ready(),
        3 => set_master_volume(memory, pointer),
        4 => get_master_volume(),
        5 => load_wave(memory, pointer),
        6 => load_wave_from_memory(memory, pointer),
        7 => is_wave_ready(memory, pointer),
        8 => load_sound(memory, pointer),
        9 => load_sound_from_wave(memory, pointer),
        10 => load_sound_alias(memory, pointer),
        11 => is_sound_ready(memory, pointer),
        12 => update_sound(memory, pointer),
        13 => unload_wave(memory, pointer),
        14 => unload_sound(memory, pointer),
        15 => unload_sound_alias(memory, pointer),
        16 => export_wave(memory, pointer),
        17 => export_wave_as_code(memory, pointer),
        18 => play_sound(memory, pointer),
        19 => stop_sound(memory, pointer),
        20 => pause_sound(memory, pointer),
        21 => resume_sound(memory, pointer),
        22 => is_sound_playing(memory, pointer),
        23 => set_sound_volume(memory, pointer),
        24 => set_sound_pitch(memory, pointer),
        25 => set_sound_pan(memory, pointer),
        26 => wave_copy(memory, pointer),
        27 => wave_crop(memory, pointer),
        28 => wave_format(memory, pointer),
        29 => load_wave_samples(memory, pointer),
        30 => unload_wave_samples(memory, pointer),
        31 => load_music_stream(memory, pointer),
        32 => load_music_stream_from_memory(memory, pointer),
        33 => is_music_ready(memory, pointer),
        34 => unload_music_stream(memory, pointer),
        35 => play_music_stream(memory, pointer),
        36 => is_music_stream_playing(memory, pointer),
        37 => update_music_stream(memory, pointer),
        38 => stop_music_stream(memory, pointer),
        39 => pause_music_stream(memory, pointer),
        40 => resume_music_stream(memory, pointer),
        41 => seek_music_stream(memory, pointer),
        42 => set_music_volume(memory, pointer),
        43 => set_music_pitch(memory, pointer),
        44 => set_music_pan(memory, pointer),
        45 => get_music_time_length(memory, pointer),
        46 => get_music_time_played(memory, pointer),
        47 => load_audio_stream(memory, pointer),
        48 => is_audio_stream_ready(memory, pointer),
        49 => unload_audio_stream(memory, pointer),
        50 => update_audio_stream(memory, pointer),
        51 => is_audio_stream_processed(memory, pointer),
        52 => play_audio_stream(memory, pointer),
        53 => pause_audio_stream(memory, pointer),
        54 => resume_audio_stream(memory, pointer),
        55 => is_audio_stream_playing(memory, pointer),
        56 => stop_audio_stream(memory, pointer),
        57 => set_audio_stream_volume(memory, pointer),
        58 => set_audio_stream_pitch(memory, pointer),
        59 => set_audio_stream_pan(memory, pointer),
        60 => set_audio_stream_buffer_size_default(memory, pointer),
        61 => set_audio_stream_callback(memory, pointer),
        62 => attach_audio_stream_processor(memory, pointer),
        63 => detach_audio_stream_processor(memory, pointer),
        64 => attach_audio_mixed_processor(memory, pointer),
        65 => detach_audio_mixed_processor(memory, pointer),
        _ => None,
    } {
        for x in 1..=result_cells.len() {
            (*memory)[pointer - x] = result_cells[x - 1];
        }
    }
}

/// Initialize audio device and context
unsafe fn init_audio_device() -> Option<Vec<BaseType>> {
    raylib::ffi::InitAudioDevice();
    None
}

/// Close the audio device and context
unsafe fn close_audio_device() -> Option<Vec<BaseType>> {
    raylib::ffi::CloseAudioDevice();
    None
}

/// Check if audio device has been initialized successfully
unsafe fn is_audio_device_ready() -> Option<Vec<BaseType>> {
    raylib::ffi::IsAudioDeviceReady();
    None
}

/// Set master volume (listener)
unsafe fn set_master_volume(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get master volume (listener)
unsafe fn get_master_volume() -> Option<Vec<BaseType>> {
    unimplemented!("raylib::ffi::GetMasterVolume()");
}

/// Load wave data from file
unsafe fn load_wave(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load wave from memory buffer, fileType refers to extension: i.e. '.wav'
unsafe fn load_wave_from_memory(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Checks if wave data is ready
unsafe fn is_wave_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load sound from file
unsafe fn load_sound(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load sound from wave data
unsafe fn load_sound_from_wave(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Create a new sound that shares the same sample data as the source sound, does not own the sound data
unsafe fn load_sound_alias(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Checks if a sound is ready
unsafe fn is_sound_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Update sound buffer with new data
unsafe fn update_sound(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload wave data
unsafe fn unload_wave(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload sound
unsafe fn unload_sound(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload a sound alias (does not deallocate sample data)
unsafe fn unload_sound_alias(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export wave data to file, returns true on success
unsafe fn export_wave(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Export wave sample data to code (.h), returns true on success
unsafe fn export_wave_as_code(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Play a sound
unsafe fn play_sound(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Stop playing a sound
unsafe fn stop_sound(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Pause a sound
unsafe fn pause_sound(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Resume a paused sound
unsafe fn resume_sound(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if a sound is currently playing
unsafe fn is_sound_playing(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set volume for a sound (1.0 is max level)
unsafe fn set_sound_volume(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set pitch for a sound (1.0 is base level)
unsafe fn set_sound_pitch(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set pan for a sound (0.5 is center)
unsafe fn set_sound_pan(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Copy a wave to a new wave
unsafe fn wave_copy(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Crop a wave to defined samples range
unsafe fn wave_crop(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Convert wave data to desired format
unsafe fn wave_format(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load samples data from wave as a 32bit float data array
unsafe fn load_wave_samples(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload samples data loaded with LoadWaveSamples()
unsafe fn unload_wave_samples(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load music stream from file
unsafe fn load_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load music stream from data
unsafe fn load_music_stream_from_memory(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Checks if a music stream is ready
unsafe fn is_music_ready(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload music stream
unsafe fn unload_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Start music playing
unsafe fn play_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if music is playing
unsafe fn is_music_stream_playing(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Updates buffers for music streaming
unsafe fn update_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Stop music playing
unsafe fn stop_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Pause music playing
unsafe fn pause_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Resume playing paused music
unsafe fn resume_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Seek music to a position (in seconds)
unsafe fn seek_music_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set volume for music (1.0 is max level)
unsafe fn set_music_volume(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set pitch for a music (1.0 is base level)
unsafe fn set_music_pitch(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set pan for a music (0.5 is center)
unsafe fn set_music_pan(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get music time length (in seconds)
unsafe fn get_music_time_length(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get current music time played (in seconds)
unsafe fn get_music_time_played(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Load audio stream (to stream raw audio pcm data)
unsafe fn load_audio_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Checks if an audio stream is ready
unsafe fn is_audio_stream_ready(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Unload audio stream and free memory
unsafe fn unload_audio_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Update audio stream buffers with data
unsafe fn update_audio_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if any audio stream buffers requires refill
unsafe fn is_audio_stream_processed(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Play audio stream
unsafe fn play_audio_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Pause audio stream
unsafe fn pause_audio_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Resume audio stream
unsafe fn resume_audio_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Check if audio stream is playing
unsafe fn is_audio_stream_playing(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Stop audio stream
unsafe fn stop_audio_stream(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set volume for audio stream (1.0 is max level)
unsafe fn set_audio_stream_volume(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set pitch for audio stream (1.0 is base level)
unsafe fn set_audio_stream_pitch(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Set pan for audio stream (0.5 is centered)
unsafe fn set_audio_stream_pan(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Default size for new audio streams
unsafe fn set_audio_stream_buffer_size_default(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Audio thread callback to request new data
unsafe fn set_audio_stream_callback(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Attach audio stream processor to stream, receives the samples as <float>s
unsafe fn attach_audio_stream_processor(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Detach audio stream processor from stream
unsafe fn detach_audio_stream_processor(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Attach audio stream processor to the entire audio pipeline, receives the samples as <float>s
unsafe fn attach_audio_mixed_processor(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Detach audio stream processor from the entire audio pipeline
unsafe fn detach_audio_mixed_processor(
    memory: &mut Vec<BaseType>,
    pointer: usize,
) -> Option<Vec<BaseType>> {
    unimplemented!()
}
