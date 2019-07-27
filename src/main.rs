use std::path::Path;
use std::fs;

fn main() {
    let mut inputs: Vec<String> = std::env::args().collect();
    inputs.remove(0); // 最初の要素（=実行ファイル）を削除
    let mut invalid_inputs: Vec<String> = Vec::new(); 
    let mut valid_inputs: Vec<String> = Vec::new(); 
    for input in &inputs {
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
    let metadata = fs::metadata(target).unwrap();
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
    fs::OpenOptions::new().write(true).truncate(true).open(file).unwrap();
}

fn make_dir_enpty(dir: &str) {
    // println!("[dir]\t{}", dir);
    for entry in fs::read_dir(dir).unwrap() {
        let entry_path = entry.unwrap().path();
        let entry_path_as_osstr = entry_path.into_os_string();
        let entry_path_as_str = entry_path_as_osstr.to_str().unwrap();
        make_empty(entry_path_as_str);
    }
}
