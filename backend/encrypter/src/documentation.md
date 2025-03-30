This documentation is included as part of a capsule file to help understanding and opening it.

# Project goal
This project aims to make a simple, secure and everlasting format to store files.
To achieve this, generic file types are converted to simple file types which assure the possibility of getting the data back even if the file format is long lost. After being packed into a single file, they are encrypted with RC4 which is a simple and fast symmetric-key encryption algorithm, which means that you only need the key and the right logic to access the encrypted files.

# Structure
## 1. Conversion and File formats
### 1.1 Images
Supported types (png, jpg, jpeg, tiff, ico, webp, bmp, gif) are transformed into a simple format structure :
#### 1.1.1 Header
The first __8 bytes__ are used to store 2 unsigned integers both using 4 bytes which indicate the __width and height__ respectively.
#### 1.1.2 Content
The content is stored as a continuous list of 4 values, each as one byte : __Red, Green, Blue, Alpha__
### 1.2 Audio
Supported types (wav, mp3, ogg) are transformed into a simple format structure :
#### 1.2.1 Header

#### 1.2.2 Content

### Video
Video files are ???
### Other file formats
For conversion into the capsule, unsupported file formats are taken as-is and an extension (.eall) is simply added at the end of the file name to mark it for packing.
For conversion back to the original format, the .eall extension is just removed from the unpacked file.
For example, this is the case for .txt files, which are simply stored as one byte per character.
## 2. Packing
The packing works by making a file in two parts, a header to store information about the files, and the files' data.
### Header
The header follows a format of : 
- 16 bytes of adress to reference to begining bit of the corresponding file date
- The name of the file as a utf-_ string
- A "|" caracter to delimit the end of the file name
And ends with an added "|" to mark the end of the header
### Data
The data is simply stored as it is in every file, following the file order of the header.
To be read, it is simply needed to get the data from the file's adress until the next file's adress or the end of the file.
## 3. Encryption
The packed file is encrypted with a key of any lenght with the RC4 symmetric key, stream cipher.
The algorithm works by first making a permutation array then applying it to the stream of values, which in this case is the packed file.
### KSA
The key-scheduling algorithm works by the following pseudocode :

```
for i from 0 to 255
    S[i] := i
endfor
j := 0
for i from 0 to 255
    j := (j + S[i] + key[i mod keylength]) mod 256
    swap values of S[i] and S[j]
endfor
```
### PRGA
The Pseudo-random generation algorithm works by the following pseudocode :
```
i := 0
j := 0
while GeneratingOutput:
    i := (i + 1) mod 256
    j := (j + S[i]) mod 256
    swap values of S[i] and S[j]
    t := (S[i] + S[j]) mod 256
    K := S[t]
    output K
endwhile
```
