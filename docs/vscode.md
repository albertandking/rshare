# VSCode 平台配置

主要记录如何在 Windows 平台中的 VSCode 中配置基于 Rust 和 Python 混合编程的环境，该环境有利于
Rust 和 Python 的编程和调试，因为其中 Rust 还依赖 Python 的虚拟环境。

1. Miniconda 管理 Python
2. 直接安装 Rust

## Windows 平台配置

需要在根目录 `.vscode` 文件夹下配置 `settings.json` 和 `launch.json` 两个文件。注意需要修改其中的路径及相关命令以使用不同的环境

其中 `settings.json` 的内容为：

```json
{
  "rust-analyzer.server.extraEnv": {
    "PYO3_PYTHON": "C:\\Users\\albert\\.conda\\envs\\pyo\\python.exe"
  },
  "terminal.integrated.profiles.windows": {
    "Conda Powershell": {
      "path": "powershell.exe",
      "args": [
        "-NoExit",
        "-Command",
        "& cmd.exe /c 'C:\\ProgramData\\Miniconda3\\Scripts\\activate.bat C:\\Users\\albert\\.conda\\envs\\pyo && powershell'"
      ]
    }
  },
  "terminal.integrated.defaultProfile.windows": "Conda Powershell"
}
```

其中 `launch.json` 的内容为：

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Python: Current File",
      "type": "python",
      "request": "launch",
      "program": "${file}",
      "console": "integratedTerminal",
      "justMyCode": true
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug main.rs",
      "cargo": {
        "args": ["build", "--bin=rshare_bin", "--package=rshare"],
        "filter": {
          "name": "rshare_bin",
          "kind": "bin"
        },
        "env": {
          "PYO3_PYTHON": "C:\\Users\\albert\\.conda\\envs\\pyo\\python.exe"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug unit tests in library 'rshare'",
      "cargo": {
        "args": ["test", "--no-run", "--lib", "--package=rshare"],
        "filter": {
          "name": "rshare",
          "kind": "lib"
        },
        "env": {
          "PYO3_PYTHON": "C:\\Users\\albert\\.conda\\envs\\pyo\\python.exe"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

## macOS 平台

需要在 macOS 中配置，主要设置
