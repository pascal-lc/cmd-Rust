# Rust 重写命令行

传统命令行程序使用 Rust 编程语言重写，CLI 复刻练习。

## 目录说明

* hello
  * 编程语言 101，屏幕打印 `Hello, world!`，致敬经典。
* cat
  * 将指定文本文件内容写入标准输出（命令行输出）
  * `-n` 选项，显示行号；
  * `-b` 选项，显示行号，不显示空行行号；
* echo
  * 输出给定内容
  * `-n` 选项，不换行输出。
* head
  * 输出指定文件的开头部分
  * `-n <num>` 选项，指定输出行数;
  * `-c <num>` 选项，其中 `<num>` 为字节数对应于字符数，即指定输出字符数。
* wc
  * 统计指定文件的字符数、单词数、行数
  * `-w` 选项，统计单词数；
  * `-c` 选项，统计字符数；
  * `-m` 选项，统计字符数，不包括换行符；
  * `-l` 选项，统计行数。

## 编译运行

Rust 环境下，使用 Rust 官方提供的包管理工具 Cargo 构建程序。

1. 进入项目目录，执行 `cargo build` 命令，编译程序；
2. 执行 `cargo run` 命令，运行程序；
3. 对于一些带选项的程序，执行 `cargo run -- <options>` 命令，其中 `<options>` 为选项参数；
4. 或者直接运行 `target/debug/<program>` 命令，其中 `<program>` 为程序名。

## 参考资料

* 《Rust 程序设计》
* [Rust 程序设计语言](https://kaisery.github.io/trpl-zh-cn/)官方教程
