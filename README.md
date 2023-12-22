# 项目介绍

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
import time
import talib
import rshare as rk
import numpy as np
from numba import njit

@njit
def cmma(bar_data, lookback):    
    # Initialize the result array.
    n = len(bar_data)
    out = np.array([np.nan for _ in range(n)])
    # For all bars starting at lookback:
    for i in range(lookback, n):
        # Calculate the moving average for the lookback.
        ma = 0
        for j in range(i - lookback, i):
            ma += bar_data[j]
        ma /= lookback
        out[i] = ma
        # Subtract the moving average from value.
    return out


# 生成数据
data_num = 10000000
data_np = np.random.rand(data_num)
timeperiod = 50

# Rust 实现
start_rs = time.time()
result_rs = rk.calculate_moving_average_rs(
    data=data_np, window_size=timeperiod
)
end_rs = time.time()
print(f"Rust 实现: {end_rs - start_rs} seconds")

# Talib 实现
start_py = time.time()
result_talib = talib.SMA(data_np, timeperiod=timeperiod)
end_py = time.time()
print(f"Talib 实现: {end_py - start_py} seconds")

# Numba 实现
start_py = time.time()
result_numba = cmma(bar_data=data_np, lookback=timeperiod)
end_py = time.time()
print(f"Numba 实现: {end_py - start_py} seconds")

# 纯 Python 实现
start_py = time.time()
result_py = rk.calculate_moving_average_py(data=data_np, window_size=timeperiod)
end_py = time.time()
print(f"纯 Python 实现: {end_py - start_py} seconds")
# print(f"纯 Python 实现: 14 seconds")

# Rust 和 C 对比

start_py = time.time()
result_talib = talib.SMA(data_np, timeperiod)
end_py = time.time()
print(f"Talib took: {end_py - start_py} seconds")

start_py = time.time()
result_np_rs = rk.calculate_moving_average_rs(data_np, timeperiod)
end_py = time.time()
print(f"RSNP took: {end_py - start_py} seconds")


if result_talib[9999999].round(6) == result_np_rs[9999999].round(6):
    print("equal")
else:
    print("not equal")


# 定义测试函数
def test_fetch_function(func, url, times):
    """
    测试函数
    :param func: 函数
    :param url: url
    :param times: 测试次数
    :return: None
    """
    start_time = time.time()  # 开始时间
    for _ in range(times):
        result = func(url=url)
        print(result)
    end_time = time.time()  # 结束时间
    total_time = end_time - start_time
    print(
        f"Function {func.__name__} executed {times} times, \
          total time taken: {total_time:.2f} seconds"
    )


address = "https://www.eastmoney.com/"

# 测试 fetch_name 函数
test_fetch_function(rk.fetch_name, address, 20)

# 测试 fetch_title 函数
test_fetch_function(rk.fetch_title, address, 20)
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
