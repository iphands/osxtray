// #![allow(warnings, unused)]

extern crate cocoa;
extern crate coreaudio_sys;
extern crate objc;

mod selectors;
mod audio;

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
    AudioObjectID,
    AudioObjectPropertyAddress,
    OSStatus,
    kAudioObjectSystemObject,
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
    let (app, button) = init_cocoa();
    let (tx, rx) = mpsc::channel();
    // let (tx_mute, rx_mute) = mpsc::channel();

    let tx_ptr = &tx as *const Sender<bool> as u64;
    let button_ptr = button as u64;

    // mute all by default
    audio::set_mic_live(false);

    thread::spawn(move || {
        input_property_listener(tx_ptr)
    });

    thread::spawn(move || {
        hardware_change_listener(tx_ptr);
    });

    // thread::spawn(move || {
    //     hack_keep_muted(rx_mute);
    // });

    thread::spawn(move || {
        let btn = button_ptr as cocoa::base::id;

        let muted =   load_image(include_bytes!("../assets/muted.png"));
        let unmuted = load_image(include_bytes!("../assets/unmuted.png"));

        loop {
            if rx.recv().unwrap() {
                unsafe { btn.setImage_(unmuted) };
                // let _ = tx_mute.send(true);
            } else {
                unsafe { btn.setImage_(muted)} ;
                // let _ = tx_mute.send(false);
            }
        }
    });

    // set the initial state of the icon
    tx.send(audio::get_mute_from_all_devices()).unwrap();
    unsafe { app.run(); }
}

// fn hack_keep_muted(rx: mpsc::Receiver<bool>) {
//     let timeout = std::time::Duration::from_millis(250);
//     let mut state = false;
//     loop {
//         match rx.recv_timeout(timeout) {
//             Ok(mic_live) => {
//                 // println!("Seent a change! {} => {}", state, mic_live);
//                 state = mic_live
//             },
//             _ => {
//                 if state == false {
//                     // println!("FDSAFSDFSAD");
//                     audio::toggle_all(false);
//                 }
//             }
//         };
//     }
// }

fn hardware_change_listener(tx_ptr: u64) {
    extern fn listener(_id: AudioObjectID,
                       _addresses_count: u32,
                       _addresses: *const AudioObjectPropertyAddress,
                       client_input: *mut c_void ) -> OSStatus {

        // TODO remove old listeners!

        let tx_ptr = client_input as u64;
        println!("Got pointer to tx: {:?}", tx_ptr);

        // setup new listener
        input_property_listener(tx_ptr);
        println!("Setup new listener");

        return 0;
    }

    audio::audio_object_add_property_listener(
        kAudioObjectSystemObject,
        &selectors::HARDWARE_CHANGE,
        Some(listener),
        tx_ptr,
    );
}

fn input_property_listener(tx_ptr: u64) {

    extern fn listener(_id: AudioObjectID,
                       _addresses_count: u32,
                       _addresses: *const AudioObjectPropertyAddress,
                       client_input: *mut c_void ) -> OSStatus {

        let tmp: *const Sender<bool> = client_input as u64 as *const Sender<bool>;
        let sender = unsafe { &*tmp };

        let mic_live = audio::get_mute_from_all_devices();
        sender.send(mic_live).unwrap();
        println!("- mute event: mic_live == {} (all inputs)", mic_live);
        return 0;
    }

    // get new default device
    let audio_obj_id: AudioObjectID = (audio::get_input_device())[0];
    println!("Got new default input device: {:?}", audio_obj_id);

    audio::audio_object_add_property_listener(
        audio_obj_id,
        &selectors::INPUT_MUTE_ADDRESS,
        Some(listener),
        tx_ptr,
    );
}
