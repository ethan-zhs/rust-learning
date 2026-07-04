# Rust 学习资料索引

当前目录已经整理成三份教程和一个配套 demo 项目。

## 1. 主教程

文件：

```text
rust_learning_tutorial.md
```

内容覆盖：

- Windows、macOS、Linux 环境搭建、Windows MSVC/VS Code 排障、Cargo、项目结构。
- 变量、类型、函数、控制流。
- 所有权、借用、引用、切片。
- struct、enum、match。
- Option、Result、集合。
- 模块、泛型、trait、生命周期。
- 闭包、迭代器、错误处理、测试。
- 智能指针、并发、宏。
- Todo 小项目。

## 2. 进阶教程

文件：

```text
rust_advanced_tutorial.md
```

内容覆盖：

- 模式匹配进阶。
- trait bound 与 where。
- 关联类型。
- Copy、Clone、Drop。
- 自定义迭代器。
- Box 与递归类型。
- channel 通道。
- From、Into、TryFrom。
- 自定义错误类型。
- 运算符重载。
- newtype 与类型别名。
- unsafe 入门。
- async/await 概念。
- workspace 与文档注释。

## 3. 生态实战教程

文件：

```text
rust_ecosystem_tutorial.md
```

内容覆盖：

- serde：序列化和反序列化。
- clap：命令行参数解析。
- tracing：日志。
- tokio：异步运行时。
- reqwest：HTTP 客户端。
- axum：Web 服务。
- sqlx：数据库访问。
- anyhow 与 thiserror：错误处理生态。
- 常见项目结构。


## 4. 多端打包教程

文件：

```text
rust_cross_platform_packaging_tutorial.md
```

内容覆盖：

- 用 Rust 写 docx 预览内核的跨平台架构。
- WASM / Web / Node.js 打包。
- iOS `.a` / `.xcframework` 打包。
- Android `.so` / `.aar` / JNI 打包。
- Windows `.dll`、macOS `.dylib` / `.xcframework`、Linux `.so` / CLI 打包。
- FFI 设计、C 头文件、Swift/Kotlin/C# 调用方式。
- 多端产物发布清单、CI 构建矩阵、性能和排版注意事项。

## 5. Demo 项目

目录：

```text
rust_demos
```

运行方式：

```powershell
cd F:\workspace\study\rust\rust_demos
cargo run --example 01_hello
```

进阶 demo 从 `21` 开始：

```powershell
cargo run --example 21_pattern_matching
cargo run --example 33_async_concept
```

运行测试：

```powershell
cargo test
```

## 6. 建议学习顺序

1. 先读 `rust_learning_tutorial.md`，跑 `01` 到 `20`。
2. 再读 `rust_advanced_tutorial.md`，跑 `21` 到 `33`。
3. 最后读 `rust_ecosystem_tutorial.md`，每章单独新建项目练真实依赖。
4. 把 Todo demo 改造成一个完整 CLI，再继续扩展成 Web API。
