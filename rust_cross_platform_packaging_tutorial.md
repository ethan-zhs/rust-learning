# Rust 多端打包教程：把 docx 预览内核发布到 WASM、iOS、Android、Windows、macOS、Linux

这份教程讲一个非常实际的场景：你用 Rust 写一个 `docx` 预览内核，希望同一套核心能力能被 Web、iOS、Android、Windows、macOS、Linux 使用。

目标不是把 Rust 代码直接塞进每个平台 UI，而是把 Rust 作为“跨平台内核”：

```text
DOCX 文件 bytes
      |
      v
Rust docx preview core
      |
      v
统一预览模型 / JSON / 图片 / 布局指令
      |
      +--> Web / WASM
      +--> iOS / Swift
      +--> Android / Kotlin / Java
      +--> Windows / C# / C++ / Electron / Tauri
      +--> macOS / Swift / Objective-C / C++ / Tauri
      +--> Linux / C / C++ / Python / Electron / Tauri
```

## 1. 先明确“多端产物”是什么

Rust 可以产出很多东西，但不同平台真正需要的产物不同。

| 平台 | 推荐产物 | 常见调用方 | 说明 |
|---|---|---|---|
| Web | `.wasm` + JS/TS 胶水代码 | TypeScript / JavaScript | 用 `wasm-bindgen` / `wasm-pack` |
| Node.js | `.wasm` + JS 包 | Node.js | 适合服务端预览、批处理 |
| iOS | `.a` / `.xcframework` + Swift bindings | Swift / Objective-C | 推荐 `staticlib` + `XCFramework`，或 UniFFI |
| Android | `.so` + JNI/Kotlin bindings，或 `.aar` | Kotlin / Java | 推荐 `cdylib` + `cargo-ndk` |
| Windows | `.dll` + `.lib` + `.h`，或 `.exe` | C# / C++ / Electron / Tauri | 库用 `cdylib`，命令行工具用 `bin` |
| macOS | `.dylib` / `.a` / `.framework` / `.xcframework` | Swift / C++ / Tauri | App 内嵌常用 `.a` 或 `.xcframework` |
| Linux | `.so` / `.a` / `.deb` / `.rpm` / `.AppImage` | C / C++ / Python / Electron | 服务器也可直接发布 CLI 二进制 |

简单记：

- 给 Web：产物是 `wasm` 包。
- 给移动端 App：产物是原生库 + 语言绑定。
- 给桌面 App：产物可以是动态库、静态库、CLI、或者被 Tauri/Electron 调用的本地模块。
- 给服务端：通常直接发布 Linux 可执行文件或动态库。

## 2. 推荐项目结构

不要把所有平台代码都堆在一个 crate 里。推荐 workspace 分层：

```text
docx-preview/
├── Cargo.toml
├── crates/
│   ├── docx_preview_core/      # 纯 Rust 核心逻辑
│   ├── docx_preview_ffi/       # C ABI / Android / iOS / 桌面动态库
│   ├── docx_preview_wasm/      # WebAssembly 导出
│   └── docx_preview_cli/       # 命令行调试工具
├── bindings/
│   ├── c/                      # .h 头文件
│   ├── swift/                  # Swift 封装
│   ├── kotlin/                 # Kotlin/JNI 封装
│   └── typescript/             # TS 类型和使用示例
├── dist/
│   ├── wasm/
│   ├── ios/
│   ├── android/
│   ├── windows/
│   ├── macos/
│   └── linux/
└── scripts/
    ├── build-wasm.ps1
    ├── build-android.ps1
    ├── build-apple.sh
    └── build-desktop.ps1
```

根 `Cargo.toml`：

```toml
[workspace]
members = [
    "crates/docx_preview_core",
    "crates/docx_preview_ffi",
    "crates/docx_preview_wasm",
    "crates/docx_preview_cli",
]
resolver = "2"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = "symbols"

[profile.release.package.docx_preview_wasm]
opt-level = "z"
```

说明：

- `docx_preview_core` 不知道 Web、iOS、Android 的存在。
- `docx_preview_ffi` 只负责把核心能力导出成稳定 ABI。
- `docx_preview_wasm` 只负责 Web 侧友好 API。
- `docx_preview_cli` 用于本地调试、CI 快速验证和服务端批处理。

## 3. docx 预览内核应该怎么设计

`docx` 本质上是一个 zip 包，里面是 XML、图片、样式、关系文件等。预览内核建议分成四层：

```text
Input Layer
  - 读取 docx bytes
  - 解 zip
  - 定位 document.xml、styles.xml、rels、media

Parse Layer
  - 解析段落、run、表格、图片、页眉页脚、脚注
  - 解析样式、编号、超链接、页面设置

Layout Layer
  - 生成页面模型
  - 计算段落、行、表格、图片位置
  - 处理分页、边距、字体、缩放

Output Layer
  - 输出 JSON 预览模型
  - 输出 HTML
  - 输出图片/矢量绘制指令
  - 输出平台 UI 可消费的数据
```

