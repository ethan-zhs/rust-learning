# Rust 学习教程：从环境搭建到能写小项目

这是一份面向初学者的 Rust 学习教程。目标不是死记语法，而是理解 Rust 的思路，并且每个知识点都能配一个可运行 demo。

配套 demo 项目在：

```text
F:\workspace\study\rust\rust_demos
```

安装 Rust 后运行：

```powershell
cd F:\workspace\study\rust\rust_demos
cargo run --example 01_hello
```

## 1. Rust 是什么

Rust 是一门系统编程语言，特点是速度快、内存安全、没有垃圾回收器。它常用于命令行工具、Web 服务、嵌入式、区块链、游戏引擎和底层组件。

Rust 最核心的学习点是所有权。刚开始你会觉得编译器很严格，但这份严格能提前挡掉很多空指针、悬垂引用和数据竞争问题。

## 2. 环境搭建：Windows、macOS、Linux

Rust 官方推荐用 `rustup` 管理工具链。`rustup` 会帮你安装和管理：

- `rustc`：Rust 编译器。
- `cargo`：项目创建、依赖管理、构建、运行、测试工具。
- `rustfmt`：代码格式化工具。
- `clippy`：代码质量检查工具。
- 标准库文档和不同平台的编译目标。

无论你使用 Windows、macOS 还是 Linux，建议都优先使用 `rustup`，不要一开始就用系统包管理器安装旧版本 Rust。

### 2.1 Windows 安装

推荐方式一：使用 `winget`。

打开 PowerShell：

```powershell
winget install Rustlang.Rustup
```

推荐方式二：使用官方安装器。

访问：

```text
https://www.rust-lang.org/tools/install
```

下载并运行适合你系统的 `rustup-init.exe`：

- 大多数 Windows 电脑选择 x64。
- ARM 设备选择 ARM64。
- 很老的 32 位系统才选择 32-bit。

安装过程中如果提示安装 Visual Studio C++ Build Tools，请按提示安装。Windows 上 Rust 需要 MSVC linker 和一些原生库，很多第三方 crate 编译时也会用到它。

安装完成后，关闭并重新打开 PowerShell，验证：

```powershell
rustc --version
cargo --version
rustup --version
```

如果提示找不到命令，通常是 PATH 还没有刷新。先关闭终端重开；如果还不行，检查：

```powershell
echo $env:Path
```

确认里面是否有：

```text
%USERPROFILE%\.cargo\bin
```

### 2.2 Windows WSL 安装

如果你在 Windows 上使用 WSL，比如 Ubuntu on WSL，不要在 WSL 里运行 Windows 的 `rustup-init.exe`。WSL 属于 Linux 环境，使用 Linux 安装方式：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后让环境变量生效：

```bash
source "$HOME/.cargo/env"
```

验证：

```bash
rustc --version
cargo --version
rustup --version
```

WSL 下如果编译时提示 linker 或 `cc` 缺失，Ubuntu/Debian 可以安装：

```bash
sudo apt update
sudo apt install build-essential
```

### 2.3 macOS 安装

先安装命令行开发工具，它包含 C 编译器和 linker：

```bash
xcode-select --install
```

然后安装 Rust：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后让环境变量生效：

```bash
source "$HOME/.cargo/env"
```

验证：

```bash
rustc --version
cargo --version
rustup --version
```

如果你使用 zsh，这是 macOS 默认 shell，重开终端一般也会自动生效。如果仍然找不到 `cargo`，检查：

```bash
echo $PATH
```

确认里面是否有：

```text
$HOME/.cargo/bin
```

### 2.4 Linux 安装

Linux 同样推荐使用 `rustup`：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

安装完成后：

```bash
source "$HOME/.cargo/env"
```

验证：

```bash
rustc --version
cargo --version
rustup --version
```

不同发行版还需要安装基础编译工具。

Ubuntu / Debian：

```bash
sudo apt update
sudo apt install build-essential pkg-config libssl-dev
```

Fedora：

```bash
sudo dnf groupinstall "Development Tools"
sudo dnf install pkg-config openssl-devel
```

Arch Linux：

```bash
sudo pacman -S base-devel pkgconf openssl
```

说明：

- `build-essential` / `Development Tools` / `base-devel` 提供 C 编译器、make、linker 等工具。
- `pkg-config` 和 OpenSSL 开发包是很多网络相关 crate 常见依赖。
- 如果你只写纯标准库 demo，可能暂时用不到 OpenSSL，但提前装好能少踩坑。

