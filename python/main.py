import time

import rshare as rk

data_num = 1000000

start_rs = time.time()
result_rs = rk.calculate_moving_average_rs(
    data=[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] * data_num, 
    window_size=5)
end_rs = time.time()
print(f"Rust implementation took: {end_rs - start_rs} seconds")
# Rust implementation took: 215.82389068603516 seconds

start_py = time.time()
result_py = rk.calculate_moving_average_py(
    data=[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] * data_num, 
    window_size=5)
end_py = time.time()
print(f"Python implementation took: {end_py - start_py} seconds")
# Python implementation took: 187.05668830871582 seconds

start_py = time.time()
result_py = rk.calculate_moving_average_talib_rs(
    data=[10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0] * data_num, 
    window_size=5)
end_py = time.time()
print(f"Rust Talib took: {end_py - start_py} seconds")



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
    print(f"Function {func.__name__} executed {times} times, \
          total time taken: {total_time:.2f} seconds")


address = "https://www.eastmoney.com/"

# 测试 fetch_name 函数
test_fetch_function(rk.fetch_name, address, 20)

# 测试 fetch_title 函数
test_fetch_function(rk.fetch_title, address, 20)
