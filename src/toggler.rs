#[allow(dead_code)]
mod selectors;
mod audio;

fn main() {
    // let vol = audio::get_volume_from_all_devices();
    let mute = audio::get_mute_from_all_devices();
    // println!("Before: {}", vol);
    audio::toggle_all(!mute);
    // println!("After:  {}", audio::get_volume_from_all_devices());
}
