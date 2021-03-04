#![allow(warnings, unused)]

use crate::selectors;
use std::os::raw::c_void;

use coreaudio_sys::{
    AudioObjectGetPropertyDataSize,
    AudioObjectAddPropertyListener,
    AudioObjectID,
    AudioObjectGetPropertyData,
    AudioObjectSetPropertyData,
    AudioObjectPropertyAddress,
    OSStatus,
    UInt32,
    kAudioObjectSystemObject,
};

fn get_volume_from_device(audio_obj_id: u32) -> f32 {
    let mut volume: f32 = 0.0;
    let _ = audio_object_get_property_data(
        audio_obj_id,
        &selectors::INPUT_VOLUME_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut 4,
        &mut volume as *mut f32,
    );
    return volume;
}

pub fn get_volume_from_all_devices() -> f32 {
    let mut size: usize = 0;
    audio_object_get_property_data_size(
        kAudioObjectSystemObject,
        &selectors::ALL_INPUTS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut size,
    );

    let mut array: Vec<AudioObjectID> = allocate_array(size);

    let _ = audio_object_get_property_data(
        kAudioObjectSystemObject,
        &selectors::INPUT_PROPERTY_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut size,
        array.as_mut_ptr(),
    );

    let mut volume_total = 0.0;
    println!("--");
    for device_id in array {
        let volume = get_volume_from_device(device_id);
        println!("  Device: {}, vol: {}", device_id, volume);
        volume_total += volume;
    }

    return volume_total;
}

pub fn toggle_all(mute: bool) {
    let mut size: usize = 0;
    audio_object_get_property_data_size(
        kAudioObjectSystemObject,
        &selectors::ALL_INPUTS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut size,
    );

    let mut array: Vec<AudioObjectID> = allocate_array(size);

    let _ = audio_object_get_property_data(
        kAudioObjectSystemObject,
        &selectors::INPUT_PROPERTY_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut size,
        array.as_mut_ptr(),
    );

    let vol = if mute { 0.5 } else { 0.0 };
    println!("--");
    for device_id in array {
        println!("  Setting {} to {}", device_id, vol);
        set_volume_on_device(device_id, vol);
    }
}

fn set_volume_on_device(audio_obj_id: u32, vol: f32) {
    let mut volume: f32 = vol;
    println!("a: {} {}", audio_obj_id, volume);
    let err = audio_object_set_property_data(
        audio_obj_id,
        &selectors::INPUT_VOLUME_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut 4,
        &mut volume as *mut f32,
    );
    println!("b: {} {} {:?}", audio_obj_id, volume, err);
}

fn audio_object_get_property_data_size<T>(
    in_object_id: AudioObjectID,
    in_address: &AudioObjectPropertyAddress,
    in_qualifier_data_size: usize,
    in_qualifier_data: *mut T,
    out_data_size: *mut usize,
) -> OSStatus {
    return unsafe {
        AudioObjectGetPropertyDataSize(
            in_object_id,
            in_address,
            in_qualifier_data_size as UInt32,
            in_qualifier_data as *mut c_void,
            out_data_size as *mut UInt32,
        )
    };
}

pub fn get_input_device() -> Vec<AudioObjectID> {
    let mut size: usize = 0;
    let _ = audio_object_get_property_data_size(
        kAudioObjectSystemObject,
        &selectors::INPUT_PROPERTY_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut size,
    );

    let mut array: Vec<AudioObjectID> = allocate_array(size);

    let _ = audio_object_get_property_data(
        kAudioObjectSystemObject,
        &selectors::INPUT_PROPERTY_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut size,
        array.as_mut_ptr(),
    );

    return array;
}

fn allocate_array<T>(size: usize) -> Vec<T> {
    let element_size = std::mem::size_of::<T>();
    let elements = size / element_size;
    let mut buffer = Vec::<T>::with_capacity(elements);
    unsafe { buffer.set_len(elements); }
    return buffer;
}

fn audio_object_set_property_data<Q, D> (
    in_object_id: AudioObjectID,
    in_address: &AudioObjectPropertyAddress,
    in_qualifier_data_size: usize,
    in_qualifier_data: *mut Q,
    in_data_size: *mut usize,
    in_data: *mut D,
) -> OSStatus {
    return unsafe {
        AudioObjectSetPropertyData(
            in_object_id,
            in_address,
            in_qualifier_data_size as UInt32,
            in_qualifier_data as *mut c_void,
            in_data_size as UInt32,
            in_data as *mut c_void,
        )
    };
}

fn audio_object_get_property_data<Q, D>(
    in_object_id: AudioObjectID,
    in_address: &AudioObjectPropertyAddress,
    in_qualifier_data_size: usize,
    in_qualifier_data: *mut Q,
    io_data_size: *mut usize,
    out_data: *mut D,
) -> OSStatus {
    return unsafe {
        AudioObjectGetPropertyData(
            in_object_id,
            in_address,
            in_qualifier_data_size as UInt32,
            in_qualifier_data as *mut c_void,
            io_data_size as *mut UInt32,
            out_data as *mut c_void,
        )
    };
}

pub fn audio_object_add_property_listener(
    in_object_id: AudioObjectID,
    in_address: &AudioObjectPropertyAddress,
    in_listener: Option<unsafe extern "C" fn(u32, u32, *const AudioObjectPropertyAddress, *mut c_void) -> i32>,
    in_listener_input_ptr: u64,
) -> OSStatus {
    return unsafe {
        AudioObjectAddPropertyListener(
            in_object_id,
            in_address,
            in_listener,
            in_listener_input_ptr as *mut c_void,
        )
    };
}
