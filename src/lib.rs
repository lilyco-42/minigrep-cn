use std::env;
use std::error::Error;
use std::fs;

pub struct 配置 {
    pub 查询: String,
    pub 文件: String,
    pub 宽泛值: bool,
}
impl 配置 {
    pub fn 新建(mut 参数: std::env::Args) -> Result<配置, &'static str> {
        参数.next();
        let 查询 = match 参数.next() {
            Some(第二参数) => 第二参数,
            None => return Err("没找到字符串"),
        };

        let 文件 = match 参数.next() {
            Some(第三参数) => 第三参数,
            None => return Err("没找到文件"),
        };

        let 宽泛值 = env::var("CASE_INSENSITIVE").is_err();
        Ok(配置 {
            查询, 文件, 宽泛值
        })
    }
}
pub fn 运行(配置参数: 配置) -> Result<(), Box<dyn Error>> {
    let 文件内容 = fs::read_to_string(&配置参数.文件)?;
    println!("文件内容:\n{}", 文件内容);
    let 结果 = if 配置参数.宽泛值 {
        搜索(&配置参数.查询, &文件内容)
    } else {
        宽泛搜索(&配置参数.查询, &文件内容)
    };
    for 行 in 结果 {
        println!("{}", 行);
    }
    Ok(())
}

pub fn 搜索<'a>(查询: &str, 文件内容: &'a str) -> Vec<&'a str> {
    文件内容.lines().filter(|行| 行.contains(查询)).collect()
}
pub fn 宽泛搜索<'a>(查询: &str, 文件内容: &'a str) -> Vec<&'a str> {
    文件内容
        .lines()
        .filter(|行| 行.to_lowercase().contains(&查询.to_lowercase()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], 搜索(query, contents));
    }
    #[test]
    fn 大小写不敏感() {
        let query = "rUsT";
        let contents = "\
       Rust:
       safe, fast, productive.
       Pick three.
Trust me.";

        assert_eq!(vec!["Rust:", "Trust me."], 宽泛搜索(query, contents));
    }
}
