import requests
from bs4 import BeautifulSoup
from typing import List
from .rshare import get_ak_version


# 调用 rshare 中的 akversion 函数
def pyakversion(name: str = "Albert") -> str:
    return "Python 函数 " + get_ak_version(name=name)


def calculate_moving_average_py(data: List[float], window_size: int) -> List[float]:
    if window_size > len(data):
        raise ValueError("Window size larger than the list size")

    moving_averages = []
    for i in range(len(data) - window_size + 1):
        window = data[i : i + window_size]
        window_average = sum(window) / window_size
        moving_averages.append(window_average)

    return moving_averages


def fetch_name(url: str) -> str:
    r = requests.get(url)
    r.encoding = "utf-8"
    soup = BeautifulSoup(r.text, features="html.parser")
    title_str = soup.find("title").text
    return title_str


if __name__ == "__main__":
    title = fetch_name("https://www.baidu.com")
    print(title)

    stock_prices = [10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]
    averages = calculate_moving_average_py(stock_prices, 5)
    print(averages)
