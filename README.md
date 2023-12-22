# 项目介绍

这是一个 Python 和 Rust 混合编程的项目，主要目的是比较 Rust 和 Python 的性能差异。目前这个项目是 [rshare](https://github.com/albertandking/rshare)，我们
在项目中通过 PYO3 和 Maturin 来将 Rust 代码包装到 Python 中，以使得用户可以通过 Python 代码来调用 Rust 写的函数。当然其中是有性能损失的。

## 说明

目前已经支持 64 位操作系统，包括 Windows，Linux，macOS 和 ARM64 架构的 Linux 系统。

## 安装

通过 `pip install rshare --upgrade -i https://pypi.org/simple` 来安装 `rshare` 体验 Rust 的极致性能！

## 使用

```python
import time
import talib
import rshare as rk

# Rust 和 C 对比
start_py = time.time()
result_talib = talib.SMA(data_np, timeperiod)
end_py = time.time()
print(f"Talib took: {end_py - start_py} seconds")

start_py = time.time()
result_np_rs = rk.calculate_moving_average_rs(data_np, timeperiod)
end_py = time.time()
print(f"RSNP took: {end_py - start_py} seconds")
```

## 构建

```shell
maturin build

maturin develop --release
```

## 版本说明

目前主要是说明开发的软件版本：

1. Python 3.11
2. Rust 1.74.1

## 注意事项

需要重新编译部分内容

## 更新

1. 推送到 main 分支
2. 修改 `Cargo.toml` 中的版本号
3. 打标签：`git tag v0.1.7`
4. 通过推送标签进行升级：`git push origin tag v0.1.7`

## 镜像推送

1. 基于原始镜像：`ghcr.io/rust-cross/manylinux2014-cross:aarch64`
2. `docker build -t jindaxiang/newopen .`
3. `docker tag local-image:tagname jindaxiang/new-repo:tagname`
4. `docker push jindaxiang/new-repo:tagname`

## 查看

1. [PyPI 版本 RShare 版本](https://pypi.org/search/?q=rshare)

## 贡献代码

1. [maturin 项目](https://github.com/PyO3/maturin)
2. [maturin 文档](https://www.maturin.rs)
3. [PyO3 项目](https://github.com/PyO3/pyo3)
4. [PyO3 文档](https://pyo3.rs)

## 关注 .vscode

用于 debug 代码，新增 .vscode 文件夹
