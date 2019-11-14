use std::process;
use std::fs;

fn main() {
    let inputs: Vec<String> = std::env::args().collect();
    if inputs.len() < 2 {
        eprintln!("input file and/or directory.");
        process::exit(1);
    }
    // // 最初の要素（=実行ファイル）は無視して入力チェック
    let invalid_inputs: Vec<&String> = inputs[1..]
        .iter()
        .filter(|input| !std::path::Path::new(input).exists())
        .collect();
    // 存在しないファイル・ディレクトリがあれば終了
    if !invalid_inputs.is_empty() {
        for invalid_input in invalid_inputs {
            eprintln!("[Error]\"{}\" is not exist.", invalid_input);
        }
        process::exit(1);
    }
    println!("valid_inputs : {:?}", &inputs[1..]);
    for valid_input in &inputs[1..] {
        make_empty(&valid_input);
    }
}

fn make_empty(target: &str) {
    if let Ok(metadata) = fs::metadata(target) {
        let file_type = metadata.file_type();
        if file_type.is_file() {
            make_file_enpty(target);
        }
        if file_type.is_dir() {
            make_dir_enpty(target);
        }
    } else {
        eprintln!("[Error]\"{}\"'s metadata is unavailable.", target);
    }
}

fn make_file_enpty(file: &str) {
    let f = fs::OpenOptions::new().write(true).truncate(true).open(file);
    match f {
        Ok(_) => println!("{}", file),
        Err(_) => eprintln!("[Error]\"{}\" is unavailable.", file),
    }
}

fn make_dir_enpty(dir: &str) {
    if let Ok(read_dir) = fs::read_dir(dir) {
        for entry in read_dir {
            let entry_path = entry.unwrap().path();
            make_empty(entry_path.to_str().unwrap());
        }
    } else {
        eprintln!("[Error]\"{}\" is unavailable.", dir);
    }
}
