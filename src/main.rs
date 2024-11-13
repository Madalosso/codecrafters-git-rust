#[allow(unused_imports)]
use std::env;
#[allow(unused_imports)]
use std::fs;
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
            println!("{}", hash_path);

            //check file existence
            let file = Path::new(&hash_path);
            if !file.exists() {
                // Panic is probably not the best way.
                // Check if theres a better way (write to sdterr and exit(1) or something)
                // Also, check what Panic does, and if I should avoid it
                panic!("File doesn't exist")
            }

            // load data from file

            // decompress it Zlib/flate2

            // extract type, length and content

            // print content
            let content = "";
            print!("{}", content)
        }
        _ => println!("unknown command: {}", args[1]),
    }
}
