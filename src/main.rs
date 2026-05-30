use minigrep::运行;
use minigrep::配置;
use std::env;
use std::process;

fn main() {
    println!("cargo run [查询字段] [文件]\nCASE_INSENSITIVE的变量\n有无控制宽泛搜索");
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
