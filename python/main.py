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
