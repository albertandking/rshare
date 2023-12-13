## 项目介绍

这是一个 Python 和 Rust 混合编程的项目

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
