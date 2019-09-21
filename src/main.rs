use std::fs;
use std::path::Path;

fn main() {
    let inputs: Vec<String> = std::env::args().collect();
    let mut invalid_inputs: Vec<String> = Vec::new();
    let mut valid_inputs: Vec<String> = Vec::new();
    // 最初の要素（=実行ファイル）は無視
    for input in &inputs[1..] {
        if Path::new(input).exists() {
            valid_inputs.push(input.to_string());
        } else {
            invalid_inputs.push(input.to_string());
        }
    }
    // 存在しないファイルがあれば終了
    if !invalid_inputs.is_empty() {
        eprintln!("[Error]");
        for invalid_input in invalid_inputs {
            eprintln!("\"{}\" is not exist.", invalid_input);
        }
        std::process::exit(-1);
    }
    println!("valid_inputs : {:?}", valid_inputs);
    for valid_input in &valid_inputs {
        make_empty(valid_input);
    }
}

fn make_empty(target: &str) {
    let metadata = fs::metadata(target)
        .unwrap_or_else(|_| panic!("\"{}\"'s metadata is unavailable.", target));
    let file_type = metadata.file_type();
    if file_type.is_file() {
        make_file_enpty(target);
    }
    if file_type.is_dir() {
        make_dir_enpty(target);
    }
}

fn make_file_enpty(file: &str) {
    // println!("[file]\t{}", file);
    println!("{}", file);
    fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file)
        .unwrap_or_else(|_| panic!("\"{}\" is unavailable.", file));
}

fn make_dir_enpty(dir: &str) {
    // println!("[dir]\t{}", dir);
    let read_dir = fs::read_dir(dir).unwrap_or_else(|_| panic!("\"{}\" is unavailable", dir));
    for entry in read_dir {
        let entry_path = entry.unwrap().path();
        let entry_path_as_osstr = entry_path.into_os_string();
        let entry_path_as_str = entry_path_as_osstr.to_str().unwrap();
        make_empty(entry_path_as_str);
    }
}
