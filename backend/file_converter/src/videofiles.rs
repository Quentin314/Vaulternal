// This file will convert video files (.mp4 .mov ...) to eternal image format (.evid) that stores the video as a list of images and a sound file (and a header for dimensions and frame count)
// Suported file types : mp4, mov, avi, mkv
extern crate image;
use crate::audiofiles;
use std::{fs, io::Read};

#[derive(Clone, Debug)]
pub struct Video {
    pub p: Vec<Vec<(u8, u8, u8, u8)>>, // rgba
    pub w: u32,
    pub h: u32,
    pub fps: u32,
    pub frame_count: u32,
}
impl Video {
    pub fn to_file(&self, path: &str) {
        let mut bytes: Vec<u8> = Vec::with_capacity(12 + (self.h * self.w * self.frame_count) as usize);
        // Add the header width then height both transformed into 4 u8
        bytes.push(((self.w >> 24) & 0xFF) as u8);
        bytes.push(((self.w >> 16) & 0xFF) as u8);
        bytes.push(((self.w >> 8) & 0xFF) as u8);
        bytes.push((self.w & 0xFF) as u8);
        bytes.push(((self.h >> 24) & 0xFF) as u8);
        bytes.push(((self.h >> 16) & 0xFF) as u8);
        bytes.push(((self.h >> 8) & 0xFF) as u8);
        bytes.push((self.h & 0xFF) as u8);
        // Add the frame count
        bytes.push(((self.frame_count >> 24) & 0xFF) as u8);
        bytes.push(((self.frame_count >> 16) & 0xFF) as u8);
        bytes.push(((self.frame_count >> 8) & 0xFF) as u8);
        bytes.push((self.frame_count & 0xFF) as u8);
        // Add the fps
        bytes.push(((self.fps >> 24) & 0xFF) as u8);
        bytes.push(((self.fps >> 16) & 0xFF) as u8);
        bytes.push(((self.fps >> 8) & 0xFF) as u8);
        bytes.push((self.fps & 0xFF) as u8);
        // Unpack the rgba tuples and add it to the array
        for frame in &self.p {
            for (red, green, blue, alpha) in frame {
                bytes.push(*red);
                bytes.push(*green);
                bytes.push(*blue);
                bytes.push(*alpha);
            }
        }
        std::fs::write(path, bytes).unwrap();
    }

    pub fn from_file(path: &str) -> Video {
        let mut file = fs::File::open(path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        // Read the header
        let w = ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32);
        let h = ((bytes[4] as u32) << 24) | ((bytes[5] as u32) << 16) | ((bytes[6] as u32) << 8) | (bytes[7] as u32);
        let frame_count = ((bytes[8] as u32) << 24) | ((bytes[9] as u32) << 16) | ((bytes[10] as u32) << 8) | (bytes[11] as u32);
        let fps = ((bytes[12] as u32) << 24) | ((bytes[13] as u32) << 16) | ((bytes[14] as u32) << 8) | (bytes[15] as u32);
        // Read the pixels
        let mut pixels: Vec<Vec<(u8, u8, u8, u8)>> = Vec::with_capacity(frame_count as usize);
        for i in 0..(frame_count) {
            let mut frame: Vec<(u8, u8, u8, u8)> = Vec::with_capacity((w * h) as usize);
            for j in 0..(w * h) {
                let red = bytes[(i * w * h * 4 + j * 4 + 16) as usize];
                let green = bytes[(i * w * h * 4 + j * 4 + 17) as usize];
                let blue = bytes[(i * w * h * 4 + j * 4 + 18) as usize];
                let alpha = bytes[(i * w * h * 4 + j * 4 + 19) as usize];
                frame.push((red, green, blue, alpha));
            }
            pixels.push(frame);
        }
        // Return the video
        Video { p: pixels, w: w, h: h, fps: fps, frame_count }
    }
    

    pub fn from(path: &str) -> Video {
        // Create a "frames" directory if it doesn't exist
        if !std::path::Path::new("frames").exists() {
            std::fs::create_dir("frames").unwrap();
        }

        let ffmpeg_exe = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "./ffmpeg" };

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
                fps = fps_match.as_str().parse::<u32>().expect("Failed to parse FPS");
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
        audio.to_file(format!("{}.eaud",path).as_str());

        // Read the frames
        let mut frames: Vec<Vec<(u8, u8, u8, u8)>> = Vec::with_capacity(frame_count as usize);
        for i in 1..frame_count+1 {
            let frame_path = format!("frames/frame_{:04}.png", i);
            let img = image::open(frame_path).unwrap().to_rgba8();
            let bytes: Vec<u8> = img.bytes().filter_map(Result::ok).collect();
            let mut pixels: Vec<(u8,u8,u8,u8)> = Vec::with_capacity(bytes.len() / 4);
            for i in 0..(bytes.len()/4) {
                pixels.push((bytes[i*4], bytes[i*4+1], bytes[i*4+2], bytes[i*4+3]));
            }
            frames.push(pixels);
        }
        // Clean up the frames directory
        fs::remove_dir_all("frames").unwrap();

        // Return the video
        Video { p: frames, w, h, fps, frame_count }
    }

    pub fn to_mp4(&self, path: &str, audio_path: &str) {
        // Create a "frames" directory if it doesn't exist
        if !std::path::Path::new("frames").exists() {
            std::fs::create_dir("frames").unwrap();
        }

        // Write the frames to the "frames" directory
        for (i, frame) in self.p.iter().enumerate() {
            let img = image::ImageBuffer::from_fn(self.w, self.h, |x, y| {
                let pixel = frame[(y * self.w + x) as usize];
                image::Rgba([pixel.0, pixel.1, pixel.2, pixel.3])
            });
            img.save(format!("frames/frame_{:04}.png", i + 1)).unwrap();
        }

        // Use audiofiles to generate wav from audio
        let audio = audiofiles::Audio::from_file(audio_path);
        audio.to_wav(format!("{}.wav",audio_path).as_str());

        // Use ffmpeg to create a video from the frames
        let ffmpeg_exe = if cfg!(target_os = "windows") { "ffmpeg.exe" } else { "./ffmpeg" };
        let output = std::process::Command::new(ffmpeg_exe)
            .args(["-framerate", &format!("{}", self.fps), "-i", "frames/frame_%04d.png", "-i", &format!("{}.wav", audio_path), "-c:v", "libx264", "-pix_fmt", "yuv420p", path])
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
}