use std::fs;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let inputs: &[String] = &args[1..];
    if is_invalid_inputs(inputs) {
        std::process::exit(1);
    }
    for valid_input in inputs {
        make_empty(valid_input);
    }
}

fn is_invalid_inputs(inputs: &[String]) -> bool {
    if inputs.len() < 1 {
        eprintln!("input file and/or directory.");
        return true;
    }
    let mut is_invalid = false;
    for input in inputs {
        if !std::path::Path::new(input).exists() {
            eprintln!("[non-existent path]{}", input);
            is_invalid = true;
        }
    }
    is_invalid
}

fn make_empty(target: &str) {
    match fs::symlink_metadata(target) {
        Ok(metadata) => {
            let file_type = metadata.file_type();
            if file_type.is_symlink() {
                eprintln!("[Skip: Symbolic link]{}", target);
            } else {
                if file_type.is_file() {
                    make_file_enpty(target);
                }
                if file_type.is_dir() {
                    make_dir_enpty(target);
                }
            }
        }
        Err(e) => eprintln!("[Error: {}]{}", e, target),
    }
}

fn make_file_enpty(file: &str) {
    let f = fs::OpenOptions::new().write(true).truncate(true).open(file);
    match f {
        Ok(_) => println!("[Done]{}", file),
        Err(e) => eprintln!("[Error: {}]{}", e, file),
    }
}

fn make_dir_enpty(dir: &str) {
    match fs::read_dir(dir) {
        Ok(read_dir) => {
            for entry in read_dir {
                let entry_path = entry.unwrap().path();
                make_empty(entry_path.to_str().unwrap());
            }
        }
        Err(e) => eprintln!("[Error: {}]{}", e, dir),
    }
}
