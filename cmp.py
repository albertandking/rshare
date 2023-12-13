import time

import rshare as rk


# 定义测试函数
def test_fetch_function(func, url, times):
    start_time = time.time()  # 开始时间
    for _ in range(times):
        result = func(url=url)
        print(result)
    end_time = time.time()  # 结束时间
    total_time = end_time - start_time
    print(f"Function {func.__name__} executed {times} times, total time taken: {total_time:.2f} seconds")


address = "https://www.eastmoney.com/"

# 测试 fetch_name 函数
test_fetch_function(rk.fetch_name, address, 20)

# 测试 fetch_title 函数
test_fetch_function(rk.fetch_title, address, 20)
