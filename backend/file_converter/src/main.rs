mod imagefiles;
mod audiofiles;

use std::io::Write;

fn main() {
    imagefiles::Image::from_webp("creeper_aw_man.webp").to_file("creeper_aw_man.webp.eimg");

    let img: imagefiles::Image = imagefiles::Image::from_file("creeper_aw_man.webp.eimg");
    img.to_webp("creeper.webp");


    let audio = audiofiles::Audio::from_wav("test2.mp3");
    audio.to_file("test2.wav.eaud");
    let audio2 = audiofiles::Audio::from_file("test2.wav.eaud");
    audio2.to_mp3("test3.mp3");
}


