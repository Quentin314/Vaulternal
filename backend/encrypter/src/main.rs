// Use RC4 to encrypt and decrypt a file then add the documentation as text at the beginning of the file

use std::env;
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let key = args[2].as_bytes();
    if args[1] == "--encrypt" {
        let data = std::fs::read("packed.e").unwrap();
        let encrypted_data = rc4(key, &data);
        let mut output_file = File::create("capsule.e").unwrap();
        let documentation = std::fs::read("documentation.txt").unwrap();
        output_file.write_all(&documentation).unwrap();
        output_file.write_all(&[0x00]).unwrap(); // Add a delimiter
        output_file.write_all(&encrypted_data).unwrap();
        println!("File encrypted and saved as capsule.e");
    }
    else if args[1] == "--decrypt" {
        let mut data = std::fs::read("capsule.e").unwrap();
        // Remove the documentation part, delimited by 0x00
        for i in 0..data.len() {
            if data[i] == 0x00 {
                data = data[(i + 1)..].to_vec();
                break;
            }
        }
        let decrypted_data = rc4(key, &data);
        let mut output_file = File::create("packed.e").unwrap();
        output_file.write_all(&decrypted_data).unwrap();
        println!("File decrypted and saved as unpacked.e");
    }
    else {
        println!("Usage: --encrypt/--decrypt <key>");
    }
}

fn rc4(key: &[u8], data: &[u8]) -> Vec<u8> {
    let mut s = ksa(key);
    let result = prga(&mut s, data);
    return result;
}

fn ksa(key: &[u8]) -> Vec<u8> {
    let mut s: Vec<u8> = (0..=255).collect();
    let mut j: usize = 0;
    for i in 0..256 {
        j = (j + s[i] as usize + key[i % key.len()] as usize) % 256;
        s.swap(i, j);
    }
    return s;
}

fn prga(s: &mut Vec<u8>, data: &[u8]) -> Vec<u8> {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let mut result: Vec<u8> = Vec::new();
    for byte in data {
        i = (i + 1) % 256;
        j = (j + s[i] as usize) % 256;
        s.swap(i, j);
        let k: u8 = s[(s[i] as usize + s[j] as usize) % 256];
        result.push(byte ^ k);
    }
    return result;
}