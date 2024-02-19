# Rust 编程语言

## 国内安装

### 环境设置

字节跳动基础架构 Dev Infra 团队提供的：[rsproxy](https://rsproxy.cn/) 公益镜像，通过该镜像可以快速搭建用来安装和配置 Rust 的环境。

```shell
export RUSTUP_DIST_SERVER="https://rsproxy.cn"
export RUSTUP_UPDATE_ROOT="https://rsproxy.cn/rustup"
```

### 安装 Rust

```shell
curl --proto '=https' --tlsv1.2 -sSf https://rsproxy.cn/rustup-init.sh | sh
```

### 国内源配置

设置 crates.io 镜像， 修改配置 `~/.cargo/config`，已支持 git 协议和 sparse 协议，>=1.68 版本建议使用 sparse-index，速度更快。

```shell
[source.crates-io]
replace-with = 'rsproxy-sparse'
[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"
[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"
[net]
git-fetch-with-cli = true
```

## 运行

```shell
cargo new demo  # 创建新项目
cargo run --release  # 以优化后的构建物运行
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
