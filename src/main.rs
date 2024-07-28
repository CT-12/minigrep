use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &Vec<String>) -> Config {
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config {
            query: query,
            file_path: file_path,
        }
    }
}

fn main() {
    // args(): 回傳一個 iterator，裡面包含著我們傳的命令列參數
    // collect(): 將 iterator 轉換成 Vec，Vec 會包含著我們傳的命令列參數
    // 顯式宣告 args 的型別來指定我們想要字串向量。雖然我們很少需要在 Rust 中詮釋型別，collect 是其中一個你常常需要詮釋的函式，因為 Rust 無法推斷出你想要何種集合。
    let args: Vec<String> = env::args().collect();
    // dbg!(args);

    let config = Config::new(&args);

    let content = fs::read_to_string(config.file_path).expect("檔案讀取錯誤");

    println!("Content:\n {}", content);
}
