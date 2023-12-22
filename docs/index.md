# 欢迎来到 RShare 文档

这是一个 Python 和 Rust 混合编程的项目，主要目的是比较 Rust 和 Python 的性能差异。目前这个项目是 [rshare](https://github.com/albertandking/rshare)，我们
在项目中通过 PYO3 和 Maturin 来将 Rust 代码包装到 Python 中，以使得用户可以通过 Python 代码来调用 Rust 写的函数。当然其中是有性能损失的。

## 说明

目前已经支持 64 位操作系统，包括 Windows，Linux，macOS 和 ARM64 架构的 Linux 系统。

## 安装

通过 `pip install rshare --upgrade -i https://pypi.org/simple` 来安装 `rshare` 体验 Rust 的极致性能！

## 使用

```python
import rshare as rk

# Rust 接口
fetch_title_str = rk.fetch_title(url="https://www.baidu.com")
print(fetch_title_str)

# Python 接口
fetch_name_str = rk.fetch_name(url="https://www.baidu.com")
print(fetch_name_str)

# Rust 接口测试计算 5 日均线
result_rs = rk.calculate_moving_average_rs(data=[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] * 10000000, window_size=5)
print(result_rs)
```
