#[allow(dead_code)]
mod selectors;
mod audio;

fn main() {
    let vol = audio::get_volume_from_all_devices();
    // println!("Before: {}", vol);
    audio::toggle_all(vol == 0.0);
    // println!("After:  {}", audio::get_volume_from_all_devices());
}
