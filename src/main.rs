use minigrep::运行;
use minigrep::配置;
use std::env;
use std::process;

fn main() {
    let 配置参数 = 配置::新建(env::args()).unwrap_or_else(|err| {
        eprintln!("{}", err);
        process::exit(1);
    });

    println!("文件: {}", 配置参数.文件);
    println!("查询: {}", 配置参数.查询);

    if let Err(err) = 运行(配置参数) {
        eprintln!("运行出错: {}", err);
        process::exit(1);
    }
}
