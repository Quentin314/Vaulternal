// This file will convert video files (.mp4 .mov ...) to eternal image format (.evid) that stores the video as a list of images and a sound file (and a header for dimensions and frame count)
// Suported file types : mp4, mov, avi, mkv
extern crate image;
use crate::audiofiles;
use std::{fs::{self, OpenOptions}, io::{Read, Write}};

fn append_to_file(save_path: &str, bytes: &[u8]) {
    let mut file = OpenOptions::new()
        .create(true)  // Create file if it doesn't exist
        .append(true)  // Open in append mode
        .open(save_path)
        .expect("Failed to open file");

    file.write_all(bytes).expect("Failed to write to file");
}

pub fn from(path: &str, save_path: &str, audio_path: &str) {
    // Create a "frames" directory if it doesn't exist
    if !std::path::Path::new("frames").exists() {
        std::fs::create_dir("frames").unwrap();
    }

    let ffmpeg_exe = if cfg!(target_os = "windows") { ".\\ffmpeg.exe" } else { "./ffmpeg" };

    // Get fps
    let output = std::process::Command::new("ffmpeg")
        .args(["-i", path])
        .output()
        .expect("Failed to execute ffmpeg");

    let stdout = String::from_utf8_lossy(&output.stderr); // FFmpeg writes metadata to stderr

    // Look for FPS in the metadata output
    let fps_regex = regex::Regex::new(r"(\d+(?:\.\d+)?) fps").unwrap();
    let mut fps = 30; // Default FPS
    if let Some(captures) = fps_regex.captures(&stdout) {
        if let Some(fps_match) = captures.get(1) {
            // Try parsing and use a fallback value if it fails
            if let Ok(parsed_fps) = fps_match.as_str().parse::<u32>() {
                fps = parsed_fps;
            } else {
                eprintln!("Failed to parse FPS, using default: {}", fps);
            }
        }
    }
    println!("FPS: {}", fps);

    // Use ffmpeg to extract frames from the video
    let output = std::process::Command::new(ffmpeg_exe)
        .args(["-i", path, "-vf", &format!("fps={}", fps), "frames/frame_%04d.png"])
        .output()
        .expect("Failed to execute ffmpeg");
    if !output.status.success() {
        panic!("ffmpeg failed with error: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Get the frame count
    let frame_count = std::fs::read_dir("frames").unwrap().count() as u32;
    // Get the first frame to get the width and height
    let first_frame = std::fs::read_dir("frames").unwrap().next().unwrap().unwrap().path();
    let img = image::open(first_frame).unwrap();
    let w = img.width();
    let h = img.height();

    // Get audio
    // Use ffmpeg to extract audio from the video
    let audio_output = std::process::Command::new(ffmpeg_exe)
        .args(&["-i", path, "-vn", "-acodec", "pcm_s16le", "-ar", "44100", "-ac", "1", format!("{}.wav",path).as_str()])
        .output()
        .expect("Failed to execute ffmpeg");
    if !audio_output.status.success() {
        panic!("ffmpeg failed with error: {}", String::from_utf8_lossy(&audio_output.stderr));
    }
    // Convert the audio to eternal audio format
    let audio = audiofiles::Audio::from_wav(format!("{}.wav",path).as_str());
    audio.to_file(format!("{}",audio_path).as_str());

    // Read the frames
    let mut bytes: Vec<u8> = Vec::with_capacity(12 + (h * w * frame_count) as usize);
    // Add the header width then height both transformed into 4 u8
    bytes.push(((w >> 24) & 0xFF) as u8);
    bytes.push(((w >> 16) & 0xFF) as u8);
    bytes.push(((w >> 8) & 0xFF) as u8);
    bytes.push((w & 0xFF) as u8);
    bytes.push(((h >> 24) & 0xFF) as u8);
    bytes.push(((h >> 16) & 0xFF) as u8);
    bytes.push(((h >> 8) & 0xFF) as u8);
    bytes.push((h & 0xFF) as u8);
    // Add the frame count
    bytes.push(((frame_count >> 24) & 0xFF) as u8);
    bytes.push(((frame_count >> 16) & 0xFF) as u8);
    bytes.push(((frame_count >> 8) & 0xFF) as u8);
    bytes.push((frame_count & 0xFF) as u8);
    // Add the fps
    bytes.push(((fps >> 24) & 0xFF) as u8);
    bytes.push(((fps >> 16) & 0xFF) as u8);
    bytes.push(((fps >> 8) & 0xFF) as u8);
    bytes.push((fps & 0xFF) as u8);
    // Unpack the rgba tuples and add it to the array
    for i in 0..frame_count {
        let img = image::open(format!("frames/frame_{:04}.png", i + 1)).unwrap();
        let img = img.to_rgba8();
        for y in 0..h {
            for x in 0..w {
                let pixel = img.get_pixel(x, y);
                bytes.push(pixel[0]);
                bytes.push(pixel[1]);
                bytes.push(pixel[2]);
                bytes.push(pixel[3]);
            }
        }
        append_to_file(save_path, &bytes);
        bytes.clear();
    }
    // Clean up the frames directory
    fs::remove_dir_all("frames").unwrap();
}

pub fn to(path: &str, audio_path: &str, save_path: &str, codec: &str) {
    // Create a "frames" directory if it doesn't exist
    if !std::path::Path::new("frames").exists() {
        std::fs::create_dir("frames").unwrap();
    }

    let mut file = fs::File::open(path).unwrap();
    let mut bytes: [u8; 16] = [0; 16];
    file.read_exact(&mut bytes).unwrap();
    // Read the header
    let w = ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32);
    let h = ((bytes[4] as u32) << 24) | ((bytes[5] as u32) << 16) | ((bytes[6] as u32) << 8) | (bytes[7] as u32);
    let frame_count = ((bytes[8] as u32) << 24) | ((bytes[9] as u32) << 16) | ((bytes[10] as u32) << 8) | (bytes[11] as u32);
    let fps = ((bytes[12] as u32) << 24) | ((bytes[13] as u32) << 16) | ((bytes[14] as u32) << 8) | (bytes[15] as u32);
    // Read the pixels
    for i in 0..(frame_count) {
        let mut frame: Vec<(u8, u8, u8, u8)> = Vec::with_capacity((w * h) as usize);
        // Read some more bytes
        let mut bytes: Vec<u8> = vec![0; (w * h * 4) as usize];
        file.read_exact(&mut bytes).unwrap();
        for j in 0..(w * h) {
            let red = bytes[(j * 4) as usize];
            let green = bytes[(j * 4 + 1) as usize];
            let blue = bytes[(j * 4 + 2) as usize];
            let alpha = bytes[(j * 4 + 3) as usize];
            frame.push((red, green, blue, alpha));
        }
        // Save the frame
        let img = image::ImageBuffer::from_fn(w, h, |x, y| {
            let pixel = frame[(y * w + x) as usize];
            image::Rgba([pixel.0, pixel.1, pixel.2, pixel.3])
        });
        img.save(format!("frames/frame_{:04}.png", i + 1)).unwrap();
    }

    // Use audiofiles to generate wav from audio
    let audio = audiofiles::Audio::from_file(audio_path);
    audio.to_wav(format!("{}.wav",audio_path).as_str());

    // Use ffmpeg to create a video from the frames
    let ffmpeg_exe = if cfg!(target_os = "windows") { ".\\ffmpeg.exe" } else { "./ffmpeg" };
    let output = std::process::Command::new(ffmpeg_exe)
        .args(["-framerate", &format!("{}", fps), "-i", "frames/frame_%04d.png", "-i", &format!("{}.wav", audio_path), "-c:v", codec, "-pix_fmt", "yuv420p", save_path])
        .output()
        .expect("Failed to execute ffmpeg");
    if !output.status.success() {
        panic!("ffmpeg failed with error: {}", String::from_utf8_lossy(&output.stderr));
    }

    // Remove wav file
    fs::remove_file(format!("{}.wav",audio_path)).unwrap();

    // Clean up the frames directory
    fs::remove_dir_all("frames").unwrap();
}