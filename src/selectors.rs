use coreaudio_sys::{
    AudioObjectPropertyAddress,
    kAudioHardwarePropertyDefaultInputDevice,
    kAudioObjectPropertyElementMaster,
    kAudioDevicePropertyVolumeScalar,
    kAudioObjectPropertyScopeGlobal,
    // kAudioDevicePropertyMute,
    kAudioObjectPropertyScopeInput,
    kAudioHardwarePropertyDevices, };

pub const INPUT_PROPERTY_ADDRESS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioHardwarePropertyDevices,
    mScope:    kAudioObjectPropertyScopeInput,
    mElement:  kAudioObjectPropertyElementMaster,
};

pub const INPUT_VOLUME_ADDRESS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioDevicePropertyVolumeScalar,
    mScope:    kAudioObjectPropertyScopeInput,
    mElement:  kAudioObjectPropertyElementMaster,
};

pub const ALL_INPUTS: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioHardwarePropertyDevices,
    mScope:    kAudioObjectPropertyScopeInput,
    mElement:  kAudioObjectPropertyElementMaster,
};

pub const HARDWARE_CHANGE: AudioObjectPropertyAddress = AudioObjectPropertyAddress {
    mSelector: kAudioHardwarePropertyDefaultInputDevice,
    mScope:    kAudioObjectPropertyScopeGlobal,
    mElement:  kAudioObjectPropertyElementMaster,
};
