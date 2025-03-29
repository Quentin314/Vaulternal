// This fill will convert audio files (.mp3 .wav ...) to eternal image format (.eaud) that stores the audio as a list of samples (and a header for sample rate)
// Suported file types : wav, mp3, ogg
extern crate rodio;
use std::{fs, io::Read};

use rodio::Source;
use hound::{WavSpec, WavWriter};

#[derive(Clone, Debug)]
pub struct Audio {
    pub s: Vec<i16>, // samples
    pub sr: u32,
}
impl Audio {
    fn to_bytes(&self) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::with_capacity(4 + (self.s.len() * 2));
        // Add the header sample rate
        bytes.push(((self.sr >> 24) & 0xFF) as u8);
        bytes.push(((self.sr >> 16) & 0xFF) as u8);
        bytes.push(((self.sr >> 8) & 0xFF) as u8);
        bytes.push((self.sr & 0xFF) as u8);
        // Unpack the samples and add it to the array
        for sample in &self.s {
            bytes.push(((sample >> 8) & 0xFF) as u8);
            bytes.push((sample & 0xFF) as u8);
        }
        return bytes;
    }
    pub fn to_file(&self, path: &str) {
        std::fs::write(path, self.to_bytes()).unwrap();
    }
    pub fn from_file(path: &str) -> Audio {
        let mut file = fs::File::open(path).unwrap();
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes).unwrap();
        // Read the header
        let sr = ((bytes[0] as u32) << 24) | ((bytes[1] as u32) << 16) | ((bytes[2] as u32) << 8) | (bytes[3] as u32);
        // Read the samples
        let mut samples: Vec<i16> = Vec::with_capacity((bytes.len() - 4) / 2);
        for i in 0..((bytes.len() - 4) / 2) {
            let sample = ((bytes[(i * 2 + 4) as usize] as i16) << 8) | (bytes[(i * 2 + 5) as usize] as i16);
            samples.push(sample);
        }
        // Return the audio
        Audio { s: samples, sr }
    }

    pub fn from_wav(path: &str) -> Audio {
        // Wav : i16
        let file = fs::File::open(path).unwrap();
        let source = rodio::Decoder::new_wav(file).unwrap();
        let sr = source.sample_rate() as u32; // Extract sample rate before consuming source
        let mut samples: Vec<i16> = Vec::new();
        for sample in source.convert_samples::<i16>() {
            samples.push(sample);
        }
        Audio { s: samples, sr }
    }

    pub fn to_wav(&self, path: &str) {
        // Wav : i16
        let spec = WavSpec {
            channels: 1,
            sample_rate: self.sr,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = WavWriter::create(path, spec).unwrap();
        for sample in &self.s {
            writer.write_sample(*sample).unwrap();
        }
        writer.finalize().unwrap();
    }


    pub fn from_mp3(path: &str) -> Audio {
        // Mp3 : i16
        let file = fs::File::open(path).unwrap();
        let source = rodio::Decoder::new_mp3(file).unwrap();
        let sr = source.sample_rate() as u32; // Extract sample rate before consuming source
        let mut samples: Vec<i16> = Vec::new();
        for sample in source.convert_samples::<i16>() {
            samples.push(sample);
        }
        Audio { s: samples, sr }
    }

    pub fn to_mp3(&self, path: &str) {
        // Mp3 : i16
        let spec = WavSpec {
            channels: 1,
            sample_rate: self.sr,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = WavWriter::create(path, spec).unwrap();
        for sample in &self.s {
            writer.write_sample(*sample).unwrap();
        }
        writer.finalize().unwrap();
    }


    pub fn from_ogg(path: &str) -> Audio {
        // Ogg : i16
        let file = fs::File::open(path).unwrap();
        let source = rodio::Decoder::new(file).unwrap();
        let sr = source.sample_rate() as u32; // Extract sample rate before consuming source
        let mut samples: Vec<i16> = Vec::new();
        for sample in source.convert_samples::<i16>() {
            samples.push(sample);
        }
        Audio { s: samples, sr }
    }

    pub fn to_ogg(&self, path: &str) {
        // Ogg : i16
        let spec = WavSpec {
            channels: 1,
            sample_rate: self.sr,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = WavWriter::create(path, spec).unwrap();
        for sample in &self.s {
            writer.write_sample(*sample).unwrap();
        }
        writer.finalize().unwrap();
    }
}