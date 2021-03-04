// #![allow(warnings, unused)]

extern crate cocoa;
extern crate coreaudio_sys;
extern crate objc;

use std::os::raw::c_void;
use std::thread;

use std::sync::mpsc;
use std::sync::mpsc::Sender;

use cocoa::base::{ nil, id, };
use cocoa::foundation::{ NSData, }; // NSAutoreleasePool };

use cocoa::appkit::{ NSApp,
                     NSApplication,
                     NSApplicationActivationPolicyProhibited,
                     NSStatusBar,
                     NSVariableStatusItemLength,
                     NSImage,
                     NSStatusItem,
                     NSButton, };

use coreaudio_sys::{
    AudioObjectGetPropertyDataSize,
    AudioObjectAddPropertyListener,
    AudioObjectID,
    AudioObjectGetPropertyData,
    AudioObjectPropertyAddress,
    OSStatus,
    UInt32,
    kAudioObjectSystemObject,
    // kAudioHardwarePropertyDefaultInputDevice,
    kAudioObjectPropertyElementMaster,
    kAudioDevicePropertyVolumeScalar,
    // kAudioDevicePropertyMute,
    kAudioObjectPropertyScopeInput,
    kAudioHardwarePropertyDevices, };

const INPUT_PROPERTY_ADDRESS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioHardwarePropertyDevices,
    mScope:    kAudioObjectPropertyScopeInput,
    mElement:  kAudioObjectPropertyElementMaster,
};

const INPUT_VOLUME_ADDRESS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioDevicePropertyVolumeScalar,
    mScope:    kAudioObjectPropertyScopeInput,
    mElement:  kAudioObjectPropertyElementMaster,
};

fn load_image<T>(array: &[T]) -> id {
    return unsafe {
        let data = NSData::dataWithBytes_length_(
            nil,
            array.as_ptr() as *const std::os::raw::c_void,
            array.len() as u64,
        );
        NSImage::initWithData_(NSImage::alloc(nil), data)
    };
}

fn init_cocoa() -> (cocoa::base::id, cocoa::base::id) {
    let app = unsafe {
        let app = NSApp();
        app
    };

    let button = unsafe {
        app.setActivationPolicy_(NSApplicationActivationPolicyProhibited);

        let status_item = NSStatusBar::systemStatusBar(nil).statusItemWithLength_(NSVariableStatusItemLength);
        let button: cocoa::base::id = status_item.button();

        button
    };

    return (app, button);
}

fn main() {
    //  let _pool = NSAutoreleasePool::new(nil);
    let (app, button) = init_cocoa();

    let audio_obj_id: AudioObjectID = (get_input_device())[0];

    let (tx, rx) = mpsc::channel();
    let tx_ptr = &tx as *const Sender<bool> as u64;
    let button_ptr = button as u64;

    thread::spawn(move || {
        input_property_listener(audio_obj_id, tx_ptr)
    });

    thread::spawn(move || {
        let btn = button_ptr as cocoa::base::id;

        let muted =   load_image(include_bytes!("../assets/muted.png"));
        let unmuted = load_image(include_bytes!("../assets/unmuted.png"));

        loop {
            if rx.recv().unwrap() {
                unsafe { btn.setImage_(unmuted) };
            } else {
                unsafe { btn.setImage_(muted)} ;
            }
        }
    });

    // set the initial state of the icon
    tx.send(0.0 != get_volume_from_device(audio_obj_id)).unwrap();
    unsafe { app.run(); }
}

fn get_volume_from_device(audio_obj_id: u32) -> f32 {
    let mut volume: f32 = 0.0;
    let _ = audio_object_get_property_data(
        audio_obj_id,
        &INPUT_VOLUME_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut 4,
        &mut volume as *mut f32,
    );
    return volume;
}

fn get_input_device() -> Vec<AudioObjectID> {
    let mut size: usize = 0;
    let _ = audio_object_get_property_data_size(
        kAudioObjectSystemObject,
        &INPUT_PROPERTY_ADDRESS,
        0,
        std::ptr::null_mut::<c_void>(),
        &mut size,
    );

    let mut array: Vec<AudioObjectID> = allocate_array(size);

    let _ = audio_object_get_property_data(
        kAudioObjectSystemObject,
        &INPUT_PROPERTY_ADDRESS,
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

fn input_property_listener(audio_object_id: AudioObjectID, tx_ptr: u64) {

    extern fn listener(id: AudioObjectID,
                       _addresses_count: u32,
                       _addresses: *const AudioObjectPropertyAddress,
                       client_input: *mut c_void ) -> OSStatus {

        let tmp: *const Sender<bool> = client_input as u64 as *const Sender<bool>;
        let sender = unsafe { &*tmp };
        sender.send(0.0 != get_volume_from_device(id)).unwrap();
        return 0;
    }

    audio_object_add_property_listener(
        audio_object_id,
        &INPUT_VOLUME_ADDRESS,
        Some(listener),
        tx_ptr,
    );
}

fn audio_object_add_property_listener(
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
