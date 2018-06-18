use rodio::*;
use std::fs::File;
use std::io::BufReader;
use std::thread::sleep;
use std::time::Duration;
use std::collections::HashMap;

pub struct AudioContext {
    tracks: Vec<Sink>
}

enum ThreadStatus {
    Playing,
    Paused,
    Inactive
}

/// An unsigned integer between 0 and 100 that specifies volume to play an audio track at.
pub type Volume = u8;

fn volume_bounds_check(vol: Volume) -> bool {
    vol <= 100
}

impl AudioContext {
    pub fn new() -> AudioContext {
        AudioContext {
            tracks: Vec::new()
        }
    }
    /// Loads an audio track from a designated file, and maps it to a designated string identifier. Returns Err(()) on failure to open the file.
    pub fn load(&mut self, path: &str, name: &str) -> Result<(), ()> {
        unimplemented!()
    }
    /// Plays a specific audio track on a newly generated thread. Returns Err(()) if sound is not loaded. On success, returns a thread identifier,
    /// represented by a u64.
    pub fn play_on_new_thread(&mut self, sound_name: &str) -> Result<u64, ()> {
        unimplemented!()
    }
    /// Plays a specific audio track on an already existing thread. Fails if sound or thread is uninitialized
    pub fn play_once_thread_is_available(&mut self, sound_name: &str, thread: u64) -> Result<(), ()> {
        unimplemented!()
    }
    /// Unlinks a certain sound track identifier and unloads the sound from memory. May cause unexpected results if a thread is using the track.
    pub fn unload(&mut self, name: &str) -> Result<(), ()> {
        unimplemented!()
    }
    /// Stops playback on a thread, but does not set it to inactive
    pub fn pause(&mut self, thread: u64) -> Result<(), ()> {
        unimplemented!()
    }
    /// Resumes audio playback on a thread if it has been paused
    pub fn resume(&mut self, thread: u64) -> Result<(), ()> {
        unimplemented!()
    }
    /// Set a volume for a specific thread. Volume must be between 0 and 100 (inclusive)
    pub fn set_volume(&mut self, thread: u64, volume: Volume) -> Result<(), ()> {
        unimplemented!()
    }
    /// Forcibly stops thread and marks it inactive
    pub fn stop_thread(&mut self, thread: u64) -> Result<(), ()> {
        unimplemented!()
    }
    /// Cleans up any unused threads. Returns a new mapping of threads that were still active to their new identifiers.
    pub fn cleanup(&mut self) -> HashMap<u64, u64> {
        unimplemented!()
    }
    /// Not finished
    pub fn set_audio_device(&mut self) {
        unimplemented!()
    }
}