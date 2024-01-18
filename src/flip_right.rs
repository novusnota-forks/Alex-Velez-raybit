use crate::BaseType;

const FN_COUNT: usize = 66;
const FN_MAP: [unsafe fn(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>>;
    FN_COUNT] = [
    init_audio_device,
    close_audio_device,
    is_audio_device_ready,
    set_master_volume,
    get_master_volume,
    load_wave,
    load_wave_from_memory,
    is_wave_ready,
    load_sound,
    load_sound_from_wave,
    load_sound_alias,
    is_sound_ready,
    update_sound,
    unload_wave,
    unload_sound,
    unload_sound_alias,
    export_wave,
    export_wave_as_code,
    play_sound,
    stop_sound,
    pause_sound,
    resume_sound,
    is_sound_playing,
    set_sound_volume,
    set_sound_pitch,
    set_sound_pan,
    wave_copy,
    wave_crop,
    wave_format,
    load_wave_samples,
    unload_wave_samples,
    load_music_stream,
    load_music_stream_from_memory,
    is_music_ready,
    unload_music_stream,
    play_music_stream,
    is_music_stream_playing,
    update_music_stream,
    stop_music_stream,
    pause_music_stream,
    resume_music_stream,
    seek_music_stream,
    set_music_volume,
    set_music_pitch,
    set_music_pan,
    get_music_time_length,
    get_music_time_played,
    load_audio_stream,
    is_audio_stream_ready,
    unload_audio_stream,
    update_audio_stream,
    is_audio_stream_processed,
    play_audio_stream,
    pause_audio_stream,
    resume_audio_stream,
    is_audio_stream_playing,
    stop_audio_stream,
    set_audio_stream_volume,
    set_audio_stream_pitch,
    set_audio_stream_pan,
    set_audio_stream_buffer_size_default,
    set_audio_stream_callback,
    attach_audio_stream_processor,
    detach_audio_stream_processor,
    attach_audio_mixed_processor,
    detach_audio_mixed_processor,
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

/// Initialize audio device and context
unsafe fn init_audio_device(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
    raylib::ffi::InitAudioDevice();
    None
}

/// Close the audio device and context
unsafe fn close_audio_device(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::CloseAudioDevice();
    None
}

/// Check if audio device has been initialized successfully
unsafe fn is_audio_device_ready(
    _memory: &mut Vec<BaseType>,
    _pointer: usize,
) -> Option<Vec<BaseType>> {
    raylib::ffi::IsAudioDeviceReady();
    None
}

/// Set master volume (listener)
unsafe fn set_master_volume(memory: &mut Vec<BaseType>, pointer: usize) -> Option<Vec<BaseType>> {
    unimplemented!()
}

/// Get master volume (listener)
unsafe fn get_master_volume(_memory: &mut Vec<BaseType>, _pointer: usize) -> Option<Vec<BaseType>> {
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
