mod imagefiles;
mod audiofiles;
mod videofiles;

fn main() {
    imagefiles::Image::from("creeper.jpg").to_file("creeper.jpg.eimg");

    let img: imagefiles::Image = imagefiles::Image::from_file("creeper.jpg.eimg");
    img.to_rgb("creeper2.png");


    let audio = audiofiles::Audio::from_mp3("test.mp3");
    audio.to_file("test.mp3.eaud");
    let audio2 = audiofiles::Audio::from_file("test.mp3.eaud");
    audio2.to_mp3("testtesttest.mp3");

    /*let video = videofiles::Video::from("bad_apple.mp4");
    video.to_file("bad_apple.mp4.evid");*/
    let video2 = videofiles::Video::from_file("bad_apple.mp4.evid");
    video2.to_mp4("bad_apple2.mp4", "bad_apple.mp4.eaud");
}