### 2.5 更新、卸载和文档

更新 Rust：

```bash
rustup update
```

卸载 Rust：

```bash
rustup self uninstall
```

打开本地文档：

```bash
rustup doc
```

查看当前工具链：

```bash
rustup show
```

安装常用组件：

```bash
rustup component add rustfmt clippy
```

格式化和检查：

```bash
cargo fmt
cargo clippy
```

### 2.6 多端目录和命令差异

Windows PowerShell：

```powershell
cd F:\workspace\study\rust\rust_demos
cargo run --example 01_hello
```

macOS / Linux / WSL：

```bash
cd ~/workspace/study/rust/rust_demos
cargo run --example 01_hello
```

如果你的项目不在这个目录，把 `cd` 后面的路径换成自己的实际路径即可。

Windows 路径常用反斜杠：

```text
F:\workspace\study\rust
```

macOS / Linux 路径常用斜杠：

```text
/Users/your_name/workspace/study/rust
/home/your_name/workspace/study/rust
```

### 2.7 推荐编辑器

推荐 VS Code，并安装：

- `rust-analyzer`：代码提示、跳转、类型提示、错误提示。
- `Even Better TOML`：更好地编辑 `Cargo.toml`。
- `CodeLLDB`：调试 Rust 程序时常用。

VS Code 三端都能用：

- Windows：正常安装 VS Code。
- macOS：安装 VS Code 后，把 `code` 命令加入 PATH 会更方便。
- Linux：用发行版软件商店、`.deb`、`.rpm` 或包管理器安装。

### 2.8 常见安装问题

问题一：`cargo` 或 `rustc` 找不到。

解决：重开终端，或确认 PATH 中有 Cargo bin 目录。

Windows：

```text
%USERPROFILE%\.cargo\bin
```

macOS / Linux：

```text
$HOME/.cargo/bin
```

问题二：`rustc --version` 提示没有默认工具链。

现象：

```text
rustup could not choose a version of rustc to run
no default is configured
```

原因：`rustup` 装好了，但还没有下载并设置默认 Rust 工具链。

解决：

```powershell
rustup default stable
rustc --version
cargo --version
```

问题三：VS Code 终端里找不到 `rustup` / `cargo`。

现象：普通 PowerShell 能用 `cargo`，但 VS Code 终端提示：

```text
无法将“rustup”项识别为 cmdlet、函数、脚本文件或可运行程序的名称
```

原因：安装 Rust 时 VS Code 已经打开，VS Code 继承的是旧 PATH。

解决：完全退出所有 VS Code 窗口，然后重新打开。仍不行时，检查 Cargo bin：

```powershell
Test-Path "$env:USERPROFILE\.cargo\bin\rustup.exe"
```

如果返回 `True`，临时修复当前终端：

```powershell
$env:Path += ";$env:USERPROFILE\.cargo\bin"
rustup show
```

永久修复用户 PATH：

```powershell
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
$cargoBin = "$env:USERPROFILE\.cargo\bin"

if ($userPath -notlike "*$cargoBin*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$cargoBin", "User")
}
```

然后完全退出 VS Code，再重新打开。

问题四：Windows 编译时报 `link.exe not found`。

现象：

```text
error: linker `link.exe` not found
note: the msvc targets depend on the msvc linker but `link.exe` was not found
note: VS Code is a different product, and is not sufficient
```

原因：Rust 使用的是 MSVC target，需要 Visual Studio C++ Build Tools 里的 linker。VS Code 只是编辑器，不包含 MSVC linker。

先检查：

```powershell
where.exe link
```

如果找不到，安装或修改 Build Tools：

```powershell
winget install --id Microsoft.VisualStudio.2022.BuildTools -e --force --override "--add Microsoft.VisualStudio.Workload.VCTools --includeRecommended --passive --norestart"
```

也可以打开 Visual Studio Installer，修改“Visual Studio 生成工具 2022”，勾选：

```text
使用 C++ 的桌面开发
```

至少确保包含：

```text
MSVC v143 C++ build tools
Windows 10/11 SDK
C++ CMake tools for Windows
```

问题五：`link.exe` 找到了，但提示 `kernel32.lib` 找不到。

现象：

