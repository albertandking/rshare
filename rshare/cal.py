import requests
from bs4 import BeautifulSoup


def fetch_name(url: str) -> str:
    r = requests.get(url)
    r.encoding = "utf-8"
    soup = BeautifulSoup(r.text, features="html.parser")
    title_str = soup.find("title").text
    return title_str


if __name__ == '__main__':
    title = fetch_name("https://www.baidu.com")
    print(title)
