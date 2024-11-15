use flate2::{self, FlushDecompress, Status};
use std::env;
use std::fs;
use std::io;
use std::path::Path;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    eprintln!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "init" => {
            fs::create_dir(".git").unwrap();
            fs::create_dir(".git/objects").unwrap();
            fs::create_dir(".git/refs").unwrap();
            fs::write(".git/HEAD", "ref: refs/heads/main\n").unwrap();
            println!("Initialized git directory")
        }
        "cat-file" => {
            // TODO: Check if working dir was git init

            // Note -p is a flag, so it might be optional and at any position
            // refactor this later to better handle flags and optional flags
            assert_eq!(args.len(), 4, "usage: cat-file -p <hash>");
            assert_eq!(args[2], "-p", "usage: cat-file -p <hash>");
            let hash = &args[3];
            assert_eq!(hash.len(), 40, "Hash size invalid (must be 40 chars long)");

            const GIT_OBJECTS_PATH: &str = ".git/objects";
            // let hashPath = GIT_OBJECTS_PATH + hash[:2] +"/"+ hash[2:];

            // STUDY: Investigate/Understand why this won't work without the &
            // let hashPath = format!(
            //     "{}/{}/{}",
            //     GIT_OBJECTS_PATH,
            //     hash[0..2],
            //     hash[2..hash.len()]
            // );

            // STUDY: Why this won't generate an issue? (what if hash provide has length == 1?)
            let hash_path = format!("{}/{}/{}", GIT_OBJECTS_PATH, &hash[0..2], &hash[2..]);

            //check file existence
            let file = Path::new(&hash_path);
            if !file.exists() {
                // Panic is probably not the best way.
                // Check if theres a better way (write to sdterr and exit(1) or something)
                // Also, check what Panic does, and if I should avoid it
                panic!("File doesn't exist")
            }

            // load data from file
            // let mut file_content: Vec<u8>;

            let file_content = match read_file_contents(&hash_path) {
                Ok(content) => content,
                Err(_error) => panic!("Failed to read content from hash file: {}", _error),
            };

            // decompress it Zlib/flate2
            let mut decompressor = flate2::Decompress::new(true);
            let mut uncompressed_content: Vec<u8> = Vec::new();

            // Q: Would make sense to add this on a block so the buffer gets free once leaving the block?
            let mut buffer: Vec<u8> = vec![0; 4096];
            loop {
                match decompressor.decompress(&file_content, &mut buffer, FlushDecompress::None) {
                    Ok(Status::StreamEnd) => {
                        uncompressed_content
                            .extend_from_slice(&buffer[..decompressor.total_out() as usize]);
                        break;
                    }
                    Ok(Status::Ok) => {
                        uncompressed_content
                            .extend_from_slice(&buffer[..decompressor.total_out() as usize]);
                        buffer.resize(buffer.len() * 2, 0); // Double the buffer size
                    }
                    Ok(Status::BufError) => {
                        panic!("Failed to decompress file content")
                    }
                    Err(error) => panic!("Failed to decompress file content: {}", error),
                }
            }
            // Split by byte 0 and
            // extract [[type, length] , [content]]
            let parts: Vec<&[u8]> = uncompressed_content.split(|&x| x == 0).collect();

            let (_, data) = match parts.as_slice() {
                [metadata, data, ..] => (metadata, data),
                _ => panic!("Expected at least two parts after splitting by zero byte"),
            };

            // let metadata_string = match String::from_utf8(metadata.to_vec()) {
            //     Ok(string) => string,
            //     Err(error) => panic!("Failed to convert metadata to string: {}", error),
            // };

            // Though this works, it doesn't leverage on the data size input
            let data_string = match String::from_utf8(data.to_vec()) {
                Ok(string) => string,
                Err(error) => panic!("Failed to convert data to string: {}", error),
            };

            // Check: Add compilation conditional tags so it's easy to switch between debug and "release mode"
            // println!("{}", metadata_string);
            print!("{}", data_string);
        }
        _ => println!("unknown command: {}", args[1]),
    }
}

// TODO: Check if we could use the same struct to verify the existence + read content (File::open + file.read_to_string)
fn read_file_contents(path: &str) -> Result<Vec<u8>, io::Error> {
    let contents = fs::read(path)?;
    Ok(contents)
}
