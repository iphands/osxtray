use coreaudio_sys::{
    AudioObjectPropertyAddress,
    kAudioHardwarePropertyDefaultInputDevice,
    kAudioHardwarePropertyDevices,

    kAudioObjectPropertyElementMaster,
    // kAudioObjectPropertyElementWildcard,
    kAudioObjectPropertyScopeGlobal,
    kAudioObjectPropertyScopeInput,


    kAudioDevicePropertyMute,
    // kAudioDevicePropertyScopeInput,
    // kAudioDevicePropertyScopeOutput,
    // kAudioObjectPropertyScopeOutput,
    // kAudioObjectPropertyScopeWildcard,
    kAudioDevicePropertyVolumeScalar,
};

const INPUT_SCOPE: u32 = kAudioObjectPropertyScopeInput;
const ELEMENT: u32 = kAudioObjectPropertyElementMaster;

pub const INPUT_PROPERTY_ADDRESS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioHardwarePropertyDevices,
    mScope:    INPUT_SCOPE,
    mElement:  ELEMENT,
};

pub const INPUT_VOLUME_ADDRESS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioDevicePropertyVolumeScalar,
    mScope:    INPUT_SCOPE,
    mElement:  ELEMENT,
};

pub const INPUT_MUTE_ADDRESS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioDevicePropertyMute,
    mScope:    INPUT_SCOPE,
    mElement:  ELEMENT,
};

pub const ALL_INPUTS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioHardwarePropertyDevices,
    mScope:    INPUT_SCOPE,
    mElement:  ELEMENT,
};

pub const HARDWARE_CHANGE: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioHardwarePropertyDefaultInputDevice,
    mScope:    kAudioObjectPropertyScopeGlobal,
    mElement:  ELEMENT,
};
