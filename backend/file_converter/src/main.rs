use std::env;

mod imagefiles;
mod audiofiles;
mod videofiles;

const OUTPUT_DIR: &str = "output/";

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: --convert <file_path> | --deconvert <file_path>");
        return;
    }
    // Get the path to the video file
    let path = &args[2];
    // Check if the file exists
    if !std::path::Path::new(&format!("{}{}", OUTPUT_DIR,path)).exists() {
        println!("File does not exist: {}", path);
        return;
    }
    if args[1] == "--convert" {
        // Match the file extension
        let ext = std::path::Path::new(path).extension().unwrap().to_str().unwrap();
        match ext {
            "png" => {
                // Convert the image to the eternal format
                let img = imagefiles::Image::from(path);
                img.to_file(format!("{}{}.eimg", OUTPUT_DIR, path).as_str());
            }
            "jpg" => {
                // Convert the image to the eternal format
                let img = imagefiles::Image::from(path);
                img.to_file(format!("{}{}.eimg", OUTPUT_DIR, path).as_str());
            }
            "tiff" => {
                // Convert the image to the eternal format
                let img = imagefiles::Image::from(path);
                img.to_file(format!("{}{}.eimg", OUTPUT_DIR, path).as_str());
            }
            "gif" => {
                // Convert the image to the eternal format
                let img = imagefiles::Image::from(path);
                img.to_file(format!("{}{}.eimg", OUTPUT_DIR, path).as_str());
            }
            "webp" => {
                // Convert the image to the eternal format
                let img = imagefiles::Image::from(path);
                img.to_file(format!("{}{}.eimg", OUTPUT_DIR, path).as_str());
            }
            "ico" => {
                // Convert the image to the eternal format
                let img = imagefiles::Image::from(path);
                img.to_file(format!("{}{}.eimg", OUTPUT_DIR, path).as_str());
            }
            "bmp" => {
                // Convert the image to the eternal format
                let img = imagefiles::Image::from(path);
                img.to_file(format!("{}{}.eimg", OUTPUT_DIR, path).as_str());
            }
            "wav" => {
                // Convert the audio to the eternal format
                let audio = audiofiles::Audio::from_wav(path);
                audio.to_file(format!("{}{}.eaud", OUTPUT_DIR, path).as_str());
            }
            "mp3" => {
                // Convert the audio to the eternal format
                let audio = audiofiles::Audio::from_mp3(path);
                audio.to_file(format!("{}{}.eaud", OUTPUT_DIR, path).as_str());
            }
            "ogg" => {
                // Convert the audio to the eternal format
                let audio = audiofiles::Audio::from_ogg(path);
                audio.to_file(format!("{}{}.eaud", OUTPUT_DIR, path).as_str());
            }
            "mp4" => {
                // Convert the video to the eternal format
                videofiles::from(path, format!("{}{}.evid", OUTPUT_DIR, path).as_str(), format!("{}{}.eaud", OUTPUT_DIR, path).as_str());
            }
            "avi" => {
                // Convert the video to the eternal format
                videofiles::from(path, format!("{}{}.evid", OUTPUT_DIR, path).as_str(), format!("{}{}.eaud", OUTPUT_DIR, path).as_str());
            }
            "mkv" => {
                // Convert the video to the eternal format
                videofiles::from(path, format!("{}{}.evid", OUTPUT_DIR, path).as_str(), format!("{}{}.eaud", OUTPUT_DIR, path).as_str());
            }
            "mov" => {
                // Convert the video to the eternal format
                videofiles::from(path, format!("{}{}.evid", OUTPUT_DIR, path).as_str(), format!("{}{}.eaud", OUTPUT_DIR, path).as_str());
            }
            _ => {
                // Copy the file to the output directory with .eall extension
                let output_path = format!("{}{}.eall", OUTPUT_DIR, path);
                std::fs::copy(path, &output_path).unwrap();
            }
        }
    }
    else if args[1] == "--deconvert" {
        // Get the file extension
        let ext = std::path::Path::new(path).extension().unwrap().to_str().unwrap();
        match ext {
            "eimg" => {
                // Match the second to last file extension
                let original_ext = std::path::Path::new(path).file_stem().unwrap().to_str().unwrap();
                let raw_name = original_ext.split('.').collect::<Vec<&str>>()[0];
                let original_ext = original_ext.split('.').collect::<Vec<&str>>()[1];
                // Convert the eternal image to the original format
                let img = imagefiles::Image::from_file(path);
                if original_ext == "jpg" {
                    img.to_rgb(format!("{}{}.{}", OUTPUT_DIR, raw_name, original_ext).as_str());
                }
                else {
                    img.to_rgba(format!("{}{}.{}", OUTPUT_DIR, raw_name, original_ext).as_str());
                }
            }
            "eaud" => {
                // Match the second to last file extension
                let original_ext = std::path::Path::new(path).file_stem().unwrap().to_str().unwrap();
                let raw_name = original_ext.split('.').collect::<Vec<&str>>()[0];
                let original_ext = original_ext.split('.').collect::<Vec<&str>>()[1];
                // Convert the eternal audio to the original format
                let audio = audiofiles::Audio::from_file(path);
                match original_ext {
                    "wav" => {
                        audio.to_wav(format!("{}{}.{}", OUTPUT_DIR, raw_name, original_ext).as_str());
                    }
                    "mp3" => {
                        audio.to_mp3(format!("{}{}.{}", OUTPUT_DIR, raw_name, original_ext).as_str());
                    }
                    "ogg" => {
                        audio.to_ogg(format!("{}{}.{}", OUTPUT_DIR, raw_name, original_ext).as_str());
                    }
                    _ => {
                        println!("Unsupported audio format: {}", original_ext);
                        return;
                    }
                }
            }
            "evid" => {
                // Match the second to last file extension
                let original_ext = std::path::Path::new(path).file_stem().unwrap().to_str().unwrap();
                let raw_name = original_ext.split('.').collect::<Vec<&str>>()[0];
                let original_ext = original_ext.split('.').collect::<Vec<&str>>()[1];
                let codec = match original_ext {
                    "mp4" => "libx264",
                    "avi" => "libxvid",
                    "mkv" => "libx264",
                    "mov" => "libx264",
                    _ => {
                        println!("Unsupported video format: {}", original_ext);
                        return;
                    }
                };
                // Convert the eternal video to the original format
                videofiles::to(&format!("{}{}", OUTPUT_DIR, path), format!("{}{}.{}.eaud", OUTPUT_DIR, raw_name, original_ext).as_str(), format!("{}{}.{}", OUTPUT_DIR, raw_name, original_ext).as_str(), codec);
            }
            _ => {
                // Copy the file to the output directory with .eall extension
                let output_path = format!("{}{}.eall", OUTPUT_DIR, path);
                std::fs::copy(path, &output_path).unwrap();
            }
        }
    }
}
