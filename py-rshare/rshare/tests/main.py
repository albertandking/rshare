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
data_num = 1000
data_np = np.random.rand(data_num)

# data_np = np.array([1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0] * 1000000)
timeperiod = 5

# Rust 实现的时间测试
start_rs = time.perf_counter()
result_rs = rk.calculate_moving_average_rs(data=data_np, window_size=timeperiod)
end_rs = time.perf_counter()
print(f"Rust  实现: {end_rs - start_rs} seconds")
# 基 Rust 实现: 0.00680940 seconds
# 纯 Rust 实现: 0.00518641 seconds

# Rust In 实现的时间测试
start_rs = time.perf_counter()
result_rs = rk.calculate_moving_average_in_rs(data=data_np, window_size=timeperiod)
end_rs = time.perf_counter()
print(f"Rust In 实现: {end_rs - start_rs} seconds")

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

temp_list = []
for i in range(5):
    data_num = 100000000
    data_np = np.random.rand(data_num)
    timeperiod = 50

    # Rust 和 C 对比
    start_py = time.perf_counter_ns()
    result_talib = talib.SMA(data_np, timeperiod)
    end_py = time.perf_counter_ns()
    c_time = end_py - start_py
    print(f"基于 talib  的耗时: {c_time} seconds")

    start_py = time.perf_counter_ns()
    data_np = np.random.rand(data_num)
    result_np_rs = rk.calculate_moving_average_rs(data=data_np, window_size=timeperiod)
    end_py = time.perf_counter_ns()
    r_time = end_py - start_py
    print(f"基于 rshare 的耗时: {r_time} seconds")
    temp_list.append(c_time / r_time)
    # print(f"Rust 优化比例: {c_time / r_time}")

print(f"Rust 优化比例: {np.mean(temp_list)}")

# 校验 Rust 和 Talib 实现的结果是否一致
if result_talib[9999].round(6) == result_np_rs[9999].round(6):
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
test_fetch_function(rk.fetch_name, address, 20)

# 对 fetch_title 函数进行测试
test_fetch_function(rk.get_title, address, 20)
