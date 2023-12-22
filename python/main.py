import time
import talib
import rshare as rk
import numpy as np
from numba import njit

@njit
def cmma(bar_data, lookback):
    # 初始化结果数组
    n = len(bar_data)
    out = np.array([np.nan for _ in range(n)])
    # 从lookback开始遍历所有数据点
    for i in range(lookback, n):
        # 计算lookback期的移动平均
        ma = 0
        for j in range(i - lookback, i):
            ma += bar_data[j]
        ma /= lookback
        out[i] = ma
        # 从当前值减去移动平均值
    return out


# 生成随机数据
data_num = 1000000
data_np = np.random.rand(data_num)
timeperiod = 50

# Rust 实现的时间测试
start_rs = time.perf_counter()
result_rs = rk.calculate_moving_average_rs(
    data=data_np, window_size=timeperiod
)
end_rs = time.perf_counter()
print(f"Rust 实现: {end_rs - start_rs} seconds")

# Talib 实现的时间测试
start_py = time.perf_counter()
result_talib = talib.SMA(data_np, timeperiod=timeperiod)
end_py = time.perf_counter()
print(f"Talib 实现: {end_py - start_py} seconds")

# Numba 实现的时间测试
start_py = time.perf_counter()
result_numba = cmma(bar_data=data_np, lookback=timeperiod)
end_py = time.perf_counter()
print(f"Numba 实现: {end_py - start_py} seconds")

# 纯 Python 实现的时间测试
start_py = time.perf_counter()
result_py = rk.calculate_moving_average_py(data=data_np, window_size=timeperiod)
end_py = time.perf_counter()
print(f"纯 Python 实现: {end_py - start_py} seconds")

# Rust 和 C 实现的性能对比
start_py = time.perf_counter()
result_talib = talib.SMA(data_np, timeperiod)
end_py = time.perf_counter()
print(f"Talib took: {end_py - start_py} seconds")

start_py = time.perf_counter()
result_np_rs = rk.calculate_moving_average_rs(data_np, timeperiod)
end_py = time.perf_counter()
print(f"RSNP took: {end_py - start_py} seconds")

# 校验 Rust 和 Talib 实现的结果是否一致
if result_talib[99999].round(6) == result_np_rs[99999].round(6):
    print("equal")
else:
    print("not equal")


# 定义测试函数
def test_fetch_function(func, url, times):
    """
    测试给定函数的性能
    :param func: 要测试的函数
    :param url: 用于函数的url
    :param times: 测试次数
    :return: 无返回值
    """
    start_time = time.perf_counter()  # 记录开始时间
    for _ in range(times):
        result = func(url=url)
        print(result)
    end_time = time.perf_counter()  # 记录结束时间
    total_time = end_time - start_time
    print(
        f"Function {func.__name__} executed {times} times, \
          total time taken: {total_time:.2f} seconds"
    )


address = "https://www.eastmoney.com/"

# 对 fetch_name 函数进行测试
test_fetch_function(rk.fetch_name, address, 10)

# 对 fetch_title 函数进行测试
test_fetch_function(rk.fetch_title, address, 10)