```text
LINK : fatal error LNK1181: 无法打开输入文件“kernel32.lib”
```

原因：Windows SDK 没有装好，或者当前终端没有加载 MSVC/Windows SDK 环境变量。

先确认 SDK 文件是否存在：

```powershell
Get-ChildItem "C:\Program Files (x86)\Windows Kits\10\Lib" -Recurse -Filter kernel32.lib
```

如果文件不存在，回到 Visual Studio Installer，给 Build Tools 补装 Windows 10/11 SDK。

如果文件存在，但 Cargo 仍然报错，说明当前终端没加载开发环境。打开开始菜单里的：

```text
Developer PowerShell for VS 2022
```

或者：

```text
Developer Command Prompt for VS 2022
```

然后运行：

```powershell
cd F:\workspace\study\rust\rust_demos
cargo run --example 02_variables
```

如果你想在 VS Code 里运行，推荐从 Developer PowerShell 启动 VS Code：

```powershell
code F:\workspace\study\rust
```

这样 VS Code 会继承 `link.exe` 和 Windows SDK 的环境变量。

问题六：验证 Windows MSVC 环境是否正常。

在 Developer PowerShell 或 Developer Command Prompt 中执行：

```powershell
where.exe link
where.exe cl
rustc --version
cargo --version
cd F:\workspace\study\rust\rust_demos
cargo run --example 02_variables
```

能看到 demo 输出就说明环境完整：

```text
language = Rust
count = 2
shadowed value + 1 = 43
MAX_SCORE = 100
```

问题七：macOS / Linux 编译时提示 linker、cc、gcc、clang 找不到。

解决：安装平台对应的 C/C++ 构建工具。

- macOS：`xcode-select --install`。
- Ubuntu/Debian：`sudo apt install build-essential`。
- Fedora：`sudo dnf groupinstall "Development Tools"`。
- Arch：`sudo pacman -S base-devel`。

问题八：国内网络下载慢。

解决：可以配置 crates.io 镜像源或使用公司/学校代理。先保证 `rustup` 安装成功，再处理 Cargo 下载依赖的问题。
## 3. Cargo 入门

创建项目：

```powershell
cargo new hello_rust
cd hello_rust
cargo run
```

典型结构：

```text
hello_rust
├── Cargo.toml
└── src
    └── main.rs
```

常用命令：

```powershell
cargo run       # 编译并运行
cargo build     # 编译
cargo check     # 快速检查，不生成最终可执行文件
cargo test      # 运行测试
cargo fmt       # 格式化
cargo clippy    # 代码质量检查
```

本教程 demo：

```powershell
cargo run --example 01_hello
```

## 4. Hello World

Rust 程序入口是 `main` 函数：

```rust
fn main() {
    println!("Hello, Rust!");
}
```

解释：`fn` 定义函数，`main` 是入口，`println!` 是宏，语句通常以分号结束。

运行 demo：

```powershell
cargo run --example 01_hello
```

## 5. 变量、常量与遮蔽

Rust 变量默认不可变：

```rust
let name = "Rust";
```

要修改变量，必须写 `mut`：

```rust
let mut count = 1;
count += 1;
```

常量使用 `const`，必须标注类型：

```rust
const MAX_SCORE: u32 = 100;
```

遮蔽是创建一个同名新变量，常用于类型转换：

```rust
let value = "42";
let value: i32 = value.parse().unwrap();
```

运行 demo：

```powershell
cargo run --example 02_variables
```

## 6. 基本数据类型

常见类型：

- 整数：`i32`、`i64`、`u32`、`usize`。
- 浮点数：`f32`、`f64`。
- 布尔：`bool`。
- 字符：`char`，支持 Unicode。
- 元组：`(i32, f64, bool)`。
- 数组：`[i32; 3]`，长度固定。

示例：

```rust
let age: u32 = 18;
let price: f64 = 19.9;
let ok: bool = true;
let letter: char = 'R';
let point = (10, 20);
let numbers = [1, 2, 3];
```

运行 demo：

```powershell
cargo run --example 03_types
```

## 7. 函数、表达式与控制流

Rust 函数参数必须写类型：

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

最后一行没有分号时，它就是返回值。Rust 的 `if` 也可以是表达式：

```rust
let level = if score >= 60 { "pass" } else { "fail" };
```

常见循环：

