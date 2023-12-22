# Rust 编程语言

## 安装

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 运行

```shell
cargo new demo
cargo run --release
```

## 案例

```rust
fn main() {
    println!("Hello World");
}
```

```rust
use rshare::fetch_title; // 假设您的库名为 rshare

fn main() {
    let url = "https://example.com";
    let title = fetch_title(url).unwrap();
    println!("Fetched title: {}", title);
}
```