跨端最稳的输出不是“直接渲染 UI”，而是输出中间模型：

```json
{
  "pages": [
    {
      "width": 794,
      "height": 1123,
      "blocks": [
        {
          "type": "paragraph",
          "x": 72,
          "y": 96,
          "runs": [
            { "text": "Hello", "font_size": 16, "bold": true }
          ]
        }
      ]
    }
  ]
}
```

这样每个平台只做两件事：

1. 把 docx bytes 传给 Rust 内核。
2. 根据 Rust 返回的预览模型绘制 UI。

## 4. 核心 crate：保持纯 Rust

`crates/docx_preview_core/Cargo.toml`：

```toml
[package]
name = "docx_preview_core"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1", features = ["derive"] }
serde_json = "1"
zip = "2"
quick-xml = "0.36"
thiserror = "1"
```

核心 API 示例：

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewDocument {
    pub pages: Vec<PreviewPage>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PreviewPage {
    pub width: f32,
    pub height: f32,
    pub blocks: Vec<PreviewBlock>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PreviewBlock {
    Paragraph { x: f32, y: f32, text: String },
    Image { x: f32, y: f32, width: f32, height: f32, id: String },
}

#[derive(Debug, thiserror::Error)]
pub enum PreviewError {
    #[error("invalid docx zip")]
    InvalidZip,
    #[error("missing main document xml")]
    MissingDocumentXml,
    #[error("parse error: {0}")]
    Parse(String),
}

pub fn render_docx_to_model(bytes: &[u8]) -> Result<PreviewDocument, PreviewError> {
    // 真实项目里这里会解 zip、读 XML、解析样式、计算布局。
    Ok(PreviewDocument {
        pages: vec![PreviewPage {
            width: 794.0,
            height: 1123.0,
            blocks: vec![PreviewBlock::Paragraph {
                x: 72.0,
                y: 96.0,
                text: format!("docx bytes: {}", bytes.len()),
            }],
        }],
    })
}

pub fn render_docx_to_json(bytes: &[u8]) -> Result<String, PreviewError> {
    let model = render_docx_to_model(bytes)?;
    serde_json::to_string(&model).map_err(|error| PreviewError::Parse(error.to_string()))
}
```

核心层注意事项：

- 不依赖 iOS、Android、浏览器 API。
- 不直接读文件路径，优先接收 `&[u8]`，这样 Web、移动端、桌面都方便。
- 不直接返回复杂 Rust 类型给外部语言，跨边界时用 JSON、句柄、字节 buffer。
- 错误用内部 enum，导出层再转成平台友好的错误码或异常。

## 5. FFI crate：给 iOS、Android、桌面用

`crates/docx_preview_ffi/Cargo.toml`：

```toml
[package]
name = "docx_preview_ffi"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "staticlib"]

[dependencies]
docx_preview_core = { path = "../docx_preview_core" }
```

`crate-type` 的含义：

- `cdylib`：生成给其他语言加载的动态库，Windows 是 `.dll`，macOS 是 `.dylib`，Linux/Android 是 `.so`。
- `staticlib`：生成静态系统库，Linux/macOS 通常是 `.a`，Windows MSVC 通常是 `.lib`。
- `rlib`：给 Rust 编译器用，不适合作为跨语言发布产物。

FFI 导出示例：

```rust
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::ptr;

#[repr(C)]
pub struct PreviewBuffer {
    pub ptr: *mut u8,
    pub len: usize,
    pub cap: usize,
}

impl PreviewBuffer {
    fn empty() -> Self {
        Self {
            ptr: ptr::null_mut(),
            len: 0,
            cap: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn docx_preview_render_json(
    input_ptr: *const u8,
    input_len: usize,
    out: *mut PreviewBuffer,
) -> i32 {
    if input_ptr.is_null() || out.is_null() {
        return -1;
    }

    let result = catch_unwind(AssertUnwindSafe(|| {
        let input = unsafe { std::slice::from_raw_parts(input_ptr, input_len) };
        docx_preview_core::render_docx_to_json(input)
    }));

    match result {
        Ok(Ok(json)) => {
            let mut bytes = json.into_bytes();
            let buffer = PreviewBuffer {
                ptr: bytes.as_mut_ptr(),
                len: bytes.len(),
                cap: bytes.capacity(),
            };
            std::mem::forget(bytes);
            unsafe { *out = buffer; }
            0
        }
        Ok(Err(_error)) => {
            unsafe { *out = PreviewBuffer::empty(); }
            -2
        }
        Err(_panic) => {
            unsafe { *out = PreviewBuffer::empty(); }
            -3
        }
    }
}

#[no_mangle]
pub extern "C" fn docx_preview_free_buffer(buffer: PreviewBuffer) {
    if buffer.ptr.is_null() {
        return;
    }

    unsafe {
        let _ = Vec::from_raw_parts(buffer.ptr, buffer.len, buffer.cap);
    }
}
```

FFI 规则很重要：

- Rust panic 不能跨 FFI 边界传出去，必须 `catch_unwind` 或设置 `panic = "abort"`。
- 不要让外部语言直接释放 Rust 分配的内存，提供 `free` 函数。
- 不要跨 FFI 传 `String`、`Vec<T>`、Rust enum、trait object。
- 用 `#[repr(C)]` 固定结构体布局。
- 导出函数用 `extern "C"` 和 `#[no_mangle]`。
- 外部传入的 pointer 都是不可信的，必须做空指针和长度检查。

## 6. 生成 C 头文件

推荐用 `cbindgen` 生成 `.h`：

```bash
cargo install cbindgen
cbindgen crates/docx_preview_ffi --lang c --output bindings/c/docx_preview.h
```

期望头文件大概长这样：

```c
#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct PreviewBuffer {
  uint8_t *ptr;
  uintptr_t len;
  uintptr_t cap;
} PreviewBuffer;

int32_t docx_preview_render_json(const uint8_t *input_ptr,
                                 uintptr_t input_len,
                                 struct PreviewBuffer *out);

void docx_preview_free_buffer(struct PreviewBuffer buffer);
```

有了 `.h`，C、C++、Objective-C、Swift bridging、C# P/Invoke 都能接。

## 7. WASM 打包

Web 不走 C ABI，推荐单独建 `docx_preview_wasm`。

`crates/docx_preview_wasm/Cargo.toml`：

```toml
[package]
name = "docx_preview_wasm"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib", "rlib"]

[dependencies]
docx_preview_core = { path = "../docx_preview_core" }
wasm-bindgen = "0.2"
console_error_panic_hook = "0.1"
```

`src/lib.rs`：

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn render_docx_json(bytes: &[u8]) -> Result<String, JsValue> {
    docx_preview_core::render_docx_to_json(bytes)
        .map_err(|error| JsValue::from_str(&error.to_string()))
}
```

安装工具：

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
```

构建浏览器包：

```bash
wasm-pack build crates/docx_preview_wasm --target web --release --out-dir ../../dist/wasm/web
```

构建 bundler 包：

```bash
wasm-pack build crates/docx_preview_wasm --target bundler --release --out-dir ../../dist/wasm/bundler
```

构建 Node.js 包：

```bash
wasm-pack build crates/docx_preview_wasm --target nodejs --release --out-dir ../../dist/wasm/nodejs
```

典型产物：

```text
dist/wasm/web/
├── docx_preview_wasm_bg.wasm
├── docx_preview_wasm.js
├── docx_preview_wasm.d.ts
├── package.json
└── README.md
```

Web 使用示例：

```ts
import init, { render_docx_json } from "./dist/wasm/web/docx_preview_wasm.js";

await init();

const file = await fetch("/demo.docx").then((res) => res.arrayBuffer());
const json = render_docx_json(new Uint8Array(file));
const model = JSON.parse(json);
console.log(model.pages.length);
```

WASM 注意事项：

- 浏览器中不要直接读本地路径，接收 `Uint8Array`。
- 大 docx 会占内存，必要时做分页解析或 worker 后台解析。
- 预览计算建议放 Web Worker，避免阻塞 UI。
- 如果输出很大，JSON 简单但有开销；后续可升级为二进制格式，比如 MessagePack、FlatBuffers。

## 8. iOS 打包

iOS 推荐产物：

```text
DocxPreview.xcframework
├── ios-arm64/libdocx_preview_ffi.a
├── ios-arm64-simulator/libdocx_preview_ffi.a
└── Headers/docx_preview.h
```

安装 Rust target：

```bash
rustup target add aarch64-apple-ios
rustup target add aarch64-apple-ios-sim
```

如果你还要支持老 Intel Mac 上的 iOS Simulator：

```bash
rustup target add x86_64-apple-ios
```

构建真机：

```bash
cargo build -p docx_preview_ffi --release --target aarch64-apple-ios
```

构建 Apple Silicon 模拟器：

```bash
cargo build -p docx_preview_ffi --release --target aarch64-apple-ios-sim
```

构建 Intel 模拟器：

```bash
cargo build -p docx_preview_ffi --release --target x86_64-apple-ios
```

生成 `.xcframework`：

```bash
mkdir -p dist/ios/headers
cp bindings/c/docx_preview.h dist/ios/headers/

xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libdocx_preview_ffi.a \
  -headers dist/ios/headers \
  -library target/aarch64-apple-ios-sim/release/libdocx_preview_ffi.a \
  -headers dist/ios/headers \
  -output dist/ios/DocxPreview.xcframework
```

如果你同时构建了 `x86_64-apple-ios`，可以先把两个模拟器库合并成 fat lib：

```bash
lipo -create \
  target/aarch64-apple-ios-sim/release/libdocx_preview_ffi.a \
  target/x86_64-apple-ios/release/libdocx_preview_ffi.a \
  -output dist/ios/libdocx_preview_ffi_sim.a
```

然后用 `libdocx_preview_ffi_sim.a` 创建 simulator slice。

Swift 调用方式有两条路。

路线 A：C header + Swift wrapper。

```swift
import Foundation

public final class DocxPreview {
    public static func render(data: Data) throws -> String {
        var out = PreviewBuffer(ptr: nil, len: 0, cap: 0)

        let code: Int32 = data.withUnsafeBytes { rawBuffer in
            let ptr = rawBuffer.bindMemory(to: UInt8.self).baseAddress
            return docx_preview_render_json(ptr, data.count, &out)
        }

        guard code == 0, let ptr = out.ptr else {
            throw NSError(domain: "DocxPreview", code: Int(code))
        }

        let bytes = UnsafeBufferPointer(start: ptr, count: out.len)
        let json = String(decoding: bytes, as: UTF8.self)
        docx_preview_free_buffer(out)
        return json
    }
}
```

路线 B：UniFFI。

UniFFI 可以自动生成 Swift/Kotlin/Python/Ruby 等绑定，适合业务接口较多的库。缺点是打包流程更复杂，仍然要自己把 native library 放到 iOS/Android 项目中。

iOS 注意事项：

- App Store 上架通常更偏好静态库或 framework，不建议随便动态加载外部 `.dylib`。
- iOS 真机和模拟器 target 不一样，必须分开构建。
- Swift 侧不要忘记调用 Rust 提供的 free 函数。
- 如果预览内核较重，Swift 侧应放到后台线程运行。

## 9. Android 打包

Android 推荐产物：

```text
app/src/main/jniLibs/
├── arm64-v8a/libdocx_preview_ffi.so
├── armeabi-v7a/libdocx_preview_ffi.so
├── x86/libdocx_preview_ffi.so
└── x86_64/libdocx_preview_ffi.so
```

现代 Android 至少建议发布：

- `arm64-v8a`：绝大多数现代真机。
- `armeabi-v7a`：老 32 位 ARM 设备，如果你还要支持。
- `x86_64`：模拟器。
- `x86`：老模拟器或特殊设备，很多项目可不支持。

Rust target 对应关系：

| Android ABI | Rust target |
|---|---|
| `arm64-v8a` | `aarch64-linux-android` |
| `armeabi-v7a` | `armv7-linux-androideabi` |
| `x86_64` | `x86_64-linux-android` |
| `x86` | `i686-linux-android` |

安装 targets：

```bash
rustup target add aarch64-linux-android
rustup target add armv7-linux-androideabi
rustup target add x86_64-linux-android
rustup target add i686-linux-android
```

安装 Android Studio 和 NDK 后，安装 `cargo-ndk`：

```bash
cargo install cargo-ndk
```

构建：

```bash
cargo ndk \
  -t arm64-v8a \
  -t armeabi-v7a \
  -t x86_64 \
  -o dist/android/jniLibs \
  build -p docx_preview_ffi --release
```

输出：

```text
dist/android/jniLibs/
├── arm64-v8a/libdocx_preview_ffi.so
├── armeabi-v7a/libdocx_preview_ffi.so
└── x86_64/libdocx_preview_ffi.so
```

Kotlin JNI 示例：

```kotlin
package com.example.docxpreview

class DocxPreviewNative {
    companion object {
        init {
            System.loadLibrary("docx_preview_ffi")
        }
    }

    external fun renderDocxJson(bytes: ByteArray): String
}
```

如果你手写 JNI，Rust 侧需要导出 JNI 命名函数，例如：

```rust
use jni::objects::{JByteArray, JClass, JString};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_com_example_docxpreview_DocxPreviewNative_renderDocxJson(
    mut env: JNIEnv,
    _class: JClass,
    input: JByteArray,
) -> JString {
    let bytes = env.convert_byte_array(input).unwrap_or_default();
    let json = docx_preview_core::render_docx_to_json(&bytes)
        .unwrap_or_else(|error| format!("{{\"error\":\"{}\"}}", error));
    env.new_string(json).expect("create Java string")
}
```

Android 打包成 AAR 的结构：

```text
docx-preview-android/
├── build.gradle.kts
└── src/main/
    ├── java/com/example/docxpreview/DocxPreviewNative.kt
    └── jniLibs/
        ├── arm64-v8a/libdocx_preview_ffi.so
        ├── armeabi-v7a/libdocx_preview_ffi.so
        └── x86_64/libdocx_preview_ffi.so
```

Android 注意事项：

- 最终给 Android app 最方便的是 `.aar`，里面包含 Kotlin/Java API 和 `jniLibs`。
- 如果只给 `.so`，调用方还要自己写 JNI/Kotlin 封装。
- 大文件预览不要在主线程做，放到 `Dispatchers.Default` 或后台线程。
- JNI 层尽量薄，复杂逻辑全部放 Rust core。

## 10. Windows 打包

Windows 常见产物：

```text
docx_preview_ffi.dll
 docx_preview_ffi.dll.lib
 docx_preview.h
```

安装 target：

```powershell
rustup target add x86_64-pc-windows-msvc
rustup target add aarch64-pc-windows-msvc
```

构建 x64：

```powershell
cargo build -p docx_preview_ffi --release --target x86_64-pc-windows-msvc
```

构建 ARM64：

```powershell
cargo build -p docx_preview_ffi --release --target aarch64-pc-windows-msvc
```

输出位置：

```text
target/x86_64-pc-windows-msvc/release/docx_preview_ffi.dll
target/x86_64-pc-windows-msvc/release/docx_preview_ffi.dll.lib
```

C# P/Invoke 示例：

```csharp
using System;
using System.Runtime.InteropServices;
using System.Text;

[StructLayout(LayoutKind.Sequential)]
public struct PreviewBuffer
{
    public IntPtr ptr;
    public UIntPtr len;
    public UIntPtr cap;
}

public static class DocxPreviewNative
{
    [DllImport("docx_preview_ffi", CallingConvention = CallingConvention.Cdecl)]
    private static extern int docx_preview_render_json(
        byte[] input,
        UIntPtr inputLen,
        out PreviewBuffer output
    );

    [DllImport("docx_preview_ffi", CallingConvention = CallingConvention.Cdecl)]
    private static extern void docx_preview_free_buffer(PreviewBuffer buffer);

    public static string Render(byte[] docxBytes)
    {
        int code = docx_preview_render_json(docxBytes, (UIntPtr)docxBytes.Length, out var output);
        if (code != 0) throw new Exception($"Rust error: {code}");

        byte[] jsonBytes = new byte[(int)output.len];
        Marshal.Copy(output.ptr, jsonBytes, 0, jsonBytes.Length);
        docx_preview_free_buffer(output);
        return Encoding.UTF8.GetString(jsonBytes);
    }
}
```

Windows 注意事项：

- MSVC target 通常更适合 Windows 桌面生态。
- `.dll` 要放在 `.exe` 同目录，或放入 PATH 可搜索目录。
- 如果给 C++，同时给 `.h` 和 `.dll.lib`。
- 如果给 C#，用 P/Invoke；如果给 Electron，可通过 Node native addon 或本地服务进程调用。

## 11. macOS 打包

macOS 目标：

```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

构建 Intel：

```bash
cargo build -p docx_preview_ffi --release --target x86_64-apple-darwin
```

构建 Apple Silicon：

```bash
cargo build -p docx_preview_ffi --release --target aarch64-apple-darwin
```

动态库产物：

```text
target/x86_64-apple-darwin/release/libdocx_preview_ffi.dylib
target/aarch64-apple-darwin/release/libdocx_preview_ffi.dylib
```

静态库产物：

```text
target/x86_64-apple-darwin/release/libdocx_preview_ffi.a
target/aarch64-apple-darwin/release/libdocx_preview_ffi.a
```

合并 universal 静态库：

```bash
lipo -create \
  target/x86_64-apple-darwin/release/libdocx_preview_ffi.a \
  target/aarch64-apple-darwin/release/libdocx_preview_ffi.a \
  -output dist/macos/libdocx_preview_ffi_universal.a
```

macOS 也可以创建 `.xcframework`，同时给 macOS 和 iOS 使用：

```bash
xcodebuild -create-xcframework \
  -library dist/macos/libdocx_preview_ffi_universal.a \
  -headers bindings/c \
  -library target/aarch64-apple-ios/release/libdocx_preview_ffi.a \
  -headers bindings/c \
  -output dist/apple/DocxPreview.xcframework
```

macOS 注意事项：

- 发布给 Swift App，`.xcframework` 体验最好。
- 发布给 C/C++，`.dylib` + `.h` 或 `.a` + `.h` 即可。
- 上架或分发 macOS App 时，还会涉及签名、公证、rpath，这属于 App 打包层，不是 Rust 编译层。

## 12. Linux 打包

Linux 常见 target：

```bash
rustup target add x86_64-unknown-linux-gnu
rustup target add x86_64-unknown-linux-musl
rustup target add aarch64-unknown-linux-gnu
```

GNU 动态链接构建：

```bash
cargo build -p docx_preview_ffi --release --target x86_64-unknown-linux-gnu
```

产物：

```text
target/x86_64-unknown-linux-gnu/release/libdocx_preview_ffi.so
```

musl 静态链接 CLI：

```bash
cargo build -p docx_preview_cli --release --target x86_64-unknown-linux-musl
```

产物：

```text
target/x86_64-unknown-linux-musl/release/docx_preview_cli
```

Linux 发布方式：

- 给服务端：发布单个 CLI 二进制最方便。
- 给桌面 App：发布 `.so`，放到应用目录并配置 `LD_LIBRARY_PATH` 或 rpath。
- 给系统安装：打 `.deb`、`.rpm`。
- 给通用桌面用户：考虑 AppImage、Flatpak。

Linux 注意事项：

- `gnu` 目标依赖系统 glibc 版本，老系统可能跑不了新系统编出来的二进制。
- `musl` 更适合静态链接 CLI，但不是所有依赖都对 musl 友好。
- 如果依赖 OpenSSL，跨 Linux 发行版会更麻烦；能用 Rustls 时更省心。

## 13. 桌面 App 集成方式

### 13.1 Tauri

Tauri 本身就是 Rust 生态，最简单方式是让 Tauri 后端直接依赖 `docx_preview_core`：

```toml
[dependencies]
docx_preview_core = { path = "../../crates/docx_preview_core" }
```

Tauri command：

```rust
#[tauri::command]
fn render_docx(bytes: Vec<u8>) -> Result<String, String> {
    docx_preview_core::render_docx_to_json(&bytes).map_err(|error| error.to_string())
}
```

这种方式不需要 FFI，是桌面端最舒服的 Rust-to-Rust 集成。

### 13.2 Electron

Electron 有三种方式：

1. Web 侧用 WASM 包。
2. Node 主进程加载 native addon。
3. Electron 调用本地 CLI 或本地服务进程。

如果你的 docx 内核计算较重，推荐：

- 小文件：WASM。
- 大文件：Node native / CLI 子进程，避免阻塞渲染进程。

### 13.3 C++ / Qt

用 `.dll` / `.dylib` / `.so` + `.h`：

```cpp
PreviewBuffer out;
int code = docx_preview_render_json(bytes.data(), bytes.size(), &out);
if (code == 0) {
    std::string json(reinterpret_cast<char*>(out.ptr), out.len);
    docx_preview_free_buffer(out);
}
```

## 14. CLI 调试工具

`docx_preview_cli` 很重要。它让你不用打开 iOS/Android/Web 项目，也能快速验证核心能力。

`crates/docx_preview_cli/Cargo.toml`：

```toml
[package]
name = "docx_preview_cli"
version = "0.1.0"
edition = "2021"

[dependencies]
docx_preview_core = { path = "../docx_preview_core" }
clap = { version = "4", features = ["derive"] }
```

`src/main.rs`：

```rust
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    input: String,
    #[arg(short, long)]
    output: Option<String>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let bytes = fs::read(&args.input)?;
    let json = docx_preview_core::render_docx_to_json(&bytes)?;

    if let Some(output) = args.output {
        fs::write(output, json)?;
    } else {
        println!("{json}");
    }

    Ok(())
}
```

运行：

```bash
cargo run -p docx_preview_cli -- demo.docx -o preview.json
```

## 15. CI 构建矩阵

建议 CI 至少做这些检查：

```text
Core tests:
  cargo test -p docx_preview_core

WASM:
  wasm-pack build crates/docx_preview_wasm --target web --release

Android:
  cargo ndk -t arm64-v8a -t x86_64 -o dist/android/jniLibs build -p docx_preview_ffi --release

iOS:
  cargo build -p docx_preview_ffi --release --target aarch64-apple-ios
  cargo build -p docx_preview_ffi --release --target aarch64-apple-ios-sim

Windows:
  cargo build -p docx_preview_ffi --release --target x86_64-pc-windows-msvc

macOS:
  cargo build -p docx_preview_ffi --release --target aarch64-apple-darwin
  cargo build -p docx_preview_ffi --release --target x86_64-apple-darwin

Linux:
  cargo build -p docx_preview_ffi --release --target x86_64-unknown-linux-gnu
```

GitHub Actions 大致拆分：

- `ubuntu-latest`：Linux、WASM、Android。
- `macos-latest`：macOS、iOS、XCFramework。
- `windows-latest`：Windows DLL。

注意：iOS 构建必须在 macOS runner 上做。

## 16. 产物发布清单

一次完整发布可以长这样：

```text
dist/
├── wasm/
│   ├── web/
│   │   ├── docx_preview_wasm_bg.wasm
│   │   ├── docx_preview_wasm.js
│   │   └── docx_preview_wasm.d.ts
│   ├── nodejs/
│   └── bundler/
├── ios/
│   └── DocxPreview.xcframework
├── android/
│   ├── docx-preview.aar
│   └── jniLibs/
│       ├── arm64-v8a/libdocx_preview_ffi.so
│       └── x86_64/libdocx_preview_ffi.so
├── windows/
│   └── x86_64/
│       ├── docx_preview_ffi.dll
│       ├── docx_preview_ffi.dll.lib
│       └── docx_preview.h
├── macos/
│   ├── DocxPreview.xcframework
│   └── libdocx_preview_ffi_universal.a
└── linux/
    └── x86_64/
        ├── libdocx_preview_ffi.so
        └── docx_preview_cli
```

## 17. 版本管理

建议版本号统一：

```text
docx-preview-core      0.1.0
docx-preview-wasm      0.1.0
docx-preview-ios       0.1.0
docx-preview-android   0.1.0
docx-preview-desktop   0.1.0
```

对外 API 要稳定，尤其是：

- JSON 预览模型字段。
- FFI 函数名和参数。
- 错误码。
- WASM TypeScript 类型。
- Swift/Kotlin 包名。

推荐语义化版本：

- 修 bug：`0.1.1`
- 增加兼容字段：`0.2.0`
- 删除字段或改 FFI 签名：`1.0.0` 之后必须升 major。

## 18. 错误码设计

FFI 不适合直接抛异常。可以设计统一错误码：

```text
0      success
-1     invalid argument
-2     parse failed
-3     panic caught
-4     unsupported feature
-5     out of memory
```

如果需要详细错误，提供额外函数：

```rust
#[no_mangle]
pub extern "C" fn docx_preview_last_error() -> PreviewBuffer {
    // 返回线程局部错误字符串
}
```

但更简单的方式是让主函数返回 JSON：

```json
{
  "ok": false,
  "error": {
    "code": "missing_document_xml",
    "message": "missing word/document.xml"
  }
}
```

## 19. 性能和体积优化

`Cargo.toml`：

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
strip = "symbols"
panic = "abort"
```

WASM 体积优化：

```toml
[profile.release.package.docx_preview_wasm]
opt-level = "z"
```

可选工具：

```bash
cargo install wasm-opt
```

或者用 Binaryen 的 `wasm-opt`：

```bash
wasm-opt -Oz input.wasm -o output.wasm
```

性能建议：

- docx 解析尽量流式处理，避免一次复制多份 XML。
- 图片资源单独输出 id，不要全部 base64 塞进 JSON。
- 字体测量和分页是难点，需要平台字体差异策略。
- WASM 大文件处理放 worker。
- 移动端使用后台线程。

## 20. 字体和排版的跨端难点

docx 预览内核最难的不是打包，而是“不同平台渲染一致”。

难点：

- 同名字体在 Windows、macOS、Android、iOS 上不一定存在。
- 字体 fallback 不同会导致换行不同。
- 表格自动布局很复杂。
- 图片锚定、浮动、环绕、页眉页脚、脚注都会影响分页。
- Word 的布局规则有大量历史兼容行为。

建议策略：

1. 内核输出“逻辑布局模型”，首版不要追求 100% Word 像素级一致。
2. 字体测量抽象成 trait：

```rust
pub trait FontMetrics {
    fn measure_text(&self, text: &str, font_family: &str, font_size: f32) -> TextSize;
}
```

3. Web 用 Canvas / browser API 测量。
4. iOS 用 CoreText 测量。
5. Android 用 Paint / StaticLayout 测量。
6. 桌面可以用 cosmic-text、skia、fontdue、swash 等 Rust 侧字体方案。

如果你希望 Rust 内核完全负责布局，就要把字体文件也纳入内核管理，保证所有平台用同一套字体和测量逻辑。

## 21. 两种架构选择

### 21.1 Rust 只解析，不负责最终排版

Rust 输出结构化内容：段落、表格、图片、样式。

优点：

- 简单，跨端容易。
- UI 可由平台原生控件渲染。
- 初期开发快。

缺点：

- 各端显示不完全一致。
- 分页和复杂布局难统一。

适合：快速做预览、办公系统、移动端轻量展示。

### 21.2 Rust 解析 + 排版，平台只绘制

Rust 输出页面坐标和绘制指令。

优点：

- 多端一致性更好。
- 可以做缩放、翻页、搜索、高亮。

缺点：

- 字体测量、分页、表格布局难度大。
- 性能和内存压力更高。

适合：专业文档预览器、批注、精确分页、打印。

### 21.3 Rust 解析 + 排版 + 光栅化

Rust 直接输出图片或 tile。

优点：

- 各端显示最一致。
- 平台 UI 最简单。

缺点：

- 体积大，交互弱。
- 文本选择、搜索、可访问性更难。
- 移动端和 Web 内存压力大。

适合：只读预览、缩略图、服务端渲染。

## 22. 建议你采用的路线

如果你要做一个“docx 预览内核，多端可用”，建议路线：

第一阶段：核心解析 + JSON 模型。

```text
docx bytes -> Rust parse -> preview JSON -> 各端自己渲染
```

第二阶段：加入布局模型。

```text
docx bytes -> Rust parse -> Rust layout -> page model -> 各端 canvas/native draw
```

第三阶段：做平台专用优化。

```text
Web: WASM + Worker + Canvas
Android: .aar + Kotlin wrapper + background thread
iOS: XCFramework + Swift wrapper + background queue
Desktop: Tauri direct Rust dependency or native dynamic library
Server: Linux CLI / service
```

第四阶段：发布 SDK。

```text
npm package
Swift Package / XCFramework
Android AAR
Windows DLL package
macOS XCFramework
Linux .so / CLI
```

## 23. 最小命令清单

WASM：

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build crates/docx_preview_wasm --target web --release --out-dir ../../dist/wasm/web
```

iOS：

```bash
rustup target add aarch64-apple-ios aarch64-apple-ios-sim
cargo build -p docx_preview_ffi --release --target aarch64-apple-ios
cargo build -p docx_preview_ffi --release --target aarch64-apple-ios-sim
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libdocx_preview_ffi.a \
  -headers bindings/c \
  -library target/aarch64-apple-ios-sim/release/libdocx_preview_ffi.a \
  -headers bindings/c \
  -output dist/ios/DocxPreview.xcframework
```

Android：

```bash
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android
cargo install cargo-ndk
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -o dist/android/jniLibs build -p docx_preview_ffi --release
```

Windows：

```powershell
rustup target add x86_64-pc-windows-msvc
cargo build -p docx_preview_ffi --release --target x86_64-pc-windows-msvc
```

macOS：

```bash
rustup target add x86_64-apple-darwin aarch64-apple-darwin
cargo build -p docx_preview_ffi --release --target x86_64-apple-darwin
cargo build -p docx_preview_ffi --release --target aarch64-apple-darwin
```

Linux：

```bash
rustup target add x86_64-unknown-linux-gnu x86_64-unknown-linux-musl
cargo build -p docx_preview_ffi --release --target x86_64-unknown-linux-gnu
cargo build -p docx_preview_cli --release --target x86_64-unknown-linux-musl
```

## 24. 常见坑

### 24.1 为什么不用一个 crate 同时做 FFI 和 WASM？

可以，但不推荐。WASM 的导出方式和 native FFI 完全不同，依赖也不同。分开 crate 更清楚。

### 24.2 iOS 为什么常用 staticlib？

iOS 对动态库加载和 App 分发有自己的限制。Rust 内核嵌入 App 时，静态库或 framework/xcframework 更顺。

### 24.3 Android 为什么要多个 `.so`？

不同 CPU ABI 需要不同机器码。`arm64-v8a` 的 `.so` 不能给 `x86_64` 模拟器用。

### 24.4 WASM 能不能直接复用 FFI C ABI？

理论上可以走低级 wasm ABI，但实际 Web 项目推荐 `wasm-bindgen`，它会生成 JS/TS 绑定，处理字符串、数组、错误更舒服。

### 24.5 JSON 会不会太慢？

首版用 JSON 最稳。等模型稳定后，再换二进制协议。不要过早优化边界协议。

### 24.6 Rust 内核是否能保证 Word 级别兼容？

能做，但工作量很大。docx 预览难点在排版兼容，不在 Rust 打包。建议先定义可接受的兼容目标，比如：正文、标题、图片、表格、分页，再逐步补复杂特性。

## 25. 参考资料

- Rust Reference: Linkage
  `https://doc.rust-lang.org/reference/linkage.html`
- rustc book: Platform Support
  `https://doc.rust-lang.org/rustc/platform-support.html`
- Rustonomicon: FFI
  `https://doc.rust-lang.org/nomicon/ffi.html`
- wasm-bindgen Guide
  `https://rustwasm.github.io/docs/wasm-bindgen/`
- wasm-pack build
  `https://rustwasm.github.io/docs/wasm-pack/commands/build.html`
- Android NDK: Android ABIs
  `https://developer.android.com/ndk/guides/abis`
- cargo-ndk
  `https://github.com/bbqsrc/cargo-ndk`
- UniFFI User Guide
  `https://mozilla.github.io/uniffi-rs/latest/`