```rust
while n < 3 {
    n += 1;
}

for item in 1..=3 {
    println!("{item}");
}

let result = loop {
    break 10;
};
```

运行 demo：

```powershell
cargo run --example 04_functions_control_flow
```

## 8. 所有权

所有权是 Rust 最重要的概念。三条规则：

1. 每个值都有一个所有者。
2. 同一时间只能有一个所有者。
3. 所有者离开作用域时，值会被释放。

示例：

```rust
let s1 = String::from("hello");
let s2 = s1;
// println!("{s1}"); // 编译错误，s1 已经移动
println!("{s2}");
```

`String` 数据在堆上，赋值时默认移动所有权。想复制堆数据，使用 `clone`：

```rust
let s1 = String::from("hello");
let s2 = s1.clone();
```

像 `i32`、`bool`、`char` 这类简单类型实现了 `Copy`，赋值后原变量仍可使用。

运行 demo：

```powershell
cargo run --example 05_ownership
```

## 9. 借用、引用与切片

函数不想拿走所有权时，可以借用：

```rust
fn len(s: &String) -> usize {
    s.len()
}
```

需要修改时，用可变引用：

```rust
fn add_suffix(s: &mut String) {
    s.push_str("!");
}
```

借用规则：同一时间可以有多个不可变引用；同一时间只能有一个可变引用；可变引用和不可变引用不能同时活跃。

切片是对一段连续数据的引用：

```rust
let text = String::from("hello world");
let hello = &text[0..5];
```

运行 demo：

```powershell
cargo run --example 06_borrowing_slices
```

## 10. 结构体 struct

结构体用于描述一组相关数据：

```rust
struct User {
    name: String,
    age: u8,
}
```

给结构体实现方法：

```rust
impl User {
    fn is_adult(&self) -> bool {
        self.age >= 18
    }
}
```

`&self` 表示借用当前对象。

运行 demo：

```powershell
cargo run --example 07_structs
```

## 11. 枚举 enum 与 match

枚举表示一个值可能属于几种情况之一：

```rust
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
```

`match` 必须覆盖所有情况：

```rust
match direction {
    Direction::Up => println!("up"),
    Direction::Down => println!("down"),
    Direction::Left => println!("left"),
    Direction::Right => println!("right"),
}
```

枚举可以携带数据：

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

运行 demo：

```powershell
cargo run --example 08_enums_match
```

## 12. Option 与 Result

Rust 没有空指针，使用 `Option<T>` 表达“可能有值”：

```rust
let value: Option<i32> = Some(10);
let empty: Option<i32> = None;
```

使用 `Result<T, E>` 表达“可能成功，也可能失败”：

```rust
fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err(String::from("cannot divide by zero"))
    } else {
        Ok(a / b)
    }
}
```

运行 demo：

```powershell
cargo run --example 09_option_result
```

## 13. 常用集合

`Vec<T>` 是动态数组：

```rust
let mut numbers = Vec::new();
numbers.push(1);
```

`String` 是可增长字符串：

```rust
let mut text = String::from("hello");
text.push_str(" rust");
```

`HashMap<K, V>` 是键值对：

```rust
use std::collections::HashMap;
let mut scores = HashMap::new();
scores.insert(String::from("Alice"), 90);
```

运行 demo：

```powershell
cargo run --example 10_collections
```

## 14. 模块、包与 crate

模块用于组织代码：

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}
```

`pub` 表示公开。几个概念：package 是一个 Cargo 项目；crate 是编译单元；module 是代码组织单元。

运行 demo：

```powershell
cargo run --example 11_modules
```

## 15. 泛型与 特征（trait）

泛型让代码适配多种类型：

```rust
fn first<T>(items: &[T]) -> Option<&T> {
    items.first()
}
```

trait 表示一种能力或约定：

```rust
trait Summary {
    fn summary(&self) -> String;
}
```

运行 demo：

```powershell
cargo run --example 12_generics_traits
```

## 16. 生命周期

生命周期用于说明引用之间的有效范围关系。多数时候编译器会自动推断。

典型例子：

```rust
fn longest<'a>(a: &'a str, b: &'a str) -> &'a str {
    if a.len() >= b.len() { a } else { b }
}
```

`'a` 表示返回引用不会比输入引用活得更久。

运行 demo：

```powershell
cargo run --example 13_lifetimes
```

