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
    // 为 Rust Analyzer 语言服务器指定环境变量，影响语言服务器进程
    "rust-analyzer.server.extraEnv": {
        // 设定 PYO3_PYTHON 环境变量，指定 Rust 中使用 PyO3 时的 Python 解释器路径
        "PYO3_PYTHON": "C:\\Users\\albert\\.conda\\envs\\pyo\\python.exe"
    },
    // 为通过 Rust Analyzer 运行或调试的可执行文件指定额外的环境变量
    "rust-analyzer.runnables.extraEnv": {
        // 设定 PYO3_PYTHON 环境变量，用于运行或调试时确定 Python 解释器路径
        "PYO3_PYTHON": "C:\\Users\\albert\\.conda\\envs\\pyo\\python.exe"
    },
    // 定义 VS Code 集成终端中的自定义终端配置
    "terminal.integrated.profiles.windows": {
        // 创建一个名为 "Conda Powershell" 的终端配置
        "Conda Powershell": {
            "path": "powershell.exe",
            // 配置参数以启动并激活指定的 Conda 环境，然后继续使用 PowerShell
            "args": ["-NoExit", "-Command", "& cmd.exe /c 'C:\\ProgramData\\Miniconda3\\Scripts\\activate.bat C:\\Users\\albert\\.conda\\envs\\pyo && powershell'"]
        }
    },
    // 设置 "Conda Powershell" 为集成终端的默认配置
    "terminal.integrated.defaultProfile.windows": "Conda Powershell"
}
```

其中 `launch.json` 的内容为：

```json
{
  "version": "0.2.0", // 配置文件的版本，确保兼容性
  "configurations": [
    {
      // 调试Python代码的配置
      "name": "Python: Current File", // 配置的显示名称
      "type": "python", // 指定调试器类型，这里是Python
      "request": "launch", // 请求类型，这里是启动调试会话
      "program": "${file}", // 要调试的当前文件的路径
      "console": "integratedTerminal", // 在集成终端中启动调试
      "justMyCode": true // 只调试用户的代码，不调试库代码
    },
    {
      // 调试Rust可执行文件main.rs的配置
      "type": "lldb", // 使用LLDB调试器
      "request": "launch", // 启动调试会话
      "name": "Debug main.rs", // 配置的显示名称
      "cargo": {
        "args": ["build", "--bin=rshare_bin", "--package=rshare"], // Cargo的构建参数
        "filter": {
          "name": "rshare_bin", // 要构建的二进制文件的名称
          "kind": "bin" // 指定构建的类型是二进制文件
        },
        "env": {
          "PYO3_PYTHON": "C:\\Users\\albert\\.conda\\envs\\pyo\\python.exe" // 设置环境变量，指定PyO3使用的Python解释器路径
        }
      },
      "args": [], // 启动调试会话时传递给程序的参数
      "cwd": "${workspaceFolder}" // 调试会话的当前工作目录
    },
    {
      // 调试Rust库lib.rs中的单元测试的配置
      "type": "lldb", // 使用LLDB调试器
      "request": "launch", // 启动调试会话
      "name": "Debug unit tests in library 'rshare'", // 配置的显示名称
      "cargo": {
        "args": ["test", "--no-run", "--lib", "--package=rshare"], // Cargo的测试参数，--no-run表示编译测试但不运行
        "filter": {
          "name": "rshare", // 指定库的名称
          "kind": "lib" // 指定构建的类型是库
        },
        "env": {
          "PYO3_PYTHON": "C:\\Users\\albert\\.conda\\envs\\pyo\\python.exe" // 设置环境变量，指定PyO3使用的Python解释器路径
        }
      },
      "args": [], // 启动调试会话时传递给程序的参数
      "cwd": "${workspaceFolder}" // 调试会话的当前工作目录
    }
  ]
}
```

## macOS 平台

需要在 macOS 中配置，主要设置
