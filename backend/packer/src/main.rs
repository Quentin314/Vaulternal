// Packs all eternal files into a single file with a header indicating for each file : the begining of the data in the packed file and the file name (with extension at the end)

use std::env;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct FileInfo {
    lenght : u128,
    name : String,
}

#[derive(Debug)]
struct HeaderFile {
    adress : u128,
    name : Vec<u8>,
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 2 {
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
    let mut packed_file = std::fs::File::create("packed.e").unwrap();
    for file in &files {
        let mut adress_bytes: Vec<u8> = Vec::new();
        for i in (0..16).rev() {
            adress_bytes.push(((adress >> (i * 8)) & 0xFF) as u8);
        }
        packed_file.write_all(&mut adress_bytes).unwrap();
        packed_file.write_all(&mut file.name.as_bytes().to_vec()).unwrap();
        packed_file.write_all(&[b'|']).unwrap();
        adress += file.lenght;
    }
    packed_file.write_all(&[b'|']).unwrap();

    // Write the files content
    for file_info in &files {
        // Open the file buffer
        let file = std::fs::File::open(file_info.name.clone()).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut read_lenght = file_info.lenght;
        loop {
            let mut buffer = vec![0;4096];
            reader.read(&mut buffer).unwrap();
            if read_lenght < 4096 {
                buffer = buffer[0..read_lenght as usize].to_vec().try_into().unwrap();
                println!("End of file");
                packed_file.write_all(&buffer).unwrap();
                break;
            }
            packed_file.write_all(&buffer).unwrap();
            read_lenght -= 4096;
        }
    }
}

fn to_file_info(file_names: Vec<String>) -> Vec<FileInfo> {
    let mut files: Vec<FileInfo> = Vec::with_capacity(file_names.len());
    for file_name in file_names {
        // Open the file
        let file = std::fs::File::open(file_name.clone()).unwrap();
        files.push(FileInfo {
            lenght: file.metadata().unwrap().len() as u128,
            name: file_name,
        });
    }
    return files;
}

fn unpack_files() {
    // Read the packed file
    let packed_file = std::fs::File::open("packed.e").unwrap();
    let file_lenght = packed_file.metadata().unwrap().len() as u128;
    let mut reader = std::io::BufReader::new(packed_file);
    // Get the header
    let mut header: Vec<u8> = Vec::new();
    let mut read_lenght = file_lenght.clone();
    loop {
        let mut buffer = Vec::new();
        reader.read_until(b'|', &mut buffer).unwrap();
        if buffer.len() <= 1 || read_lenght == 0 {
            header.append(&mut buffer);
            break;
        }
        header.append(&mut buffer);
        read_lenght -= buffer.len() as u128;
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
        let mut buffer = vec![0; 4096];
        let mut read_lenght;
        if i < files.len() - 1 {
            read_lenght = files[i+1].adress - files[i].adress;
        }
        else {
            read_lenght = file_lenght - files[i].adress;
        }
        loop {
            if read_lenght < 4096 {
                buffer = vec![0; read_lenght as usize];
                reader.read_exact(&mut buffer).unwrap();
                file.write_all(&buffer).unwrap();
                break;
            }
            reader.read_exact(&mut buffer).unwrap();
            file.write_all(&buffer).unwrap();
            read_lenght -= 4096;
        }
    }

}