## 17. 闭包与迭代器

闭包是匿名函数：

```rust
let base = 10;
let add_base = |x| x + base;
```

迭代器可以链式处理集合：

```rust
let doubled: Vec<i32> = numbers.iter().map(|n| n * 2).collect();
```

常见方法：`map` 转换，`filter` 过滤，`fold` 累计，`collect` 收集结果。

运行 demo：

```powershell
cargo run --example 14_closures_iterators
```

## 18. 错误处理与文件读写

Rust 推荐显式处理错误：

```rust
use std::fs;

fn read_text(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}
```

`?` 可以把错误向上传递：

```rust
fn load_name(path: &str) -> Result<String, std::io::Error> {
    let text = fs::read_to_string(path)?;
    Ok(text.trim().to_string())
}
```

运行 demo：

```powershell
cargo run --example 15_errors_files
```

## 19. 测试

Rust 内置测试框架：

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_numbers() {
        assert_eq!(add(2, 3), 5);
    }
}
```

运行库测试：

```powershell
cargo test
```

运行断言 demo：

```powershell
cargo run --example 16_tests
```

## 20. 智能指针

常见智能指针：

- `Box<T>`：把值放在堆上。
- `Rc<T>`：单线程引用计数。
- `RefCell<T>`：运行时借用检查。
- `Arc<T>`：线程安全引用计数。
- `Mutex<T>`：互斥锁。

运行 demo：

```powershell
cargo run --example 17_smart_pointers
```

## 21. 并发

Rust 可以用线程执行并发任务：

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("hello from thread");
});

handle.join().unwrap();
```

多线程共享数据常用 `Arc<Mutex<T>>`。

运行 demo：

```powershell
cargo run --example 18_concurrency
```

## 22. 宏

宏是编译期展开的代码生成工具。你常用的这些都是宏：

```rust
println!("hello");
vec![1, 2, 3];
format!("name: {}", "Rust");
```

运行 demo：

```powershell
cargo run --example 19_macros
```

## 23. 小项目：命令行 Todo

Todo demo 会把这些知识串起来：

- `struct` 表示任务。
- `enum` 表示状态。
- `Vec` 保存任务。
- 方法实现行为。
- 迭代器筛选未完成任务。

运行：

```powershell
cargo run --example 20_project_todo
```

## 24. 推荐学习路线

1. 跑通环境和 `cargo run`。
2. 学变量、类型、函数、控制流。
3. 重点练所有权、借用、切片。
4. 学 struct、enum、match。
5. 学 Option、Result、集合。
6. 学模块、泛型、trait、生命周期。
7. 学错误处理、测试、文件读写。
8. 学智能指针、并发、宏。
9. 做一个命令行工具或小 Web 服务。

## 25. 常见问题

### 为什么 String 赋值后原变量不能用了？

因为发生了所有权移动，Rust 防止两个变量同时拥有同一块堆内存。

### 什么时候用 String，什么时候用 &str？

需要拥有、修改、保存字符串时用 `String`；只是读取一段字符串时用 `&str`。

### 为什么可变引用只能有一个？

为了避免多个地方同时修改同一份数据。Rust 在编译期阻止数据竞争。

### 什么时候用 unwrap？

学习阶段可以用。真实项目里优先用 `match`、`?` 或 `expect("清楚的错误信息")`。

## 26. 一次运行所有 demo

安装 Rust 后：

```powershell
cd F:\workspace\study\rust\rust_demos
cargo run --example 01_hello
cargo run --example 02_variables
cargo run --example 03_types
cargo run --example 04_functions_control_flow
cargo run --example 05_ownership
cargo run --example 06_borrowing_slices
cargo run --example 07_structs
cargo run --example 08_enums_match
cargo run --example 09_option_result
cargo run --example 10_collections
cargo run --example 11_modules
cargo run --example 12_generics_traits
cargo run --example 13_lifetimes
cargo run --example 14_closures_iterators
cargo run --example 15_errors_files
cargo run --example 16_tests
cargo run --example 17_smart_pointers
cargo run --example 18_concurrency
cargo run --example 19_macros
cargo run --example 20_project_todo
cargo test
```

Rust 的学习曲线前面会有一点陡，尤其是所有权和生命周期。别急，先多跑 demo，再尝试自己改几行，让编译器告诉你哪里不对，这是学习 Rust 最快的方法。
