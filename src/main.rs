use std::env;
use std::process;

use minigrep::{self, Config};

fn main() {
    // args(): 回傳一個 iterator，裡面包含著我們傳的命令列參數
    // collect(): 將 iterator 轉換成 Vec，Vec 會包含著我們傳的命令列參數
    // 顯式宣告 args 的型別來指定我們想要字串向量。雖然我們很少需要在 Rust 中詮釋型別，collect 是其中一個你常常需要詮釋的函式，因為 Rust 無法推斷出你想要何種集合。
    // let args: Vec<String> = env::args().collect();
    // dbg!(args);

    // unwrap_or_else: 讓我們能夠定義一些非 panic! 的處理方法，傳入一個函式(F)作為 unwrap_or_else 的參數
    // 如果 Result 的結果是 Err(err)，就會把 err 作為參數傳入 F 當中執行
    // 下面使用匿名函數作為參數傳入 unwrap_or_else 當中
    let config = Config::build(env::args()).unwrap_or_else(|err|{
        eprintln!("解析引數時發生錯誤：{}", err); // 將錯誤訊息用標準錯誤輸出
        process::exit(1);
    });

    if let Err(err) = minigrep::run(config){
        eprintln!("應用程式錯誤: {}", err);
        process::exit(1);
    }
}
