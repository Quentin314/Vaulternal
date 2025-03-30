// Packs all eternal files into a single file with a header indicating for each file : the begining of the data in the packed file and the file name (with extension at the end)

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct FileInfo {
    lenght : u128,
    name : String,
    content : Vec<u8>,
}

#[derive(Debug)]
struct HeaderFile {
    adress : u128,
    name : Vec<u8>,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        println!("Usage: --pack/--unpack <file1> <file2> ...");
        return;
    }
    if args[1] == "--pack" {
        // Get the list of files from the command line arguments
        pack_files(args[2..].to_vec());
    } else if args[1] == "--unpack" {
        // Unpack the files
        unpack_files();
    } else {
        println!("Usage: --pack/--unpack <file1> <file2> ...");
    }
}

fn pack_files(file_names: Vec<String>) {
    // Get the list of files from the command line arguments
    let files = to_file_info(file_names);
    // Get the header length
    let mut header_length = 1;
    for file in &files {
        header_length += file.name.len() as u128 + 17;
    };


    // Write the header with the file names and their adresses in the packed file
    let mut adress: u128 = header_length;
    let mut packed_bytes: Vec<u8> = Vec::with_capacity(header_length as usize);
    for file in &files {
        let mut adress_bytes: Vec<u8> = Vec::new();
        for i in (0..16).rev() {
            adress_bytes.push(((adress >> (i * 8)) & 0xFF) as u8);
        }
        packed_bytes.append(&mut adress_bytes);
        packed_bytes.append(&mut file.name.as_bytes().to_vec());
        packed_bytes.push(b'|');
        adress += file.lenght;
    }
    packed_bytes.push("|".as_bytes()[0]);

    // Write the files content
    for file in &files {
        packed_bytes.append(&mut file.content.clone());
    }
    // Write the packed file
    let mut packed_file = std::fs::File::create("packed.e").unwrap();
    packed_file.write_all(packed_bytes.as_slice()).unwrap();
}

fn to_file_info(file_names: Vec<String>) -> Vec<FileInfo> {
    let mut files: Vec<FileInfo> = Vec::with_capacity(file_names.len());
    for file_name in file_names {
        // Open the file
        let file:Vec<u8> = std::fs::read(file_name.clone()).unwrap();
        files.push(FileInfo {
            lenght: file.len() as u128,
            name: file_name,
            content: file,
        });
    }
    return files;
}

fn unpack_files() {
    // Read the packed file
    let packed_file = std::fs::read("packed.e").unwrap();

    // Get the header
    let mut header: Vec<u8> = Vec::new();
    for i in 0..packed_file.len() {
        if packed_file[i] == b'|' && packed_file[i + 1] == b'|' {
            header.push(packed_file[i]);
            header.push(packed_file[i + 1]);
            break;
        }
        header.push(packed_file[i]);
    }
    
    // Turn the header into a file list
    let mut files: Vec<HeaderFile> = Vec::new();
    let mut i = 0;
    while i + 1 < header.len() && !(header[i] == b'|' && header[i+1] == b'|') {
        let mut adress: u128 = 0;
        for j in 0..16{
            adress += (header[i+j] as u128) << ((15-j) * 8);
        }
        i += 16;
        let mut name_vec:Vec<u8> = Vec::new();
        while header[i] != b'|' {
            name_vec.push(header[i]);
            i += 1;
        }
        files.push(HeaderFile {
            adress: adress,
            name: name_vec,
        });
        i += 1;
    }
    
    // Unpack the files
    for i in 0..files.len() {
        let file_content: Vec<u8> = if i + 1 < files.len() {
            packed_file[(files[i].adress as usize)..(files[i + 1].adress as usize)].to_vec()
        } else {
            packed_file[(files[i].adress as usize)..].to_vec()
        };
        let mut file_name= String::new();
        let mut file_extension= String::new();
        for j in 0..files[i].name.len() {
            if files[i].name[j] == b'.' {
                file_name = String::from_utf8(files[i].name[0..j].to_vec()).unwrap();
                file_extension = String::from_utf8(files[i].name[j+1..].to_vec()).unwrap();
                break;
            }
        }
        let mut file = File::create(format!("{}unpacked.{}", file_name, file_extension)).unwrap();
        file.write_all(file_content.as_slice()).unwrap();
    }

}