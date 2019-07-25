use std::path::Path;

fn main() {
    let mut inputs: Vec<String> = std::env::args().collect();
    inputs.remove(0); // 最初の要素（=実行ファイル）を削除
    let mut invalid_inputs: Vec<String> = Vec::new(); 
    let mut valid_inputs: Vec<String> = Vec::new(); 
    for input in inputs.iter() {
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
}
