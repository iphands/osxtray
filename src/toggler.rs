#[allow(dead_code)]
mod selectors;
mod audio;

fn main() {
    // let vol = audio::get_volume_from_all_devices();
    let mic_live = audio::get_mute_from_all_devices();
    // println!("Before: {}", vol);
    audio::set_mic_live(!mic_live);
    // println!("After:  {}", audio::get_volume_from_all_devices());
}
