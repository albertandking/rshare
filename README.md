## 项目介绍

这是一个 Python 和 Rust 混合编程的项目，主要目的是比较 Rust 和 Python 的性能差异。

## 说明

目前已经支持 64 位操作系统，包括 Windows，Linux，macOS 和 ARM64 架构的 Linux 系统。

## 安装

通过 `pip install rshare --upgrade` 来安装 `rshare` 体验 Rust 的极速体验！

## 使用

```python
import rshare as rk

# Rust 接口
fetch_title_str = rk.fetch_title(url="https://www.baidu.com")
print(fetch_title_str)

# Python 接口
fetch_name_str = rk.fetch_name(url="https://www.baidu.com")
print(fetch_name_str)
```

## 版本说明

1. Python 3.11
2. Rust 1.74.1

## 注意事项

需要重新编译部分内容

## 更新

1. 推送到 main
2. 修改 `Cargo.toml` 中的版本号
3. 打标签：`git tag v0.1.7`; 
4. 通过推送标签进行升级：`git push origin v0.1.7`

## 镜像推送

1. `docker build -t jindaxiang/newopen .`
2. `docker tag local-image:tagname jindaxiang/new-repo:tagname`
3. `docker push jindaxiang/new-repo:tagname`

## 查看

1. PyPI 版本：https://pypi.org/search/?q=rshare
