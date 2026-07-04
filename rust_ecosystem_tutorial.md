# Rust 生态实战补充教程

这份教程补齐真实项目常用生态。这里会用第三方 crate，所以示例不是放进 `rust_demos` 的默认依赖中，避免当前项目因为没有联网下载依赖而无法编译。

推荐做法：每学一章，就新建一个独立 Cargo 项目练习。

## 1. 添加第三方依赖

常用方式：

```powershell
cargo add serde --features derive
cargo add serde_json
```

也可以手动编辑 `Cargo.toml`：

```toml
[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
```

## 2. serde：序列化和反序列化

用途：把 Rust 结构体和 JSON、YAML、TOML 等格式互相转换。

创建项目：

```powershell
cargo new serde_demo
cd serde_demo
cargo add serde --features derive
cargo add serde_json
```

`src/main.rs`：

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u64,
    name: String,
    active: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let user = User {
        id: 1,
        name: String::from("Alice"),
        active: true,
    };

    let json = serde_json::to_string_pretty(&user)?;
    println!("{json}");

    let decoded: User = serde_json::from_str(&json)?;
    println!("{decoded:?}");

    Ok(())
}
```

运行：

```powershell
cargo run
```

## 3. clap：命令行参数解析

用途：写专业 CLI 工具。

```powershell
cargo new clap_demo
cd clap_demo
cargo add clap --features derive
```

`src/main.rs`：

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "todo")]
#[command(about = "A tiny todo CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Add { title: String },
    List,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Add { title } => println!("add task: {title}"),
        Command::List => println!("list tasks"),
    }
}
```

运行：

```powershell
cargo run -- add "learn rust"
cargo run -- list
```

## 4. tracing：日志

用途：真实项目中记录运行状态、错误和请求链路。

```powershell
cargo new tracing_demo
cd tracing_demo
cargo add tracing
cargo add tracing-subscriber
```

`src/main.rs`：

```rust
use tracing::{error, info, warn};

fn main() {
    tracing_subscriber::fmt::init();

    info!("app started");
    warn!("this is a warning");
    error!("this is an error");
}
```

运行：

```powershell
cargo run
```

## 5. tokio：异步运行时

用途：异步网络、定时任务、并发 I/O。

```powershell
cargo new tokio_demo
cd tokio_demo
cargo add tokio --features full
```

`src/main.rs`：

```rust
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let a = tokio::spawn(async {
        sleep(Duration::from_millis(300)).await;
        "task a"
    });

    let b = tokio::spawn(async {
        sleep(Duration::from_millis(100)).await;
        "task b"
    });

    println!("{}", a.await.unwrap());
    println!("{}", b.await.unwrap());
}
```

运行：

```powershell
cargo run
```

## 6. reqwest：HTTP 客户端

用途：调用 HTTP API。

```powershell
cargo new reqwest_demo
cd reqwest_demo
cargo add tokio --features full
cargo add reqwest --features json
cargo add serde_json
```

`src/main.rs`：

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let value: serde_json::Value = reqwest::get("https://httpbin.org/json")
        .await?
        .json()
        .await?;

    println!("{value:#}");
    Ok(())
}
```

运行：

```powershell
cargo run
```

## 7. axum：Web 服务

用途：写 HTTP API。

```powershell
cargo new axum_demo
cd axum_demo
cargo add axum
cargo add tokio --features full
cargo add serde --features derive
```

`src/main.rs`：

```rust
use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;

#[derive(Serialize)]
struct Health {
    ok: bool,
}

async fn health() -> Json<Health> {
    Json(Health { ok: true })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/health", get(health));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("listening on http://{addr}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

运行：

```powershell
cargo run
```

访问：

```text
http://127.0.0.1:3000/health
```

## 8. sqlx：数据库访问

用途：异步访问 PostgreSQL、MySQL、SQLite。

SQLite 入门最省事：

```powershell
cargo new sqlx_demo
cd sqlx_demo
cargo add tokio --features full
cargo add sqlx --features runtime-tokio,sqlite
```

`src/main.rs`：

```rust
use sqlx::sqlite::SqlitePoolOptions;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;

    sqlx::query(
        "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)"
    )
    .execute(&pool)
    .await?;

    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind("Alice")
        .execute(&pool)
        .await?;

    let row: (String,) = sqlx::query_as("SELECT name FROM users WHERE id = 1")
        .fetch_one(&pool)
        .await?;

    println!("name = {}", row.0);
    Ok(())
}
```

## 9. anyhow 与 thiserror

`anyhow` 适合应用层快速处理错误；`thiserror` 适合库或领域层定义清晰错误类型。

```powershell
cargo add anyhow
cargo add thiserror
```

应用层：

```rust
use anyhow::{Context, Result};
use std::fs;

fn main() -> Result<()> {
    let text = fs::read_to_string("config.toml")
        .context("failed to read config.toml")?;
    println!("{text}");
    Ok(())
}
```

领域错误：

```rust
use thiserror::Error;

#[derive(Debug, Error)]
enum UserError {
    #[error("name cannot be empty")]
    EmptyName,
}
```

## 10. 常见项目结构

小型项目：

```text
src/
├── main.rs
├── config.rs
├── error.rs
├── model.rs
└── service.rs
```

中型项目：

```text
src/
├── main.rs
├── api/
├── domain/
├── infrastructure/
└── application/
```

workspace 项目：

```text
crates/
├── app/
├── core/
├── api/
└── storage/
```

## 11. 推荐实战路线

1. 用 `clap` 写一个 Todo CLI。
2. 用 `serde` 把 Todo 保存成 JSON 文件。
3. 用 `tracing` 加日志。
4. 用 `tokio` 做异步版本。
5. 用 `axum` 暴露 HTTP API。
6. 用 `sqlx` 保存到 SQLite。
7. 拆成 workspace：`core`、`cli`、`api`、`storage`。

走完这条线，你就从 Rust 语法学习进入真实工程开发了。
