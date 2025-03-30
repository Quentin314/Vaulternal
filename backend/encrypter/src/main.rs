// Use RC4 to encrypt and decrypt a file then add the documentation as text at the beginning of the file

use std::env;
use std::fs::File;
use std::io::{prelude::*, BufReader};
fn main() {
    let args = env::args().collect::<Vec<String>>();
    let key = args[2].as_bytes();
    if args[1] == "--encrypt" {
        // Create the output file, add the documentation, and encrypt the data chunk by chunk
        let mut output_file = File::create("capsule.eternal").unwrap();
        let documentation = std::fs::read("documentation.md").unwrap();
        output_file.write_all(&documentation).unwrap();
        output_file.write_all(&[0x00]).unwrap(); // Add a delimiter
        // Read the packed file and encrypt it
        let packed_file = File::open("packed.e").unwrap();
        // Read the packed file in chunks of 4096 bytes
        let mut read_lenght:i128 = packed_file.metadata().unwrap().len() as i128;
        let mut reader = BufReader::new(packed_file);
        loop {
            println!("Read length: {}", read_lenght);
            let mut buffer = vec![0; 4096];
            reader.read(&mut buffer).unwrap();
            if read_lenght < 4096 {
                buffer = buffer[0..read_lenght as usize].to_vec().try_into().unwrap();
                let encrypted_data = rc4(key, &buffer);
                output_file.write_all(&encrypted_data).unwrap();
                break;
            }
            let encrypted_data = rc4(key, &buffer);
            output_file.write_all(&encrypted_data).unwrap();
            read_lenght -= buffer.len() as i128;
        }
        println!("File encrypted and saved as capsule.eternal");
    }
    else if args[1] == "--decrypt" {
        let encrypted_file = File::open("capsule.eternal").unwrap();
        let mut decrypted_lenght:i128 = encrypted_file.metadata().unwrap().len() as i128;
        let mut reader = BufReader::new(encrypted_file);
        let mut output_file = File::create("packed.e").unwrap();
        let mut buffer = Vec::new();
        loop { // Skip the documentation
            let mut byte = [0; 1];
            if reader.read_exact(&mut byte).is_err() {
                break;
            }
            if byte[0] == 0x00 {
                break;
            }
            buffer.push(byte[0]);
            decrypted_lenght -= 1;
        }
        // Read the encrypted file in chunks of 4096 bytes
        loop {
            let mut buffer = vec![0; 4096];
            reader.read(&mut buffer).unwrap();
            if decrypted_lenght < 4096 {
                buffer = buffer[0..decrypted_lenght as usize-1].to_vec();
                let decrypted_data = rc4(key, &buffer);
                output_file.write_all(&decrypted_data).unwrap();
                break;
            }
            let decrypted_data = rc4(key, &buffer);
            output_file.write_all(&decrypted_data).unwrap();
            decrypted_lenght -= buffer.len() as i128;
        }
        
        println!("File decrypted and saved as packed.e");
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