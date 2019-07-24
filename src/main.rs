use std::path::Path;

fn main() {
    let mut args: Vec<String> = std::env::args().collect();
    args.remove(0); // 最初の要素(=バイナリ) を削除
    let entries = args.iter().filter(|e| Path::new(e).exists());
    for entry in entries {
        println!("{:?}", entry);
    }
}
