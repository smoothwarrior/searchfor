

use std::{fs, fs::DirEntry, env, io, path::Path};

fn exit() {
    println!("Usage: searchfor -s [search] -l [location, default ./] -[options]");
}

const VERSION: &str = "0.2.2";

#[allow(unused_assignments)]
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut search_word = &String::new();
    let mut search_dir = Path::new(".");

    if args.contains(&String::from("-v")) {
        println!("Version: {}", VERSION);
        return Ok(())
    }

    if args.contains(&String::from("-s")) {
        let mut sw_index = args.iter().position(|n| n == "-s").unwrap();
        sw_index += 1;
        search_word = &args[sw_index];
    }
    else {
        exit();
        return Err(io::Error::other("no search_word found."))
    }

    if args.contains(&String::from("-l")) {
        let mut sl_index = args.iter().position(|n| n == "-l").unwrap();
        sl_index += 1;
        search_dir = &Path::new(&args[sl_index]);
    }

    let mut dir_output: bool = false;
    let mut exclude_extensions: bool = false;

    if args.contains(&String::from("-d")) {
        dir_output = true;
    }
    if args.contains(&String::from("-x")) {
        exclude_extensions = true;
    }

    let data = SearchData {
        sw: &search_word,
        sl: &search_dir,
        dir: &dir_output,
        ext: &exclude_extensions
    };

    visit_dir(&data)?;

    Ok(())
}

fn visit_dir(data: &SearchData) -> io::Result<()> {
    if data.sl.is_dir() {
        for entry in fs::read_dir(data.sl)? {
            let entry = entry?;
            let path = entry.path();
            let file_name = entry.file_name().into_string().unwrap();
            if path.is_dir() {
                if file_name.contains(data.sw) && data.dir == &true {
                    search_match(&entry).unwrap();
                }

                let data_new = SearchData {
                    sw: &data.sw,
                    sl: &path,
                    dir: &data.dir,
                    ext: &data.ext
                };

                visit_dir(&data_new)?;
            }
            else {
                if data.ext == &true {
                    if file_name.contains(data.sw) && !file_name.contains(".") {
                        search_match(&entry).unwrap();
                    }
                }
                else {
                    if file_name.contains(data.sw) {
                        search_match(&entry).unwrap();
                    }
                }
            }
            drop(entry);
        }
    }
    Ok(())
}

fn search_match(entry: &DirEntry) -> io::Result<()> {
    let file_type = if entry.file_type().unwrap().is_file() {
        String::from("File")
    }
    else {
        String::from("Directory")
    };
    let entry_meta = entry.metadata().unwrap();
    let mut file_size = entry_meta.len();
    let mut byte_size = "bytes";
    if file_size > 1024 {
        file_size = file_size / 1024;
        byte_size = "kb";
    }
    if file_size > 1024 {
        file_size = file_size / 1024;
        byte_size = "mb";
    }
    if file_size > 1024 {
        file_size = file_size / 1024;
        byte_size = "gb";
    }
    println!("Name: {:?} type: {:?} bytes: {:?} {}\n{:?}\n", entry.file_name().into_string().unwrap(), file_type, file_size, byte_size, entry.path());
    Ok(())
}

struct SearchData <'a> {
    sw: &'a String,
    sl: &'a Path,
    dir: &'a bool,
    ext: &'a bool
}
