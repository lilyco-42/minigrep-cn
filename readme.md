# minigrep 
## 核心内容
- use std::env;
- use std::error::Error;
- use std::fs;
- 迭代器 VS clone
```
   env   提供运行参数读取和环境变量读取
   error <Box dyn Error> 运行时捕捉错误
   fs    读取文件

   迭代器 文件内容.lines().filter(|行| 行.contains(查询)).collect()  避免clone，而零成本抽象
   迭代器 std::env::Args 读取用户输入 mut 是必需的，因为调用 next() 会改变迭代器的内部状态（消费元素）
```
[minigrep资料](https://rustwiki.org/zh-CN/book/ch12-00-an-io-project.html)
[迭代器优化for，clone](https://rustwiki.org/zh-CN/book/ch13-03-improving-our-io-project.html)
# cargo run [查询字段] [文件]
## CASE_INSENSITIVE的变量 有无控制宽泛搜索（大小写不敏感）/普通搜素
