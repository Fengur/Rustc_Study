use std::fs::File;
use std::io::ErrorKind;
use stf::io::Read;

fn main() {
    println!("Hello, world!");
    // panic!("crash 啦啦啦");
    // let v = vec![1,2];
    // v[100];
// 为了获取有效信息的log 必须启用debug run模式
// 当不使用 --release 参数运行 cargo build 或 cargo run 时 debug 标识会默认启用
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
// RUST_BACKTRACE=1 cargo run
// 阅读 backtrace 的关键是从头开始读直到发现你编写的文件。这就是问题的发源地。这一行往上是你的代码调用的代码；往下则是调用你的代码的代码。这些行可能包含核心 Rust 代码，标准库代码或用到的 crate 代码。

    // let fuck = File::open("fuckyou.txt");
    // let fuck = match fuck{
    //     Ok(file) => file,
    //     Err(error) => {
    //         panic!("打开文件错误：{:?}",error)
    //     },
    // };


    let caonima = File::open("caonima.wav");
    let ganniniang = match caonima{
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("caonima.wav"){
                Ok(fc) => fc,
                Err(error) => panic!("创建文件失败 {:?}",error),
            },
            other_error => panic!("其他失败原因： {:?}",other_error),
        },
    };


    let qunima = File::open("qunima.pdf").map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("caonima.txt").unwrap_or_else(|error| {
                panic!("创建操你妈文件失败 {:?}",error);
            })
        }else{
            panic!("打开文件失败 {:?}",error);
        }
    });

    // let test_un_wrap = File::open("cao.img").unwrap();
    let test_un_expect = File::open("gannilaomu.text").expect("ganiniang 打开失败了");
    

}



fn read_usename_from_file() -> Result<String,io::Error>{
    let fuckyou = File::open("caonimeifu.txt");

    let mut f = match fuckyou{
        Ok(file) => file,
        Err(e) => return Err(e),
    };
}