# Rust 进阶知识补充教程

这份补充教程接在 `rust_learning_tutorial.md` 后面，覆盖更完整的 Rust 语言知识点。这里的 demo 仍然尽量使用标准库，安装 Rust 后可以直接在 `rust_demos` 项目中运行。

运行示例：

```powershell
cd F:\workspace\study\rust\rust_demos
cargo run --example 21_pattern_matching
```

## 1. 模式匹配进阶

Rust 的模式匹配不只用于 `match`，还会出现在 `let`、函数参数、`if let`、`while let`、`for` 中。

### 1.1 if let

当你只关心一种情况时，用 `if let` 比完整 `match` 更清爽。

```rust
let value = Some(3);

if let Some(n) = value {
    println!("{n}");
}
```

### 1.2 while let

适合不断取出某种值，直到模式不匹配。

```rust
let mut stack = vec![1, 2, 3];

while let Some(n) = stack.pop() {
    println!("{n}");
}
```

### 1.3 解构

```rust
let point = (3, 5);
let (x, y) = point;
```

结构体也可以解构：

```rust
struct User {
    name: String,
    age: u8,
}

let user = User { name: String::from("Alice"), age: 18 };
let User { name, age } = user;
```

运行 demo：

```powershell
cargo run --example 21_pattern_matching
```

## 2. trait bound 与 where

泛型函数经常需要限制类型必须具备某种能力。

```rust
use std::fmt::Display;

fn print_twice<T: Display>(value: T) {
    println!("{value}");
    println!("{value}");
}
```

当约束很多时，用 `where` 更清楚：

```rust
fn compare_and_print<T, U>(a: T, b: U)
where
    T: Display + PartialOrd,
    U: Display,
{
    println!("{a}, {b}");
}
```

运行 demo：

```powershell
cargo run --example 22_trait_bounds_where
```

## 3. 关联类型

trait 可以定义关联类型。迭代器的 `Item` 就是最常见的关联类型。

```rust
trait Counter {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

关联类型适合表达“实现这个 trait 时，会确定一个相关类型”。

运行 demo：

```powershell
cargo run --example 23_associated_types
```

## 4. Copy、Clone 与 Drop

`Copy` 表示按位复制，赋值后原变量还能用。常见的数字、布尔、字符都是 `Copy`。

```rust
let a = 1;
let b = a;
println!("{a}, {b}");
```

`Clone` 表示显式复制，常用于堆数据：

```rust
let a = String::from("hello");
let b = a.clone();
```

`Drop` 表示值离开作用域时自动执行清理逻辑。文件句柄、锁、网络连接都依赖这个机制。

运行 demo：

```powershell
cargo run --example 24_copy_clone_drop
```

## 5. 自定义迭代器

实现 `Iterator` trait，就能使用 `map`、`filter`、`collect` 等能力。

```rust
struct Counter {
    current: u32,
    max: u32,
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        if self.current <= self.max {
            Some(self.current)
        } else {
            None
        }
    }
}
```

运行 demo：

```powershell
cargo run --example 25_custom_iterator
```

## 6. Box 与递归类型

Rust 需要在编译期知道类型大小。递归类型如果直接包含自己，大小会无限递归。

错误思路：

```rust
// enum List {
//     Cons(i32, List),
//     Nil,
// }
```

正确做法是用 `Box` 打断递归：

```rust
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

运行 demo：

```powershell
cargo run --example 26_box_recursive
```

## 7. 通道 channel

线程之间可以用 channel 发送消息。

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

thread::spawn(move || {
    tx.send(String::from("hello")).unwrap();
});

println!("{}", rx.recv().unwrap());
```

运行 demo：

```powershell
cargo run --example 27_channels
```

## 8. From、Into 与 TryFrom

Rust 推荐用 trait 表达类型转换。

`From` 表示一定能成功的转换：

```rust
impl From<&str> for UserName {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}
```

`TryFrom` 表示可能失败的转换：

```rust
impl TryFrom<i32> for Age {
    type Error = String;
}
```

运行 demo：

```powershell
cargo run --example 28_conversions
```

## 9. 自定义错误类型

真实项目里，错误不应该永远用 `String`。可以自定义错误枚举，并实现 `Display` 和 `Error`。

```rust
#[derive(Debug)]
enum AppError {
    EmptyName,
    InvalidAge,
}
```

运行 demo：

```powershell
cargo run --example 29_custom_error
```

## 10. 运算符重载

Rust 可以通过实现 trait 来支持 `+`、`-` 等运算符。

```rust
use std::ops::Add;

impl Add for Point {
    type Output = Point;
}
```

运行 demo：

```powershell
cargo run --example 30_operator_overload
```

## 11. newtype 与类型别名

类型别名只是换名字：

```rust
type UserId = u64;
```

newtype 是包一层新类型：

```rust
struct UserId(u64);
```

newtype 更安全，因为 `UserId` 和 `OrderId` 即使内部都是 `u64`，也不会被混用。

运行 demo：

```powershell
cargo run --example 31_newtype
```

## 12. unsafe 入门

`unsafe` 不代表代码一定危险，而是表示有些规则需要程序员自己保证。

常见能力：

- 解引用裸指针。
- 调用 unsafe 函数。
- 访问或修改可变静态变量。
- 实现 unsafe trait。
- 访问 union 字段。

初学阶段原则：尽量不用 `unsafe`。如果必须用，把范围控制到最小，并写清楚安全前提。

运行 demo：

```powershell
cargo run --example 32_unsafe_intro
```

## 13. async/await 概念

`async fn` 返回的是 Future。Future 本身是惰性的，只有被执行器轮询时才会推进。

真实项目中通常使用 Tokio：

```rust
#[tokio::main]
async fn main() {
    println!("hello async");
}
```

标准库没有通用异步运行时。本教程提供一个极简 `block_on`，只用于理解概念，不建议在真实项目里使用。

运行 demo：

```powershell
cargo run --example 33_async_concept
```

## 14. 工作区 workspace

大型项目可以拆成多个 crate，用 workspace 管理。

```toml
[workspace]
members = [
    "crates/api",
    "crates/core",
    "crates/cli",
]
```

常见拆分：

- `core`：业务核心逻辑。
- `api`：Web 服务。
- `cli`：命令行入口。
- `storage`：数据库访问。

## 15. 文档注释

`///` 用于生成 API 文档。

```rust
/// Adds two numbers.
///
/// # Examples
///
/// ```
/// assert_eq!(my_crate::add(2, 3), 5);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

生成文档：

```powershell
cargo doc --open
```

## 16. 完整学习顺序

建议把 Rust 分成四层学：

1. 基础语法层：变量、类型、函数、控制流、集合。
2. 内存模型层：所有权、借用、生命周期、智能指针。
3. 抽象能力层：struct、enum、trait、泛型、迭代器、错误处理。
4. 工程生态层：Cargo、测试、文档、异步、Web、数据库、CLI、日志。

如果能把这四层串起来，你就不只是“会写 Rust 语法”，而是能开始写像样的 Rust 项目。

## 17. 运行本补充教程 demo

```powershell
cd F:\workspace\study\rust\rust_demos
cargo run --example 21_pattern_matching
cargo run --example 22_trait_bounds_where
cargo run --example 23_associated_types
cargo run --example 24_copy_clone_drop
cargo run --example 25_custom_iterator
cargo run --example 26_box_recursive
cargo run --example 27_channels
cargo run --example 28_conversions
cargo run --example 29_custom_error
cargo run --example 30_operator_overload
cargo run --example 31_newtype
cargo run --example 32_unsafe_intro
cargo run --example 33_async_concept
```
